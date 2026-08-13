#!/usr/bin/env bash
set -euo pipefail

repo_root="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd -P)"
workspace_root="$(cd "${repo_root}/../.." && pwd -P)"
baseline_crate="${PKS_API_BASELINE_CRATE:-${workspace_root}/pocketstation-lab/artifacts/w20-final-requalification/pks-20260812-w20-final-requalification-9/run-1/package/pocketstation-0.1.2.crate}"
expected_baseline_sha="4b41973fe910c6571836e0393a7bcf300bd74f81b0acb1997cb9bb1b899a97d4"
required_tool_version="cargo-semver-checks 0.48.0"

[[ -f "${baseline_crate}" ]] || {
  echo "accepted Rust API baseline crate is missing: ${baseline_crate}" >&2
  exit 1
}
actual_baseline_sha="$(shasum -a 256 "${baseline_crate}" | awk '{print $1}')"
[[ "${actual_baseline_sha}" == "${expected_baseline_sha}" ]] || {
  echo "Rust API baseline hash mismatch" >&2
  exit 1
}
[[ "$(cargo semver-checks --version)" == "${required_tool_version}" ]] || {
  echo "expected ${required_tool_version}" >&2
  exit 1
}

scratch="$(mktemp -d "${TMPDIR:-/tmp}/pks-api-compatibility.XXXXXX")"
trap 'rm -rf "${scratch}"' EXIT
tar -xf "${baseline_crate}" -C "${scratch}"

cargo semver-checks check-release \
  --manifest-path "${repo_root}/Cargo.toml" \
  --baseline-root "${scratch}/pocketstation-0.1.2" \
  --default-features \
  --release-type minor \
  --color never

echo "Rust API compatibility: PASS"
echo "baseline_sha256=${actual_baseline_sha}"
echo "tool=${required_tool_version}"
