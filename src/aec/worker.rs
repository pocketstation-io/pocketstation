use super::{processor::EchoProcessor, AecConfiguration, ObservationState};
use crate::{
    AsyncNode, AsyncNodeFuture, AsyncOperatorPrepareContext, NodeError, OperatorId, SignalEnvelope,
};
use std::sync::mpsc::{self, SyncSender};
use tokio::sync::oneshot;
use tokio::task::JoinHandle;

type Reply = oneshot::Sender<Result<Vec<SignalEnvelope>, NodeError>>;

enum Command {
    Process {
        port: String,
        input: Box<SignalEnvelope>,
        reply: Reply,
    },
    Flush(Reply),
}

/// Tracks cancellation of a lifecycle future, not the completion of native work.
/// This guard only lives on the asynchronous worker boundary, never a callback.
struct PendingResponse {
    observations: ObservationState,
    completed: bool,
}

impl PendingResponse {
    fn new(observations: ObservationState) -> Self {
        Self {
            observations,
            completed: false,
        }
    }

    fn complete<T>(&mut self, result: &Result<T, NodeError>) {
        if let Err(error) = result {
            self.observations.fail(error.to_string());
        }
        self.completed = true;
    }
}

impl Drop for PendingResponse {
    fn drop(&mut self) {
        if !self.completed {
            self.observations.interrupt();
        }
    }
}

/// One blocking worker owns the engine for its lifetime; no per-frame task spawn.
/// Replies yield the Session executor. The queue holds one pending command.
pub(crate) struct AecWorker {
    configuration: AecConfiguration,
    operator_id: OperatorId,
    observations: ObservationState,
    sender: Option<SyncSender<Command>>,
    task: Option<JoinHandle<()>>,
}

impl AecWorker {
    pub(crate) fn new(
        configuration: AecConfiguration,
        operator_id: OperatorId,
        observations: ObservationState,
    ) -> Self {
        Self {
            configuration,
            operator_id,
            observations,
            sender: None,
            task: None,
        }
    }

    fn send(&self, command: Command) -> Result<(), NodeError> {
        self.sender
            .as_ref()
            .ok_or_else(|| NodeError::Process("AEC worker is stopped".into()))?
            .try_send(command)
            .map_err(|e| NodeError::Process(format!("AEC command was not accepted: {e}")))
    }

    async fn stop(&mut self) -> Result<(), NodeError> {
        // Closing the queue also works after failure or a cancelled response.
        // No stop message can be stranded behind a full command queue.
        self.sender.take();
        if let Some(task) = self.task.take() {
            if let Err(error) = task.await {
                let message = format!("AEC native worker failed: {error}");
                self.observations.fail(message.clone());
                return Err(NodeError::Process(message));
            }
        }
        Ok(())
    }
}

impl AsyncNode for AecWorker {
    fn prepare<'a>(
        &'a mut self,
        cx: &'a AsyncOperatorPrepareContext,
    ) -> AsyncNodeFuture<'a, Result<(), NodeError>> {
        Box::pin(async move {
            if cx.inputs().len() != 2 || self.task.is_some() {
                return Err(NodeError::Prepare(
                    "AEC requires two inputs and one worker lifecycle".into(),
                ));
            }
            let (sender, receiver) = mpsc::sync_channel(1);
            let (ready, response) = oneshot::channel();
            let configuration = self.configuration;
            let operator_id = self.operator_id.clone();
            let observations = self.observations.clone();
            // The runtime retains ownership if preparation or close is cancelled.
            // Dropping this object's sender closes the queue; the worker then exits.
            let task = tokio::task::spawn_blocking(move || {
                let mut engine = EchoProcessor::new(configuration, operator_id, observations);
                if let Err(error) = engine.prepare() {
                    engine.fail(&error);
                    let _ = ready.send(Err(error));
                    return;
                }
                if ready.send(Ok(())).is_err() {
                    engine.stop();
                    return;
                }
                while let Ok(command) = receiver.recv() {
                    let (result, reply) = match command {
                        Command::Process { port, input, reply } => {
                            (engine.accept(&port, *input), reply)
                        }
                        Command::Flush(reply) => (engine.flush(), reply),
                    };
                    if let Err(error) = &result {
                        engine.fail(error);
                    }
                    let failed = result.is_err();
                    let _ = reply.send(result);
                    if failed {
                        break;
                    }
                }
                engine.stop();
            });
            self.sender = Some(sender);
            self.task = Some(task);
            let mut pending = PendingResponse::new(self.observations.clone());
            let result = response
                .await
                .map_err(|_| NodeError::Prepare("AEC worker initialization failed".into()))
                .and_then(|result| result);
            pending.complete(&result);
            result
        })
    }

    fn process<'a>(
        &'a mut self,
        _input: SignalEnvelope,
    ) -> AsyncNodeFuture<'a, Result<Vec<SignalEnvelope>, NodeError>> {
        Box::pin(async { Err(NodeError::Process("AEC requires named inputs".into())) })
    }

    fn process_port<'a>(
        &'a mut self,
        port: &'a str,
        input: SignalEnvelope,
    ) -> AsyncNodeFuture<'a, Result<Vec<SignalEnvelope>, NodeError>> {
        Box::pin(async move {
            let (reply, response) = oneshot::channel();
            let mut pending = PendingResponse::new(self.observations.clone());
            let submitted = self.send(Command::Process {
                port: port.to_owned(),
                input: Box::new(input),
                reply,
            });
            if let Err(error) = submitted {
                let result = Err(error);
                pending.complete(&result);
                return result;
            }
            let result = response
                .await
                .map_err(|_| NodeError::Process("AEC processing response lost".into()))
                .and_then(|result| result);
            pending.complete(&result);
            result
        })
    }

    fn flush<'a>(&'a mut self) -> AsyncNodeFuture<'a, Result<Vec<SignalEnvelope>, NodeError>> {
        Box::pin(async move {
            let (reply, response) = oneshot::channel();
            let mut pending = PendingResponse::new(self.observations.clone());
            if let Err(error) = self.send(Command::Flush(reply)) {
                let result = Err(error);
                pending.complete(&result);
                return result;
            }
            let result = response
                .await
                .map_err(|_| NodeError::Process("AEC flush response lost".into()))
                .and_then(|result| result);
            pending.complete(&result);
            result
        })
    }

    fn cancel<'a>(&'a mut self) -> AsyncNodeFuture<'a, Result<(), NodeError>> {
        Box::pin(async move { self.stop().await })
    }
    fn close<'a>(&'a mut self) -> AsyncNodeFuture<'a, Result<(), NodeError>> {
        Box::pin(async move { self.stop().await })
    }
}

#[cfg(test)]
#[path = "worker_tests.rs"]
mod tests;
