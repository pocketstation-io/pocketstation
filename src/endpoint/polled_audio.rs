use std::sync::Arc;

use crate::endpoint::polled_audio_driver::PolledAudioEndpointFactory;
pub use crate::endpoint::polled_audio_driver::{
    PolledAudioEndpointConfig, PolledAudioEndpointConfigError, PolledAudioReceipt,
};
use crate::endpoint::EndpointDriverFactory;

pub const POLLED_AUDIO_OPERATOR_ID: &str = "io.pocketstation.endpoint.polled-audio.v1";

/// Declares application-polled audio and retains its bounded receipt.
///
/// This type keeps the concrete bounded endpoint registration authority and
/// receipt together so every language adapter consumes the same
/// compiled Session path.
pub struct PolledAudioEndpoint {
    factory: Arc<PolledAudioEndpointFactory>,
    receipt: PolledAudioReceipt,
}

impl PolledAudioEndpoint {
    #[doc = "Creates a new `PolledAudioEndpoint`."]
    pub fn new(config: PolledAudioEndpointConfig) -> Result<Self, PolledAudioEndpointConfigError> {
        let (factory, receipt) = PolledAudioEndpointFactory::new(config)?;
        Ok(Self {
            factory: Arc::new(factory),
            receipt,
        })
    }

    #[doc = "Returns the receipt held by `PolledAudioEndpoint`."]
    pub fn receipt(&self) -> PolledAudioReceipt {
        self.receipt.clone()
    }

    pub(crate) fn factory(&self) -> Arc<dyn EndpointDriverFactory> {
        self.factory.clone()
    }
}
