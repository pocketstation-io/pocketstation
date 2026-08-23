use std::sync::Arc;

use crate::endpoint::polled_audio_driver::PolledAudioEndpointFactory;
pub use crate::endpoint::polled_audio_driver::{
    PolledAudioEndpointConfig, PolledAudioEndpointConfigError, PolledAudioReceipt,
};
use crate::endpoint::EndpointDriverFactory;

pub const POLLED_AUDIO_OPERATOR_ID: &str = "io.pocketstation.endpoint.polled-audio.v1";

/// Safe composition owner for application-polled audio.
///
/// This type keeps the concrete bounded endpoint registration authority and
/// safe receipt together so every language adapter consumes the canonical
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
