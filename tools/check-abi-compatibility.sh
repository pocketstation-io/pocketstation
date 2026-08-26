#!/usr/bin/env bash
set -euo pipefail

repo_root="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd -P)"
baseline="${repo_root}/docs/compatibility/c-abi-v1.baseline"
baseline_crate="${PKS_ABI_BASELINE_CRATE:?set PKS_ABI_BASELINE_CRATE to the accepted pocketstation 0.1.2 crate}"
expected_baseline_sha="${PKS_ABI_BASELINE_SHA256:-$(awk -F= '$1 == "baseline_crate_sha256" {print $2}' "${baseline}")}"
expected_header_sha="$(awk -F= '$1 == "header_sha256" {print $2}' "${baseline}")"

[[ -f "${baseline_crate}" ]] || {
  echo "accepted C ABI baseline crate is missing: ${baseline_crate}" >&2
  exit 1
}
actual_baseline_sha="$(shasum -a 256 "${baseline_crate}" | awk '{print $1}')"
[[ "${actual_baseline_sha}" == "${expected_baseline_sha}" ]] || {
  echo "C ABI baseline crate hash mismatch" >&2
  exit 1
}
scratch="$(mktemp -d "${TMPDIR:-/tmp}/pks-abi-compatibility.XXXXXX")"
trap 'rm -rf "${scratch}"' EXIT
tar -xf "${baseline_crate}" -C "${scratch}"
baseline_source="$(find "${scratch}" -mindepth 1 -maxdepth 1 -type d -print -quit)"
[[ -n "${baseline_source}" ]] || {
  echo "C ABI baseline archive has no package root" >&2
  exit 1
}
actual_baseline_header_sha="$(shasum -a 256 "${baseline_source}/include/pocketstation.h" | awk '{print $1}')"
[[ "${actual_baseline_header_sha}" == "${expected_header_sha}" ]] || {
  echo "accepted C ABI baseline header hash mismatch" >&2
  exit 1
}
actual_header_sha="$(shasum -a 256 "${repo_root}/include/pocketstation.h" | awk '{print $1}')"

cc -std=c11 -Wall -Wextra -Werror \
  -I"${repo_root}/include" "${repo_root}/tools/abi-layout-probe.c" \
  -o "${scratch}/current-layout"
"${scratch}/current-layout" >"${scratch}/current-layout.txt"
awk '/^(type|field) / {print}' "${baseline}" >"${scratch}/expected-layout.txt"
awk '/^(type|field) / {print}' "${scratch}/current-layout.txt" >"${scratch}/current-layout-contract.txt"
sort -u "${scratch}/expected-layout.txt" -o "${scratch}/expected-layout.txt"
sort -u "${scratch}/current-layout-contract.txt" -o "${scratch}/current-layout-contract.txt"
comm -23 "${scratch}/expected-layout.txt" "${scratch}/current-layout-contract.txt" \
  >"${scratch}/missing-layout.txt"
[[ ! -s "${scratch}/missing-layout.txt" ]] || {
  echo "accepted C ABI type or field layout is missing or changed:" >&2
  cat "${scratch}/missing-layout.txt" >&2
  exit 1
}

cargo build --manifest-path "${repo_root}/Cargo.toml" --locked --release --lib
case "$(uname -s)" in
  Darwin)
    native_library="${repo_root}/target/release/libpocketstation.dylib"
    nm -gU "${native_library}" \
      | awk '$2 ~ /^[TDS]$/ && $3 ~ /^_pks_/ {sub(/^_/, "", $3); print $3}' \
      | sort -u >"${scratch}/current-symbols.txt"
    ;;
  Linux)
    native_library="${repo_root}/target/release/libpocketstation.so"
    nm -D --defined-only "${native_library}" \
      | awk '$2 ~ /^[TDBR]$/ && $3 ~ /^pks_/ {print $3}' \
      | sort -u >"${scratch}/current-symbols.txt"
    ;;
  *)
    echo "unsupported ABI-symbol inspection host: $(uname -s)" >&2
    exit 1
    ;;
esac
[[ -f "${native_library}" ]] || {
  echo "native library was not built: ${native_library}" >&2
  exit 1
}
awk '/^symbol / {print $2}' "${baseline}" | sort -u >"${scratch}/expected-symbols.txt"
comm -23 "${scratch}/expected-symbols.txt" "${scratch}/current-symbols.txt" \
  >"${scratch}/missing-symbols.txt"
[[ ! -s "${scratch}/missing-symbols.txt" ]] || {
  echo "accepted C ABI symbol is missing:" >&2
  cat "${scratch}/missing-symbols.txt" >&2
  exit 1
}

rg -q '^#define PKS_SESSION_ABI_MAJOR 1u$' "${repo_root}/include/pocketstation.h"
rg -q '^#define PKS_SESSION_ABI_MINOR 1u$' "${repo_root}/include/pocketstation.h"
rg -q '^#define PKS_EXTENSION_ABI_MAJOR 1u$' "${repo_root}/include/pocketstation.h"
rg -q '^#define PKS_EXTENSION_ABI_MINOR 2u$' "${repo_root}/include/pocketstation.h"
cargo test --manifest-path "${repo_root}/Cargo.toml" --locked \
  --test protocol_compatibility

echo "C ABI and PKSS compatibility: PASS"
echo "baseline_sha256=${actual_baseline_sha}"
echo "header_sha256=${actual_header_sha}"
