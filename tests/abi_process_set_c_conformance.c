#include "pocketstation.h"
#include <stdint.h>
#include <stddef.h>
#include <string.h>

_Static_assert(sizeof(PksSessionProcessInstance) == 40, "process record size");
_Static_assert(sizeof(PksSessionProcessDiscovery) == 48, "discovery record size");
_Static_assert(sizeof(PksSessionProcessSetDeclaration) == 24, "set record size");
_Static_assert(sizeof(PksSessionExternalSourceMetrics) == 88, "source metrics size");
_Static_assert(sizeof(PksSessionStopOutcome) == 80, "stop outcome size");
_Static_assert(offsetof(PksSessionProcessInstance, process_start_time_ns) == 16, "exact u64 offset");
_Static_assert(PKS_SESSION_STATUS_BACKEND_FAILURE == 14u, "old status stable");
_Static_assert(PKS_SESSION_HANDLE_AUDIO_BATCH == 3u, "old handle stable");

static PksSessionEngineConfig config(void) {
    PksSessionEngineConfig value = {sizeof(PksSessionEngineConfig), 1, 1,
        32, 32, 8, 64, 1, 1000, 32, 32, 8, 4};
    return value;
}
int main(void) {
    PksSessionAbiVersion version = {0};
    if (pks_session_abi_get_version(&version).code || version.abi_major != 1 || version.abi_minor != 2) return 1;
    if (pks_session_abi_is_compatible(1, 0, sizeof(PksSessionAbiVersion)).code || pks_session_abi_is_compatible(1, 1, sizeof(PksSessionAbiVersion)).code) return 2;
    PksSessionHandle engine = {0}, session = {0};
    PksSessionEngineConfig options = config();
    if (pks_session_engine_create(&options, &engine).code) return 3;
    static const uint8_t key[] = "not-opened-by-declaration";
    PksSessionProcessInstance member = {sizeof(member), 1, 2, 7,
        PKS_SESSION_PROCESS_PLATFORM_MACOS, UINT64_MAX, {key, sizeof(key)-1}};
    PksSessionProcessSetDeclaration declaration = {sizeof(declaration), 1, 2, &member, 0, sizeof(member)};
    PksSessionStatus status = pks_session_create_process_set(engine, &declaration, &session);
    if (status.code != PKS_SESSION_STATUS_INVALID_ARGUMENT || status.detail != PKS_SESSION_PROCESS_SET_DETAIL_INVALID) return 4;
    declaration.member_count = 1;
    declaration.abi_minor = 1;
    if (pks_session_create_process_set(engine, &declaration, &session).code != PKS_SESSION_STATUS_UNSUPPORTED_ABI_MINOR) return 5;
    declaration.abi_minor = 2;
    member.stable_key.data = NULL;
    if (pks_session_create_process_set(engine, &declaration, &session).code != PKS_SESSION_STATUS_NULL_ARGUMENT) return 6;
    member.stable_key.data = key;
    member.process_start_time_ns = 0;
    if (pks_session_create_process_set(engine, &declaration, &session).code != PKS_SESSION_STATUS_INVALID_ARGUMENT) return 7;
    member.process_start_time_ns = UINT64_MAX;
    status = pks_session_create_process_set(engine, &declaration, &session);
    if (status.code == PKS_SESSION_STATUS_BACKEND_FAILURE && status.detail == PKS_SESSION_PROCESS_SET_DETAIL_UNSUPPORTED) {
        if (pks_session_engine_destroy(engine).code) return 8;
        return 0;
    }
    if (status.code) return 9;
    if (pks_session_compile(engine, session).code) return 10;
    PksSessionStopOutcome outcome = {0};
    outcome.struct_size_bytes = sizeof(outcome); outcome.abi_major = 1; outcome.abi_minor = 2;
    if (pks_session_stop_outcome_poll(engine, session, &outcome).code != PKS_SESSION_STATUS_INVALID_LIFECYCLE_STATE) return 11;
    uint32_t source_count = 999;
    if (pks_session_external_source_metrics_count(engine, session, &source_count).code != PKS_SESSION_STATUS_INVALID_LIFECYCLE_STATE) return 12;
    /* No start/capture: this ordinary consumer qualifies declaration assembly. */
    if (pks_session_destroy(engine, session).code) return 13;
    if (pks_session_engine_destroy(engine).code) return 14;
    return 0;
}
