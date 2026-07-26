#!/usr/bin/env bash
# Regenerate ffi/pocketstation_audio.h via cbindgen and copy it to every SDK
# that links against pocketstation as a C FFI consumer.
#
# Run this after any change to crates/pks-audio/src/ffi.rs.
# The generated header is gitignored in pocketstation; the copies in each SDK
# repo are the canonical ones committed to version control.
#
# Usage:
#   ./scripts/sync-ffi-header.sh
#
# Prerequisites:
#   - cargo build succeeds (cbindgen runs as part of build.rs)
#   - sdk-ios and sdk-android repos are checked out at ../sdk-ios and ../sdk-android
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"
REPO_ROOT="$(cd "$SCRIPT_DIR/.." && pwd)"
HEADER="$REPO_ROOT/ffi/pocketstation_audio.h"

echo "Building pks-audio (runs cbindgen via build.rs)..."
cargo build -p pks-audio 2>&1 | grep -v "^$" || true

if [[ ! -f "$HEADER" ]]; then
    echo "ERROR: $HEADER not found after build. Check build.rs." >&2
    exit 1
fi

SDK_IOS="$REPO_ROOT/../sdk-ios/Sources/PocketStationAudioFFI/pocketstation_audio.h"
SDK_ANDROID="$REPO_ROOT/../sdk-android/sdk/src/main/cpp/pocketstation_audio.h"

copy_if_changed() {
    local src="$1"
    local dst="$2"
    local label="$3"
    if [[ ! -f "$dst" ]] || ! diff -q "$src" "$dst" > /dev/null 2>&1; then
        cp "$src" "$dst"
        echo "  updated: $label"
    else
        echo "  unchanged: $label"
    fi
}

echo "Syncing header to SDKs..."
copy_if_changed "$HEADER" "$SDK_IOS"     "sdk-ios"
copy_if_changed "$HEADER" "$SDK_ANDROID" "sdk-android"

echo ""
echo "Done. Commit the updated header in each SDK repo."
echo "  sdk-ios:     $SDK_IOS"
echo "  sdk-android: $SDK_ANDROID"
