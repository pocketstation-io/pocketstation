#!/usr/bin/env bash
set -euo pipefail

script_dir="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
repo_root="$(cd "${script_dir}/.." && pwd)"
cd "${repo_root}"

echo "=== CODE_PROTOCOL check ==="

rust_files=()
while IFS= read -r file; do
  rust_files+=("$file")
done < <(rg --files src examples tests benches 2>/dev/null | rg '\.rs$' || true)

if [ "${#rust_files[@]}" -eq 0 ]; then
  echo "FAIL: no Rust source files found under the single package"
  exit 1
fi

echo "scope: full workspace (${#rust_files[@]} Rust files)"

echo "LAW-1: unit suffixes..."
violations="$(
  rg -nio --pcre2 \
    '\b(?=[A-Za-z0-9_]*(?:duration|latency|delay|timeout|interval|rate|frequency|gain|size|budget|count|sample|frame|byte|peak|loudness|bitrate|loss|timestamp|capacity))[A-Za-z_][A-Za-z0-9_]*\s*:\s*(?:u32|f32|i32|u64)\b' \
    "${rust_files[@]}" 2>/dev/null \
    | rg -iv '_hz|_ms|_s|_ns|_us|_db|_dbfs|_lufs|_kbps|_pct|_ratio|_linear|_bytes|_samples|_frames|_count|_total|_index|_slot|_len|_id|_mask|_num|_bits|//' \
    || true
)"
if [ -n "$violations" ]; then
  echo "  FAIL: missing unit suffixes:"
  echo "$violations"
  exit 1
fi
echo "  pass"

echo "LAW-10: no section banners..."
banners="$(
  rg -n '^\s*//\s*([-=*#]{4,}|─{4,})' "${rust_files[@]}" 2>/dev/null || true
)"
if [ -n "$banners" ]; then
  echo "  FAIL: section divider banners found:"
  echo "$banners"
  exit 1
fi
echo "  pass"

echo "LAW-14: every unsafe block has a SAFETY comment..."
unsafe_violations=""
while IFS=: read -r file line_number _; do
  context_start=$((line_number > 4 ? line_number - 4 : 1))
  context="$(sed -n "${context_start},${line_number}p" "$file")"
  if ! rg -q 'SAFETY:' <<<"$context"; then
    unsafe_violations+="${file}:${line_number}"$'\n'
  fi
done < <(rg -n 'unsafe \{' "${rust_files[@]}" 2>/dev/null || true)
if [ -n "$unsafe_violations" ]; then
  echo "  FAIL: unsafe blocks without a preceding SAFETY comment:"
  printf '%s' "$unsafe_violations"
  exit 1
fi
echo "  pass"

echo "LAW-15: hot path purity..."
cargo test --quiet --features internal-testing --test runtime_plan_router_alloc
cargo test --quiet --features internal-testing --test codec_hot_path_alloc
echo "  pass"

echo "LAW-16: test naming..."
bad_tests="$(
  rg -n -U --pcre2 \
    '#\[test\]\s*\n\s*fn\s+(?!given_[a-z0-9_]+_when_[a-z0-9_]+_then_[a-z0-9_]+)' \
    "${rust_files[@]}" 2>/dev/null || true
)"
if [ -n "$bad_tests" ]; then
  echo "  FAIL: test functions not using given_when_then naming:"
  echo "$bad_tests"
  exit 1
fi
echo "  pass"

echo "LAW-13: forbidden v2.3 vocabulary (room/listener/track)..."
vocab="$(
  rg -ni --pcre2 '\b(room|listener|track)\b' "${rust_files[@]}" 2>/dev/null \
    | rg -v '//|graph_session|bus_subscription' \
    || true
)"
if [ -n "$vocab" ]; then
  echo "  FAIL: v3.0 vocabulary required (Session/BusSubscription/AudioBus):"
  echo "$vocab"
  exit 1
fi
echo "  pass"

echo "LAW-18: no dumping-ground modules..."
dumping_grounds="$(
  find . -type d \( -name "utils" -o -name "helpers" -o -name "common" -o -name "misc" \) \
    2>/dev/null | rg -v '\.git|node_modules|target' || true
)"
if [ -n "$dumping_grounds" ]; then
  echo "  FAIL: dumping-ground folder found:"
  echo "$dumping_grounds"
  exit 1
fi
echo "  pass"

echo "LAW-22: semantic choices are typed..."
semantic_bools="$(
  rg -n --pcre2 \
    '\b(reason|cause|category|policy|mode|direction|role|strategy|ownership|outcome|overrun)\s*:\s*bool\b' \
    "${rust_files[@]}" 2>/dev/null || true
)"
if [ -n "$semantic_bools" ]; then
  echo "  FAIL: semantic boolean fields or parameters found:"
  echo "$semantic_bools"
  exit 1
fi
echo "  pass"

echo "Rust format..."
cargo fmt --all -- --check

echo "Rust clippy..."
cargo clippy --all-targets --all-features -- -D warnings

echo "=== All CODE_PROTOCOL checks passed ==="
