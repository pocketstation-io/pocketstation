#include "pks_session.h"

PksSessionStatus pks_session_conformance_engine_create(
    const PksSessionEngineConfig *config,
    PksSessionHandle *output_engine);
PksSessionStatus pks_session_conformance_panic(void);

static PksSessionStatus poll_audio_until_ready(
    PksSessionHandle engine,
    PksSessionHandle session,
    PksSessionAudioBatch *batch) {
  PksSessionStatus status = {0};
  for (uint32_t attempt = 0; attempt < 1000000u; ++attempt) {
    status = pks_session_audio_poll(engine, session, batch);
    if (status.code != PKS_SESSION_STATUS_WOULD_BLOCK) {
      return status;
    }
  }
  return status;
}

int main(void) {
  static const uint8_t app_name[] = "deterministic application";
  PksSessionEngineConfig config = {
      .struct_size_bytes = sizeof(PksSessionEngineConfig),
      .abi_major = PKS_SESSION_ABI_MAJOR,
      .abi_minor = PKS_SESSION_ABI_MINOR,
      .source_queue_capacity_frames = 8,
      .capture_frame_capacity_frames = 8,
      .capture_runtime_event_capacity_count = 8,
      .runtime_work_budget_frames = 64,
      .runtime_idle_poll_ms = 1,
      .runtime_ready_timeout_ms = 1000,
      .session_event_capacity_count = 32,
      .audio_queue_capacity_frames = 8,
      .audio_max_batch_frames = 1,
      .audio_max_outstanding_leases = 1,
  };
  PksSessionAppMicDeclaration declaration = {
      .struct_size_bytes = sizeof(PksSessionAppMicDeclaration),
      .abi_major = PKS_SESSION_ABI_MAJOR,
      .abi_minor = PKS_SESSION_ABI_MINOR,
      .application_name = {app_name, sizeof(app_name) - 1},
  };
  PksSessionHandle engine = {0};
  PksSessionHandle session = {0};
  PksSessionAudioBatch first_batch = {0};
  PksSessionAudioBatch exhausted_batch = {0};
  PksSessionAudioBatch second_batch = {0};
  PksSessionAudioFrame first_frame = {0};
  PksSessionAudioFrame second_frame = {0};
  PksSessionAudioFrame frame_after_stop = {0};
  PksSessionMetricsSnapshot metrics = {0};
  uint32_t source_metrics_count = 0;
  uint32_t route_metrics_count = 0;
  PksSessionSourceMetrics source_metrics = {0};
  PksSessionRouteMetrics route_metrics = {0};
  PksSessionAbiVersion version = {0};

  PksSessionStatus status =
      pks_session_conformance_engine_create(&config, &engine);
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
  if (status.code != PKS_SESSION_STATUS_OK) {
    return 4;
  }
  status = poll_audio_until_ready(engine, session, &first_batch);
  if (status.code != PKS_SESSION_STATUS_OK || first_batch.frame_count != 1u ||
      first_batch.abi_major != PKS_SESSION_ABI_MAJOR) {
    return 5;
  }
  status = pks_session_audio_batch_frame(
      engine, first_batch.handle, 0, &first_frame);
  if (status.code != PKS_SESSION_STATUS_OK || first_frame.sample_count != 4u ||
      first_frame.samples == 0 || first_frame.samples[0] != 0.125f) {
    return 6;
  }
  status = pks_session_audio_poll(engine, session, &exhausted_batch);
  if (status.code != PKS_SESSION_STATUS_NO_CAPACITY) {
    return 7;
  }
  status = pks_session_metrics_poll(engine, session, &metrics);
  if (status.code != PKS_SESSION_STATUS_OK ||
      metrics.audio_outstanding_leases_count != 1u ||
      metrics.audio_lease_exhausted_total != 1u) {
    return 8;
  }
  status =
      pks_session_source_metrics_count(engine, session, &source_metrics_count);
  if (status.code != PKS_SESSION_STATUS_OK || source_metrics_count != 2u) {
    return 25;
  }
  status =
      pks_session_route_metrics_count(engine, session, &route_metrics_count);
  if (status.code != PKS_SESSION_STATUS_OK || route_metrics_count != 2u) {
    return 26;
  }
  status =
      pks_session_source_metrics_at(engine, session, 0u, &source_metrics);
  if (status.code != PKS_SESSION_STATUS_OK || source_metrics.stem_id == 0u ||
      source_metrics.ingress_queue_capacity_frames !=
          config.source_queue_capacity_frames) {
    return 20;
  }
  status = pks_session_route_metrics_at(engine, session, 0u, &route_metrics);
  if (status.code != PKS_SESSION_STATUS_OK || route_metrics.route_id == 0u ||
      route_metrics.endpoint_id == 0u ||
      route_metrics.endpoint_observation_stage !=
          PKS_SESSION_ENDPOINT_OBSERVATION_LIVE) {
    return 21;
  }
  status =
      pks_session_source_metrics_at(engine, session, 2u, &source_metrics);
  if (status.code != PKS_SESSION_STATUS_INDEX_OUT_OF_RANGE) {
    return 22;
  }
  status = pks_session_audio_batch_release(engine, first_batch.handle);
  if (status.code != PKS_SESSION_STATUS_OK) {
    return 9;
  }
  status = poll_audio_until_ready(engine, session, &second_batch);
  if (status.code != PKS_SESSION_STATUS_OK || second_batch.frame_count != 1u) {
    return 10;
  }
  status = pks_session_audio_batch_frame(
      engine, second_batch.handle, 0, &second_frame);
  if (status.code != PKS_SESSION_STATUS_OK ||
      second_frame.stem_id == first_frame.stem_id ||
      second_frame.samples[0] != 0.125f) {
    return 11;
  }
  status = pks_session_stop(engine, session);
  if (status.code != PKS_SESSION_STATUS_OK) {
    return 12;
  }
  status = pks_session_route_metrics_at(engine, session, 0u, &route_metrics);
  if (status.code != PKS_SESSION_STATUS_OK ||
      route_metrics.endpoint_observation_stage !=
          PKS_SESSION_ENDPOINT_OBSERVATION_FINALIZED ||
      route_metrics.endpoint_finalization_failures_total != 0u) {
    return 23;
  }
  status = pks_session_audio_batch_frame(
      engine, second_batch.handle, 0, &frame_after_stop);
  if (status.code != PKS_SESSION_STATUS_OK ||
      frame_after_stop.samples != second_frame.samples ||
      frame_after_stop.samples[0] != 0.125f) {
    return 13;
  }
  status = pks_session_audio_batch_release(engine, second_batch.handle);
  if (status.code != PKS_SESSION_STATUS_OK) {
    return 14;
  }
  status = pks_session_audio_batch_release(engine, second_batch.handle);
  if (status.code != PKS_SESSION_STATUS_STALE_HANDLE) {
    return 15;
  }
  status = pks_session_conformance_panic();
  if (status.code != PKS_SESSION_STATUS_INTERNAL_PANIC) {
    return 16;
  }
  status = pks_session_abi_get_version(&version);
  if (status.code != PKS_SESSION_STATUS_OK ||
      version.abi_major != PKS_SESSION_ABI_MAJOR) {
    return 17;
  }
  status = pks_session_destroy(engine, session);
  if (status.code != PKS_SESSION_STATUS_OK) {
    return 18;
  }
  status = pks_session_route_metrics_at(engine, session, 0u, &route_metrics);
  if (status.code != PKS_SESSION_STATUS_STALE_HANDLE) {
    return 24;
  }
  status = pks_session_engine_destroy(engine);
  if (status.code != PKS_SESSION_STATUS_OK) {
    return 19;
  }
  return 0;
}
