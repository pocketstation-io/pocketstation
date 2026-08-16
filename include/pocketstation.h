#ifndef POCKETSTATION_H
#define POCKETSTATION_H

#include <stdint.h>

#ifdef __cplusplus
extern "C" {
#endif

#define PKS_SESSION_ABI_MAJOR 1u
#define PKS_SESSION_ABI_MINOR 1u

#define PKS_EXTENSION_ABI_MAJOR 1u
#define PKS_EXTENSION_ABI_MINOR 2u
#define PKS_EXTENSION_LIBRARY_ENTRYPOINT_V1 "pks_extension_library_v1"

typedef uint32_t PksSessionStatusCode;
#define PKS_SESSION_STATUS_OK 0u
#define PKS_SESSION_STATUS_NULL_ARGUMENT 1u
#define PKS_SESSION_STATUS_UNSUPPORTED_ABI_MAJOR 3u
#define PKS_SESSION_STATUS_INVALID_STRUCT_SIZE 4u
#define PKS_SESSION_STATUS_INVALID_HANDLE 5u
#define PKS_SESSION_STATUS_STALE_HANDLE 6u
#define PKS_SESSION_STATUS_NO_CAPACITY 7u
#define PKS_SESSION_STATUS_INTERNAL_PANIC 8u
#define PKS_SESSION_STATUS_MISALIGNED_POINTER 9u
#define PKS_SESSION_STATUS_INVALID_ARGUMENT 10u
#define PKS_SESSION_STATUS_FOREIGN_HANDLE 11u
#define PKS_SESSION_STATUS_INVALID_LIFECYCLE_STATE 12u
#define PKS_SESSION_STATUS_WOULD_BLOCK 13u
#define PKS_SESSION_STATUS_BACKEND_FAILURE 14u
#define PKS_SESSION_STATUS_CANCELLED 15u
#define PKS_SESSION_STATUS_INDEX_OUT_OF_RANGE 16u
#define PKS_SESSION_STATUS_UNSUPPORTED_ABI_MINOR 17u

typedef uint32_t PksSessionHandleKind;
#define PKS_SESSION_HANDLE_INVALID 0u
#define PKS_SESSION_HANDLE_ENGINE 1u
#define PKS_SESSION_HANDLE_SESSION 2u
#define PKS_SESSION_HANDLE_AUDIO_BATCH 3u

typedef uint32_t PksSessionLifecycleState;
#define PKS_SESSION_LIFECYCLE_DRAFT 0u
#define PKS_SESSION_LIFECYCLE_COMPILED 1u
#define PKS_SESSION_LIFECYCLE_RUNNING 2u
#define PKS_SESSION_LIFECYCLE_STOPPED 3u
#define PKS_SESSION_LIFECYCLE_FAILED 4u
#define PKS_SESSION_LIFECYCLE_STARTING 5u
#define PKS_SESSION_LIFECYCLE_STOPPING 6u

typedef uint32_t PksSessionEventKind;
#define PKS_SESSION_EVENT_LIFECYCLE 0u
#define PKS_SESSION_EVENT_SOURCE_FAILURE 1u
#define PKS_SESSION_EVENT_ENDPOINT_FAILURE 2u
#define PKS_SESSION_EVENT_ROLLBACK_FAILURE 3u
#define PKS_SESSION_EVENT_FINALIZATION_FAILURE 4u
#define PKS_SESSION_EVENT_TERMINAL 5u

typedef uint32_t PksSessionSampleFormat;
#define PKS_SESSION_SAMPLE_F32_INTERLEAVED 1u

typedef uint32_t PksSessionEndpointObservationStage;
#define PKS_SESSION_ENDPOINT_OBSERVATION_UNAVAILABLE 0u
#define PKS_SESSION_ENDPOINT_OBSERVATION_LIVE 1u
#define PKS_SESSION_ENDPOINT_OBSERVATION_FINALIZED 2u

typedef struct {
  uint32_t struct_size_bytes;
  uint16_t abi_major;
  uint16_t abi_minor;
} PksSessionAbiVersion;

typedef struct {
  PksSessionHandleKind kind;
  uint32_t slot_index;
  uint64_t generation;
  uint64_t scope_id;
} PksSessionHandle;

typedef struct {
  PksSessionStatusCode code;
  uint32_t detail;
} PksSessionStatus;

typedef struct {
  const uint8_t *data;
  uint32_t len_bytes;
} PksSessionUtf8;

typedef uint32_t PksExtensionKind;
#define PKS_EXTENSION_KIND_SOURCE 1u
#define PKS_EXTENSION_KIND_OPERATOR 2u
#define PKS_EXTENSION_KIND_ENDPOINT 3u

typedef uint32_t PksExtensionPortDirection;
#define PKS_EXTENSION_PORT_INPUT 1u
#define PKS_EXTENSION_PORT_OUTPUT 2u

typedef struct {
  uint32_t struct_size_bytes;
  uint16_t abi_major;
  uint16_t abi_minor;
} PksExtensionAbiVersion;

typedef struct {
  uint32_t struct_size_bytes;
  uint16_t abi_major;
  uint16_t abi_minor;
  PksExtensionKind kind;
  uint32_t revision;
  uint32_t generation;
  uint32_t port_count;
  PksSessionUtf8 extension_id;
} PksExtensionDescriptor;

typedef struct {
  uint32_t struct_size_bytes;
  uint16_t abi_major;
  uint16_t abi_minor;
  PksExtensionPortDirection direction;
  uint32_t required;
  PksSessionUtf8 name;
  PksSessionUtf8 signal_id;
  PksSessionUtf8 semantic_role;
  PksSessionUtf8 schema;
} PksExtensionPort;

#define PKS_EXTENSION_SIGNAL_END_OF_STREAM 1u
#define PKS_EXTENSION_SIGNAL_TERMINAL 2u

typedef struct {
  uint32_t struct_size_bytes;
  uint16_t abi_major;
  uint16_t abi_minor;
  const uint8_t *data;
  uint32_t len_bytes;
  uint32_t flags;
  uint64_t observed_timestamp_ns;
  uint64_t source_timestamp_ns;
  uint64_t duration_ns;
  uint64_t sequence_number;
} PksExtensionSignalView;

typedef struct {
  uint32_t struct_size_bytes;
  uint16_t abi_major;
  uint16_t abi_minor;
  uint8_t *data;
  uint32_t capacity_bytes;
  uint32_t len_bytes;
  uint32_t flags;
  uint64_t observed_timestamp_ns;
  uint64_t source_timestamp_ns;
  uint64_t duration_ns;
} PksExtensionSignalBuffer;

typedef PksSessionStatus (*PksExtensionPrepareCallback)(void *context);
/*
 * Extension callbacks execute only on PocketStation control, blocking, or
 * asynchronous worker threads. PocketStation never invokes them from a
 * capture callback or realtime audio partition. Calls for one instance are
 * serialized. A callback must not retain signal view/buffer pointers after it
 * returns. Returning a non-OK status records a typed extension failure.
 *
 * registration_context remains caller-owned until destroy_registration.
 * Each successful create transfers one instance context to PocketStation;
 * destroy_instance is invoked exactly once after all calls for that instance
 * finish. destroy_registration is invoked exactly once after the final
 * instance and registration use finish. No callback may unwind across this C
 * boundary.
 */
typedef PksSessionStatus (*PksExtensionValidateConfigurationCallback)(
    void *registration_context,
    PksSessionUtf8 configuration);
typedef PksSessionStatus (*PksExtensionCreateCallback)(
    void *registration_context,
    PksSessionUtf8 configuration,
    void **output_instance_context);
typedef PksSessionStatus (*PksExtensionSourceNextCallback)(
    void *context,
    uint32_t cancellation_requested,
    PksExtensionSignalBuffer *output);
typedef PksSessionStatus (*PksExtensionOperatorProcessCallback)(
    void *context,
    const PksExtensionSignalView *input,
    PksExtensionSignalBuffer *output);
typedef PksSessionStatus (*PksExtensionEndpointConsumeCallback)(
    void *context,
    const PksExtensionSignalView *input);
typedef PksSessionStatus (*PksExtensionStopCallback)(void *context);
typedef PksSessionStatus (*PksExtensionFinishCallback)(void *context);
typedef void (*PksExtensionDestroyCallback)(void *context);

typedef struct {
  uint32_t struct_size_bytes;
  uint16_t abi_major;
  uint16_t abi_minor;
  void *registration_context;
  uint32_t max_payload_bytes;
  uint32_t reserved;
  PksExtensionValidateConfigurationCallback validate_configuration;
  PksExtensionCreateCallback create;
  PksExtensionPrepareCallback prepare;
  PksExtensionSourceNextCallback source_next;
  PksExtensionOperatorProcessCallback operator_process;
  PksExtensionEndpointConsumeCallback endpoint_consume;
  PksExtensionStopCallback request_stop;
  PksExtensionFinishCallback finish;
  PksExtensionDestroyCallback destroy_instance;
  PksExtensionDestroyCallback destroy_registration;
} PksExtensionCallbacks;

/*
 * A packaged native extension exports exactly one function named
 * pks_extension_library_v1 with the PksExtensionLibraryEntrypoint signature.
 * The host resolves that symbol from a caller-supplied absolute library path.
 * It never searches the ambient process library path.
 *
 * acquire_registration writes one descriptor and callback table by value and
 * returns a borrowed port array. PocketStation copies all descriptor and port
 * data before the call for the next index. On OK, ownership of the callback
 * registration_context transfers to PocketStation and ends through exactly
 * one destroy_registration call. On error, ownership remains with the
 * extension library.
 *
 * PocketStation keeps the library loaded until every acquired instance and
 * registration context has been destroyed. Entry-point and acquisition calls,
 * like all executable extension callbacks, must not unwind across C.
 */
typedef PksSessionStatus (*PksExtensionAcquireRegistrationCallback)(
    void *library_context,
    uint32_t registration_index,
    PksExtensionDescriptor *output_descriptor,
    const PksExtensionPort **output_ports,
    uint32_t *output_port_count,
    PksExtensionCallbacks *output_callbacks);

typedef struct {
  uint32_t struct_size_bytes;
  uint16_t abi_major;
  uint16_t abi_minor;
  uint32_t registration_count;
  uint32_t reserved;
  void *library_context;
  PksExtensionAcquireRegistrationCallback acquire_registration;
} PksExtensionLibrary;

typedef PksSessionStatus (*PksExtensionLibraryEntrypoint)(
    PksExtensionLibrary *output_library);

typedef struct {
  uint32_t struct_size_bytes;
  uint16_t abi_major;
  uint16_t abi_minor;
  PksSessionUtf8 source_id;
  PksSessionUtf8 source_output_port;
  PksSessionUtf8 operator_id;
  PksSessionUtf8 operator_input_port;
  PksSessionUtf8 operator_output_port;
  PksSessionUtf8 endpoint_id;
  PksSessionUtf8 endpoint_input_port;
} PksExtensionPipelineDeclaration;

typedef struct {
  uint32_t struct_size_bytes;
  uint16_t abi_major;
  uint16_t abi_minor;
  uint32_t external_source_count;
  uint32_t operator_count;
  uint32_t derived_route_count;
  uint32_t reserved;
  uint64_t maximum_buffered_payload_bytes;
  uint64_t source_emitted_total;
  uint64_t source_dropped_total;
  uint64_t source_failure_total;
  uint64_t source_cancellation_total;
  uint64_t operator_input_capacity_signals;
  uint64_t operator_input_depth_signals;
  uint64_t operator_input_peak_signals;
  uint64_t operator_input_enqueued_total;
  uint64_t operator_input_dropped_total;
  uint64_t operator_processed_total;
  uint64_t operator_output_emitted_total;
  uint64_t operator_output_dropped_total;
  uint64_t operator_process_failure_total;
  uint64_t operator_timeout_total;
  uint64_t operator_cancellation_total;
  uint64_t route_capacity_signals;
  uint64_t route_depth_signals;
  uint64_t route_peak_signals;
  uint64_t route_enqueued_total;
  uint64_t route_received_total;
  uint64_t route_dropped_total;
  uint64_t endpoint_received_total;
  uint64_t endpoint_delivered_total;
  uint64_t endpoint_dropped_total;
  uint64_t endpoint_failure_total;
} PksExtensionMetricsSnapshot;

typedef struct {
  uint32_t struct_size_bytes;
  uint16_t abi_major;
  uint16_t abi_minor;
  uint32_t source_queue_capacity_frames;
  uint32_t capture_frame_capacity_frames;
  uint32_t capture_runtime_event_capacity_count;
  uint32_t runtime_work_budget_frames;
  uint64_t runtime_idle_poll_ms;
  uint64_t runtime_ready_timeout_ms;
  uint32_t session_event_capacity_count;
  uint32_t audio_queue_capacity_frames;
  uint32_t audio_max_batch_frames;
  uint32_t audio_max_outstanding_leases;
} PksSessionEngineConfig;

typedef struct {
  uint32_t struct_size_bytes;
  uint16_t abi_major;
  uint16_t abi_minor;
  PksSessionUtf8 application_name;
} PksSessionAppMicDeclaration;

typedef struct {
  uint32_t struct_size_bytes;
  uint16_t abi_major;
  uint16_t abi_minor;
  PksSessionEventKind kind;
  PksSessionLifecycleState lifecycle_state;
  uint32_t reserved;
  uint64_t session_id;
  uint64_t stem_id;
  uint64_t endpoint_id;
  uint64_t route_id;
  uint64_t failures_total;
} PksSessionEvent;

typedef struct {
  uint32_t struct_size_bytes;
  uint16_t abi_major;
  uint16_t abi_minor;
  uint32_t reserved;
  uint64_t event_capacity_count;
  uint64_t event_depth_count;
  uint64_t event_peak_depth_count;
  uint64_t events_enqueued_total;
  uint64_t events_dropped_total;
  uint64_t event_receiver_closed_total;
  uint64_t audio_queue_capacity_frames;
  uint64_t audio_queue_depth_frames;
  uint64_t audio_queue_peak_frames;
  uint64_t audio_frames_received_total;
  uint64_t audio_frames_delivered_total;
  uint64_t audio_queue_full_drops_total;
  uint64_t audio_invalid_ownership_drops_total;
  uint64_t audio_lease_capacity_count;
  uint64_t audio_outstanding_leases_count;
  uint64_t audio_lease_exhausted_total;
  uint64_t audio_batches_polled_total;
  uint64_t audio_frames_polled_total;
} PksSessionMetricsSnapshot;

typedef struct {
  uint32_t struct_size_bytes;
  uint16_t abi_major;
  uint16_t abi_minor;
  uint64_t stem_id;
  uint64_t capture_callback_buffers_total;
  uint64_t capture_frames_enqueued_total;
  uint64_t capture_pool_exhausted_total;
  uint64_t capture_dispatch_queue_full_total;
  uint64_t capture_invalid_buffer_total;
  uint64_t capture_oversized_buffer_total;
  uint64_t capture_stream_errors_total;
  uint64_t capture_timestamp_epoch_clamps_total;
  uint64_t capture_stream_delivered_frames_total;
  uint64_t capture_stream_dropped_newest_frames_total;
  uint64_t capture_runtime_events_enqueued_total;
  uint64_t capture_runtime_events_dropped_total;
  uint64_t ingress_queue_capacity_frames;
  uint64_t ingress_queue_depth_frames;
  uint64_t ingress_queue_peak_frames;
  uint64_t ingress_frames_enqueued_total;
  uint64_t ingress_frames_delivered_total;
  uint64_t ingress_frames_rejected_full_total;
  uint64_t ingress_frames_rejected_cancelled_total;
  uint64_t ingress_frames_discarded_total;
} PksSessionSourceMetrics;

typedef struct {
  uint32_t struct_size_bytes;
  uint16_t abi_major;
  uint16_t abi_minor;
  uint64_t route_id;
  uint64_t endpoint_id;
  uint64_t edge_queue_capacity_frames;
  uint64_t edge_queue_depth_frames;
  uint64_t edge_queue_peak_frames;
  uint64_t edge_frames_enqueued_total;
  uint64_t edge_frames_delivered_total;
  uint64_t edge_frames_dropped_total;
  uint64_t edge_overruns_total;
  uint64_t edge_receiver_unavailable_drops_total;
  uint64_t edge_queue_full_drops_total;
  uint64_t edge_shared_reference_exhausted_drops_total;
  uint64_t edge_branch_pool_exhausted_drops_total;
  uint64_t edge_invalid_copy_policy_drops_total;
  uint64_t edge_freeze_failed_drops_total;
  uint64_t edge_discontinuities_total;
  uint64_t edge_source_identity_discontinuities_total;
  uint64_t edge_sequence_discontinuities_total;
  uint64_t edge_timestamp_discontinuities_total;
  uint64_t edge_lineage_epoch_discontinuities_total;
  uint64_t edge_manually_reported_discontinuities_total;
  uint64_t edge_enqueue_to_receive_samples_total;
  uint64_t edge_enqueue_to_receive_invalid_order_total;
  uint64_t edge_enqueue_to_receive_p50_ns;
  uint64_t edge_enqueue_to_receive_p95_ns;
  uint64_t edge_enqueue_to_receive_p99_ns;
  uint64_t edge_enqueue_to_receive_max_ns;
  uint64_t edge_source_timestamp_to_receive_samples_total;
  uint64_t edge_source_timestamp_to_receive_missing_total;
  uint64_t edge_source_timestamp_to_receive_future_total;
  uint64_t edge_source_timestamp_to_receive_p50_ns;
  uint64_t edge_source_timestamp_to_receive_p95_ns;
  uint64_t edge_source_timestamp_to_receive_p99_ns;
  uint64_t edge_source_timestamp_to_receive_max_ns;
  uint64_t edge_worker_failures_total;
  uint64_t edge_shutdown_discarded_total;
  uint64_t endpoint_frames_received_total;
  uint64_t endpoint_frames_delivered_total;
  uint64_t endpoint_frames_dropped_total;
  uint64_t endpoint_discontinuities_total;
  uint64_t endpoint_failures_total;
  PksSessionEndpointObservationStage endpoint_observation_stage;
  uint32_t reserved;
  uint64_t endpoint_finalization_failures_total;
} PksSessionRouteMetrics;

typedef struct {
  uint32_t struct_size_bytes;
  uint16_t abi_major;
  uint16_t abi_minor;
  uint32_t frame_count;
  uint32_t reserved;
  PksSessionHandle handle;
} PksSessionAudioBatch;

typedef struct {
  uint32_t struct_size_bytes;
  uint16_t abi_major;
  uint16_t abi_minor;
  PksSessionSampleFormat sample_format;
  uint32_t sample_rate_hz;
  uint32_t channel_count;
  uint64_t session_id;
  uint64_t source_id;
  uint64_t stem_id;
  uint64_t clock_id;
  uint64_t sequence_num;
  uint64_t timestamp_start_ns;
  uint64_t duration_ns;
  uint32_t source_generation;
  uint32_t reserved;
  uint64_t discontinuity_epoch;
  uint64_t permission_epoch;
  uint64_t endpoint_id;
  uint64_t connector_id;
  uint64_t route_id;
  const float *samples;
  uint32_t sample_count;
  uint32_t reserved_tail;
} PksSessionAudioFrame;

PksSessionStatus pks_session_abi_get_version(
    PksSessionAbiVersion *output_version);
PksSessionStatus pks_session_abi_is_compatible(
    uint16_t requested_abi_major,
    uint16_t requested_abi_minor,
    uint32_t requested_struct_size_bytes);
PksSessionStatus pks_extension_abi_get_version(
    PksExtensionAbiVersion *output_version);
PksSessionStatus pks_extension_abi_is_compatible(
    uint16_t requested_abi_major,
    uint16_t requested_abi_minor,
    uint32_t requested_struct_size_bytes);
PksSessionStatus pks_extension_descriptor_validate(
    const PksExtensionDescriptor *descriptor,
    const PksExtensionPort *ports,
    uint32_t port_count);
/* Implemented and exported by a packaged extension library, not by Core. */
PksSessionStatus pks_extension_library_v1(
    PksExtensionLibrary *output_library);
PksSessionStatus pks_session_engine_register_extension(
    PksSessionHandle engine,
    const PksExtensionDescriptor *descriptor,
    const PksExtensionPort *ports,
    uint32_t port_count,
    const PksExtensionCallbacks *callbacks);
PksSessionStatus pks_session_create_extension_pipeline(
    PksSessionHandle engine,
    const PksExtensionPipelineDeclaration *declaration,
    PksSessionHandle *output_session);

PksSessionStatus pks_session_extension_metrics_poll(
    PksSessionHandle engine,
    PksSessionHandle session,
    PksExtensionMetricsSnapshot *output_metrics);
PksSessionStatus pks_session_engine_create(
    const PksSessionEngineConfig *config,
    PksSessionHandle *output_engine);
PksSessionStatus pks_session_engine_destroy(PksSessionHandle engine);
PksSessionStatus pks_session_engine_is_live(
    PksSessionHandle engine,
    uint32_t *output_is_live);
PksSessionStatus pks_session_create_app_mic(
    PksSessionHandle engine,
    const PksSessionAppMicDeclaration *declaration,
    PksSessionHandle *output_session);
PksSessionStatus pks_session_compile(
    PksSessionHandle engine,
    PksSessionHandle session);
PksSessionStatus pks_session_start(
    PksSessionHandle engine,
    PksSessionHandle session);
PksSessionStatus pks_session_stop(
    PksSessionHandle engine,
    PksSessionHandle session);
PksSessionStatus pks_session_get_state(
    PksSessionHandle engine,
    PksSessionHandle session,
    PksSessionLifecycleState *output_state);
PksSessionStatus pks_session_destroy(
    PksSessionHandle engine,
    PksSessionHandle session);
PksSessionStatus pks_session_event_poll(
    PksSessionHandle engine,
    PksSessionHandle session,
    PksSessionEvent *output_event);
PksSessionStatus pks_session_metrics_poll(
    PksSessionHandle engine,
    PksSessionHandle session,
    PksSessionMetricsSnapshot *output_metrics);
PksSessionStatus pks_session_source_metrics_count(
    PksSessionHandle engine,
    PksSessionHandle session,
    uint32_t *output_count);
PksSessionStatus pks_session_route_metrics_count(
    PksSessionHandle engine,
    PksSessionHandle session,
    uint32_t *output_count);
PksSessionStatus pks_session_source_metrics_at(
    PksSessionHandle engine,
    PksSessionHandle session,
    uint32_t source_index,
    PksSessionSourceMetrics *output_metrics);
PksSessionStatus pks_session_route_metrics_at(
    PksSessionHandle engine,
    PksSessionHandle session,
    uint32_t route_index,
    PksSessionRouteMetrics *output_metrics);
PksSessionStatus pks_session_audio_poll(
    PksSessionHandle engine,
    PksSessionHandle session,
    PksSessionAudioBatch *output_batch);
PksSessionStatus pks_session_audio_batch_frame(
    PksSessionHandle engine,
    PksSessionHandle batch,
    uint32_t frame_index,
    PksSessionAudioFrame *output_frame);
PksSessionStatus pks_session_audio_batch_release(
    PksSessionHandle engine,
    PksSessionHandle batch);

/*
 * Compatibility codec ABI. These symbols remain in libpocketstation for the
 * current Swift/Kotlin migration window; there is no separate codec engine.
 */
typedef enum PksCodecErrorCode {
  PksCodecErrorCode_InvalidPointer = -1,
  PksCodecErrorCode_Encode = -2,
  PksCodecErrorCode_OutputTooSmall = -3,
  PksCodecErrorCode_InvalidFrame = -4,
  PksCodecErrorCode_InternalPanic = -5,
} PksCodecErrorCode;

#define PKS_CODEC_INVALID_POINTER PksCodecErrorCode_InvalidPointer
#define PKS_CODEC_ENCODE_ERROR PksCodecErrorCode_Encode
#define PKS_CODEC_OUTPUT_TOO_SMALL PksCodecErrorCode_OutputTooSmall
#define PKS_CODEC_INVALID_FRAME PksCodecErrorCode_InvalidFrame
#define PKS_CODEC_INTERNAL_PANIC PksCodecErrorCode_InternalPanic

typedef struct PksOpusEncoder PksOpusEncoder;

uintptr_t pks_opus_max_packet_bytes(void);
PksOpusEncoder *pks_opus_encoder_create(
    unsigned int sample_rate,
    uint8_t channels,
    unsigned int bitrate_kbps);
void pks_opus_encoder_destroy(PksOpusEncoder *encoder);
int pks_opus_encoder_set_bitrate(
    PksOpusEncoder *encoder,
    unsigned int bitrate_kbps);
int pks_encode_opus(
    PksOpusEncoder *encoder,
    const float *pcm,
    uintptr_t sample_count,
    unsigned char *output,
    uintptr_t output_capacity);

#ifdef __cplusplus
}
#endif

#endif
