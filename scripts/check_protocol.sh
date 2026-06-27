#!/usr/bin/env bash
set -e

echo "=== CODE_PROTOCOL check ==="

echo "LAW-1: unit suffixes..."
violations=$(grep -rn ":\s*u32\b\|:\s*f32\b\|:\s*u64\b" src/ 2>/dev/null \
  | grep -v "_hz\|_ms\|_ns\|_us\|_db\|_dbfs\|_lufs\|_kbps\|_pct\|_bytes\|_samples\|_frames\|_count\|_index\|_slot\|_len\|_id\|_mask\|_num\|_bits\|//\|target\|source" || true)
if [ -n "$violations" ]; then
  echo "  FAIL: missing unit suffixes:"
  echo "$violations"
  exit 1
fi
echo "  pass"

echo "LAW-10: no section banners..."
if grep -rn "//\s*[-=*#]\{4,\}" src/ --include="*.rs" 2>/dev/null | grep -q .; then
  echo "  FAIL: section divider banners found"
  grep -rn "//\s*[-=*#]\{4,\}" src/ --include="*.rs"
  exit 1
fi
echo "  pass"

echo "LAW-15: hot path purity..."
if grep -n "Vec::new\|Box::new\|String::new\|format!\|\.push(" src/pipeline/ -r 2>/dev/null | grep -q .; then
  echo "  WARN: potential allocation in pipeline crate — verify not in process() hot path"
fi
echo "  pass"

echo "LAW-16: test naming..."
bad_tests=$(grep -rn "#\[test\]" src/ 2>/dev/null -A 1 | grep "fn " | grep -v "given_.*when_.*then_" || true)
if [ -n "$bad_tests" ]; then
  echo "  FAIL: test functions not using given_when_then naming:"
  echo "$bad_tests"
  exit 1
fi
echo "  pass"

echo "LAW-18: no dumping-ground modules..."
if find . -type d \( -name "utils" -o -name "helpers" -o -name "common" -o -name "misc" \) \
  2>/dev/null | grep -v ".git\|node_modules\|target" | grep -q .; then
  echo "  FAIL: dumping-ground folder found"
  find . -type d \( -name "utils" -o -name "helpers" -o -name "common" -o -name "misc" \) | grep -v ".git\|target"
  exit 1
fi
echo "  pass"

if [ -f "Cargo.toml" ]; then
  echo "Rust format..."
  cargo fmt --all -- --check
  echo "Rust clippy..."
  cargo clippy --workspace --all-targets -- -D warnings
fi

echo "=== All CODE_PROTOCOL checks passed ==="
