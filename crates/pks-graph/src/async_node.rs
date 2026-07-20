use std::future::Future;
use std::pin::Pin;

use crate::node::{NodeError, PrepareContext};
use crate::signal::{SignalClass, SignalSpec};
use pks_frame::AudioFrame;

pub type AsyncNodeFuture<'a, T> = Pin<Box<dyn Future<Output = T> + Send + 'a>>;

#[derive(Debug)]
pub enum AsyncSignal {
    Audio(AudioFrame),
    Text(String),
    Event(Vec<u8>),
    Binary(Vec<u8>),
    Control(Vec<u8>),
}

impl AsyncSignal {
    pub fn signal_spec(&self) -> SignalSpec {
        match self {
            Self::Audio(_) => SignalSpec::audio(),
            Self::Text(_) => SignalSpec::text(crate::signal::TextFormat::Utf8),
            Self::Event(_) => SignalSpec::event(crate::signal::EventFormat::Json),
            Self::Binary(_) => {
                SignalSpec::new(SignalClass::Binary(crate::signal::BinaryFormat::Raw))
            }
            Self::Control(_) => SignalSpec::control(),
        }
    }
}

#[derive(Debug)]
pub struct AsyncEnvelope {
    pub signal: AsyncSignal,
    pub sequence_number: u64,
    pub timestamp_ns: u64,
}

impl AsyncEnvelope {
    pub fn new(signal: AsyncSignal, sequence_number: u64, timestamp_ns: u64) -> Self {
        Self {
            signal,
            sequence_number,
            timestamp_ns,
        }
    }
}

/// Async operator contract for model, connector, transport, and control-plane work.
///
/// `AsyncNode` is intentionally separate from `RuntimeNode`: realtime nodes process
/// `AudioFrame` synchronously on alloc-free executors, while async nodes may await,
/// allocate, and perform I/O only after a Bridge has moved data off the hot path.
pub trait AsyncNode: Send {
    fn prepare<'a>(
        &'a mut self,
        cx: &'a PrepareContext,
    ) -> AsyncNodeFuture<'a, Result<(), NodeError>>;

    fn process<'a>(
        &'a mut self,
        input: AsyncEnvelope,
    ) -> AsyncNodeFuture<'a, Result<Option<AsyncEnvelope>, NodeError>>;

    fn flush<'a>(&'a mut self) -> AsyncNodeFuture<'a, Result<(), NodeError>> {
        Box::pin(async { Ok(()) })
    }

    fn close<'a>(&'a mut self) -> AsyncNodeFuture<'a, Result<(), NodeError>> {
        Box::pin(async { Ok(()) })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pks_frame::{SampleFormat, SampleSpec};

    fn prepare_cx() -> PrepareContext {
        PrepareContext::new(SampleSpec::new(48_000, 1, SampleFormat::F32Interleaved))
    }

    fn block_on_ready<T>(future: AsyncNodeFuture<'_, T>) -> T {
        use std::sync::Arc;
        use std::task::{Context, Poll, Wake, Waker};

        struct NoopWake;

        impl Wake for NoopWake {
            fn wake(self: Arc<Self>) {}
        }

        let waker = Waker::from(Arc::new(NoopWake));
        let mut cx = Context::from_waker(&waker);
        let mut future = future;
        match future.as_mut().poll(&mut cx) {
            Poll::Ready(value) => value,
            Poll::Pending => panic!("test future unexpectedly pending"),
        }
    }

    struct EchoAsyncNode {
        prepared: bool,
    }

    impl AsyncNode for EchoAsyncNode {
        fn prepare<'a>(
            &'a mut self,
            _cx: &'a PrepareContext,
        ) -> AsyncNodeFuture<'a, Result<(), NodeError>> {
            Box::pin(async move {
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
                    return Err(NodeError::Process("async node not prepared".to_owned()));
                }
                Ok(Some(input))
            })
        }
    }

    #[test]
    fn given_text_signal_when_signal_spec_then_text_class_is_returned() {
        let signal = AsyncSignal::Text("partial transcript".to_owned());
        assert!(signal
            .signal_spec()
            .class
            .is_compatible_with(&SignalSpec::text(crate::signal::TextFormat::Utf8).class));
    }

    #[test]
    fn given_echo_async_node_when_process_after_prepare_then_envelope_is_returned() {
        let mut node = EchoAsyncNode { prepared: false };
        block_on_ready(node.prepare(&prepare_cx())).unwrap();

        let envelope = AsyncEnvelope::new(AsyncSignal::Control(Vec::new()), 7, 9);
        let output = block_on_ready(node.process(envelope)).unwrap().unwrap();

        assert_eq!(output.sequence_number, 7);
        assert_eq!(output.timestamp_ns, 9);
        assert!(matches!(output.signal, AsyncSignal::Control(_)));
    }

    #[test]
    fn given_echo_async_node_when_process_before_prepare_then_error_is_returned() {
        let mut node = EchoAsyncNode { prepared: false };
        let envelope = AsyncEnvelope::new(AsyncSignal::Control(Vec::new()), 0, 0);
        let error = block_on_ready(node.process(envelope)).unwrap_err();

        assert!(matches!(error, NodeError::Process(_)));
    }
}
