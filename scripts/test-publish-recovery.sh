#!/usr/bin/env bash
set -euo pipefail

script_dir="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
publisher="${script_dir}/publish.sh"
real_cargo="$(command -v cargo)"
temporary_root="$(mktemp -d "${TMPDIR:-/tmp}/pks-publish-recovery.XXXXXX")"
trap 'rm -rf "${temporary_root}"' EXIT

fail() {
    echo "publish recovery contract FAIL: $*" >&2
    exit 1
}

make_fixture() {
    fixture_root="$1"
    mkdir -p "${fixture_root}/bin"

    cp "${script_dir}/../Cargo.toml" "${fixture_root}/Cargo.toml"

    cat >"${fixture_root}/bin/cargo" <<'EOF'
#!/usr/bin/env bash
set -euo pipefail
if [[ "${1:-}" != "publish" ]]; then
    exec "${PKS_TEST_REAL_CARGO}" "$@"
fi

crate=""
while [[ "$#" -gt 0 ]]; do
    if [[ "$1" == "--package" ]]; then
        crate="${2:-}"
        break
    fi
    shift
done
[[ -n "${crate}" ]] || exit 97
printf '%s\n' "${crate}" >>"${PKS_TEST_PUBLISH_LOG}"

if [[ "${PKS_TEST_PUBLISH_MODE}" == "rate-limit-once" \
    && "${crate}" == "${PKS_TEST_MISSING_CRATE}" \
    && ! -f "${PKS_TEST_RATE_LIMIT_MARKER}" ]]; then
    : >"${PKS_TEST_RATE_LIMIT_MARKER}"
    echo "the remote server responded with an error (status 429 Too Many Requests): You have published too many new crates in a short period of time. Please try again after ${PKS_TEST_RETRY_TIMESTAMP} and see https://crates.io/docs/rate-limits for more details." >&2
    exit 101
fi

if [[ "${PKS_TEST_PUBLISH_MODE}" == "fatal" \
    && "${crate}" == "${PKS_TEST_MISSING_CRATE}" ]]; then
    echo "the remote server responded with status 403 Forbidden" >&2
    exit 101
fi

echo "Published ${crate}"
EOF

    cat >"${fixture_root}/bin/curl" <<'EOF'
#!/usr/bin/env bash
set -euo pipefail
url="${*: -1}"
path="${url#${PKS_REGISTRY_API_BASE_URL}/}"
if [[ "${path}" == */0.1.0 ]]; then
    crate="${path%/0.1.0}"
    case "${PKS_TEST_REGISTRY_MODE}" in
        partial)
            case "${crate}" in
                pks-metrics|pks-timing|pks-frame|pks-capture|pks-caps|pks-capture-windows)
                    printf '200'
                    ;;
                *)
                    printf '404'
                    ;;
            esac
            ;;
        one-missing)
            if [[ "${crate}" == "${PKS_TEST_MISSING_CRATE}" ]]; then
                printf '404'
            else
                printf '200'
            fi
            ;;
        *)
            exit 96
            ;;
    esac
else
    crate="${path}"
    if [[ "${PKS_TEST_REGISTRY_MODE}" == "partial" \
        || "${crate}" == "${PKS_TEST_MISSING_CRATE}" ]]; then
        printf '404'
    else
        printf '200'
    fi
fi
EOF

    cat >"${fixture_root}/bin/sleep" <<'EOF'
#!/usr/bin/env bash
set -euo pipefail
printf '%s\n' "${1:-}" >>"${PKS_TEST_SLEEP_LOG}"
EOF

    chmod +x \
        "${fixture_root}/bin/cargo" \
        "${fixture_root}/bin/curl" \
        "${fixture_root}/bin/sleep"
}

run_publisher() {
    fixture_root="$1"
    registry_mode="$2"
    publish_mode="$3"
    missing_crate="$4"
    output_file="$5"
    first_creation_delay_seconds="$6"

    : >"${fixture_root}/publish.log"
    : >"${fixture_root}/sleep.log"
    rm -f "${fixture_root}/rate-limit.marker"

    env \
        PATH="${fixture_root}/bin:${PATH}" \
        PKS_TEST_REAL_CARGO="${real_cargo}" \
        PKS_TEST_PUBLISH_LOG="${fixture_root}/publish.log" \
        PKS_TEST_SLEEP_LOG="${fixture_root}/sleep.log" \
        PKS_TEST_RATE_LIMIT_MARKER="${fixture_root}/rate-limit.marker" \
        PKS_TEST_REGISTRY_MODE="${registry_mode}" \
        PKS_TEST_PUBLISH_MODE="${publish_mode}" \
        PKS_TEST_MISSING_CRATE="${missing_crate}" \
        PKS_TEST_RETRY_TIMESTAMP="Tue, 28 Jul 2026 20:49:10 GMT" \
        PKS_REGISTRY_API_BASE_URL="https://registry.invalid/crates" \
        PKS_PUBLISH_PROPAGATION_DELAY_SECONDS=0 \
        PKS_PUBLISH_FIRST_CREATION_DELAY_SECONDS="${first_creation_delay_seconds}" \
        PKS_PUBLISH_RETRY_SAFETY_SECONDS=5 \
        PKS_PUBLISH_MAX_ATTEMPTS=3 \
        PKS_NOW_EPOCH_SECONDS=1785271690 \
        bash "${publisher}" >"${output_file}" 2>&1
}

fixture="${temporary_root}/fixture"
make_fixture "${fixture}"

run_publisher \
    "${fixture}" \
    partial \
    success \
    pks-capture-macos \
    "${fixture}/partial.out" \
    130

expected_missing="$(
    printf '%s\n' \
        pks-capture-macos \
        pks-capture-linux \
        pks-graph \
        pks-runtime \
        pks-dsp \
        pks-endpoint \
        pks-nodes \
        pks-session \
        pocketstation
)"
actual_missing="$(sort "${fixture}/publish.log")"
expected_missing="$(printf '%s\n' "${expected_missing}" | sort)"
[[ "${actual_missing}" == "${expected_missing}" ]] ||
    fail "partial recovery did not skip six visible versions and resume nine missing versions"
[[ "$(wc -l <"${fixture}/sleep.log" | tr -d ' ')" == "8" ]] ||
    fail "partial recovery did not pace each remaining first-crate creation"
if awk '$0 != 130 { exit 1 }' "${fixture}/sleep.log"; then
    :
else
    fail "partial recovery used an unexpected first-crate delay"
fi

run_publisher \
    "${fixture}" \
    one-missing \
    rate-limit-once \
    pks-capture-macos \
    "${fixture}/rate-limit.out" \
    0
[[ "$(cat "${fixture}/publish.log")" == $'pks-capture-macos\npks-capture-macos' ]] ||
    fail "429 recovery did not retry exactly the missing crate"
[[ "$(cat "${fixture}/sleep.log")" == "65" ]] ||
    fail "429 recovery did not honor retry time plus safety"

if run_publisher \
    "${fixture}" \
    one-missing \
    fatal \
    pks-capture-macos \
    "${fixture}/fatal.out" \
    0; then
    fail "non-429 publish failure did not fail closed"
fi
[[ "$(cat "${fixture}/publish.log")" == "pks-capture-macos" ]] ||
    fail "non-429 publish failure retried unexpectedly"
[[ ! -s "${fixture}/sleep.log" ]] ||
    fail "non-429 publish failure slept unexpectedly"

echo "publish recovery contract PASS"
