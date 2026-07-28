#include "pks_session.h"
#include <stddef.h>
#include <stdint.h>

#define PKS_SESSION_ABI_1_0_MINOR 0u
#define PKS_SESSION_ABI_1_0_METRICS_SIZE_BYTES 160u
#define PKS_SESSION_ABI_1_0_CANARY UINT64_C(0x57A11C0DEC0FFEE1)

typedef struct {
  PksSessionMetricsSnapshot metrics;
  uint64_t tail_canary;
} PksSessionAbi10MetricsCanary;

#if defined(__STDC_VERSION__) && __STDC_VERSION__ >= 201112L
_Static_assert(sizeof(PksSessionMetricsSnapshot) ==
                   PKS_SESSION_ABI_1_0_METRICS_SIZE_BYTES,
               "ABI 1.1 changed the ABI 1.0 aggregate metrics record");
_Static_assert(offsetof(PksSessionAbi10MetricsCanary, tail_canary) ==
                   PKS_SESSION_ABI_1_0_METRICS_SIZE_BYTES,
               "canary must immediately follow the ABI 1.0 record");
#endif

int main(void) {
  static const uint8_t app_name[] =
      "__pks_session_c_abi_1_0_missing_application__";
  PksSessionEngineConfig config = {
      .struct_size_bytes = sizeof(PksSessionEngineConfig),
      .abi_major = PKS_SESSION_ABI_MAJOR,
      .abi_minor = PKS_SESSION_ABI_1_0_MINOR,
      .source_queue_capacity_frames = 32,
      .capture_frame_capacity_frames = 32,
      .capture_runtime_event_capacity_count = 8,
      .runtime_work_budget_frames = 64,
      .runtime_idle_poll_ms = 1,
      .runtime_ready_timeout_ms = 1000,
      .session_event_capacity_count = 32,
      .audio_queue_capacity_frames = 32,
      .audio_max_batch_frames = 8,
      .audio_max_outstanding_leases = 4,
  };
  PksSessionAppMicDeclaration declaration = {
      .struct_size_bytes = sizeof(PksSessionAppMicDeclaration),
      .abi_major = PKS_SESSION_ABI_MAJOR,
      .abi_minor = PKS_SESSION_ABI_1_0_MINOR,
      .application_name = {app_name, sizeof(app_name) - 1},
  };
  PksSessionHandle engine = {0};
  PksSessionHandle session = {0};
  PksSessionAbi10MetricsCanary output = {
      .metrics = {0},
      .tail_canary = PKS_SESSION_ABI_1_0_CANARY,
  };

  PksSessionStatus status = pks_session_engine_create(&config, &engine);
  if (status.code != PKS_SESSION_STATUS_OK) {
    return 1;
  }
  status = pks_session_create_app_mic(engine, &declaration, &session);
  if (status.code != PKS_SESSION_STATUS_OK) {
    return 2;
  }
  status = pks_session_compile(engine, session);
  if (status.code != PKS_SESSION_STATUS_OK) {
    return 3;
  }
  status = pks_session_start(engine, session);
  if (status.code != PKS_SESSION_STATUS_BACKEND_FAILURE) {
    return 4;
  }
  status = pks_session_metrics_poll(engine, session, &output.metrics);
  if (status.code != PKS_SESSION_STATUS_OK) {
    return 5;
  }
  if (output.metrics.struct_size_bytes !=
          PKS_SESSION_ABI_1_0_METRICS_SIZE_BYTES ||
      output.tail_canary != PKS_SESSION_ABI_1_0_CANARY) {
    return 6;
  }
  status = pks_session_destroy(engine, session);
  if (status.code != PKS_SESSION_STATUS_OK) {
    return 7;
  }
  status = pks_session_engine_destroy(engine);
  if (status.code != PKS_SESSION_STATUS_OK) {
    return 8;
  }
  return 0;
}
