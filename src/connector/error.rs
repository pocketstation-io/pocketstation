use std::fmt;
use std::sync::Arc;

use crate::{EndpointFailure, EndpointFailureRetryability, EndpointFailureStage};

#[doc = "Sets the maximum supported connector error code bytes."]
pub const MAX_CONNECTOR_ERROR_CODE_BYTES: usize = 160;
#[doc = "Sets the maximum supported connector error message bytes."]
pub const MAX_CONNECTOR_ERROR_MESSAGE_BYTES: usize = 4_096;

#[derive(Clone, PartialEq, Eq, Hash)]
#[doc = "Represents connector error code in the PocketStation API."]
pub struct ConnectorErrorCode(Arc<str>);

impl ConnectorErrorCode {
    #[doc = "Creates a new `ConnectorErrorCode`."]
    pub fn new(value: impl AsRef<str>) -> Result<Self, ConnectorErrorCodeError> {
        let value = value.as_ref();
        if value.trim().is_empty() {
            return Err(ConnectorErrorCodeError::Empty);
        }
        if value.len() > MAX_CONNECTOR_ERROR_CODE_BYTES {
            return Err(ConnectorErrorCodeError::TooLong);
        }
        if !value.bytes().all(|byte| {
            byte.is_ascii_lowercase() || byte.is_ascii_digit() || matches!(byte, b'.' | b'_' | b'-')
        }) {
            return Err(ConnectorErrorCodeError::InvalidCharacter);
        }
        Ok(Self(Arc::from(value)))
    }

    #[doc = "Returns the stable string representation of `ConnectorErrorCode`."]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Debug for ConnectorErrorCode {
    #[doc = "Formats `ConnectorErrorCode` with the requested formatter."]
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_tuple("ConnectorErrorCode")
            .field(&self.0)
            .finish()
    }
}

impl fmt::Display for ConnectorErrorCode {
    #[doc = "Formats `ConnectorErrorCode` with the requested formatter."]
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, thiserror::Error)]
#[doc = "Classifies failures reported as connector error code error."]
pub enum ConnectorErrorCodeError {
    #[error("connector error code cannot be empty")]
    #[doc = "Represents an empty value or collection."]
    Empty,
    #[error("connector error code exceeds the byte limit")]
    #[doc = "Reports too long."]
    TooLong,
    #[error("connector error code contains an invalid character")]
    #[doc = "Reports invalid character."]
    InvalidCharacter,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[doc = "Selects the connector error stage used by PocketStation."]
pub enum ConnectorErrorStage {
    #[doc = "Reports configuration."]
    Configuration,
    #[doc = "Reports prepare."]
    Prepare,
    #[doc = "Reports startup."]
    Startup,
    #[doc = "Reports readiness."]
    Readiness,
    #[doc = "Reports delivery."]
    Delivery,
    #[doc = "Reports retry."]
    Retry,
    #[doc = "Reports shutdown."]
    Shutdown,
    #[doc = "Reports join."]
    Join,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[doc = "Enumerates the supported connector retryability cases."]
pub enum ConnectorRetryability {
    #[doc = "Represents the never case of `ConnectorRetryability`."]
    Never,
    #[doc = "Represents the retryable case of `ConnectorRetryability`."]
    Retryable,
    #[doc = "Represents the retry after reconfiguration case of `ConnectorRetryability`."]
    RetryAfterReconfiguration,
}

#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
#[error("connector {stage:?} failed with {code}: {message}")]
#[doc = "Reports a connector error."]
pub struct ConnectorError {
    code: ConnectorErrorCode,
    stage: ConnectorErrorStage,
    retryability: ConnectorRetryability,
    message: String,
}

impl ConnectorError {
    #[doc = "Creates a new `ConnectorError`."]
    pub fn new(
        code: ConnectorErrorCode,
        stage: ConnectorErrorStage,
        retryability: ConnectorRetryability,
        message: impl Into<String>,
    ) -> Result<Self, ConnectorErrorBuildError> {
        let message = message.into();
        if message.trim().is_empty() {
            return Err(ConnectorErrorBuildError::EmptyMessage);
        }
        if message.len() > MAX_CONNECTOR_ERROR_MESSAGE_BYTES {
            return Err(ConnectorErrorBuildError::MessageTooLarge);
        }
        Ok(Self {
            code,
            stage,
            retryability,
            message,
        })
    }

    #[doc = "Returns the stable error or status code represented by `ConnectorError`."]
    pub fn code(&self) -> &ConnectorErrorCode {
        &self.code
    }

    #[doc = "Returns the stage associated with `ConnectorError`."]
    pub const fn stage(&self) -> ConnectorErrorStage {
        self.stage
    }

    #[doc = "Returns the retryability associated with `ConnectorError`."]
    pub const fn retryability(&self) -> ConnectorRetryability {
        self.retryability
    }

    #[doc = "Returns the diagnostic message associated with `ConnectorError`."]
    pub fn message(&self) -> &str {
        &self.message
    }

    #[doc = "Converts `ConnectorError` into endpoint failure."]
    pub fn into_endpoint_failure(self) -> EndpointFailure {
        let stage = match self.stage {
            ConnectorErrorStage::Configuration | ConnectorErrorStage::Prepare => {
                EndpointFailureStage::Prepare
            }
            ConnectorErrorStage::Startup | ConnectorErrorStage::Readiness => {
                EndpointFailureStage::Start
            }
            ConnectorErrorStage::Shutdown => EndpointFailureStage::RequestStop,
            ConnectorErrorStage::Delivery
            | ConnectorErrorStage::Retry
            | ConnectorErrorStage::Join => EndpointFailureStage::JoinFinalize,
        };
        let retryability = match self.retryability {
            ConnectorRetryability::Never => EndpointFailureRetryability::Never,
            ConnectorRetryability::Retryable => EndpointFailureRetryability::Retryable,
            ConnectorRetryability::RetryAfterReconfiguration => {
                EndpointFailureRetryability::ReconfigurationRequired
            }
        };
        EndpointFailure::new(stage, format!("{}: {}", self.code, self.message))
            .with_external_details(self.code.as_str(), retryability)
    }

    pub(crate) fn internal(
        code: &'static str,
        stage: ConnectorErrorStage,
        message: impl Into<String>,
    ) -> Self {
        Self::internal_with_retryability(code, stage, ConnectorRetryability::Never, message)
    }

    pub(crate) fn internal_with_retryability(
        code: &'static str,
        stage: ConnectorErrorStage,
        retryability: ConnectorRetryability,
        message: impl Into<String>,
    ) -> Self {
        let mut message = message.into();
        if message.len() > MAX_CONNECTOR_ERROR_MESSAGE_BYTES {
            let mut boundary = MAX_CONNECTOR_ERROR_MESSAGE_BYTES;
            while boundary > 0 && !message.is_char_boundary(boundary) {
                boundary -= 1;
            }
            message.truncate(boundary);
        }
        if message.trim().is_empty() {
            message = "internal connector failure".to_owned();
        }
        Self {
            code: ConnectorErrorCode(Arc::from(code)),
            stage,
            retryability,
            message,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, thiserror::Error)]
#[doc = "Classifies failures reported as connector error build error."]
pub enum ConnectorErrorBuildError {
    #[error("connector error message cannot be empty")]
    #[doc = "Reports empty message."]
    EmptyMessage,
    #[error("connector error message exceeds the byte limit")]
    #[doc = "Reports message too large."]
    MessageTooLarge,
}
