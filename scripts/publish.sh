#!/usr/bin/env bash
# Validate or publish the one PocketStation Cargo package.
set -euo pipefail

mode=publish
case "${1:-}" in
  --dry-run) mode=dry-run ;;
  --check-source) mode=check-source ;;
  "") ;;
  *) echo "usage: $0 [--dry-run|--check-source]" >&2; exit 2 ;;
esac
[[ "$#" -le 1 ]] || { echo "publisher accepts one operation" >&2; exit 2; }

for command in cargo git jq; do
  command -v "${command}" >/dev/null 2>&1 || {
    echo "required publication command is unavailable" >&2
    exit 2
  }
done

repo_root="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
metadata="$(cargo metadata --manifest-path "${repo_root}/Cargo.toml" --locked --no-deps --format-version 1)"
package_count="$(jq '.packages | length' <<<"${metadata}")"
workspace_count="$(jq '.workspace_members | length' <<<"${metadata}")"
package_name="$(jq -r '.packages[0].name' <<<"${metadata}")"
package_version="$(jq -r '.packages[0].version' <<<"${metadata}")"
registry_role="$(jq -r '.packages[0].metadata.pocketstation["registry-role"] // ""' <<<"${metadata}")"

if [[ "${package_count}" != "1" || "${workspace_count}" != "1" || "${package_name}" != "pocketstation" ]]; then
  echo "publish gate requires exactly one workspace package named pocketstation" >&2
  exit 1
fi
if [[ "${registry_role}" != "public-product" ]]; then
  echo "pocketstation must be the single public-product registry artifact" >&2
  exit 1
fi

check_source() {
  local expected_tag="pocketstation-v${package_version}"
  local tag_commit checkout_commit checkout_tree main_commit
  if [[ "${PKS_RELEASE_TAG:-}" != "${expected_tag}" ]]; then
    echo "release tag must match the checked-out package version" >&2
    return 1
  fi
  if ! tag_commit="$(git -C "${repo_root}" rev-parse --verify "refs/tags/${expected_tag}^{commit}" 2>/dev/null)"; then
    echo "selected release tag is unavailable" >&2
    return 1
  fi
  checkout_commit="$(git -C "${repo_root}" rev-parse HEAD)"
  checkout_tree="$(git -C "${repo_root}" rev-parse 'HEAD^{tree}')"
  if [[ "${checkout_commit}" != "${tag_commit}" ]]; then
    echo "checked-out source must equal the selected release tag" >&2
    return 1
  fi
  if [[ -n "$(git -C "${repo_root}" status --porcelain=v1 --untracked-files=normal)" ]]; then
    echo "release source must be clean" >&2
    return 1
  fi
  if ! git -C "${repo_root}" fetch --quiet --no-tags origin main >/dev/null 2>&1; then
    echo "release main reference is unavailable" >&2
    return 1
  fi
  main_commit="$(git -C "${repo_root}" rev-parse --verify refs/remotes/origin/main)"
  if ! git -C "${repo_root}" merge-base --is-ancestor "${tag_commit}" "${main_commit}"; then
    echo "release commit must be contained in origin/main" >&2
    return 1
  fi
  case "${PKS_RELEASE_EVENT:-}" in
    release)
      if [[ "${PKS_RELEASE_CONTROLLER_REF:-}" != "refs/tags/${expected_tag}" ||
        "${PKS_RELEASE_CONTROLLER_SHA:-}" != "${tag_commit}" ]]; then
        echo "release event must identify the selected tag commit" >&2
        return 1
      fi
      ;;
    workflow_dispatch)
      if [[ "${PKS_RELEASE_CONTROLLER_REF:-}" != "refs/heads/main" ||
        "${PKS_RELEASE_CONTROLLER_SHA:-}" != "${main_commit}" ]]; then
        echo "publication recovery must be controlled by current main" >&2
        return 1
      fi
      ;;
    *)
      echo "publication requires a release or guarded recovery event" >&2
      return 1
      ;;
  esac
  echo "publication source: commit=${checkout_commit} tree=${checkout_tree} version=${package_version}"
}

if [[ "${mode}" != dry-run || -n "${PKS_RELEASE_EVENT:-}" ]]; then
  check_source
fi
if [[ "${mode}" == check-source ]]; then
  exit 0
fi
if [[ "${mode}" == dry-run ]]; then
  cargo publish --manifest-path "${repo_root}/Cargo.toml" --locked --dry-run
else
  command -v curl >/dev/null 2>&1 || {
    echo "required publication command is unavailable" >&2
    exit 2
  }
  # curl 8.4 introduced enforcement for unknown-length transfers as well.
  curl_version="$(curl --disable --version 2>/dev/null)" || {
    echo "registry visibility query requires curl 8.4 or newer" >&2
    exit 1
  }
  if [[ ! "${curl_version}" =~ ^curl\ ([0-9]{1,4})\.([0-9]{1,4})\.[0-9]+ ]]; then
    echo "registry visibility query requires curl 8.4 or newer" >&2
    exit 1
  fi
  curl_major="${BASH_REMATCH[1]}"
  curl_minor="${BASH_REMATCH[2]}"
  if (( 10#${curl_major} < 8 || (10#${curl_major} == 8 && 10#${curl_minor} < 4) )); then
    echo "registry visibility query requires curl 8.4 or newer" >&2
    exit 1
  fi
  registry_api_base_url="${PKS_REGISTRY_API_BASE_URL:-https://crates.io/api/v1/crates}"
  if [[ "${registry_api_base_url}" != https://* ]]; then
    echo "registry visibility query requires HTTPS" >&2
    exit 1
  fi
  registry_url="${registry_api_base_url}/${package_name}/${package_version}"
  if ! registry_status="$(
    curl --disable \
      --silent \
      --show-error \
      --proto '=https' \
      --proto-redir '=https' \
      --max-redirs 0 \
      --connect-timeout 3 \
      --max-time 10 \
      --max-filesize 262144 \
      --output /dev/null \
      --write-out '%{http_code}' \
      --header 'User-Agent: pocketstation-release-publisher' \
      "${registry_url}" 2>/dev/null
  )"; then
    echo "registry visibility query failed" >&2
    exit 1
  fi
  case "${registry_status}" in
    200)
      echo "${package_name} ${package_version} is already visible on crates.io; no publication attempted"
      ;;
    404)
      cargo publish --manifest-path "${repo_root}/Cargo.toml" --locked
      ;;
    *)
      echo "registry visibility query returned an unexpected HTTP status" >&2
      exit 1
      ;;
  esac
fi
