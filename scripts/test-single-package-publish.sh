#!/usr/bin/env bash
# Regression guard for the single-package publish contract.
set -euo pipefail

repo_root="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
metadata="$(cargo metadata --manifest-path "${repo_root}/Cargo.toml" --no-deps --format-version 1)"

if [[ "$(jq '.packages | length' <<<"${metadata}")" != "1" ]]; then
  echo "expected one publishable package" >&2
  exit 1
fi
if [[ "$(jq -r '.packages[0].name' <<<"${metadata}")" != "pocketstation" ]]; then
  echo "expected pocketstation as the only package" >&2
  exit 1
fi
if rg -q 'pks-[a-z].*=.*path' "${repo_root}/Cargo.toml"; then
  echo "legacy internal package dependency leaked into root manifest" >&2
  exit 1
fi

package_files="$(cargo package \
  --manifest-path "${repo_root}/Cargo.toml" \
  --locked \
  --list)"
for forbidden_prefix in .github/ AGENTS.md PHASE docs/ scripts/; do
  while IFS= read -r package_file; do
    if [[ "${package_file}" == "${forbidden_prefix}"* ]]; then
      echo "internal execution material leaked into package: ${forbidden_prefix}" >&2
      exit 1
    fi
  done <<<"${package_files}"
done
for required_path in Cargo.toml README.md build.rs include/pocketstation.h src/lib.rs; do
  if ! grep -Fx "${required_path}" <<<"${package_files}" >/dev/null; then
    echo "required package file is missing: ${required_path}" >&2
    exit 1
  fi
done

echo "single-package publish contract: PASS"
