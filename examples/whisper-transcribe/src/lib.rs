use std::io::Cursor;
use std::path::{Path, PathBuf};
use std::process::Stdio;
use std::time::Duration;

use pocketstation::{
    AsyncNode, AsyncNodeFuture, AsyncOperatorFactory, AsyncOperatorManifest,
    AsyncOperatorPrepareContext, AudioCaps, ChannelLayout, ConfigError, CopyPolicy, EdgeContract,
    ExecutionPartition, MediaCaps, Multiplicity, NodeDescriptor, NodeError, NodeTypeId,
    OperatorCancellationPolicy, OperatorConfiguration, OperatorDeadlinePolicy,
    OperatorFailurePolicy, OperatorId, OperatorOutputRolePolicy, OperatorPermissionPolicy,
    PortDirection, PortSpec, SafetyContract, SampleFormat, SemanticRole, SignalDerivation,
    SignalEnvelope, SignalLineage, SignalPayload, SignalSpec, SignalTiming, SourceId, TextFormat,
};
use tempfile::TempDir;
use tokio::process::{Child, Command};

mod process_evidence;

use process_evidence::ActiveProcessEvidence;
pub use process_evidence::{
    ProcessEvidenceError, ProcessOutcome, WhisperProcessEvidence, WhisperProcessReceipt,
};

pub struct WhisperConnector {
    binary_path: PathBuf,
    model_path: PathBuf,
    language: String,
    use_gpu: bool,
    process_timeout_ms: u32,
    window_duration_ms: u32,
    sample_rate_hz: Option<u32>,
    channels: Option<u8>,
    pending_samples: Vec<f32>,
    pending_origin: Option<WindowOrigin>,
    stream_origin: Option<WindowOrigin>,
    completed_transcripts: Vec<String>,
    active_child: Option<Child>,
    process_evidence: Option<WhisperProcessEvidence>,
    active_process_evidence: Option<ActiveProcessEvidence>,
    prepared: bool,
}

#[derive(Clone, Copy)]
struct WindowOrigin {
    timestamp_ns: u64,
    source_id: Option<SourceId>,
    lineage: Option<SignalLineage>,
    timing: SignalTiming,
    last_sequence_number: u64,
    timestamp_end_ns: u64,
}

pub const WHISPER_OPERATOR_ID: &str = "community.whisper.cpp.stt.v1";
pub const TRANSCRIPT_PARTIAL_ROLE: &str = "transcript.partial";
pub const TRANSCRIPT_FINAL_ROLE: &str = "transcript.final";

pub fn transcript_partial_spec() -> SignalSpec {
    SignalSpec::text(TextFormat::Utf8).with_role(TRANSCRIPT_PARTIAL_ROLE)
}

pub fn transcript_final_spec() -> SignalSpec {
    SignalSpec::text(TextFormat::Utf8).with_role(TRANSCRIPT_FINAL_ROLE)
}
const WHISPER_OPERATOR_REVISION: u32 = 1;
const WHISPER_OPERATOR_GENERATION: u32 = 1;

const LANGUAGE_CONFIGURATION_KEY: &str = "language";
const GPU_CONFIGURATION_KEY: &str = "gpu";
const WINDOW_DURATION_MS_CONFIGURATION_KEY: &str = "window_duration_ms";
const PROCESS_TIMEOUT_MS_CONFIGURATION_KEY: &str = "process_timeout_ms";
const DEFAULT_PROCESS_TIMEOUT_MS: u32 = 60_000;
const PROCESS_TIMEOUT_CLEANUP_MARGIN_MS: u32 = 250;
pub const EVIDENCE_ROOT_CONFIGURATION_KEY: &str = "evidence_root";
pub const EVIDENCE_CASE_ID_CONFIGURATION_KEY: &str = "evidence_case_id";

pub struct WhisperOperatorFactory {
    binary_path: PathBuf,
    model_path: PathBuf,
    default_language: String,
    manifest: AsyncOperatorManifest,
}

impl WhisperOperatorFactory {
    pub fn new(
        binary_path: impl Into<PathBuf>,
        model_path: impl Into<PathBuf>,
        default_language: impl Into<String>,
    ) -> Self {
        let audio = MediaCaps::Audio(AudioCaps {
            sample_rate_hz: None,
            frame_samples: None,
            channel_layout: ChannelLayout::Any,
            format: SampleFormat::F32Interleaved,
        });
        let input_edge = EdgeContract::realtime_audio()
            .with_media(audio)
            .with_copy_policy(CopyPolicy::CopyToBranchPool);
        let output_edge = EdgeContract::bounded_async().with_media(MediaCaps::Text);
        let node = NodeDescriptor::new(
            NodeTypeId::from("operator.transcription.whisper-cpp"),
            "Whisper.cpp transcription",
            vec![PortSpec::new(
                "audio",
                PortDirection::Input,
                SignalSpec::audio(),
                audio,
                Multiplicity::One,
                true,
            )
            .expect("audio input port")],
            vec![PortSpec::new(
                "transcript",
                PortDirection::Output,
                SignalSpec::text(TextFormat::Utf8).with_role("transcript"),
                MediaCaps::Text,
                Multiplicity::One,
                true,
            )
            .expect("text output port")],
            ExecutionPartition::BlockingWorker,
            SafetyContract::BlockingAllowed,
            true,
        )
        .expect("operator node descriptor");
        let manifest = AsyncOperatorManifest::new(
            OperatorId::new(WHISPER_OPERATOR_ID),
            WHISPER_OPERATOR_REVISION,
            WHISPER_OPERATOR_GENERATION,
            node,
            input_edge,
            output_edge,
            32,
            OperatorPermissionPolicy {
                network_allowed: false,
                filesystem_allowed: true,
            },
            OperatorDeadlinePolicy {
                process_timeout_ms: DEFAULT_PROCESS_TIMEOUT_MS
                    .saturating_add(PROCESS_TIMEOUT_CLEANUP_MARGIN_MS),
            },
            OperatorCancellationPolicy::DiscardQueued,
            OperatorFailurePolicy::StopWorker,
            OperatorOutputRolePolicy {
                allowed: vec![
                    SemanticRole::new(TRANSCRIPT_PARTIAL_ROLE),
                    SemanticRole::new(TRANSCRIPT_FINAL_ROLE),
                ],
                terminal: vec![SemanticRole::new(TRANSCRIPT_FINAL_ROLE)],
            },
        )
        .expect("valid operator manifest");
        Self {
            binary_path: binary_path.into(),
            model_path: model_path.into(),
            default_language: default_language.into(),
            manifest,
        }
    }
}

impl AsyncOperatorFactory for WhisperOperatorFactory {
    fn manifest(&self) -> &AsyncOperatorManifest {
        &self.manifest
    }

    fn validate_config(&self, configuration: &OperatorConfiguration) -> Result<(), ConfigError> {
        if configuration
            .get(LANGUAGE_CONFIGURATION_KEY)
            .is_some_and(|language| language.trim().is_empty())
        {
            return Err(ConfigError::Invalid {
                key: LANGUAGE_CONFIGURATION_KEY.to_owned(),
                reason: "language cannot be empty".to_owned(),
            });
        }
        validate_positive_u32(configuration, WINDOW_DURATION_MS_CONFIGURATION_KEY)?;
        validate_positive_u32(configuration, PROCESS_TIMEOUT_MS_CONFIGURATION_KEY)?;
        if configuration
            .get(GPU_CONFIGURATION_KEY)
            .is_some_and(|gpu| !matches!(gpu, "true" | "false"))
        {
            return Err(ConfigError::Invalid {
                key: GPU_CONFIGURATION_KEY.to_owned(),
                reason: "expected true or false".to_owned(),
            });
        }
        match (
            configuration.get(EVIDENCE_ROOT_CONFIGURATION_KEY),
            configuration.get(EVIDENCE_CASE_ID_CONFIGURATION_KEY),
        ) {
            (None, None) => {}
            (Some(root), Some(case_id)) if !root.trim().is_empty() => {
                WhisperProcessEvidence::new(root, case_id).map_err(|error| {
                    ConfigError::Invalid {
                        key: EVIDENCE_CASE_ID_CONFIGURATION_KEY.to_owned(),
                        reason: error.to_string(),
                    }
                })?;
            }
            _ => {
                return Err(ConfigError::Invalid {
                    key: EVIDENCE_ROOT_CONFIGURATION_KEY.to_owned(),
                    reason: "evidence_root and evidence_case_id must be supplied together"
                        .to_owned(),
                });
            }
        }
        Ok(())
    }

    fn resolve_manifest(
        &self,
        configuration: &OperatorConfiguration,
    ) -> Result<AsyncOperatorManifest, ConfigError> {
        self.validate_config(configuration)?;
        let provider_timeout_ms = configuration
            .get_u32(PROCESS_TIMEOUT_MS_CONFIGURATION_KEY)
            .unwrap_or(DEFAULT_PROCESS_TIMEOUT_MS);
        AsyncOperatorManifest::new(
            self.manifest.operator_id().clone(),
            self.manifest.revision(),
            self.manifest.generation(),
            self.manifest.node().clone(),
            self.manifest.input_edge(),
            self.manifest.output_edge(),
            self.manifest.queue_capacity_frames(),
            self.manifest.permission(),
            OperatorDeadlinePolicy {
                process_timeout_ms: provider_timeout_ms
                    .saturating_add(PROCESS_TIMEOUT_CLEANUP_MARGIN_MS),
            },
            self.manifest.cancellation(),
            self.manifest.failure(),
            self.manifest.output_roles().clone(),
        )
        .map_err(|error| ConfigError::Invalid {
            key: PROCESS_TIMEOUT_MS_CONFIGURATION_KEY.to_owned(),
            reason: error.to_string(),
        })
    }

    fn create(
        &self,
        configuration: &OperatorConfiguration,
    ) -> Result<Box<dyn AsyncNode>, NodeError> {
        self.validate_config(configuration)?;
        let language = configuration
            .get(LANGUAGE_CONFIGURATION_KEY)
            .unwrap_or(&self.default_language);
        let mut connector = WhisperConnector::new(&self.binary_path, &self.model_path, language)
            .with_gpu(configuration.get(GPU_CONFIGURATION_KEY) == Some("true"));
        if let Some(window_duration_ms) =
            configuration.get_u32(WINDOW_DURATION_MS_CONFIGURATION_KEY)
        {
            connector = connector.with_window_duration_ms(window_duration_ms);
        }
        if let Some(process_timeout_ms) =
            configuration.get_u32(PROCESS_TIMEOUT_MS_CONFIGURATION_KEY)
        {
            connector = connector.with_process_timeout_ms(process_timeout_ms);
        }
        if let (Some(root), Some(case_id)) = (
            configuration.get(EVIDENCE_ROOT_CONFIGURATION_KEY),
            configuration.get(EVIDENCE_CASE_ID_CONFIGURATION_KEY),
        ) {
            connector = connector
                .with_process_evidence(root, case_id)
                .map_err(|error| NodeError::Prepare(error.to_string()))?;
        }
        Ok(Box::new(connector))
    }
}

fn validate_positive_u32(
    configuration: &OperatorConfiguration,
    key: &str,
) -> Result<(), ConfigError> {
    let Some(raw_value) = configuration.get(key) else {
        return Ok(());
    };
    if raw_value.parse::<u32>().is_ok_and(|value| value > 0) {
        return Ok(());
    }
    Err(ConfigError::Invalid {
        key: key.to_owned(),
        reason: "expected a positive integer".to_owned(),
    })
}

impl WhisperConnector {
    pub fn new(
        binary_path: impl Into<PathBuf>,
        model_path: impl Into<PathBuf>,
        language: impl Into<String>,
    ) -> Self {
        Self {
            binary_path: binary_path.into(),
            model_path: model_path.into(),
            language: language.into(),
            use_gpu: false,
            process_timeout_ms: DEFAULT_PROCESS_TIMEOUT_MS,
            window_duration_ms: 30_000,
            sample_rate_hz: None,
            channels: None,
            pending_samples: Vec::new(),
            pending_origin: None,
            stream_origin: None,
            completed_transcripts: Vec::new(),
            active_child: None,
            process_evidence: None,
            active_process_evidence: None,
            prepared: false,
        }
    }

    pub fn with_gpu(mut self, use_gpu: bool) -> Self {
        self.use_gpu = use_gpu;
        self
    }

    pub fn with_window_duration_ms(mut self, window_duration_ms: u32) -> Self {
        self.window_duration_ms = window_duration_ms;
        self
    }

    pub fn with_process_timeout_ms(mut self, process_timeout_ms: u32) -> Self {
        self.process_timeout_ms = process_timeout_ms;
        self
    }

    pub fn with_process_evidence(
        mut self,
        root: impl Into<PathBuf>,
        case_id: impl Into<String>,
    ) -> Result<Self, ProcessEvidenceError> {
        self.process_evidence = Some(WhisperProcessEvidence::new(root, case_id)?);
        Ok(self)
    }

    async fn require_file(path: &Path, label: &str) -> Result<(), NodeError> {
        let metadata = tokio::fs::metadata(path)
            .await
            .map_err(|error| NodeError::Prepare(format!("{label} {}: {error}", path.display())))?;
        if !metadata.is_file() {
            return Err(NodeError::Prepare(format!(
                "{label} is not a file: {}",
                path.display()
            )));
        }
        Ok(())
    }

    async fn transcribe(&mut self, wav_bytes: &[u8]) -> Result<String, NodeError> {
        let temporary_dir = if self.process_evidence.is_none() {
            Some(TempDir::new().map_err(|error| {
                NodeError::Process(format!("create working directory: {error}"))
            })?)
        } else {
            None
        };
        let mut evidence = match self.process_evidence.as_mut() {
            Some(configuration) => Some(
                configuration
                    .begin(wav_bytes)
                    .await
                    .map_err(|error| NodeError::Process(error.to_string()))?,
            ),
            None => None,
        };
        let workspace = if let Some(active) = evidence.as_ref() {
            active
                .input_wav_path()
                .parent()
                .map(Path::to_path_buf)
                .ok_or_else(|| {
                    NodeError::Process("evidence input has no invocation directory".to_owned())
                })?
        } else {
            temporary_dir
                .as_ref()
                .map(|directory| directory.path().to_path_buf())
                .ok_or_else(|| {
                    NodeError::Process("Whisper invocation has no workspace".to_owned())
                })?
        };
        let wav_path = workspace.join("input.wav");
        let output_prefix = workspace.join("transcript");
        let output_path = workspace.join("transcript.txt");
        let stdout_path = workspace.join("stdout.log");
        let stderr_path = workspace.join("stderr.log");
        if evidence.is_none() {
            tokio::fs::write(&wav_path, wav_bytes)
                .await
                .map_err(|error| NodeError::Process(format!("write WAV input: {error}")))?;
        }

        let argv = self.command_argv(&wav_path, &output_prefix)?;
        if let Some(active) = evidence.as_mut() {
            active.set_argv(argv.clone());
        }
        let mut command = Command::new(&argv[0]);
        command.args(&argv[1..]);
        let stdout_file = std::fs::File::create(&stdout_path)
            .map_err(|error| NodeError::Process(format!("create whisper stdout: {error}")))?;
        let stderr_file = std::fs::File::create(&stderr_path)
            .map_err(|error| NodeError::Process(format!("create whisper stderr: {error}")))?;
        command
            .stdout(Stdio::from(stdout_file))
            .stderr(Stdio::from(stderr_file))
            .kill_on_drop(true);
        self.active_child = Some(
            command
                .spawn()
                .map_err(|error| NodeError::Process(format!("start whisper-cli: {error}")))?,
        );
        if let Some(active) = evidence.as_mut() {
            active.set_pid(self.active_child.as_ref().and_then(Child::id));
        }
        self.active_process_evidence = evidence;
        let wait = {
            let child = self.active_child.as_mut().ok_or_else(|| {
                NodeError::Process("whisper-cli child ownership was lost".to_owned())
            })?;
            tokio::time::timeout(
                Duration::from_millis(u64::from(self.process_timeout_ms)),
                child.wait(),
            )
            .await
        };
        let status = match wait {
            Ok(status) => {
                self.active_child.take();
                let status = status.map_err(|error| {
                    NodeError::Process(format!("wait for whisper-cli: {error}"))
                })?;
                if let Some(mut active) = self.active_process_evidence.take() {
                    active.mark_wait_observed();
                    let outcome = if status.success() {
                        ProcessOutcome::Succeeded
                    } else {
                        ProcessOutcome::ProviderFailed
                    };
                    active
                        .complete(outcome, Some(status.to_string()), true)
                        .await
                        .map_err(|error| NodeError::Process(error.to_string()))?;
                }
                status
            }
            Err(_) => {
                self.terminate_active_child(ProcessOutcome::TimedOut)
                    .await?;
                return Err(NodeError::ProcessTimeout {
                    timeout_ms: self.process_timeout_ms,
                });
            }
        };
        if !status.success() {
            let stderr = tokio::fs::read_to_string(&stderr_path)
                .await
                .unwrap_or_else(|error| format!("read stderr failed: {error}"));
            return Err(NodeError::Process(format!(
                "whisper-cli exited with {status}: {}",
                stderr.trim()
            )));
        }

        let transcript = tokio::fs::read_to_string(&output_path)
            .await
            .map_err(|error| NodeError::Process(format!("read transcript: {error}")))?;
        Ok(transcript.trim().to_owned())
    }

    fn command_argv(
        &self,
        wav_path: &Path,
        output_prefix: &Path,
    ) -> Result<Vec<String>, NodeError> {
        let path = |value: &Path, label: &str| {
            value.to_str().map(str::to_owned).ok_or_else(|| {
                NodeError::Process(format!("{label} must be valid UTF-8 for process evidence"))
            })
        };
        let mut argv = vec![path(&self.binary_path, "whisper-cli path")?];
        if !self.use_gpu {
            argv.push("-ng".to_owned());
        }
        argv.extend([
            "--model".to_owned(),
            path(&self.model_path, "Whisper model path")?,
            "--file".to_owned(),
            path(wav_path, "Whisper input path")?,
            "--language".to_owned(),
            self.language.clone(),
            "--no-timestamps".to_owned(),
            "--output-txt".to_owned(),
            "--output-file".to_owned(),
            path(output_prefix, "Whisper output prefix")?,
        ]);
        Ok(argv)
    }

    async fn terminate_active_child(&mut self, outcome: ProcessOutcome) -> Result<(), NodeError> {
        let Some(mut child) = self.active_child.take() else {
            if let Some(active) = self.active_process_evidence.take() {
                active
                    .complete(outcome, None, true)
                    .await
                    .map_err(|error| NodeError::Process(error.to_string()))?;
            }
            return Ok(());
        };
        let exited = child
            .try_wait()
            .map_err(|error| NodeError::Process(format!("inspect whisper-cli child: {error}")))?;
        let mut status = exited.map(|status| status.to_string());
        if exited.is_none() {
            if let Some(active) = self.active_process_evidence.as_mut() {
                active.mark_kill_requested();
            }
            child
                .kill()
                .await
                .map_err(|error| NodeError::Process(format!("kill whisper-cli child: {error}")))?;
            let waited = child
                .wait()
                .await
                .map_err(|error| NodeError::Process(format!("reap whisper-cli child: {error}")))?;
            status = Some(waited.to_string());
        }
        if let Some(mut active) = self.active_process_evidence.take() {
            active.mark_wait_observed();
            active
                .complete(outcome, status, true)
                .await
                .map_err(|error| NodeError::Process(error.to_string()))?;
        }
        Ok(())
    }

    fn encode_wav(
        samples: &[f32],
        sample_rate_hz: u32,
        channels: u8,
    ) -> Result<Vec<u8>, NodeError> {
        let mut cursor = Cursor::new(Vec::new());
        {
            let specification = hound::WavSpec {
                channels: channels.into(),
                sample_rate: sample_rate_hz,
                bits_per_sample: 16,
                sample_format: hound::SampleFormat::Int,
            };
            let mut writer = hound::WavWriter::new(&mut cursor, specification)
                .map_err(|error| NodeError::Process(format!("create WAV window: {error}")))?;
            for sample in samples {
                let pcm = (sample.clamp(-1.0, 1.0) * f32::from(i16::MAX)).round() as i16;
                writer
                    .write_sample(pcm)
                    .map_err(|error| NodeError::Process(format!("write WAV window: {error}")))?;
            }
            writer
                .finalize()
                .map_err(|error| NodeError::Process(format!("finalize WAV window: {error}")))?;
        }
        Ok(cursor.into_inner())
    }

    async fn process_audio(
        &mut self,
        input: SignalEnvelope,
    ) -> Result<Option<SignalEnvelope>, NodeError> {
        let frame = match input.payload() {
            SignalPayload::Audio(frame) => frame,
            _ => unreachable!("process_audio is called only for typed audio"),
        };
        let sample_rate_hz = self.sample_rate_hz.ok_or_else(|| {
            NodeError::Process("connector sample rate is not prepared".to_owned())
        })?;
        let channels = self
            .channels
            .ok_or_else(|| NodeError::Process("connector channels are not prepared".to_owned()))?;
        if frame.sample_rate_hz() != sample_rate_hz {
            return Err(NodeError::Process(format!(
                "audio sample rate changed from {sample_rate_hz} Hz to {} Hz",
                frame.sample_rate_hz()
            )));
        }
        if frame.channels() != channels {
            return Err(NodeError::Process(format!(
                "audio channel count changed from {channels} to {}",
                frame.channels()
            )));
        }
        if frame.format() != SampleFormat::F32Interleaved {
            return Err(NodeError::Process(
                "Whisper typed audio requires interleaved f32 PCM".to_owned(),
            ));
        }

        let window_samples =
            sample_rate_hz as usize * channels as usize * self.window_duration_ms as usize / 1_000;
        if frame.samples().len() > window_samples {
            return Err(NodeError::Process(format!(
                "audio frame has {} samples, exceeding bounded window of {window_samples} samples",
                frame.samples().len()
            )));
        }
        let frame_samples_per_channel = frame.samples().len() / channels as usize;
        let frame_duration_ns =
            frame_samples_per_channel as u64 * 1_000_000_000 / u64::from(sample_rate_hz);
        let frame_timestamp_end_ns = input.timestamp_ns().saturating_add(frame_duration_ns);
        if let Err(reason) = update_origin(&mut self.stream_origin, &input, frame_timestamp_end_ns)
        {
            self.reset_stream();
            return Err(NodeError::Process(reason));
        }
        if let Err(reason) = update_origin(&mut self.pending_origin, &input, frame_timestamp_end_ns)
        {
            self.reset_stream();
            return Err(NodeError::Process(reason));
        }

        self.pending_samples
            .extend_from_slice(frame.samples());
        if self.pending_samples.len() < window_samples {
            return Ok(None);
        }

        let samples = std::mem::take(&mut self.pending_samples);
        let origin = self
            .pending_origin
            .take()
            .ok_or_else(|| NodeError::Process("Whisper window lost its origin".to_owned()))?;
        let transcript = match self
            .transcribe_samples(samples, sample_rate_hz, channels)
            .await
        {
            Ok(transcript) => transcript,
            Err(error) => {
                self.reset_stream();
                return Err(error);
            }
        };
        self.completed_transcripts.push(transcript.clone());
        self.output_for_origin(transcript, transcript_partial_spec(), origin)
            .map(Some)
    }

    async fn transcribe_samples(
        &mut self,
        samples: Vec<f32>,
        sample_rate_hz: u32,
        channels: u8,
    ) -> Result<String, NodeError> {
        let wav_bytes = Self::encode_wav(&samples, sample_rate_hz, channels)?;
        self.transcribe(&wav_bytes).await
    }

    fn output_for_origin(
        &self,
        transcript: String,
        signal_spec: SignalSpec,
        origin: WindowOrigin,
    ) -> Result<SignalEnvelope, NodeError> {
        let mut output = SignalEnvelope::untracked(
            SignalPayload::Text(transcript),
            signal_spec,
            origin.timestamp_ns,
        );
        if let Some(lineage) = origin.lineage {
            output = output.with_lineage(lineage, origin.timing).with_derivation(
                SignalDerivation::new(
                    lineage,
                    origin.timing,
                    OperatorId::new(WHISPER_OPERATOR_ID),
                    WHISPER_OPERATOR_REVISION,
                    WHISPER_OPERATOR_GENERATION,
                    None,
                )
                .map_err(|error| NodeError::Process(error.to_string()))?,
            );
        }
        Ok(output)
    }

    fn reset_stream(&mut self) {
        self.pending_samples.clear();
        self.pending_origin = None;
        self.stream_origin = None;
        self.completed_transcripts.clear();
    }
}

fn update_origin(
    origin: &mut Option<WindowOrigin>,
    input: &SignalEnvelope,
    frame_timestamp_end_ns: u64,
) -> Result<(), String> {
    if let Some(origin) = origin.as_mut() {
        validate_window_continuation(origin, input, frame_timestamp_end_ns)
    } else {
        let sequence_number = input
            .sequence_number()
            .ok_or_else(|| "audio input has no sequence authority".to_owned())?;
        *origin = Some(WindowOrigin {
            timestamp_ns: input.timestamp_ns(),
            source_id: input.source_id(),
            lineage: input.lineage(),
            timing: input.timing(),
            last_sequence_number: sequence_number,
            timestamp_end_ns: frame_timestamp_end_ns,
        });
        Ok(())
    }
}

fn validate_window_continuation(
    origin: &mut WindowOrigin,
    input: &SignalEnvelope,
    frame_timestamp_end_ns: u64,
) -> Result<(), String> {
    if origin.source_id != input.source_id() {
        return Err("source identity changed inside a Whisper window".to_owned());
    }
    match (origin.lineage.as_mut(), input.lineage()) {
        (Some(base), Some(next)) => {
            let identity_matches = base.session_id() == next.session_id()
                && base.source_id() == next.source_id()
                && base.stream_id() == next.stream_id()
                && base.clock_id() == next.clock_id()
                && base.source_generation() == next.source_generation()
                && base.discontinuity_epoch() == next.discontinuity_epoch()
                && base.policy_epoch() == next.policy_epoch();
            if !identity_matches {
                return Err("lineage authority changed inside a Whisper window".to_owned());
            }
            if next.sequence_number() != origin.last_sequence_number + 1
                || input.timestamp_ns() != origin.timestamp_end_ns
                || frame_timestamp_end_ns < input.timestamp_ns()
            {
                return Err("lineage range is not contiguous inside a Whisper window".to_owned());
            }
            origin.timestamp_end_ns = frame_timestamp_end_ns;
            origin.timing = origin
                .timing
                .with_duration_ns(Some(
                    origin.timestamp_end_ns.saturating_sub(origin.timestamp_ns),
                ))
                .map_err(|error| error.to_string())?;
            origin.last_sequence_number = next.sequence_number();
        }
        (None, None) => {
            let sequence_number = input
                .sequence_number()
                .ok_or_else(|| "audio input has no sequence authority".to_owned())?;
            if sequence_number != origin.last_sequence_number + 1
                || input.timestamp_ns() != origin.timestamp_end_ns
                || frame_timestamp_end_ns < input.timestamp_ns()
            {
                return Err("timestamp range is not contiguous inside a Whisper window".to_owned());
            }
            origin.timestamp_end_ns = frame_timestamp_end_ns;
            origin.timing = origin
                .timing
                .with_duration_ns(Some(
                    origin.timestamp_end_ns.saturating_sub(origin.timestamp_ns),
                ))
                .map_err(|error| error.to_string())?;
            origin.last_sequence_number = sequence_number;
        }
        _ => {
            return Err("lineage presence changed inside a Whisper window".to_owned());
        }
    }
    Ok(())
}

impl AsyncNode for WhisperConnector {
    fn prepare<'a>(
        &'a mut self,
        context: &'a AsyncOperatorPrepareContext,
    ) -> AsyncNodeFuture<'a, Result<(), NodeError>> {
        Box::pin(async move {
            Self::require_file(&self.binary_path, "whisper-cli").await?;
            Self::require_file(&self.model_path, "Whisper model").await?;
            if self.window_duration_ms == 0 {
                return Err(NodeError::Prepare(
                    "Whisper window duration must be greater than zero".to_owned(),
                ));
            }
            if self.process_timeout_ms == 0 {
                return Err(NodeError::Prepare(
                    "Whisper process timeout must be greater than zero".to_owned(),
                ));
            }
            let audio = context
                .inputs()
                .iter()
                .find_map(|input| match input.media() {
                    MediaCaps::Audio(audio) => Some(audio),
                    _ => None,
                })
                .ok_or_else(|| {
                    NodeError::Prepare("Whisper requires a negotiated PCM input edge".to_owned())
                })?;
            if audio.format != SampleFormat::F32Interleaved {
                return Err(NodeError::Prepare(
                    "Whisper typed audio requires interleaved f32 PCM".to_owned(),
                ));
            }
            self.sample_rate_hz = audio.sample_rate_hz;
            self.channels = audio.channel_layout.channel_count();
            if self.sample_rate_hz.is_none() || self.channels.is_none() {
                return Err(NodeError::Prepare(
                    "Whisper requires concrete PCM sample rate and channel layout".to_owned(),
                ));
            }
            self.reset_stream();
            self.prepared = true;
            Ok(())
        })
    }

    fn process<'a>(
        &'a mut self,
        input: SignalEnvelope,
    ) -> AsyncNodeFuture<'a, Result<Vec<SignalEnvelope>, NodeError>> {
        Box::pin(async move {
            if !self.prepared {
                return Err(NodeError::Process("connector is not prepared".to_owned()));
            }
            match input.payload() {
                SignalPayload::Audio(_) => {
                    Ok(self.process_audio(input).await?.into_iter().collect())
                }
                SignalPayload::Bytes(bytes) => {
                    let transcript = self.transcribe(bytes.as_slice()).await?;
                    Ok(vec![input.map_payload(
                        SignalPayload::Text(transcript),
                        transcript_final_spec(),
                    )])
                }
                _ => Err(NodeError::Process(format!(
                    "expected typed audio or binary WAV signal, received {:?}",
                    input.signal_spec().class()
                ))),
            }
        })
    }

    fn flush<'a>(&'a mut self) -> AsyncNodeFuture<'a, Result<Vec<SignalEnvelope>, NodeError>> {
        Box::pin(async move {
            if self.stream_origin.is_none() {
                return Ok(Vec::new());
            }
            let sample_rate_hz = self.sample_rate_hz.ok_or_else(|| {
                NodeError::Process("connector sample rate is not prepared".to_owned())
            })?;
            let channels = self.channels.ok_or_else(|| {
                NodeError::Process("connector channels are not prepared".to_owned())
            })?;
            if !self.pending_samples.is_empty() {
                let samples = std::mem::take(&mut self.pending_samples);
                self.pending_origin.take().ok_or_else(|| {
                    NodeError::Process("Whisper window lost its origin".to_owned())
                })?;
                let transcript = self
                    .transcribe_samples(samples, sample_rate_hz, channels)
                    .await?;
                self.completed_transcripts.push(transcript);
            }
            let origin = self
                .stream_origin
                .take()
                .ok_or_else(|| NodeError::Process("Whisper stream lost its origin".to_owned()))?;
            let transcript = self.completed_transcripts.join(" ");
            self.completed_transcripts.clear();
            Ok(vec![self.output_for_origin(
                transcript,
                transcript_final_spec(),
                origin,
            )?])
        })
    }

    fn cancel<'a>(&'a mut self) -> AsyncNodeFuture<'a, Result<(), NodeError>> {
        Box::pin(async move {
            self.terminate_active_child(ProcessOutcome::Cancelled)
                .await?;
            self.reset_stream();
            Ok(())
        })
    }

    fn close<'a>(&'a mut self) -> AsyncNodeFuture<'a, Result<(), NodeError>> {
        Box::pin(async move {
            self.terminate_active_child(ProcessOutcome::Closed).await?;
            self.reset_stream();
            Ok(())
        })
    }
}

#[cfg(all(test, unix))]
mod tests {
    use std::os::unix::fs::PermissionsExt;
    use pocketstation::{
        AudioBufferPool, AudioFrame, BinaryFormat, ClockDomainId, PortPrepareContext,
        FrameLineage, SampleFormat, SampleSpec, SessionId, SignalLineage, SignalSpec, SignalTiming,
        SourceId, StemId, StreamId,
    };

    use super::*;

    fn test_prepare_context(sample_spec: SampleSpec) -> AsyncOperatorPrepareContext {
        let channel_layout = match sample_spec.channels {
            1 => ChannelLayout::Mono,
            2 => ChannelLayout::Stereo,
            _ => ChannelLayout::Any,
        };
        let audio = MediaCaps::Audio(AudioCaps {
            sample_rate_hz: Some(sample_spec.sample_rate_hz),
            frame_samples: None,
            channel_layout,
            format: sample_spec.format,
        });
        let input_contract = EdgeContract::realtime_audio().with_media(audio);
        let output_contract = EdgeContract::bounded_async().with_media(MediaCaps::Text);
        AsyncOperatorPrepareContext::new(
            ExecutionPartition::BlockingWorker,
            vec![
                PortPrepareContext::new(
                    None,
                    "audio",
                    PortDirection::Input,
                    SignalSpec::audio(),
                    audio,
                    input_contract,
                    32,
                )
                .unwrap(),
                PortPrepareContext::new(
                    None,
                    "transcript",
                    PortDirection::Output,
                    SignalSpec::text(TextFormat::Utf8).with_role("transcript"),
                    MediaCaps::Text,
                    output_contract,
                    8,
                )
                .unwrap(),
            ],
        )
        .unwrap()
    }

    async fn connector_fixture(delay_seconds: Option<&str>) -> (TempDir, WhisperConnector) {
        let fixture = TempDir::new().unwrap();
        let binary = fixture.path().join("whisper-cli");
        let model = fixture.path().join("model.bin");
        let delay = delay_seconds
            .map(|seconds| format!("sleep {seconds}\n"))
            .unwrap_or_default();
        let script = format!(
            "#!/bin/sh\n{delay}while [ $# -gt 0 ]; do if [ \"$1\" = \"--output-file\" ]; then shift; printf 'hello from local whisper\\n' > \"$1.txt\"; fi; shift; done\n"
        );
        tokio::fs::write(&binary, script).await.unwrap();
        let mut permissions = tokio::fs::metadata(&binary).await.unwrap().permissions();
        permissions.set_mode(0o755);
        tokio::fs::set_permissions(&binary, permissions)
            .await
            .unwrap();
        tokio::fs::write(&model, b"model").await.unwrap();
        let connector = WhisperConnector::new(&binary, &model, "en");
        (fixture, connector)
    }

    fn audio_frame(source_id: SourceId, sequence_number: u64) -> AudioFrame {
        let pool = AudioBufferPool::new(1, 160);
        let mut buffer = pool.acquire().unwrap();
        buffer.try_copy_from_slice(&[0.25; 160]).unwrap();
        AudioFrame::try_new(
            StreamId::new(7),
            source_id,
            sequence_number,
            sequence_number * 10_000_000,
            SampleSpec::new(16_000, 1, SampleFormat::F32Interleaved),
            buffer,
        )
        .unwrap()
    }

    fn lineaged_envelope(
        source_id: SourceId,
        sequence_number: u64,
        discontinuity_epoch: u64,
        permission_epoch: u64,
    ) -> SignalEnvelope {
        SignalEnvelope::from_audio(
            audio_frame(source_id, sequence_number),
            Some(
                FrameLineage::try_new(
                    SessionId::new(3),
                    source_id,
                    StemId::new(5),
                    ClockDomainId::new(7),
                    sequence_number,
                    sequence_number * 10_000_000,
                    10_000_000,
                    1,
                    discontinuity_epoch,
                    permission_epoch,
                )
                .unwrap(),
            ),
        )
    }

    fn binary_envelope(bytes: Vec<u8>, sequence_number: u64, timestamp_ns: u64) -> SignalEnvelope {
        SignalEnvelope::untracked(
            SignalPayload::Bytes(bytes),
            SignalSpec::binary(BinaryFormat::Raw),
            timestamp_ns,
        )
        .with_lineage(
            SignalLineage::try_new(
                SessionId::new(3),
                StreamId::new(7),
                SourceId::new(23),
                ClockDomainId::new(7),
                sequence_number,
                1,
                0,
                1,
            )
            .expect("valid test lineage"),
            SignalTiming::try_new(
                Some(timestamp_ns),
                timestamp_ns,
                Some(timestamp_ns),
                None,
            )
            .expect("valid test timing"),
        )
    }

    async fn prepared_window_connector() -> (TempDir, WhisperConnector) {
        let (fixture, mut connector) = connector_fixture(None).await;
        connector = connector.with_window_duration_ms(20);
        connector
            .prepare(&test_prepare_context(SampleSpec::new(
                16_000,
                1,
                SampleFormat::F32Interleaved,
            )))
            .await
            .unwrap();
        (fixture, connector)
    }

    #[test]
    fn given_instance_timeout_when_manifest_resolves_then_deadline_matches_configuration() {
        let factory = WhisperOperatorFactory::new("whisper-cli", "model.bin", "en");
        let configuration =
            OperatorConfiguration::new().with(PROCESS_TIMEOUT_MS_CONFIGURATION_KEY, "7");

        let manifest = factory.resolve_manifest(&configuration).unwrap();

        assert_eq!(
            manifest.deadline().process_timeout_ms,
            7 + PROCESS_TIMEOUT_CLEANUP_MARGIN_MS
        );
        assert_eq!(
            factory.manifest().deadline().process_timeout_ms,
            DEFAULT_PROCESS_TIMEOUT_MS + PROCESS_TIMEOUT_CLEANUP_MARGIN_MS
        );
    }

    #[tokio::test]
    async fn given_wav_envelope_when_connector_runs_then_text_lineage_is_preserved() {
        let (_fixture, mut connector) = connector_fixture(None).await;
        let context =
            test_prepare_context(SampleSpec::new(16_000, 1, SampleFormat::F32Interleaved));
        connector.prepare(&context).await.unwrap();
        let output = connector
            .process(binary_envelope(b"RIFF fixture".to_vec(), 42, 99))
            .await
            .unwrap()
            .into_iter()
            .next()
            .unwrap();

        assert_eq!(output.sequence_number(), Some(42));
        assert_eq!(output.timestamp_ns(), 99);
        assert_eq!(
            output.signal_spec().role().map(|role| role.as_str()),
            Some("transcript.final")
        );
        assert!(
            matches!(output.payload(), SignalPayload::Text(text) if text == "hello from local whisper")
        );
    }

    #[tokio::test]
    async fn given_missing_binary_when_prepare_runs_then_connector_fails_closed() {
        let mut connector = WhisperConnector::new("/missing/whisper-cli", "/missing/model", "en");
        let context =
            test_prepare_context(SampleSpec::new(16_000, 1, SampleFormat::F32Interleaved));

        let error = connector.prepare(&context).await.unwrap_err();
        assert!(matches!(error, NodeError::Prepare(_)));
    }

    #[tokio::test]
    async fn given_hung_provider_when_deadline_expires_then_child_is_killed_and_reaped() {
        let (_fixture, mut connector) = connector_fixture(Some("1")).await;
        connector = connector.with_process_timeout_ms(5);
        connector
            .prepare(&test_prepare_context(SampleSpec::new(
                16_000,
                1,
                SampleFormat::F32Interleaved,
            )))
            .await
            .unwrap();

        let error = connector
            .process(binary_envelope(b"RIFF fixture".to_vec(), 0, 0))
            .await
            .unwrap_err();

        assert!(matches!(error, NodeError::ProcessTimeout { timeout_ms: 5 }));
    }

    #[tokio::test]
    async fn given_process_evidence_when_provider_succeeds_then_actual_invocation_is_persisted() {
        let evidence_root = TempDir::new().unwrap();
        let (_fixture, mut connector) = connector_fixture(None).await;
        connector = connector
            .with_process_evidence(evidence_root.path(), "success")
            .unwrap();
        connector
            .prepare(&test_prepare_context(SampleSpec::new(
                16_000,
                1,
                SampleFormat::F32Interleaved,
            )))
            .await
            .unwrap();

        connector
            .process(binary_envelope(b"RIFF fixture".to_vec(), 0, 0))
            .await
            .unwrap();

        let invocation = evidence_root.path().join("success-0000");
        let receipt: serde_json::Value = serde_json::from_slice(
            &tokio::fs::read(invocation.join("receipt.json"))
                .await
                .unwrap(),
        )
        .unwrap();
        assert_eq!(receipt["outcome"], "succeeded");
        assert_eq!(receipt["argv"][1], "-ng");
        assert_eq!(
            receipt["argv"][5],
            invocation.join("input.wav").to_string_lossy().as_ref()
        );
        assert_eq!(
            receipt["argv"][11],
            invocation.join("transcript").to_string_lossy().as_ref()
        );
        assert!(receipt["pid"].as_u64().is_some());
        assert_eq!(receipt["kill_requested"], false);
        assert_eq!(receipt["wait_observed"], true);
        assert_eq!(receipt["reaped"], true);
        assert_eq!(receipt["content_hash_algorithm"], "fnv1a64");
        assert!(invocation.join("input.wav").is_file());
        assert!(invocation.join("stdout.log").is_file());
        assert!(invocation.join("stderr.log").is_file());
        assert!(invocation.join("transcript.txt").is_file());
        assert!(receipt["input_wav_hash"].as_str().is_some());
        assert!(receipt["transcript_hash"].as_str().is_some());
    }

    #[tokio::test]
    async fn given_process_evidence_when_provider_times_out_then_kill_and_reap_are_persisted() {
        let evidence_root = TempDir::new().unwrap();
        let (_fixture, mut connector) = connector_fixture(Some("1")).await;
        connector = connector
            .with_process_timeout_ms(5)
            .with_process_evidence(evidence_root.path(), "timeout")
            .unwrap();
        connector
            .prepare(&test_prepare_context(SampleSpec::new(
                16_000,
                1,
                SampleFormat::F32Interleaved,
            )))
            .await
            .unwrap();

        let error = connector
            .process(binary_envelope(b"RIFF fixture".to_vec(), 0, 0))
            .await
            .unwrap_err();

        assert!(matches!(error, NodeError::ProcessTimeout { timeout_ms: 5 }));
        let receipt: serde_json::Value = serde_json::from_slice(
            &tokio::fs::read(
                evidence_root
                    .path()
                    .join("timeout-0000")
                    .join("receipt.json"),
            )
            .await
            .unwrap(),
        )
        .unwrap();
        assert_eq!(receipt["outcome"], "timed_out");
        assert_eq!(receipt["kill_requested"], true);
        assert_eq!(receipt["wait_observed"], true);
        assert_eq!(receipt["reaped"], true);
    }

    #[tokio::test]
    async fn given_outer_cancellation_when_process_is_active_then_child_receipt_is_finalized() {
        let evidence_root = TempDir::new().unwrap();
        let (_fixture, mut connector) = connector_fixture(Some("1")).await;
        connector = connector
            .with_process_evidence(evidence_root.path(), "cancel")
            .unwrap();
        connector
            .prepare(&test_prepare_context(SampleSpec::new(
                16_000,
                1,
                SampleFormat::F32Interleaved,
            )))
            .await
            .unwrap();
        {
            let process = connector.process(binary_envelope(b"RIFF fixture".to_vec(), 0, 0));
            tokio::pin!(process);
            tokio::select! {
                result = &mut process => panic!("provider unexpectedly completed: {result:?}"),
                _ = tokio::time::sleep(Duration::from_millis(20)) => {}
            }
        }

        connector.cancel().await.unwrap();

        let receipt: serde_json::Value = serde_json::from_slice(
            &tokio::fs::read(
                evidence_root
                    .path()
                    .join("cancel-0000")
                    .join("receipt.json"),
            )
            .await
            .unwrap(),
        )
        .unwrap();
        assert_eq!(receipt["outcome"], "cancelled");
        assert_eq!(receipt["kill_requested"], true);
        assert_eq!(receipt["wait_observed"], true);
        assert_eq!(receipt["reaped"], true);
    }

    #[tokio::test]
    async fn given_typed_audio_when_window_fills_then_partial_precedes_one_final_transcript() {
        let source_id = SourceId::new(23);
        let (_fixture, mut connector) = connector_fixture(None).await;
        connector = connector.with_window_duration_ms(20);
        connector
            .prepare(&test_prepare_context(SampleSpec::new(
                16_000,
                1,
                SampleFormat::F32Interleaved,
            )))
            .await
            .unwrap();

        let first = lineaged_envelope(source_id, 8, 0, 1);
        assert!(connector.process(first).await.unwrap().is_empty());

        let second = lineaged_envelope(source_id, 9, 0, 1);
        let output = connector
            .process(second)
            .await
            .unwrap()
            .into_iter()
            .next()
            .unwrap();

        assert_eq!(output.sequence_number(), Some(8));
        assert_eq!(output.timestamp_ns(), 80_000_000);
        assert_eq!(output.source_id(), Some(source_id));
        assert_eq!(
            output.signal_spec().role().map(|role| role.as_str()),
            Some("transcript.partial")
        );
        assert!(
            matches!(output.payload(), SignalPayload::Text(text) if text == "hello from local whisper")
        );

        let final_output = connector.flush().await.unwrap().pop().unwrap();
        assert_eq!(final_output.sequence_number(), Some(8));
        assert_eq!(final_output.timestamp_ns(), 80_000_000);
        assert_eq!(final_output.source_id(), Some(source_id));
        assert_eq!(
            final_output.signal_spec().role().map(|role| role.as_str()),
            Some("transcript.final")
        );
        assert!(connector.flush().await.unwrap().is_empty());
    }

    #[tokio::test]
    async fn given_lineaged_window_when_transcribed_then_derived_range_covers_every_frame() {
        let (_fixture, mut connector) = prepared_window_connector().await;
        assert!(connector
            .process(lineaged_envelope(SourceId::new(23), 0, 0, 1))
            .await
            .unwrap()
            .is_empty());

        let output = connector
            .process(lineaged_envelope(SourceId::new(23), 1, 0, 1))
            .await
            .unwrap()
            .into_iter()
            .next()
            .unwrap();
        let derived = output.derivation().unwrap();

        assert_eq!(derived.upstream_timing().source_timestamp_ns(), Some(0));
        assert_eq!(derived.upstream_timing().duration_ns(), Some(20_000_000));
        assert_eq!(derived.upstream_timing().timestamp_end_ns(), Some(20_000_000));
        assert_eq!(derived.operator_id().as_str(), WHISPER_OPERATOR_ID);
    }

    #[tokio::test]
    async fn given_two_complete_windows_when_finished_then_partials_and_single_final_cover_stream()
    {
        let (_fixture, mut connector) = prepared_window_connector().await;
        let mut partials = Vec::new();
        for sequence_number in 0..4 {
            partials.extend(
                connector
                    .process(lineaged_envelope(SourceId::new(23), sequence_number, 0, 1))
                    .await
                    .unwrap(),
            );
        }

        assert_eq!(partials.len(), 2);
        assert!(partials.iter().all(|output| {
            output.signal_spec().role().map(|role| role.as_str())
                == Some("transcript.partial")
        }));

        let finals = connector.flush().await.unwrap();
        assert_eq!(finals.len(), 1);
        let final_output = &finals[0];
        assert_eq!(
            final_output.signal_spec().role().map(|role| role.as_str()),
            Some("transcript.final")
        );
        let derived = final_output.derivation().unwrap();
        assert_eq!(derived.upstream_lineage().sequence_number(), 0);
        assert_eq!(derived.upstream_timing().duration_ns(), Some(40_000_000));
        assert_eq!(derived.upstream_timing().timestamp_end_ns(), Some(40_000_000));
        assert!(matches!(
            final_output.payload(),
            SignalPayload::Text(text)
                if text == "hello from local whisper hello from local whisper"
        ));
        assert!(connector.flush().await.unwrap().is_empty());
    }

    #[tokio::test]
    async fn given_source_change_inside_window_when_processed_then_window_is_rejected_and_reset() {
        let (_fixture, mut connector) = prepared_window_connector().await;
        assert!(connector
            .process(lineaged_envelope(SourceId::new(23), 0, 0, 1))
            .await
            .unwrap()
            .is_empty());

        let error = connector
            .process(lineaged_envelope(SourceId::new(24), 1, 0, 1))
            .await
            .unwrap_err();

        assert!(
            matches!(error, NodeError::Process(message) if message.contains("source identity"))
        );
        assert!(connector.pending_samples.is_empty());
        assert!(connector.pending_origin.is_none());
    }

    #[tokio::test]
    async fn given_discontinuity_change_inside_window_when_processed_then_window_is_rejected() {
        let (_fixture, mut connector) = prepared_window_connector().await;
        assert!(connector
            .process(lineaged_envelope(SourceId::new(23), 0, 0, 1))
            .await
            .unwrap()
            .is_empty());

        let error = connector
            .process(lineaged_envelope(SourceId::new(23), 1, 1, 1))
            .await
            .unwrap_err();

        assert!(
            matches!(error, NodeError::Process(message) if message.contains("lineage authority"))
        );
    }

    #[tokio::test]
    async fn given_permission_change_inside_window_when_processed_then_window_is_rejected() {
        let (_fixture, mut connector) = prepared_window_connector().await;
        assert!(connector
            .process(lineaged_envelope(SourceId::new(23), 0, 0, 1))
            .await
            .unwrap()
            .is_empty());

        let error = connector
            .process(lineaged_envelope(SourceId::new(23), 1, 0, 2))
            .await
            .unwrap_err();

        assert!(
            matches!(error, NodeError::Process(message) if message.contains("lineage authority"))
        );
    }

}
