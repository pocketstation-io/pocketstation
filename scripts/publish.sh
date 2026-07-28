#!/usr/bin/env bash
# Validate or publish the registry-supported PocketStation facade closure.
# Usage: ./scripts/publish.sh [--dry-run]
set -euo pipefail

dry_run=false
if [[ "${1:-}" == "--dry-run" ]]; then
    dry_run=true
elif [[ -n "${1:-}" ]]; then
    echo "usage: $0 [--dry-run]" >&2
    exit 2
fi

for command in awk cargo cmp cp jq sort tsort; do
    if ! command -v "${command}" >/dev/null 2>&1; then
        echo "required command is unavailable: ${command}" >&2
        exit 2
    fi
done

repo_root="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
metadata_file="$(mktemp "${TMPDIR:-/tmp}/pks-publish-metadata.XXXXXX")"
workspace_names_file="$(mktemp "${TMPDIR:-/tmp}/pks-publish-workspace.XXXXXX")"
closure_file="$(mktemp "${TMPDIR:-/tmp}/pks-publish-closure.XXXXXX")"
next_closure_file="$(mktemp "${TMPDIR:-/tmp}/pks-publish-next-closure.XXXXXX")"
direct_dependencies_file="$(
    mktemp "${TMPDIR:-/tmp}/pks-publish-direct-dependencies.XXXXXX"
)"
edges_file="$(mktemp "${TMPDIR:-/tmp}/pks-publish-edges.XXXXXX")"
trap 'rm -f \
    "${metadata_file}" \
    "${workspace_names_file}" \
    "${closure_file}" \
    "${next_closure_file}" \
    "${direct_dependencies_file}" \
    "${edges_file}"' EXIT

cargo metadata \
    --manifest-path "${repo_root}/Cargo.toml" \
    --format-version 1 \
    --no-deps >"${metadata_file}"

jq -r '
    .workspace_members as $workspace_members
    | .packages[]
    | select(.id as $id | any($workspace_members[]; . == $id))
    | .name
' "${metadata_file}" | sort -u >"${workspace_names_file}"

if ! awk '$0 == "pocketstation" { found = 1 } END { exit !found }' \
    "${workspace_names_file}"; then
    echo "public pocketstation facade is absent from the workspace" >&2
    exit 1
fi

# Derive the exact workspace normal/target dependency closure of the public
# facade. Dev-only and build-only dependencies do not become registry products.
printf '%s\n' "pocketstation" >"${closure_file}"
while true; do
    jq -r \
        --rawfile closure "${closure_file}" \
        --rawfile workspace_names "${workspace_names_file}" '
        ($closure | split("\n") | map(select(length > 0))) as $closure_names
        | ($workspace_names | split("\n") | map(select(length > 0))) as $workspace
        | .packages[]
        | select(.name as $name | any($closure_names[]; . == $name))
        | .dependencies[]
        | select(.kind == null and .source == null)
        | .name
        | select(. as $dependency | any($workspace[]; . == $dependency))
    ' "${metadata_file}" >"${direct_dependencies_file}"

    sort -u \
        "${closure_file}" \
        "${direct_dependencies_file}" >"${next_closure_file}"
    if cmp -s "${closure_file}" "${next_closure_file}"; then
        break
    fi
    cp "${next_closure_file}" "${closure_file}"
done

role_errors=0
public_facade_count=0
while IFS=$'\t' read -r name role publishable in_closure; do
    case "${role}" in
        public-facade)
            public_facade_count=$((public_facade_count + 1))
            if [[ "${name}" != "pocketstation" \
                || "${publishable}" != "true" \
                || "${in_closure}" != "true" ]]; then
                echo "invalid public-facade registry role: ${name}" >&2
                role_errors=$((role_errors + 1))
            fi
            ;;
        facade-dependency)
            if [[ "${name}" == "pocketstation" \
                || "${publishable}" != "true" \
                || "${in_closure}" != "true" ]]; then
                echo "facade-dependency is outside the publish closure: ${name}" >&2
                role_errors=$((role_errors + 1))
            fi
            ;;
        deferred | example)
            if [[ "${publishable}" != "false" \
                || "${in_closure}" != "false" ]]; then
                echo "non-registry package leaks into the publish closure: ${name}" >&2
                role_errors=$((role_errors + 1))
            fi
            ;;
        *)
            echo "missing or unknown registry role for workspace package: ${name}" >&2
            role_errors=$((role_errors + 1))
            ;;
    esac
done < <(
    jq -r \
        --rawfile closure "${closure_file}" '
        ($closure | split("\n") | map(select(length > 0))) as $closure_names
        | .workspace_members as $workspace_members
        | .packages[]
        | select(.id as $id | any($workspace_members[]; . == $id))
        | [
            .name,
            (.metadata.pocketstation["registry-role"] // ""),
            (if .publish == [] then "false" else "true" end),
            (if (.name as $name | any($closure_names[]; . == $name))
             then "true"
             else "false"
             end)
          ]
        | @tsv
    ' "${metadata_file}"
)

if [[ "${public_facade_count}" -ne 1 ]]; then
    echo "expected exactly one public-facade registry role" >&2
    role_errors=$((role_errors + 1))
fi
if [[ "${role_errors}" -ne 0 ]]; then
    echo "registry role validation failed with ${role_errors} error(s)" >&2
    exit 1
fi

jq -r '
    .packages[]
    | select(.name as $name | any($closure_names[]; . == $name))
    | . as $package
    | "__workspace_root \($package.name)",
      ($package.dependencies[]
        | select(.kind == null and .source == null)
        | select(.name as $name | any($closure_names[]; . == $name))
        | "\(.name) \($package.name)")
' \
    --argjson closure_names "$(
        jq -R -s 'split("\n") | map(select(length > 0))' "${closure_file}"
    )" \
    "${metadata_file}" >"${edges_file}"

crates=()
while IFS= read -r crate; do
    crates+=("${crate}")
done < <(tsort "${edges_file}" | awk '$0 != "__workspace_root"')

closure_count="$(awk 'NF { count += 1 } END { print count + 0 }' "${closure_file}")"
if [[ "${#crates[@]}" -ne "${closure_count}" ]]; then
    echo "dependency ordering omitted a facade-closure package" >&2
    exit 1
fi

last_index=$((${#crates[@]} - 1))
if [[ "${crates[$last_index]}" != "pocketstation" ]]; then
    echo "dependency order does not place pocketstation last" >&2
    exit 1
fi

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

echo "Validated ${total} facade-closure crates in Cargo dependency order."
