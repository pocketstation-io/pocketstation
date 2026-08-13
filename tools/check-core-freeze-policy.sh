#!/usr/bin/env bash
set -euo pipefail

script_dir="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
repo_root="$(cd "${script_dir}/.." && pwd)"
cd "${repo_root}"

policy="docs/development/compatibility-and-freeze.md"
adr="docs/adr/AUDIO-034-core-1-0-extension-freeze.md"
template=".github/PULL_REQUEST_TEMPLATE.md"

for required in "${policy}" "${adr}" "${template}"; do
  if [[ ! -f "${required}" ]]; then
    echo "FAIL: Core freeze authority is missing: ${required}" >&2
    exit 1
  fi
done

required_policy_terms=(
  "2026-08-13"
  "2028-08-13"
  "external source"
  "operator"
  "endpoint/connector"
  "transport"
  "SDK projection"
  "sidecar"
  "correctness"
  "security"
  "OS/toolchain"
  "measured performance"
  "API/ABI"
  "execution primitive"
)

for term in "${required_policy_terms[@]}"; do
  if ! rg -Fq "${term}" "${policy}"; then
    echo "FAIL: Core freeze policy is missing required term: ${term}" >&2
    exit 1
  fi
done

if ! rg -Fq "**Status:** Accepted; 24-month extension-first freeze active" "${adr}"; then
  echo "FAIL: Core freeze ADR is not active" >&2
  exit 1
fi

classification_count="$(rg -c '^- \[ \] (No Core semantic change|Correctness bug|Security vulnerability|OS/API/toolchain breakage|Measured performance regression|API/ABI compatibility repair|Missing execution primitive)' "${template}")"
if [[ "${classification_count}" != "7" ]]; then
  echo "FAIL: PR template must expose exactly seven approved Core classifications" >&2
  exit 1
fi

for heading in "### Extension-model analysis" "### Compatibility and realtime impact"; do
  if ! rg -Fq "${heading}" "${template}"; then
    echo "FAIL: PR template is missing: ${heading}" >&2
    exit 1
  fi
done

if [[ "${GITHUB_EVENT_NAME:-}" == "pull_request" && -n "${GITHUB_EVENT_PATH:-}" ]]; then
  if [[ ! -f "${GITHUB_EVENT_PATH}" ]]; then
    echo "FAIL: GitHub pull-request event payload is missing" >&2
    exit 1
  fi

  body="$(jq -r '.pull_request.body // ""' "${GITHUB_EVENT_PATH}")"
  checked_count="$(printf '%s\n' "${body}" | rg -c '^- \[[xX]\] (No Core semantic change|Correctness bug|Security vulnerability|OS/API/toolchain breakage|Measured performance regression|API/ABI compatibility repair|Missing execution primitive)' || true)"
  if [[ "${checked_count}" != "1" ]]; then
    echo "FAIL: pull request must select exactly one Core freeze classification" >&2
    exit 1
  fi
  for heading in "### Extension-model analysis" "### Compatibility and realtime impact"; do
    if ! printf '%s\n' "${body}" | rg -Fq "${heading}"; then
      echo "FAIL: pull request body is missing ${heading}" >&2
      exit 1
    fi
  done
fi

echo "PASS: Core 1.0 extension-first freeze policy is active and review-enforced"
