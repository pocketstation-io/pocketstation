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

for command in awk cargo cmp cp curl grep jq python3 sed sort tsort; do
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
publish_log_file="$(mktemp "${TMPDIR:-/tmp}/pks-publish-command.XXXXXX")"
trap 'rm -f \
    "${metadata_file}" \
    "${workspace_names_file}" \
    "${closure_file}" \
    "${next_closure_file}" \
    "${direct_dependencies_file}" \
    "${edges_file}" \
    "${publish_log_file}"' EXIT

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
first_creation_delay_seconds="${PKS_PUBLISH_FIRST_CREATION_DELAY_SECONDS:-130}"
publish_max_attempts="${PKS_PUBLISH_MAX_ATTEMPTS:-3}"
retry_safety_seconds="${PKS_PUBLISH_RETRY_SAFETY_SECONDS:-5}"
registry_api_base_url="${PKS_REGISTRY_API_BASE_URL:-https://crates.io/api/v1/crates}"

for numeric_value in \
    "${propagation_delay_seconds}" \
    "${first_creation_delay_seconds}" \
    "${retry_safety_seconds}"; do
    if [[ ! "${numeric_value}" =~ ^[0-9]+$ ]]; then
        echo "publish delay values must be non-negative integers" >&2
        exit 2
    fi
done
if [[ ! "${publish_max_attempts}" =~ ^[1-5]$ ]]; then
    echo "PKS_PUBLISH_MAX_ATTEMPTS must be an integer from 1 through 5" >&2
    exit 2
fi

registry_status() {
    local url="$1"
    local status

    if ! status="$(
        curl \
            --silent \
            --show-error \
            --output /dev/null \
            --write-out '%{http_code}' \
            --header 'User-Agent: pocketstation-release-publisher' \
            "${url}"
    )"; then
        echo "registry visibility query failed: ${url}" >&2
        return 2
    fi
    case "${status}" in
        200)
            return 0
            ;;
        404)
            return 1
            ;;
        *)
            echo "registry visibility query returned HTTP ${status}: ${url}" >&2
            return 2
            ;;
    esac
}

retry_wait_seconds() {
    local retry_timestamp="$1"
    local now_epoch_seconds

    if [[ -n "${PKS_NOW_EPOCH_SECONDS:-}" ]]; then
        now_epoch_seconds="${PKS_NOW_EPOCH_SECONDS}"
    else
        now_epoch_seconds="$(python3 -c 'import time; print(int(time.time()))')"
    fi
    python3 -c '
import email.utils
import sys

retry_at = email.utils.parsedate_to_datetime(sys.argv[1])
retry_epoch = int(retry_at.timestamp())
now_epoch = int(sys.argv[2])
safety = int(sys.argv[3])
print(max(safety, retry_epoch - now_epoch + safety))
' "${retry_timestamp}" "${now_epoch_seconds}" "${retry_safety_seconds}"
}

publish_crate() {
    local crate="$1"
    local attempt=1
    local retry_timestamp
    local wait_seconds

    while true; do
        if cargo publish \
            --manifest-path "${repo_root}/Cargo.toml" \
            --package "${crate}" \
            --locked >"${publish_log_file}" 2>&1; then
            cat "${publish_log_file}"
            return 0
        fi

        cat "${publish_log_file}" >&2
        if ! grep -Fq 'status 429 Too Many Requests' "${publish_log_file}"; then
            echo "publish failed without a retryable crates.io 429: ${crate}" >&2
            return 1
        fi
        if [[ "${attempt}" -ge "${publish_max_attempts}" ]]; then
            echo "publish exhausted ${publish_max_attempts} attempts after crates.io 429: ${crate}" >&2
            return 1
        fi

        retry_timestamp="$(
            sed -n \
                's/.*Please try again after \(.* GMT\) and see .*/\1/p' \
                "${publish_log_file}" |
                tail -n 1
        )"
        if [[ -z "${retry_timestamp}" ]]; then
            echo "crates.io 429 omitted a parseable retry timestamp; refusing immediate retry" >&2
            return 1
        fi
        if ! wait_seconds="$(retry_wait_seconds "${retry_timestamp}")"; then
            echo "crates.io 429 retry timestamp is invalid: ${retry_timestamp}" >&2
            return 1
        fi

        echo "crates.io requested retry after ${retry_timestamp}; waiting ${wait_seconds}s"
        sleep "${wait_seconds}"
        attempt=$((attempt + 1))
    done
}

total="${#crates[@]}"
for index in "${!crates[@]}"; do
    crate="${crates[$index]}"
    crate_version="$(
        jq -r \
            --arg crate "${crate}" \
            '.packages[] | select(.name == $crate) | .version' \
            "${metadata_file}"
    )"
    if [[ -z "${crate_version}" || "${crate_version}" == "null" ]]; then
        echo "workspace version is unavailable for ${crate}" >&2
        exit 1
    fi
    step=$((index + 1))
    if [[ "${dry_run}" == "true" ]]; then
        echo "[${step}/${total}] Validate package: ${crate} ${crate_version}"
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
        version_url="${registry_api_base_url}/${crate}/${crate_version}"
        if registry_status "${version_url}"; then
            echo "[${step}/${total}] Skip visible: ${crate} ${crate_version}"
            continue
        else
            registry_result=$?
            if [[ "${registry_result}" -ne 1 ]]; then
                exit "${registry_result}"
            fi
        fi

        crate_url="${registry_api_base_url}/${crate}"
        if registry_status "${crate_url}"; then
            first_creation=false
        else
            registry_result=$?
            if [[ "${registry_result}" -ne 1 ]]; then
                exit "${registry_result}"
            fi
            first_creation=true
        fi

        echo "[${step}/${total}] Publish missing: ${crate} ${crate_version}"
        publish_crate "${crate}"
        if [[ "${step}" -lt "${total}" ]]; then
            if [[ "${first_creation}" == "true" \
                && "${first_creation_delay_seconds}" -gt 0 ]]; then
                echo "Pacing first-crate creation for ${first_creation_delay_seconds}s"
                sleep "${first_creation_delay_seconds}"
            elif [[ "${first_creation}" != "true" \
                && "${propagation_delay_seconds}" -gt 0 ]]; then
                sleep "${propagation_delay_seconds}"
            fi
        fi
    fi
done

echo "Validated ${total} facade-closure crates in Cargo dependency order."
