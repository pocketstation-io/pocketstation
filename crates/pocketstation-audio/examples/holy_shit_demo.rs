// Phase 0 exit gate — this example must compile and run.
// It exercises the full AudioGraph API surface defined in pocketstation-graph.
// run() is a scaffold (returns immediately); Phase 1 wires real scheduling.
// BUILD_GUIDE note: 'gain_db' replaces the conceptual 'db' shorthand per CODE_PROTOCOL LAW 1.

use pocketstation_graph::{
    deepgram, local_model, openai_realtime, AudioGraph, ModelNode, PolicyNode, SinkNode,
    SourceNode, TransformNode, TransportNode,
};

fn main() -> Result<(), pocketstation_graph::GraphError> {
    let mut graph = AudioGraph::new();

    let mic = graph.source(SourceNode::Mic);
    let discord = graph.source(SourceNode::App("Discord".to_owned()));
    let spotify = graph.source(SourceNode::App("Spotify".to_owned()));

    let vad = graph.transform(TransformNode::VAD);
    let stt = graph.model(ModelNode::Transcribe(deepgram()));
    let agent = graph.model(ModelNode::SpeechToSpeech(openai_realtime()));
    let emotion = graph.model(ModelNode::EmotionDetect(local_model()));
    let duck = graph.policy(PolicyNode::Duck {
        target: spotify.selector(),
        gain_db: -12.0,
        attack_ms: 40,
        release_ms: 400,
    });

    let relay = graph.transport(TransportNode::Relay("room-demo".to_owned()));
    let rec = graph.sink(SinkNode::MultiStemRecording("demo-session".to_owned()));
    let browser = graph.sink(SinkNode::Browser);

    graph.connect(mic.out("voice"), vad.in_("audio"))?;
    graph.connect(vad.out("speech"), stt.in_("audio"))?;
    graph.connect(vad.out("speech"), agent.in_("audio"))?;
    graph.connect(stt.out("transcript"), relay.in_("events"))?;

    graph.connect(discord.out("audio"), emotion.in_("audio"))?;
    graph.connect(emotion.out("stress_signal"), relay.in_("events"))?;

    graph.connect(
        [mic.out("voice"), discord.out("audio")],
        duck.in_("sidechain"),
    )?;
    graph.connect(spotify.out("music"), duck.in_("program"))?;
    graph.connect(duck.out("audio"), relay.in_("music"))?;

    graph.connect(
        [mic.out("voice"), discord.out("audio"), spotify.out("music")],
        rec.in_("stems"),
    )?;

    graph.connect(agent.out("audio"), relay.in_("agent_voice"))?;
    graph.connect(relay.out("mix"), browser.in_("audio"))?;

    let plan = graph.compile()?;

    println!(
        "holy_shit_demo: graph compiled — {} nodes, {} edges",
        plan.node_count(),
        plan.edge_count(),
    );

    graph.run(plan)?;

    println!("holy_shit_demo: run complete (Phase 0 scaffold — exits immediately)");
    Ok(())
}
