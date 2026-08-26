#!/usr/bin/env bash
set -euo pipefail

engine_root="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
pks_root="$engine_root/../pks"

if [[ ! -d "$pks_root/src" ]]; then
  echo "pks single-engine ownership boundary: NOT OBSERVABLE (sibling pks checkout absent)"
  echo "standalone Core checks continue; workspace qualification must run this gate with pks present"
  exit 0
fi

pks_root="$(cd "$pks_root" && pwd)"
pks_source="$pks_root/src"

forbidden_engine_symbols='pocketstation::internal|RealtimePlanExecutor|RuntimePlanner|NodeRegistry|AudioBufferPool|CallbackCaptureBackend|PreparedCaptureBackend|PlanEdge(Sender|Receiver|Frame)|\brtrb\b'
if rg -n "$forbidden_engine_symbols" "$pks_source"; then
  echo "pks shipping source owns or imports PocketStation engine machinery" >&2
  exit 1
fi

if rg -n 'pocketstation::internal|internal-testing' "$pks_root/Cargo.toml"; then
  echo "pks bypasses the public PocketStation contract" >&2
  exit 1
fi

if [[ -d "$pks_root/connectors" ]] && find "$pks_root/connectors" -type f | rg -q .; then
  echo "pks owns a connector implementation instead of consuming an external package" >&2
  exit 1
fi

if rg -n '^(rtrb|str0m|tungstenite|base64|hound)\s*=' "$pks_root/Cargo.toml"; then
  echo "pks directly depends on runtime, codec, or transport implementation crates" >&2
  exit 1
fi

if find "$pks_source/commands" -type f \( -name 'capture.rs' -o -name 'proof.rs' -o -name 'publish.rs' -o -name 'source.rs' \) | rg -q .; then
  echo "pks contains a retired parallel capture, proof, publish, or source engine" >&2
  exit 1
fi

for required_cli_authority in cli.rs context.rs error.rs output.rs shutdown.rs; do
  [ -f "$pks_source/$required_cli_authority" ] || {
    echo "pks is missing CLI authority: $required_cli_authority" >&2
    exit 1
  }
done

if rg -n 'STATE_INDICATOR|MeterState|render_level_bar' "$pks_source"; then
  echo "pks fabricates a source measurement outside an active Session" >&2
  exit 1
fi

for retired_path in \
  "$pks_root/cli.zip" \
  "$pks_root/artifacts/live-proof" \
  "$pks_root/docs/architecture/pocketstation-v3.0.md"; do
  [ ! -e "$retired_path" ] || {
    echo "pks retains stale generated or duplicate authority: $retired_path" >&2
    exit 1
  }
done

rg -q 'derive\(Parser\)' "$pks_source/cli.rs"
rg -q 'clap_complete::generate' "$pks_source/output.rs"
rg -q 'clap_mangen::Man' "$pks_source/output.rs"
rg -q 'io\.pocketstation\.pks\.command-result' "$pks_source/output.rs"

session_command="$pks_source/commands/session_run.rs"
rg -q 'Session::builder' "$session_command"
rg -q '\.capture\(' "$session_command"
rg -q '\.start\(' "$session_command"
rg -q '\.stop\(' "$session_command"
if rg -q 'pocketstation-example-process-connector' "$pks_root/Cargo.toml"; then
  echo "pks still depends on the retired process-shaped transcription connector" >&2
  exit 1
fi
rg -q '^whisper-transcribe-example\s*=' "$pks_root/Cargo.toml"
rg -q 'WhisperOperatorFactory' "$session_command"
rg -q 'TranscriptEndpoint' "$session_command"
rg -q '^pocketstation-relay\s*=' "$pks_root/Cargo.toml"

echo "pks single-engine ownership boundary: PASS"
