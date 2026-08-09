use std::path::PathBuf;

use pocketstation::operator::{
    AsyncNode, AsyncOperatorEdgePrepareContext, AsyncOperatorPrepareContext, AudioCaps,
    ChannelLayout, EdgeContract, ExecutionPartition, MediaCaps, PortDirection, SampleFormat,
    SignalEnvelope, SignalPayload, SignalSpec, TextFormat,
};
use whisper_transcribe_example::WhisperConnector;

fn usage() -> ! {
    eprintln!("usage: whisper-transcribe-example <whisper-cli> <model.bin> <mono-16khz.wav>");
    std::process::exit(2);
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut arguments = std::env::args_os().skip(1).map(PathBuf::from);
    let binary_path = arguments.next().unwrap_or_else(|| usage());
    let model_path = arguments.next().unwrap_or_else(|| usage());
    let wav_path = arguments.next().unwrap_or_else(|| usage());
    if arguments.next().is_some() {
        usage();
    }

    let wav_bytes = tokio::fs::read(wav_path).await?;
    let mut connector = WhisperConnector::new(binary_path, model_path, "en");
    let audio = MediaCaps::Audio(AudioCaps {
        sample_rate_hz: Some(16_000),
        frame_samples: None,
        channel_layout: ChannelLayout::Mono,
        format: SampleFormat::F32Interleaved,
    });
    let mut input_contract = EdgeContract::voice_default();
    input_contract.media = audio;
    let mut output_contract = EdgeContract::typed_default();
    output_contract.media = MediaCaps::Text;
    let prepare_context = AsyncOperatorPrepareContext::new(
        ExecutionPartition::BlockingWorker,
        vec![
            AsyncOperatorEdgePrepareContext::new(
                None,
                "audio",
                PortDirection::Input,
                SignalSpec::audio(),
                audio,
                input_contract,
                32,
            )?,
            AsyncOperatorEdgePrepareContext::new(
                None,
                "transcript",
                PortDirection::Output,
                SignalSpec::text(TextFormat::Utf8).with_role("transcript"),
                MediaCaps::Text,
                output_contract,
                8,
            )?,
        ],
    )?;
    connector.prepare(&prepare_context).await?;
    let output = connector
        .process(SignalEnvelope::untracked(
            SignalPayload::Binary(wav_bytes),
            0,
        ))
        .await?
        .into_iter()
        .next()
        .ok_or("Whisper connector returned no transcript")?;
    match output.payload {
        SignalPayload::Text(transcript) => println!("{transcript}"),
        _ => return Err("Whisper connector returned a non-text signal".into()),
    }
    Ok(())
}
