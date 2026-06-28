// Fundable-demo topology, expressed as a typed GraphSpec.
//
// Wave 3: the builder assembles a GraphSpec and never executes. Compilation
// (verification + RuntimePlan) lands in Waves 4–6; this example will then print
// the compiled plan and per-edge metric IDs. Fan-in is expressed as multiple
// edges into one input port; the compiler's fan-in-mix pass lowers it (Wave 5).

use pocketstation_graph::{AudioGraph, NodeConfig};

fn main() {
    let mut graph = AudioGraph::new();

    let mic = graph.add_node("source.mic", NodeConfig::new());
    let discord = graph.add_node("source.app", NodeConfig::new().with("app", "Discord"));
    let spotify = graph.add_node("source.app", NodeConfig::new().with("app", "Spotify"));

    let vad = graph.add_node("transform.vad", NodeConfig::new());
    let stt = graph.add_node(
        "model.transcribe",
        NodeConfig::new().with("provider", "deepgram"),
    );
    let agent = graph.add_node(
        "model.speech_to_speech",
        NodeConfig::new().with("provider", "openai-realtime"),
    );
    let emotion = graph.add_node(
        "model.emotion_detect",
        NodeConfig::new().with("provider", "local"),
    );
    let duck = graph.add_node(
        "policy.duck",
        NodeConfig::new()
            .with("gain_db", "-12.0")
            .with("attack_ms", "40")
            .with("release_ms", "400"),
    );

    let relay = graph.add_node("transport.relay", NodeConfig::new().with("session", "demo"));
    let rec = graph.add_node(
        "sink.multistem_recording",
        NodeConfig::new().with("session", "demo-session"),
    );
    let browser = graph.add_node("sink.browser", NodeConfig::new());

    graph.connect(mic.out("voice"), vad.in_("audio"));
    graph.connect(vad.out("speech"), stt.in_("audio"));
    graph.connect(vad.out("speech"), agent.in_("audio"));
    graph.connect(stt.out("transcript"), relay.in_("events"));

    graph.connect(discord.out("audio"), emotion.in_("audio"));
    graph.connect(emotion.out("stress_signal"), relay.in_("events"));

    // sidechain fan-in: mic + discord both drive the duck sidechain
    graph.connect(mic.out("voice"), duck.in_("sidechain"));
    graph.connect(discord.out("audio"), duck.in_("sidechain"));
    graph.connect(spotify.out("music"), duck.in_("program"));
    graph.connect(duck.out("audio"), relay.in_("music"));

    // multi-stem fan-in: all three sources recorded as separate stems
    graph.connect(mic.out("voice"), rec.in_("stems"));
    graph.connect(discord.out("audio"), rec.in_("stems"));
    graph.connect(spotify.out("music"), rec.in_("stems"));

    graph.connect(agent.out("audio"), relay.in_("agent_voice"));
    graph.connect(relay.out("mix"), browser.in_("audio"));

    let spec = graph.into_spec();

    println!(
        "holy_shit_demo: GraphSpec built — {} nodes, {} edges",
        spec.node_count(),
        spec.edge_count(),
    );
    let type_ids: Vec<&str> = spec.nodes.iter().map(|n| n.type_id.as_str()).collect();
    println!("holy_shit_demo: node types: {type_ids:?}");
}
