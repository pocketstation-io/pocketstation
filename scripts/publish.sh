#!/usr/bin/env bash
# Validate or publish the one PocketStation Cargo package.
set -euo pipefail

dry_run=false
case "${1:-}" in
  --dry-run) dry_run=true ;;
  "") ;;
  *) echo "usage: $0 [--dry-run]" >&2; exit 2 ;;
esac

for command in cargo curl jq; do
  command -v "${command}" >/dev/null 2>&1 || {
    echo "required command is unavailable: ${command}" >&2
    exit 2
  }
done

repo_root="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
metadata="$(cargo metadata --manifest-path "${repo_root}/Cargo.toml" --no-deps --format-version 1)"

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

cargo package --manifest-path "${repo_root}/Cargo.toml" --locked
if [[ "${dry_run}" == true ]]; then
  cargo publish --manifest-path "${repo_root}/Cargo.toml" --locked --dry-run
else
  registry_api_base_url="${PKS_REGISTRY_API_BASE_URL:-https://crates.io/api/v1/crates}"
  registry_url="${registry_api_base_url}/${package_name}/${package_version}"
  if ! registry_status="$(
    curl \
      --silent \
      --show-error \
      --output /dev/null \
      --write-out '%{http_code}' \
      --header 'User-Agent: pocketstation-release-publisher' \
      "${registry_url}"
  )"; then
    echo "registry visibility query failed: ${registry_url}" >&2
    exit 1
  fi

  case "${registry_status}" in
    200)
      echo "${package_name} ${package_version} is already visible on crates.io; publication is complete"
      ;;
    404)
      cargo publish --manifest-path "${repo_root}/Cargo.toml" --locked
      ;;
    *)
      echo "registry visibility query returned HTTP ${registry_status}: ${registry_url}" >&2
      exit 1
      ;;
  esac
fi
