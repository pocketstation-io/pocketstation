#!/usr/bin/env bash
# Regenerate the checked codec C header and copy it to the canonical SDK codec
# boundary paths. This does not generate the Session C header and does not make
# either mobile SDK a proven packaged consumer.
#
# Usage:
#   ./scripts/sync-codec-c-header.sh [--sync|--check]
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"
REPO_ROOT="$(cd "$SCRIPT_DIR/.." && pwd)"
MODE="sync"

case "${1:-}" in
    "")
        ;;
    --sync)
        MODE="sync"
        ;;
    --check)
        MODE="check"
        ;;
    --help|-h)
        echo "Usage: $0 [--sync|--check]"
        echo "  --sync   regenerate and update checked header copies (default)"
        echo "  --check  regenerate in a temporary directory and fail on drift"
        exit 0
        ;;
    *)
        echo "ERROR: unknown argument: $1" >&2
        echo "Usage: $0 [--sync|--check]" >&2
        exit 2
        ;;
esac

if [[ "$#" -gt 1 ]]; then
    echo "ERROR: expected at most one argument" >&2
    exit 2
fi

CHECKED_HEADER="$REPO_ROOT/crates/pks-codec-c/include/pks_codec.h"
OUTPUT_DIR="$(mktemp -d "${TMPDIR:-/tmp}/pks-codec-c-header.XXXXXX")"
trap 'rm -rf "${OUTPUT_DIR}"' EXIT
GENERATED_HEADER="$OUTPUT_DIR/pks_codec.h"

echo "Generating the codec C header outside the source tree..."
PKS_CODEC_C_HEADER_OUTPUT="$GENERATED_HEADER" \
    cargo build -p pks-codec-c --locked

if [[ ! -f "$GENERATED_HEADER" ]]; then
    echo "ERROR: $GENERATED_HEADER not found after build. Check pks-codec-c/build.rs." >&2
    exit 1
fi

SDK_IOS="$REPO_ROOT/../sdk-ios/Sources/PocketStationCodecFFI/pks_codec.h"
SDK_ANDROID="$REPO_ROOT/../sdk-android/sdk/src/main/cpp/pks_codec.h"

copy_if_changed() {
    local source_path="$1"
    local destination_path="$2"
    local label="$3"
    if [[ ! -f "$destination_path" ]] ||
        ! diff -q "$source_path" "$destination_path" >/dev/null 2>&1; then
        mkdir -p "$(dirname "$destination_path")"
        cp "$source_path" "$destination_path"
        echo "  updated: $label"
    else
        echo "  unchanged: $label"
    fi
}

check_unchanged() {
    local generated_path="$1"
    local checked_path="$2"
    local label="$3"
    if [[ ! -f "$checked_path" ]]; then
        echo "ERROR: missing $label: $checked_path" >&2
        return 1
    fi
    if ! diff -u "$checked_path" "$generated_path"; then
        echo "ERROR: generated codec header differs from $label" >&2
        return 1
    fi
    echo "  current: $label"
}

if [[ "$MODE" == "check" ]]; then
    echo "Checking generated and compatibility headers..."
    check_unchanged "$GENERATED_HEADER" "$CHECKED_HEADER" \
        "pks-codec-c checked header"
    check_unchanged "$GENERATED_HEADER" "$SDK_IOS" \
        "sdk-ios codec header"
    check_unchanged "$GENERATED_HEADER" "$SDK_ANDROID" \
        "sdk-android codec header"
    echo
    echo "Codec C headers: PASS"
else
    echo "Syncing checked and compatibility headers..."
    copy_if_changed "$GENERATED_HEADER" "$CHECKED_HEADER" \
        "pks-codec-c checked header"
    copy_if_changed "$GENERATED_HEADER" "$SDK_IOS" \
        "sdk-ios codec header"
    copy_if_changed "$GENERATED_HEADER" "$SDK_ANDROID" \
        "sdk-android codec header"
    echo
    echo "Done. Commit each changed checked header in its owning repository."
fi
