# Operate a Session through C

<!-- claims: CLM-GUIDE-019-SCOPE-001,CLM-GUIDE-019-TEXT-001,CLM-GUIDE-019-TEXT-002,CLM-GUIDE-019-TEXT-003,CLM-GUIDE-019-TEXT-004,CLM-GUIDE-019-TEXT-005,CLM-GUIDE-019-TEXT-006,CLM-GUIDE-019-SOURCE-001 -->

## Scope

- **Use the versioned C ABI.** Declare, start, observe, stop, and release Sessions and extension callbacks through the public C boundary.
- **Classify public failures.** Expose stable typed errors and cross-boundary error codes without inferring retry or recovery guarantees.

The scope of **Operate a Session through C** ends at the native contracts and executable conditions cited below. Platform qualification, performance, retry, and delivery require their own explicit evidence.

## Prerequisites

The repository header, a compatible ABI version, and correct ownership for every opaque handle and callback context.

## Procedure

1. Include pocketstation.h and use its ABI version.
2. Create handles through exported functions.
3. Check every PksSessionStatus.
4. Stop before releasing runtime ownership.
5. Release each handle with its matching ABI function.

## Concrete repository example

The executable repository test `abi_session_c_success_conformance` (`test-fbd5f1d6e0ff13895c92`) shows the concrete API sequence and asserted outcome at `tests/abi_session_c_success_conformance.c:1`.

```c
#include "pocketstation.h"

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
```

```bash
cargo test --all-features abi_session_c_success_conformance
```

## Important consequence

Do not let a Rust panic, borrowed pointer, or library context escape its declared ABI lifetime.

## Verify the outcome

Every call returns an accepted `PksSessionStatus`, stop completes, and each handle is released by its matching function.

Executable evidence selected for **Operate a Session through C** is limited to each test's recorded setup and assertions:

- `abi_session_c_success_conformance` — abi session c success conformance (`tests/abi_session_c_success_conformance.c:1`; `test-fbd5f1d6e0ff13895c92`).
- `given_deterministic_session_when_polled_then_audio_lease_is_bounded_and_stable` — given deterministic session when polled then audio lease is bounded and stable (`src/abi/session/mod.rs:1036`; `test-5172b35e16680497535b`).
- `given_native_engine_when_created_then_real_session_declaration_compiles` — given native engine when created then real session declaration compiles (`src/abi/session/mod.rs:929`; `test-c204d11ecd759d78439f`).
- `abi_session_c_conformance` — abi session c conformance (`tests/abi_session_c_conformance.c:1`; `test-9e1beea6279253161031`).
- `abi_session_c_metrics_canary` — abi session c metrics canary (`tests/abi_session_c_metrics_canary.c:1`; `test-46202761e460d88bfd8e`).
- `given_fixture_session_when_started_then_two_stems_cross_canonical_engine` — given fixture session when started then two stems cross canonical engine (`tests/conformance_fixture.rs:14`; `test-82f8ec2b9c0fa3a0eb0b`).
- `given_bitrate_change_when_encode_then_still_produces_valid_packet` — given bitrate change when encode then still produces valid packet (`src/abi/codec.rs:416`; `test-f2e13a28f1aa591f3c67`).
- `given_encoder_when_destroy_null_then_no_crash` — given encoder when destroy null then no crash (`src/abi/codec.rs:384`; `test-8ba7b5b19e9d7dfbc464`).
- `given_invalid_channel_count_when_create_then_returns_null` — given invalid channel count when create then returns null (`src/abi/codec.rs:237`; `test-9fb4684ff29b5ab716fd`).
- `given_invalid_frame_size_when_encode_then_error_is_typed_without_writing` — given invalid frame size when encode then error is typed without writing (`src/abi/codec.rs:307`; `test-002ce44230f2b0ac6d7c`).
- `given_null_encoder_when_encode_then_returns_minus_one` — given null encoder when encode then returns minus one (`src/abi/codec.rs:273`; `test-657d1e2cbdcbd70cf5fa`).
- `given_null_encoder_when_set_bitrate_then_returns_minus_one` — given null encoder when set bitrate then returns minus one (`src/abi/codec.rs:408`; `test-f10bfad1b583316ad6fb`).

## Failure signals

No task-specific public error was resolved for operate a session through c; preserve the owning API's returned error.

## API reference

- [C Abi Ownership](/docs/concepts/c-abi-ownership.md)
- [C Abi](/docs/reference/c-abi.md)

| Public declaration | Kind | Declared purpose | Source |
|---|---|---|---|
| `pocketstation::abi::session::abi::PksSessionStatus` | struct | Reports the structured session status. | `src/abi/session/abi.rs:56` |
| `pocketstation::abi::session::abi::PksSessionUtf8` | struct | Borrows a UTF-8 byte range across the C Session ABI as a pointer and length. | `src/abi/session/abi.rs:101` |
| `pocketstation::abi::session::abi::PksSessionStatusCode` | enum | Provides stable C ABI status categories returned by Session operations. | `src/abi/session/abi.rs:79` |
| `pocketstation::abi::session::abi::PksSessionStatusCode::BackendFailure` | variant | Identifies the backend failure state or stage represented by `PksSessionStatusCode`. | `src/abi/session/abi.rs:93` |
| `pocketstation::abi::session::abi::PksSessionStatusCode::Cancelled` | variant | Indicates that the operation was cancelled. | `src/abi/session/abi.rs:94` |
| `pocketstation::abi::session::abi::PksSessionStatusCode::ForeignHandle` | variant | Identifies the foreign handle state or stage represented by `PksSessionStatusCode`. | `src/abi/session/abi.rs:90` |
| `pocketstation::abi::session::abi::PksSessionStatusCode::IndexOutOfRange` | variant | Identifies the index out of range state or stage represented by `PksSessionStatusCode`. | `src/abi/session/abi.rs:95` |
| `pocketstation::abi::session::abi::PksSessionStatusCode::InternalPanic` | variant | Identifies the internal panic state or stage represented by `PksSessionStatusCode`. | `src/abi/session/abi.rs:87` |

## Related documentation

- [Glossary](/docs/glossary.md)
- [PocketStation](/README.md)
- [Behavior evidence index](/docs/reference/behavior-evidence.md)
- [Rust API reference](/docs/reference/rust-api.md)
- [A native extension does not load](/docs/troubleshooting/native-extension-load.md)
- [Extension and ABI failures](/docs/errors/extensions-and-abi.md)
- [ABI and conformance model](/docs/internals/abi-conformance.md)
- [C ABI ownership](/docs/concepts/c-abi-ownership.md)

## Evidence boundary

The claims on **Operate a Session through C** are anchored to Git snapshot `136e74888962558aa846d3143a19136a70936f45` and these primary files:

- `tests/abi_session_c_success_conformance.c:3-23` (`DIRECT`)

For **Operate a Session through C**, direct source establishes only the recorded declaration or implementation. Tests, external fixtures, and qualification artifacts retain their narrower evidence classifications.
