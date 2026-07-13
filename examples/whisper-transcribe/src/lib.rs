use std::path::{Path, PathBuf};

use pks_graph::{
    AsyncEnvelope, AsyncNode, AsyncNodeFuture, AsyncSignal, NodeError, PrepareContext,
};
use tempfile::TempDir;
use tokio::process::Command;

pub struct WhisperConnector {
    binary_path: PathBuf,
    model_path: PathBuf,
    language: String,
    use_gpu: bool,
    prepared: bool,
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
            prepared: false,
        }
    }

    pub fn with_gpu(mut self, use_gpu: bool) -> Self {
        self.use_gpu = use_gpu;
        self
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

    async fn transcribe(&self, wav_bytes: &[u8]) -> Result<String, NodeError> {
        let working_dir = TempDir::new()
            .map_err(|error| NodeError::Process(format!("create working directory: {error}")))?;
        let wav_path = working_dir.path().join("input.wav");
        let output_prefix = working_dir.path().join("transcript");
        let output_path = working_dir.path().join("transcript.txt");
        tokio::fs::write(&wav_path, wav_bytes)
            .await
            .map_err(|error| NodeError::Process(format!("write WAV input: {error}")))?;

        let mut command = Command::new(&self.binary_path);
        command
            .arg("--model")
            .arg(&self.model_path)
            .arg("--file")
            .arg(&wav_path)
            .arg("--language")
            .arg(&self.language)
            .arg("--no-timestamps")
            .arg("--output-txt")
            .arg("--output-file")
            .arg(&output_prefix);
        if !self.use_gpu {
            command.arg("--no-gpu");
        }
        let output = command
            .output()
            .await
            .map_err(|error| NodeError::Process(format!("start whisper-cli: {error}")))?;
        if !output.status.success() {
            return Err(NodeError::Process(format!(
                "whisper-cli exited with {}: {}",
                output.status,
                String::from_utf8_lossy(&output.stderr).trim()
            )));
        }

        let transcript = tokio::fs::read_to_string(&output_path)
            .await
            .map_err(|error| NodeError::Process(format!("read transcript: {error}")))?;
        Ok(transcript.trim().to_owned())
    }
}

impl AsyncNode for WhisperConnector {
    fn prepare<'a>(
        &'a mut self,
        _cx: &'a PrepareContext,
    ) -> AsyncNodeFuture<'a, Result<(), NodeError>> {
        Box::pin(async move {
            Self::require_file(&self.binary_path, "whisper-cli").await?;
            Self::require_file(&self.model_path, "Whisper model").await?;
            self.prepared = true;
            Ok(())
        })
    }

    fn process<'a>(
        &'a mut self,
        input: AsyncEnvelope,
    ) -> AsyncNodeFuture<'a, Result<Option<AsyncEnvelope>, NodeError>> {
        Box::pin(async move {
            if !self.prepared {
                return Err(NodeError::Process("connector is not prepared".to_owned()));
            }
            let wav_bytes = match input.signal {
                AsyncSignal::Binary(bytes) => bytes,
                other => {
                    return Err(NodeError::Process(format!(
                        "expected binary WAV signal, received {:?}",
                        other.signal_spec().class
                    )))
                }
            };
            let transcript = self.transcribe(&wav_bytes).await?;
            Ok(Some(AsyncEnvelope::new(
                AsyncSignal::Text(transcript),
                input.sequence_number,
                input.timestamp_ns,
            )))
        })
    }
}

#[cfg(all(test, unix))]
mod tests {
    use std::os::unix::fs::PermissionsExt;

    use pks_frame::{SampleFormat, SampleSpec};

    use super::*;

    #[tokio::test]
    async fn given_wav_envelope_when_connector_runs_then_text_lineage_is_preserved() {
        let fixture = TempDir::new().unwrap();
        let binary = fixture.path().join("whisper-cli");
        let model = fixture.path().join("model.bin");
        tokio::fs::write(
            &binary,
            "#!/bin/sh\nwhile [ $# -gt 0 ]; do if [ \"$1\" = \"--output-file\" ]; then shift; printf 'hello from local whisper\\n' > \"$1.txt\"; fi; shift; done\n",
        )
        .await
        .unwrap();
        let mut permissions = tokio::fs::metadata(&binary).await.unwrap().permissions();
        permissions.set_mode(0o755);
        tokio::fs::set_permissions(&binary, permissions)
            .await
            .unwrap();
        tokio::fs::write(&model, b"model").await.unwrap();

        let mut connector = WhisperConnector::new(&binary, &model, "en");
        let context = PrepareContext::new(SampleSpec::new(16_000, 1, SampleFormat::F32Interleaved));
        connector.prepare(&context).await.unwrap();
        let output = connector
            .process(AsyncEnvelope::new(
                AsyncSignal::Binary(b"RIFF fixture".to_vec()),
                42,
                99,
            ))
            .await
            .unwrap()
            .unwrap();

        assert_eq!(output.sequence_number, 42);
        assert_eq!(output.timestamp_ns, 99);
        assert!(
            matches!(output.signal, AsyncSignal::Text(ref text) if text == "hello from local whisper")
        );
    }

    #[tokio::test]
    async fn given_missing_binary_when_prepare_runs_then_connector_fails_closed() {
        let mut connector = WhisperConnector::new("/missing/whisper-cli", "/missing/model", "en");
        let context = PrepareContext::new(SampleSpec::new(16_000, 1, SampleFormat::F32Interleaved));

        let error = connector.prepare(&context).await.unwrap_err();
        assert!(matches!(error, NodeError::Prepare(_)));
    }
}
