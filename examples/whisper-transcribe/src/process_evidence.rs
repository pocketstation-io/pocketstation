use std::path::{Path, PathBuf};
use std::time::{SystemTime, UNIX_EPOCH};

use serde::Serialize;

const RECEIPT_SCHEMA_VERSION: u32 = 1;
const CONTENT_HASH_ALGORITHM: &str = "fnv1a64";

#[derive(Debug, Clone)]
pub struct WhisperProcessEvidence {
    root: PathBuf,
    case_id: String,
    next_invocation_index: u64,
}

impl WhisperProcessEvidence {
    pub fn new(
        root: impl Into<PathBuf>,
        case_id: impl Into<String>,
    ) -> Result<Self, ProcessEvidenceError> {
        let case_id = case_id.into();
        if case_id.is_empty()
            || !case_id
                .bytes()
                .all(|byte| byte.is_ascii_alphanumeric() || matches!(byte, b'-' | b'_' | b'.'))
        {
            return Err(ProcessEvidenceError::InvalidCaseId);
        }
        Ok(Self {
            root: root.into(),
            case_id,
            next_invocation_index: 0,
        })
    }

    pub async fn begin(
        &mut self,
        input_wav: &[u8],
    ) -> Result<ActiveProcessEvidence, ProcessEvidenceError> {
        tokio::fs::create_dir_all(&self.root).await?;
        let invocation_index = self.next_invocation_index;
        self.next_invocation_index = self.next_invocation_index.saturating_add(1);
        let invocation_dir = self
            .root
            .join(format!("{}-{invocation_index:04}", self.case_id));
        tokio::fs::create_dir(&invocation_dir).await?;
        let input_wav_path = invocation_dir.join("input.wav");
        tokio::fs::write(&input_wav_path, input_wav).await?;
        Ok(ActiveProcessEvidence {
            receipt: WhisperProcessReceipt {
                schema_version: RECEIPT_SCHEMA_VERSION,
                case_id: self.case_id.clone(),
                invocation_index,
                outcome: ProcessOutcome::Started,
                argv: Vec::new(),
                pid: None,
                started_unix_ns: unix_timestamp_ns(),
                ended_unix_ns: None,
                exit_status: None,
                kill_requested: false,
                wait_observed: false,
                reaped: false,
                content_hash_algorithm: CONTENT_HASH_ALGORITHM,
                input_wav_path: input_wav_path.clone(),
                input_wav_hash: content_hash(input_wav),
                stdout_path: invocation_dir.join("stdout.log"),
                stdout_hash: None,
                stderr_path: invocation_dir.join("stderr.log"),
                stderr_hash: None,
                transcript_path: invocation_dir.join("transcript.txt"),
                transcript_hash: None,
            },
            invocation_dir,
        })
    }
}

#[derive(Debug)]
pub struct ActiveProcessEvidence {
    invocation_dir: PathBuf,
    receipt: WhisperProcessReceipt,
}

impl ActiveProcessEvidence {
    pub fn input_wav_path(&self) -> &Path {
        &self.receipt.input_wav_path
    }

    pub fn stdout_path(&self) -> &Path {
        &self.receipt.stdout_path
    }

    pub fn stderr_path(&self) -> &Path {
        &self.receipt.stderr_path
    }

    pub fn transcript_path(&self) -> &Path {
        &self.receipt.transcript_path
    }

    pub fn output_prefix(&self) -> PathBuf {
        self.invocation_dir.join("transcript")
    }

    pub fn set_pid(&mut self, pid: Option<u32>) {
        self.receipt.pid = pid;
    }

    pub fn set_argv(&mut self, argv: Vec<String>) {
        self.receipt.argv = argv;
    }

    pub fn mark_kill_requested(&mut self) {
        self.receipt.kill_requested = true;
    }

    pub fn mark_wait_observed(&mut self) {
        self.receipt.wait_observed = true;
    }

    pub async fn complete(
        mut self,
        outcome: ProcessOutcome,
        exit_status: Option<String>,
        reaped: bool,
    ) -> Result<PathBuf, ProcessEvidenceError> {
        self.receipt.outcome = outcome;
        self.receipt.exit_status = exit_status;
        self.receipt.reaped = reaped;
        self.receipt.ended_unix_ns = Some(unix_timestamp_ns());
        self.receipt.stdout_hash = hash_file_if_present(&self.receipt.stdout_path).await?;
        self.receipt.stderr_hash = hash_file_if_present(&self.receipt.stderr_path).await?;
        self.receipt.transcript_hash = hash_file_if_present(&self.receipt.transcript_path).await?;
        let receipt_path = self.invocation_dir.join("receipt.json");
        let encoded = serde_json::to_vec_pretty(&self.receipt)?;
        tokio::fs::write(&receipt_path, encoded).await?;
        Ok(receipt_path)
    }
}

#[derive(Debug, Clone, Copy, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum ProcessOutcome {
    Started,
    Succeeded,
    ProviderFailed,
    TimedOut,
    Cancelled,
    Closed,
}

#[derive(Debug, Serialize)]
pub struct WhisperProcessReceipt {
    pub schema_version: u32,
    pub case_id: String,
    pub invocation_index: u64,
    pub outcome: ProcessOutcome,
    pub argv: Vec<String>,
    pub pid: Option<u32>,
    pub started_unix_ns: u128,
    pub ended_unix_ns: Option<u128>,
    pub exit_status: Option<String>,
    pub kill_requested: bool,
    pub wait_observed: bool,
    pub reaped: bool,
    pub content_hash_algorithm: &'static str,
    pub input_wav_path: PathBuf,
    pub input_wav_hash: String,
    pub stdout_path: PathBuf,
    pub stdout_hash: Option<String>,
    pub stderr_path: PathBuf,
    pub stderr_hash: Option<String>,
    pub transcript_path: PathBuf,
    pub transcript_hash: Option<String>,
}

#[derive(Debug, thiserror::Error)]
pub enum ProcessEvidenceError {
    #[error(
        "process evidence case id must use only ASCII letters, digits, dot, dash, or underscore"
    )]
    InvalidCaseId,
    #[error("process evidence filesystem operation failed: {0}")]
    Io(#[from] std::io::Error),
    #[error("process evidence receipt serialization failed: {0}")]
    Serialize(#[from] serde_json::Error),
}

async fn hash_file_if_present(path: &Path) -> Result<Option<String>, std::io::Error> {
    match tokio::fs::read(path).await {
        Ok(bytes) => Ok(Some(content_hash(&bytes))),
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => Ok(None),
        Err(error) => Err(error),
    }
}

fn content_hash(bytes: &[u8]) -> String {
    let mut hash = 0xcbf29ce484222325_u64;
    for byte in bytes {
        hash ^= u64::from(*byte);
        hash = hash.wrapping_mul(0x100000001b3);
    }
    format!("{hash:016x}")
}

fn unix_timestamp_ns() -> u128 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map_or(0, |duration| duration.as_nanos())
}
