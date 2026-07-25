#!/usr/bin/env bash
# Publish all pocketstation crates to crates.io in dependency order (ADR-008 §26.5).
# Usage: ./scripts/publish.sh [--dry-run]
#
# --dry-run behaviour:
#   Runs `cargo publish --dry-run` for crates with no internal workspace
#   dependencies (pks-frame). For crates that depend on other
#   workspace crates, cargo publish --dry-run always fails because crates.io
#   hasn't received the upstream yet — this is a known cargo limitation.
#   Those crates are validated with `cargo check -p <crate>` instead.
set -euo pipefail

DRY_RUN=false
if [[ "${1:-}" == "--dry-run" ]]; then
    DRY_RUN=true
fi

# Publish order follows dependency graph (AUDIO-008 §26.5).
# Format: "crate_name:has_internal_deps"
# has_internal_deps=0 → full cargo publish --dry-run is safe
# has_internal_deps=1 → uses cargo check in dry-run mode (crates.io limitation)
#
# Crate graph (as of AUDIO-025 transport deferral):
#   pks-frame (leaf)
#   pks-codec → frame
#   pks-metrics (leaf)
#   pks-pipeline → frame, codec
#   pks-capture → frame
#   pks-capture-macos → capture, frame
#   pks-capture-windows → capture, frame
#   pks-capture-linux → capture, frame
#   pks-audio → all of the above
#
# pks-transport deleted per AUDIO-025; TransportKind and
# RouteEncryptionMode are inlined in pks-audio until a Rust
# transport layer justifies a real crate with RtpFrame + TransportSession.
CRATES=(
    "pks-frame:0"
    "pks-codec:1"
    "pks-metrics:0"
    "pks-pipeline:1"
    "pks-capture:1"
    "pks-capture-macos:1"
    "pks-capture-windows:1"
    "pks-capture-linux:1"
    "pks-audio:1"
)

TOTAL=${#CRATES[@]}
PROPAGATION_DELAY_SECONDS=35

for i in "${!CRATES[@]}"; do
    entry="${CRATES[$i]}"
    crate="${entry%%:*}"
    has_internal_deps="${entry##*:}"
    step=$((i + 1))

    if [[ "${DRY_RUN}" == "true" ]]; then
        if [[ "${has_internal_deps}" == "0" ]]; then
            echo "[${step}/${TOTAL}] Dry-run (cargo publish --dry-run): ${crate}..."
            if ! cargo publish --dry-run -p "${crate}"; then
                echo "ERROR: dry-run failed for ${crate}" >&2
                exit 1
            fi
        else
            # cargo publish --dry-run cannot resolve workspace deps that are not
            # yet on crates.io; use cargo check to verify compilation instead.
            echo "[${step}/${TOTAL}] Dry-run (cargo check): ${crate}..."
            if ! cargo check -p "${crate}"; then
                echo "ERROR: cargo check failed for ${crate}" >&2
                exit 1
            fi
        fi
    else
        echo "[${step}/${TOTAL}] Publishing ${crate}..."
        if ! cargo publish -p "${crate}"; then
            echo "ERROR: publish failed for ${crate}" >&2
            exit 1
        fi

        # Wait for crates.io to propagate before publishing dependents,
        # except after the last crate.
        if [[ "${step}" -lt "${TOTAL}" ]]; then
            echo "Waiting ${PROPAGATION_DELAY_SECONDS}s for crates.io propagation..."
            sleep "${PROPAGATION_DELAY_SECONDS}"
        fi
    fi
done

echo "All ${TOTAL} crates published successfully."
