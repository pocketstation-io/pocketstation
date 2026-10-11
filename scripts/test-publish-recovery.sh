#!/usr/bin/env bash
# Real source checks with local Git; registry/package operations are MOCKED.
set -euo pipefail

script_dir="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
repo_root="$(cd "${script_dir}/.." && pwd)"
real_cargo="$(command -v cargo)"
temporary_root="$(mktemp -d "${TMPDIR:-/tmp}/pks-publish-recovery.XXXXXX")"
trap 'rm -rf "${temporary_root}"' EXIT
fail() { echo "publish recovery contract FAIL: $*" >&2; exit 1; }
fixture="${temporary_root}/repo"
tools="${temporary_root}/bin"
mkdir -p "${fixture}/scripts" "${tools}"
cp "${script_dir}/publish.sh" "${fixture}/scripts/publish.sh"
cp "${repo_root}/Cargo.toml" "${fixture}/Cargo.toml"
printf 'Tagged documentation\n' >"${fixture}/README.md"
"${real_cargo}" metadata --manifest-path "${repo_root}/Cargo.toml" --locked --no-deps --format-version 1 >"${temporary_root}/metadata.json"
package_version="$(jq -r '.packages[0].version' "${temporary_root}/metadata.json")"
release_tag="pocketstation-v${package_version}"

cat >"${tools}/cargo" <<'MOCK'
#!/usr/bin/env bash
set -euo pipefail
case "${1:-}" in
  metadata) cat "${PKS_TEST_METADATA}" ;;
  publish) printf 'publish\n' >>"${PKS_TEST_PUBLISH_LOG}" ;;
  *) exit 97 ;;
esac
MOCK
cat >"${tools}/curl" <<'MOCK'
#!/usr/bin/env bash
set -euo pipefail
if [[ "$#" == 2 && "$1" == --disable && "$2" == --version ]]; then
  printf 'curl %s mock-build\n' "${PKS_TEST_CURL_VERSION}"
  exit 0
fi
printf '%s\0' "$@" >"${PKS_TEST_QUERY_ARGUMENTS}"
printf '%s\n' "${*: -1}" >>"${PKS_TEST_QUERY_LOG}"
case "${PKS_TEST_REGISTRY_MODE}" in
  visible) printf '200' ;;
  missing) printf '404' ;;
  redirect) printf '302' ;;
  error) printf '503' ;;
  unavailable) printf 'mock-sensitive-marker\n' >&2; exit 7 ;;
  oversized) exit 63 ;;
  timeout) exit 28 ;;
  *) exit 96 ;;
esac
MOCK
chmod +x "${tools}/cargo" "${tools}/curl"
git -C "${fixture}" init --quiet --initial-branch=main
git -C "${fixture}" config core.hooksPath /dev/null
git -C "${fixture}" config gc.auto 0
git -C "${fixture}" config gc.autoDetach false
git -C "${fixture}" config maintenance.auto false
git -C "${fixture}" config user.name 'Publisher contract fixture'
git -C "${fixture}" config user.email 'publisher-fixture@example.invalid'
git -C "${fixture}" add Cargo.toml README.md scripts/publish.sh
git -C "${fixture}" commit --quiet -m 'Tagged publisher fixture'
git -C "${fixture}" tag "${release_tag}"
tag_commit="$(git -C "${fixture}" rev-parse HEAD)"
git clone --quiet --bare "${fixture}" "${temporary_root}/origin.git"
git --git-dir="${temporary_root}/origin.git" config core.hooksPath /dev/null
git --git-dir="${temporary_root}/origin.git" config gc.auto 0
git --git-dir="${temporary_root}/origin.git" config gc.autoDetach false
git --git-dir="${temporary_root}/origin.git" config maintenance.auto false
git -C "${fixture}" remote add origin "${temporary_root}/origin.git"
git -C "${fixture}" checkout --quiet --detach "${release_tag}"
event=release
controller_ref="refs/tags/${release_tag}"
controller_sha="${tag_commit}"
selected_tag="${release_tag}"
curl_version=8.4.0
registry_base=https://registry.invalid/crates

run_publisher() {
  local registry_mode="$1" output_file="$2"
  shift 2
  : >"${temporary_root}/publish.log"
  : >"${temporary_root}/query.log"
  : >"${temporary_root}/query-arguments.bin"
  env PATH="${tools}:${PATH}" \
    PKS_TEST_METADATA="${temporary_root}/metadata.json" \
    PKS_TEST_PUBLISH_LOG="${temporary_root}/publish.log" \
    PKS_TEST_QUERY_LOG="${temporary_root}/query.log" \
    PKS_TEST_QUERY_ARGUMENTS="${temporary_root}/query-arguments.bin" \
    PKS_TEST_REGISTRY_MODE="${registry_mode}" \
    PKS_TEST_CURL_VERSION="${curl_version}" \
    PKS_REGISTRY_API_BASE_URL="${registry_base}" \
    PKS_RELEASE_TAG="${selected_tag}" PKS_RELEASE_EVENT="${event}" \
    PKS_RELEASE_CONTROLLER_REF="${controller_ref}" \
    PKS_RELEASE_CONTROLLER_SHA="${controller_sha}" \
    bash "${fixture}/scripts/publish.sh" "$@" >"${output_file}" 2>&1
}
source_rejected() {
  local label="$1"
  if run_publisher visible "${temporary_root}/${label}.out" --check-source; then
    fail "${label} source was accepted"
  fi
  [[ ! -s "${temporary_root}/publish.log" && ! -s "${temporary_root}/query.log" ]] ||
    fail "${label} source reached registry or publication"
}

run_publisher visible "${temporary_root}/tag-source.out" --check-source
[[ ! -s "${temporary_root}/publish.log" && ! -s "${temporary_root}/query.log" ]] ||
  fail 'source check performed publication work'
rg -q "commit=${tag_commit}" "${temporary_root}/tag-source.out" || fail 'tag identity was omitted'
selected_tag=pocketstation-v0.0.0
source_rejected wrong-version
selected_tag="${release_tag}"
controller_sha=0000000000000000000000000000000000000000
source_rejected wrong-release-controller
controller_sha="${tag_commit}"
controller_ref=refs/heads/main
source_rejected wrong-release-ref
controller_ref="refs/tags/${release_tag}"
event=push
source_rejected unsupported-event
event=release
printf 'Changed tracked documentation\n' >"${fixture}/README.md"
source_rejected dirty-source
git -C "${fixture}" restore README.md
printf 'Untracked package input\n' >"${fixture}/injected.txt"
source_rejected untracked-source
rm "${fixture}/injected.txt"

# A tag outside main fails even when event and checkout agree.
git -C "${fixture}" checkout --quiet -b outside-main
printf 'Not merged\n' >"${fixture}/unmerged.txt"
git -C "${fixture}" add unmerged.txt
git -C "${fixture}" commit --quiet -m 'Not in main'
controller_sha="$(git -C "${fixture}" rev-parse HEAD)"
git -C "${fixture}" tag --force "${release_tag}" >/dev/null
source_rejected unmerged-tag
git -C "${fixture}" tag --force "${release_tag}" "${tag_commit}" >/dev/null
git -C "${fixture}" checkout --quiet --detach "${release_tag}"
controller_sha="${tag_commit}"

# Dispatch controls main, but newer main documentation cannot replace the tag.
git -C "${fixture}" checkout --quiet main
printf 'New main documentation\n' >"${fixture}/README.md"
git -C "${fixture}" add README.md
git -C "${fixture}" commit --quiet -m 'Newer workflow controller'
main_commit="$(git -C "${fixture}" rev-parse HEAD)"
git -C "${fixture}" push --quiet origin main
git -C "${fixture}" checkout --quiet --detach "${release_tag}"
event=workflow_dispatch
controller_ref=refs/heads/main
controller_sha="${main_commit}"
run_publisher visible "${temporary_root}/recovery-source.out" --check-source
rg -q "commit=${tag_commit}" "${temporary_root}/recovery-source.out" || fail 'recovery published main source'
git -C "${fixture}" checkout --quiet main
source_rejected main-product-checkout
git -C "${fixture}" checkout --quiet --detach "${release_tag}"
controller_sha="${tag_commit}"
source_rejected stale-main-controller
controller_sha="${main_commit}"
controller_ref=refs/heads/other
source_rejected non-main-controller
controller_ref=refs/heads/main

run_publisher visible "${temporary_root}/visible.out"
[[ ! -s "${temporary_root}/publish.log" ]] || fail 'visible version was republished'
rg -q 'no publication attempted' "${temporary_root}/visible.out" || fail 'visibility claimed source verification'
expected_url="https://registry.invalid/crates/pocketstation/${package_version}"
printf '%s\0' --disable --silent --show-error --proto '=https' \
  --proto-redir '=https' --max-redirs 0 --connect-timeout 3 --max-time 10 \
  --max-filesize 262144 --output /dev/null --write-out '%{http_code}' \
  --header 'User-Agent: pocketstation-release-publisher' "${expected_url}" \
  >"${temporary_root}/expected-query-arguments.bin"
cmp "${temporary_root}/expected-query-arguments.bin" "${temporary_root}/query-arguments.bin" ||
  fail 'query lost its config, HTTPS, redirect, time or byte boundary'
[[ "$(cat "${temporary_root}/query.log")" == "${expected_url}" ]] || fail 'query used a different version'
run_publisher missing "${temporary_root}/missing.out"
[[ "$(cat "${temporary_root}/publish.log")" == publish ]] || fail 'missing version was not published once'
for registry_mode in redirect error unavailable oversized timeout; do
  if run_publisher "${registry_mode}" "${temporary_root}/${registry_mode}.out"; then
    fail "${registry_mode} did not fail closed"
  fi
  [[ ! -s "${temporary_root}/publish.log" ]] || fail "${registry_mode} attempted publication"
  if rg -q 'registry\.invalid|mock-sensitive-marker' "${temporary_root}/${registry_mode}.out"; then
    fail 'registry error exposed URL or transport details'
  fi
done
curl_version=8.3.0
if run_publisher visible "${temporary_root}/old-curl.out"; then fail 'old curl accepted unknown-length bytes'; fi
[[ ! -s "${temporary_root}/query.log" && ! -s "${temporary_root}/publish.log" ]] || fail 'old curl queried registry'
curl_version=8.4.0
registry_base=http://registry.invalid/crates
if run_publisher visible "${temporary_root}/non-https.out"; then fail 'query accepted non-HTTPS'; fi
[[ ! -s "${temporary_root}/query.log" && ! -s "${temporary_root}/publish.log" ]] || fail 'non-HTTPS queried registry'

[[ "$(grep -Fc 'ref: refs/tags/${{ github.event.release.tag_name || inputs.release_tag }}' "${repo_root}/.github/workflows/publish.yml")" == 3 ]] || fail 'not all jobs select the tag'
[[ "$(grep -Fc 'run: bash scripts/publish.sh --check-source' "${repo_root}/.github/workflows/publish.yml")" == 3 ]] || fail 'not all jobs bind source and controller'
echo 'single-package publish recovery contract: PASS (local Git; registry/package operations MOCKED)'
