#include "pocketstation.h"
#include <stddef.h>

#if defined(__STDC_VERSION__) && __STDC_VERSION__ >= 201112L
_Static_assert(sizeof(PksSessionAbiVersion) == 8, "PksSessionAbiVersion size");
_Static_assert(sizeof(PksSessionHandle) == 24, "PksSessionHandle size");
_Static_assert(sizeof(PksSessionStatus) == 8, "PksSessionStatus size");
_Static_assert(sizeof(PksSessionUtf8) == 16, "PksSessionUtf8 size");
_Static_assert(sizeof(PksSessionEngineConfig) == 56,
               "PksSessionEngineConfig size");
_Static_assert(sizeof(PksSessionAppMicDeclaration) == 24,
               "PksSessionAppMicDeclaration size");
_Static_assert(sizeof(PksSessionEvent) == 64, "PksSessionEvent size");
_Static_assert(sizeof(PksSessionMetricsSnapshot) == 160,
               "PksSessionMetricsSnapshot size");
_Static_assert(sizeof(PksSessionSourceMetrics) == 176,
               "PksSessionSourceMetrics size");
_Static_assert(sizeof(PksSessionRouteMetrics) == 352,
               "PksSessionRouteMetrics size");
_Static_assert(sizeof(PksSessionAudioBatch) == 40,
               "PksSessionAudioBatch size");
_Static_assert(sizeof(PksSessionAudioFrame) == 144,
               "PksSessionAudioFrame size");
_Static_assert(offsetof(PksSessionEvent, session_id) == 24,
               "PksSessionEvent session_id offset");
_Static_assert(offsetof(PksSessionMetricsSnapshot, event_capacity_count) == 16,
               "PksSessionMetricsSnapshot first counter offset");
_Static_assert(offsetof(PksSessionAudioBatch, handle) == 16,
               "PksSessionAudioBatch handle offset");
_Static_assert(offsetof(PksSessionAudioFrame, session_id) == 24,
               "PksSessionAudioFrame lineage offset");
_Static_assert(offsetof(PksSessionAudioFrame, samples) == 128,
               "PksSessionAudioFrame sample pointer offset");
_Static_assert(PKS_SESSION_STATUS_UNSUPPORTED_ABI_MINOR == 17u,
               "status constants drifted");
_Static_assert(PKS_SESSION_HANDLE_AUDIO_BATCH == 3u,
               "handle constants drifted");
_Static_assert(PKS_SESSION_EVENT_TERMINAL == 5u, "event constants drifted");
_Static_assert(PKS_SESSION_SAMPLE_F32_INTERLEAVED == 1u,
               "sample format constants drifted");
_Static_assert(PKS_SESSION_ENDPOINT_OBSERVATION_UNAVAILABLE == 0u,
               "endpoint observation stage constants drifted");
_Static_assert(PKS_SESSION_ENDPOINT_OBSERVATION_FINALIZED == 2u,
               "endpoint observation stage constants drifted");
#endif

int pks_session_c_header_conformance(void) {
  static const uint8_t app_name[] =
      "__pks_session_c_missing_application_source__";
  PksSessionAbiVersion version = {0};
  PksSessionEngineConfig config = {
      .struct_size_bytes = sizeof(PksSessionEngineConfig),
      .abi_major = PKS_SESSION_ABI_MAJOR,
      .abi_minor = PKS_SESSION_ABI_MINOR,
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
      .abi_minor = PKS_SESSION_ABI_MINOR,
      .application_name = {app_name, sizeof(app_name) - 1},
  };
  PksSessionHandle engine = {0};
  PksSessionHandle foreign_engine = {0};
  PksSessionHandle rejected_engine = {0};
  PksSessionHandle session = {0};
  PksSessionEvent event = {0};
  PksSessionMetricsSnapshot metrics = {0};
  uint32_t source_metrics_count = UINT32_MAX;
  uint32_t route_metrics_count = UINT32_MAX;
  PksSessionSourceMetrics source_metrics = {0};
  PksSessionAudioBatch batch = {0};
  PksSessionLifecycleState state = PKS_SESSION_LIFECYCLE_FAILED;

  PksSessionStatus status = pks_session_abi_get_version(&version);
  if (status.code != PKS_SESSION_STATUS_OK) {
    return 1;
  }
  status = pks_session_engine_create(&config, NULL);
  if (status.code != PKS_SESSION_STATUS_NULL_ARGUMENT) {
    return 101;
  }
  PksSessionEngineConfig short_config = config;
  short_config.struct_size_bytes = sizeof(PksSessionAbiVersion);
  status = pks_session_engine_create(&short_config, &rejected_engine);
  if (status.code != PKS_SESSION_STATUS_INVALID_STRUCT_SIZE) {
    return 102;
  }
  status = pks_session_engine_create(&config, &engine);
  if (status.code != PKS_SESSION_STATUS_OK) {
    return 2;
  }
  status = pks_session_create_app_mic(engine, &declaration, &session);
  if (status.code != PKS_SESSION_STATUS_OK) {
    return 3;
  }
  status = pks_session_compile(engine, session);
  if (status.code != PKS_SESSION_STATUS_OK) {
    return 4;
  }
  status = pks_session_get_state(engine, session, &state);
  if (status.code != PKS_SESSION_STATUS_OK ||
      state != PKS_SESSION_LIFECYCLE_COMPILED) {
    return 5;
  }
  status = pks_session_engine_create(&config, &foreign_engine);
  if (status.code != PKS_SESSION_STATUS_OK) {
    return 6;
  }
  status = pks_session_get_state(foreign_engine, session, &state);
  if (status.code != PKS_SESSION_STATUS_FOREIGN_HANDLE) {
    return 7;
  }
  status = pks_session_engine_destroy(foreign_engine);
  if (status.code != PKS_SESSION_STATUS_OK) {
    return 8;
  }
  status = pks_session_start(engine, session);
  if (status.code != PKS_SESSION_STATUS_BACKEND_FAILURE) {
    return 9;
  }
  status = pks_session_get_state(engine, session, &state);
  if (status.code != PKS_SESSION_STATUS_OK ||
      state != PKS_SESSION_LIFECYCLE_FAILED) {
    return 10;
  }
  status = pks_session_event_poll(engine, session, &event);
  if (status.code != PKS_SESSION_STATUS_OK ||
      event.abi_major != PKS_SESSION_ABI_MAJOR ||
      event.abi_minor != PKS_SESSION_ABI_MINOR) {
    return 11;
  }
  status = pks_session_metrics_poll(engine, session, &metrics);
  if (status.code != PKS_SESSION_STATUS_OK ||
      metrics.abi_major != PKS_SESSION_ABI_MAJOR ||
      metrics.abi_minor != PKS_SESSION_ABI_MINOR ||
      metrics.event_capacity_count != config.session_event_capacity_count) {
    return 12;
  }
  status =
      pks_session_source_metrics_count(engine, session, &source_metrics_count);
  if (status.code != PKS_SESSION_STATUS_OK || source_metrics_count != 0u) {
    return 105;
  }
  status =
      pks_session_route_metrics_count(engine, session, &route_metrics_count);
  if (status.code != PKS_SESSION_STATUS_OK || route_metrics_count != 0u) {
    return 106;
  }
  status =
      pks_session_source_metrics_at(engine, session, 0u, &source_metrics);
  if (status.code != PKS_SESSION_STATUS_INDEX_OUT_OF_RANGE) {
    return 104;
  }
  status = pks_session_audio_poll(engine, session, &batch);
  if (status.code != PKS_SESSION_STATUS_INVALID_LIFECYCLE_STATE) {
    return 13;
  }
  status = pks_session_destroy(engine, session);
  if (status.code != PKS_SESSION_STATUS_OK) {
    return 14;
  }
  status = pks_session_get_state(engine, session, &state);
  if (status.code != PKS_SESSION_STATUS_STALE_HANDLE) {
    return 103;
  }
  status = pks_session_engine_destroy(engine);
  if (status.code != PKS_SESSION_STATUS_OK) {
    return 15;
  }
  status = pks_session_engine_destroy(engine);
  if (status.code != PKS_SESSION_STATUS_STALE_HANDLE) {
    return 16;
  }
  return 0;
}

int main(void) {
  return pks_session_c_header_conformance();
}
