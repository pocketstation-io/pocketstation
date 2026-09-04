#!/usr/bin/env bash
set -euo pipefail

repo_root="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd -P)"
baseline_ref="${PKS_API_BASELINE_REF:-pocketstation-v1.1.7}"
expected_baseline_commit="${PKS_API_BASELINE_COMMIT:-172e2f270999b8880c98362d329117f676d666f2}"
required_tool_version="cargo-semver-checks 0.48.0"

actual_baseline_commit="$(git -C "${repo_root}" rev-parse "${baseline_ref}^{commit}")"
[[ "${actual_baseline_commit}" == "${expected_baseline_commit}" ]] || {
  echo "Rust API baseline ref does not resolve to its accepted commit" >&2
  exit 1
}
[[ "$(cargo semver-checks --version)" == "${required_tool_version}" ]] || {
  echo "expected ${required_tool_version}" >&2
  exit 1
}

report="$(mktemp "${TMPDIR:-/tmp}/pks-api-compatibility.XXXXXX")"
trap 'rm -f "${report}"' EXIT

set +e
cargo semver-checks check-release \
  --manifest-path "${repo_root}/Cargo.toml" \
  --baseline-rev "${baseline_ref}" \
  --default-features \
  --release-type minor \
  --color never >"${report}" 2>&1
status=$?
set -e
cat "${report}"

[[ "${status}" -eq 0 ]] || {
  echo "Rust API compatibility: FAIL" >&2
  exit "${status}"
}

echo "Rust API compatibility: PASS"
echo "baseline_ref=${baseline_ref}"
echo "baseline_commit=${actual_baseline_commit}"
echo "tool=${required_tool_version}"
