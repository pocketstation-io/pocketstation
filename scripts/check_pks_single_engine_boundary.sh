#!/usr/bin/env bash
set -euo pipefail

engine_root="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
pks_root="$(cd "$engine_root/../pks" && pwd)"
pks_source="$pks_root/src"
connector_source="$pks_root/connectors"

forbidden_engine_symbols='pocketstation::internal|RealtimePlanExecutor|RuntimePlanner|NodeRegistry|AudioBufferPool|CallbackCaptureBackend|PreparedCaptureBackend|PlanEdge(Sender|Receiver|Frame)|\brtrb\b'
if rg -n "$forbidden_engine_symbols" "$pks_source"; then
  echo "pks shipping source owns or imports PocketStation engine machinery" >&2
  exit 1
fi

if rg -n 'pocketstation::internal|internal-testing' "$connector_source" "$pks_root/Cargo.toml"; then
  echo "pks or an external connector bypasses the public PocketStation contract" >&2
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

if rg -n '\b(print|println)!' "$connector_source"; then
  echo "external connector writes command output to stdout" >&2
  exit 1
fi

if rg -n 'STATE_INDICATOR|MeterState|render_level_bar' "$pks_source"; then
  echo "pks fabricates a source measurement outside an active Session" >&2
  exit 1
fi

for retired_path in \
  "$pks_root/cli.zip" \
  "$pks_root/artifacts/live-proof" \
  "$pks_root/docs/architecture/pocketstation-v3.0.md" \
  "$pks_root/docs/roadmap/PHASE3_PROGRESS.md"; do
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
rg -q 'pocketstation-example-process-connector' "$pks_root/Cargo.toml"
rg -q 'pocketstation-relay-connector' "$pks_root/Cargo.toml"

echo "pks single-engine ownership boundary: PASS"
