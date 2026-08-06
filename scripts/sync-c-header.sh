#!/usr/bin/env bash
# Synchronize the one PocketStation C header into language SDK source trees.
set -euo pipefail

script_dir="$(cd "$(dirname "$0")" && pwd)"
repo_root="$(cd "${script_dir}/.." && pwd)"
mode="${1:---sync}"
if [[ "${mode}" != "--sync" && "${mode}" != "--check" ]]; then
  echo "usage: $0 [--sync|--check]" >&2
  exit 2
fi

source_header="${repo_root}/include/pocketstation.h"
ios_header="${repo_root}/../sdk-ios/Sources/PocketStationFFI/pocketstation.h"
android_header="${repo_root}/../sdk-android/sdk/src/main/cpp/pocketstation.h"

if [[ "${mode}" == "--check" ]]; then
  diff -u "${source_header}" "${ios_header}"
  diff -u "${source_header}" "${android_header}"
  echo "PocketStation C headers: PASS"
else
  mkdir -p "$(dirname "${ios_header}")" "$(dirname "${android_header}")"
  cp "${source_header}" "${ios_header}"
  cp "${source_header}" "${android_header}"
  echo "PocketStation C headers synchronized"
fi
