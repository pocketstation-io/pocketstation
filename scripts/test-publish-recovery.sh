#!/usr/bin/env bash
set -euo pipefail

script_dir="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
publisher="${script_dir}/publish.sh"
real_cargo="$(command -v cargo)"
temporary_root="$(mktemp -d "${TMPDIR:-/tmp}/pks-publish-recovery.XXXXXX")"
trap 'rm -rf "${temporary_root}"' EXIT

fail() {
  echo "publish recovery contract FAIL: $*" >&2
  exit 1
}

fixture="${temporary_root}/fixture"
mkdir -p "${fixture}/bin"

cat >"${fixture}/bin/cargo" <<'EOF'
#!/usr/bin/env bash
set -euo pipefail
case "${1:-}" in
  metadata)
    exec "${PKS_TEST_REAL_CARGO}" "$@"
    ;;
  package)
    exit 0
    ;;
  publish)
    printf 'publish\n' >>"${PKS_TEST_PUBLISH_LOG}"
    exit 0
    ;;
  *)
    exit 97
    ;;
esac
EOF

cat >"${fixture}/bin/curl" <<'EOF'
#!/usr/bin/env bash
set -euo pipefail
printf '%s\n' "${*: -1}" >>"${PKS_TEST_QUERY_LOG}"
case "${PKS_TEST_REGISTRY_MODE}" in
  visible) printf '200' ;;
  missing) printf '404' ;;
  error) printf '503' ;;
  unavailable) exit 7 ;;
  *) exit 96 ;;
esac
EOF

chmod +x "${fixture}/bin/cargo" "${fixture}/bin/curl"

run_publisher() {
  local registry_mode="$1"
  local output_file="$2"

  : >"${fixture}/publish.log"
  : >"${fixture}/query.log"
  env \
    PATH="${fixture}/bin:${PATH}" \
    PKS_TEST_REAL_CARGO="${real_cargo}" \
    PKS_TEST_PUBLISH_LOG="${fixture}/publish.log" \
    PKS_TEST_QUERY_LOG="${fixture}/query.log" \
    PKS_TEST_REGISTRY_MODE="${registry_mode}" \
    PKS_REGISTRY_API_BASE_URL="https://registry.invalid/crates" \
    bash "${publisher}" >"${output_file}" 2>&1
}

run_publisher visible "${fixture}/visible.out"
[[ ! -s "${fixture}/publish.log" ]] ||
  fail "registry-visible version was republished"
rg -q 'publication is complete' "${fixture}/visible.out" ||
  fail "registry-visible recovery was not reported"

run_publisher missing "${fixture}/missing.out"
[[ "$(cat "${fixture}/publish.log")" == "publish" ]] ||
  fail "missing version was not published exactly once"

if run_publisher error "${fixture}/error.out"; then
  fail "unexpected registry status did not fail closed"
fi
[[ ! -s "${fixture}/publish.log" ]] ||
  fail "registry error attempted publication"

if run_publisher unavailable "${fixture}/unavailable.out"; then
  fail "unavailable registry did not fail closed"
fi
[[ ! -s "${fixture}/publish.log" ]] ||
  fail "unavailable registry attempted publication"

package_version="$(${real_cargo} metadata --no-deps --format-version 1 | jq -r '.packages[0].version')"
expected_url="https://registry.invalid/crates/pocketstation/${package_version}"
[[ "$(cat "${fixture}/query.log")" == "${expected_url}" ]] ||
  fail "registry recovery queried a non-exact package version"

echo "single-package publish recovery contract: PASS"
