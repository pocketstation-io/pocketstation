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
            task.await
                .map_err(|e| NodeError::Process(format!("AEC native worker failed: {e}")))?;
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
            response
                .await
                .map_err(|_| NodeError::Prepare("AEC worker initialization failed".into()))?
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
            self.send(Command::Process {
                port: port.to_owned(),
                input: Box::new(input),
                reply,
            })?;
            response
                .await
                .map_err(|_| NodeError::Process("AEC processing response lost".into()))?
        })
    }

    fn flush<'a>(&'a mut self) -> AsyncNodeFuture<'a, Result<Vec<SignalEnvelope>, NodeError>> {
        Box::pin(async move {
            let (reply, response) = oneshot::channel();
            self.send(Command::Flush(reply))?;
            response
                .await
                .map_err(|_| NodeError::Process("AEC flush response lost".into()))?
        })
    }

    fn cancel<'a>(&'a mut self) -> AsyncNodeFuture<'a, Result<(), NodeError>> {
        Box::pin(async move { self.stop().await })
    }
    fn close<'a>(&'a mut self) -> AsyncNodeFuture<'a, Result<(), NodeError>> {
        Box::pin(async move { self.stop().await })
    }
}
