#!/usr/bin/env bash
set -euo pipefail

usage() {
  cat <<'USAGE'
usage: build-android-library.sh \
  --ndk-dir PATH \
  --output-root PATH \
  [--target-dir PATH]

Builds the PocketStation static library for arm64-v8a and copies it to:

  <output-root>/arm64-v8a/libpocketstation.a

The Android NDK revision is pinned to 26.1.10909125 and the minimum API is 29.
USAGE
}

script_dir="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
repository_root="$(cd "${script_dir}/.." && pwd)"
ndk_dir=""
output_root=""
target_dir=""

while [[ "$#" -gt 0 ]]; do
  case "$1" in
    --ndk-dir)
      ndk_dir="${2:-}"
      shift 2
      ;;
    --output-root)
      output_root="${2:-}"
      shift 2
      ;;
    --target-dir)
      target_dir="${2:-}"
      shift 2
      ;;
    --help|-h)
      usage
      exit 0
      ;;
    *)
      echo "unknown argument: $1" >&2
      usage >&2
      exit 2
      ;;
  esac
done

[[ -d "${ndk_dir}" ]] || {
  echo "--ndk-dir must identify an installed Android NDK" >&2
  exit 2
}
[[ -n "${output_root}" ]] || {
  echo "--output-root is required" >&2
  exit 2
}

ndk_dir="$(cd "${ndk_dir}" && pwd)"
case "${output_root}" in
  /*) ;;
  *) output_root="${PWD}/${output_root}" ;;
esac
if [[ -z "${target_dir}" ]]; then
  target_dir="${output_root}/cargo-target"
else
  case "${target_dir}" in
    /*) ;;
    *) target_dir="${PWD}/${target_dir}" ;;
  esac
fi

ndk_properties="${ndk_dir}/source.properties"
[[ -f "${ndk_properties}" ]] || {
  echo "Android NDK source.properties is missing" >&2
  exit 2
}
ndk_revision="$(
  awk -F= '/^Pkg.Revision/ {
    gsub(/^[[:space:]]+|[[:space:]]+$/, "", $2)
    print $2
    exit
  }' "${ndk_properties}"
)"
[[ "${ndk_revision}" == "26.1.10909125" ]] || {
  echo "Android NDK 26.1.10909125 is required; observed ${ndk_revision}" >&2
  exit 2
}

rust_target="aarch64-linux-android"
android_abi="arm64-v8a"
minimum_android_api="29"
rustup target list --installed | grep -Fxq "${rust_target}" || {
  echo "Rust target ${rust_target} is not installed" >&2
  exit 2
}

prebuilt_roots=()
while IFS= read -r prebuilt_root; do
  prebuilt_roots+=("${prebuilt_root}")
done < <(find "${ndk_dir}/toolchains/llvm/prebuilt" -mindepth 1 -maxdepth 1 -type d -print)
[[ "${#prebuilt_roots[@]}" -eq 1 ]] || {
  echo "Android NDK must expose exactly one host prebuilt toolchain" >&2
  exit 2
}
prebuilt_root="${prebuilt_roots[0]}"
android_linker="${prebuilt_root}/bin/aarch64-linux-android${minimum_android_api}-clang"
android_archiver="${prebuilt_root}/bin/llvm-ar"
[[ -x "${android_linker}" && -x "${android_archiver}" ]] || {
  echo "Android NDK linker or archiver is missing" >&2
  exit 2
}

toolchain_file="${script_dir}/android-arm64-v8a/android.toolchain.cmake"
[[ -f "${toolchain_file}" ]] || {
  echo "PocketStation Android toolchain wrapper is missing" >&2
  exit 2
}

mkdir -p "${output_root}/${android_abi}" "${target_dir}"
env \
  PKS_ANDROID_NDK_DIR="${ndk_dir}" \
  CMAKE_TOOLCHAIN_FILE_aarch64_linux_android="${toolchain_file}" \
  LIBOPUS_NO_PKG=1 \
  OPUS_STATIC=1 \
  CARGO_TARGET_AARCH64_LINUX_ANDROID_LINKER="${android_linker}" \
  CARGO_TARGET_AARCH64_LINUX_ANDROID_AR="${android_archiver}" \
  CARGO_TARGET_DIR="${target_dir}" \
  cargo build \
    --manifest-path "${repository_root}/Cargo.toml" \
    --locked \
    --release \
    --target "${rust_target}" \
    -p pocketstation

built_archive="${target_dir}/${rust_target}/release/libpocketstation.a"
[[ -f "${built_archive}" ]] || {
  echo "PocketStation Android archive was not produced" >&2
  exit 1
}
output_archive="${output_root}/${android_abi}/libpocketstation.a"
cp "${built_archive}" "${output_archive}"

echo "android_abi=${android_abi}"
echo "android_api=${minimum_android_api}"
echo "ndk_revision=${ndk_revision}"
echo "archive=${output_archive}"
shasum -a 256 "${output_archive}"
