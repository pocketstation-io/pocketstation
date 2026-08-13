#!/usr/bin/env bash
set -euo pipefail

repo_root="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
compiler="$repo_root/src/session/compile/mod.rs"
engine="$repo_root/src/session/lifecycle/engine.rs"
components="$repo_root/src/session/extensions/structural_nodes.rs"

if rg -n \
  'Source::(Application|Microphone)|APPLICATION_SOURCE_NODE_TYPE_ID|MICROPHONE_SOURCE_NODE_TYPE_ID|EXTERNAL_AUDIO_INGRESS_NODE_TYPE_ID|source\.(application|microphone|external-audio-ingress)' \
  "$compiler"; then
  echo "generic Session compiler contains a built-in product-node switch" >&2
  exit 1
fi

if rg -n \
  'STRUCTURAL_NODE_TYPE_IDS|APPLICATION_SOURCE_NODE_TYPE_ID|MICROPHONE_SOURCE_NODE_TYPE_ID|EXTERNAL_AUDIO_INGRESS_NODE_TYPE_ID|GENERATED_AUDIO_(INGRESS|BRIDGE)_NODE_TYPE_ID' \
  "$engine"; then
  echo "Session engine bootstrap contains a fixed structural node list" >&2
  exit 1
fi

if rg -n 'STRUCTURAL_NODE_TYPE_IDS' "$components"; then
  echo "Session component registration still relies on a fixed node-type authority" >&2
  exit 1
fi

rg -q 'struct BuiltinSourceLowerer' "$components"
rg -q 'SessionSourceLoweringContext' "$components"
rg -q '\.source_registry' "$components"
rg -q 'Arc::new\(BuiltinSourceLowerer\)' "$components"
rg -q 'audio_reentry_lowerer\(\)' "$components"

echo "registered built-in Session lowering boundary: PASS"
