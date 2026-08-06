#!/usr/bin/env bash
set -euo pipefail

repo_root="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
target_dir="${CARGO_TARGET_DIR:-${repo_root}/target}"
fixture_source="${repo_root}/tests/abi_session_c_conformance.c"
include_dir="${repo_root}/include"
output_dir="$(mktemp -d "${TMPDIR:-/tmp}/pocketstation-abi.XXXXXX")"
trap 'rm -rf "${output_dir}"' EXIT

cargo build \
  --manifest-path "${repo_root}/Cargo.toml" \
  --package pocketstation \
  --features conformance-fixtures \
  --locked

compiler="${CC:-cc}"
executable="${output_dir}/pocketstation-abi-success"

case "$(uname -s)" in
  Darwin)
    library_dir="${target_dir}/debug"
    "${compiler}" "${fixture_source}" \
      -I "${include_dir}" \
      -L "${library_dir}" \
      -lpocketstation \
      -Werror \
      -Wl,-rpath,"${library_dir}" \
      -o "${executable}"
    ;;
  Linux)
    library_dir="${target_dir}/debug"
    "${compiler}" "${fixture_source}" \
      -I "${include_dir}" \
      -L "${library_dir}" \
      -lpocketstation \
      -Werror \
      -Wl,-rpath,"${library_dir}" \
      -o "${executable}"
    ;;
  *)
    echo "unsupported host for C conformance fixture: $(uname -s)" >&2
    exit 2
    ;;
esac

if ! "${executable}" >"${output_dir}/stdout.log" 2>"${output_dir}/stderr.log"; then
  cat "${output_dir}/stdout.log" >&2
  cat "${output_dir}/stderr.log" >&2
  exit 1
fi
