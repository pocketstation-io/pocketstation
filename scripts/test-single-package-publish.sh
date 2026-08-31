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
for forbidden_prefix in .github/ AGENTS.md PHASE scripts/; do
  while IFS= read -r package_file; do
    if [[ "${package_file}" == "${forbidden_prefix}"* ]]; then
      echo "internal execution material leaked into package: ${forbidden_prefix}" >&2
      exit 1
    fi
  done <<<"${package_files}"
done

# The archive may carry only curated user-facing documentation. Internal
# engineering records remain private and untracked.
while IFS= read -r package_file; do
  case "${package_file}" in
    docs/README.md | \
      docs/architecture/overview.md | \
      docs/concepts/signals-and-streams.md | \
      docs/compatibility/README.md | \
      docs/getting-started/rust-quickstart.md | \
      docs/guides/application-audio.md | \
      docs/guides/capture-and-route.md | \
      docs/guides/connectors.md | \
      docs/guides/extensions.md | \
      docs/guides/record-and-observe.md | \
      docs/operations/platform-support.md | \
      docs/troubleshooting.md) ;;
    docs/*)
      echo "internal documentation leaked into package: ${package_file}" >&2
      exit 1
      ;;
  esac
done <<<"${package_files}"

for required_path in \
  Cargo.toml \
  README.md \
  RELEASE_NOTES.md \
  build.rs \
  docs/README.md \
  docs/architecture/overview.md \
  docs/concepts/signals-and-streams.md \
  docs/compatibility/README.md \
  docs/getting-started/rust-quickstart.md \
  docs/guides/application-audio.md \
  docs/guides/capture-and-route.md \
  docs/guides/connectors.md \
  docs/guides/extensions.md \
  docs/guides/record-and-observe.md \
  docs/operations/platform-support.md \
  docs/troubleshooting.md \
  include/pocketstation.h \
  src/lib.rs; do
  if ! grep -Fx "${required_path}" <<<"${package_files}" >/dev/null; then
    echo "required package file is missing: ${required_path}" >&2
    exit 1
  fi
done

if rg -q '^RELEASE_NOTES_[0-9]' <<<"${package_files}"; then
  echo "numbered patch release notes leaked into package" >&2
  exit 1
fi

echo "single-package publish contract: PASS"
