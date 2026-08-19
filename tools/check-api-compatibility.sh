#!/usr/bin/env bash
set -euo pipefail

repo_root="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd -P)"
baseline_ref="${PKS_API_BASELINE_REF:-pocketstation-v1.1.0}"
expected_baseline_commit="${PKS_API_BASELINE_COMMIT:-879146f665e7d9e20e8f44822d8dd99d3aa58076}"
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

if [[ "${status}" -eq 0 ]]; then
  echo "Rust API compatibility: PASS"
  echo "baseline_ref=${baseline_ref}"
  echo "baseline_commit=${actual_baseline_commit}"
  echo "tool=${required_tool_version}"
  exit 0
fi

expected_lints="$(cat <<'EOF'
enum_no_repr_variant_discriminant_changed
enum_variant_missing
inherent_method_missing
inherent_method_unsafe_added
method_parameter_count_changed
trait_missing
EOF
)"
actual_lints="$(sed -n 's/^--- failure \([^:]*\):.*/\1/p' "${report}" | sort -u)"
[[ "${actual_lints}" == "${expected_lints}" ]] || {
  echo "unapproved Rust API break category detected" >&2
  exit 1
}

approved_symbols=(
  "ConnectorManifestError::InvalidManifestEntry"
  "ConnectorManifestError::ManifestEntryTooLarge"
  "ConnectorManifestError::TooManyManifestEntries"
  "ConnectorManifestError::DuplicateManifestEntry"
  "ConnectorManifestError::DeliveryMediaMismatch"
  "ConnectorManifest::input_edge"
  "Connector::managed"
  "RegisteredConnector::declare_with_input_edge"
  "Session::load_native_extension_library"
  "pocketstation::connector::ConnectorManifest::new"
  "pocketstation::connector::RegisteredConnector::declare"
  "pocketstation::connector::ManagedConnector"
  "pocketstation::connector::ManagedConnectorFactory"
)
for symbol in "${approved_symbols[@]}"; do
  grep -F "${symbol}" "${report}" >/dev/null || {
    echo "accepted Rust API break is missing from the report: ${symbol}" >&2
    exit 1
  }
done

[[ "$(grep -c '^--- failure ' "${report}")" -eq 6 ]] || {
  echo "unexpected Rust API break count" >&2
  exit 1
}
grep -F "Summary semver requires new major version: 6 major and 0 minor checks failed" \
  "${report}" >/dev/null || {
  echo "Rust API break summary differs from the accepted surface" >&2
  exit 1
}

echo "Rust API compatibility: INTENTIONAL_BREAK_ACCEPTED"
echo "policy=cleaned connector and native trust surface remains on 1.1.x by owner decision"
echo "baseline_ref=${baseline_ref}"
echo "baseline_commit=${actual_baseline_commit}"
echo "tool=${required_tool_version}"
