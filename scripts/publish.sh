#!/usr/bin/env bash
# Publish workspace crates in the dependency order reported by Cargo metadata.
# Usage: ./scripts/publish.sh [--dry-run]
set -euo pipefail

dry_run=false
if [[ "${1:-}" == "--dry-run" ]]; then
    dry_run=true
elif [[ -n "${1:-}" ]]; then
    echo "usage: $0 [--dry-run]" >&2
    exit 2
fi

for command in cargo jq tsort; do
    if ! command -v "${command}" >/dev/null 2>&1; then
        echo "required command is unavailable: ${command}" >&2
        exit 2
    fi
done

repo_root="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
metadata_file="$(mktemp "${TMPDIR:-/tmp}/pks-publish-metadata.XXXXXX")"
edges_file="$(mktemp "${TMPDIR:-/tmp}/pks-publish-edges.XXXXXX")"
trap 'rm -f "${metadata_file}" "${edges_file}"' EXIT

cargo metadata \
    --manifest-path "${repo_root}/Cargo.toml" \
    --format-version 1 \
    --no-deps >"${metadata_file}"

jq -r '
    .workspace_members as $workspace_members
    | [.packages[]
      | select(.publish != []
        and (.id as $id | any($workspace_members[]; . == $id)))] as $packages
    | ($packages | map(.name)) as $workspace_names
    | $packages[] as $package
    | "__workspace_root \($package.name)",
      ($package.dependencies[]
        | select(.source == null
          and (.name as $name | any($workspace_names[]; . == $name)))
        | "\(.name) \($package.name)")
' "${metadata_file}" >"${edges_file}"

crates=()
while IFS= read -r crate; do
    crates+=("${crate}")
done < <(tsort "${edges_file}" | awk '$0 != "__workspace_root"')

if [[ "${#crates[@]}" -eq 0 ]]; then
    echo "no publishable workspace crates found" >&2
    exit 1
fi

ordered_crates=()
public_facade_found=false
for crate in "${crates[@]}"; do
    if [[ "${crate}" == "pocketstation" ]]; then
        public_facade_found=true
    else
        ordered_crates+=("${crate}")
    fi
done
if [[ "${public_facade_found}" != "true" ]]; then
    echo "public pocketstation facade is absent from publishable workspace crates" >&2
    exit 1
fi
ordered_crates+=("pocketstation")
crates=("${ordered_crates[@]}")

propagation_delay_seconds="${PKS_PUBLISH_PROPAGATION_DELAY_SECONDS:-35}"
total="${#crates[@]}"
for index in "${!crates[@]}"; do
    crate="${crates[$index]}"
    step=$((index + 1))
    if [[ "${dry_run}" == "true" ]]; then
        echo "[${step}/${total}] Validate package: ${crate}"
        cargo package \
            --manifest-path "${repo_root}/Cargo.toml" \
            --package "${crate}" \
            --list \
            --allow-dirty >/dev/null
        cargo check \
            --manifest-path "${repo_root}/Cargo.toml" \
            --package "${crate}" \
            --locked
    else
        echo "[${step}/${total}] Publish: ${crate}"
        cargo publish \
            --manifest-path "${repo_root}/Cargo.toml" \
            --package "${crate}" \
            --locked
        if [[ "${step}" -lt "${total}" ]]; then
            sleep "${propagation_delay_seconds}"
        fi
    fi
done

echo "Validated ${total} workspace crates in Cargo dependency order."
