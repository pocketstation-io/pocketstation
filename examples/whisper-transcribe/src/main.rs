use std::path::PathBuf;

use pocketstation::operator::{
    AsyncNode, PrepareContext, SampleFormat, SampleSpec, SignalEnvelope, SignalPayload,
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
    connector
        .prepare(&PrepareContext::new(SampleSpec::new(
            16_000,
            1,
            SampleFormat::F32Interleaved,
        )))
        .await?;
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
