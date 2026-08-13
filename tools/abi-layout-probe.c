#include "pocketstation.h"

#include <stddef.h>
#include <stdio.h>

#define TYPE_LAYOUT(type)                                                     \
  printf("type %s size=%zu align=%zu\n", #type, sizeof(type), _Alignof(type))
#define FIELD_OFFSET(type, field)                                             \
  printf("field %s.%s offset=%zu\n", #type, #field, offsetof(type, field))

int main(void) {
  TYPE_LAYOUT(PksSessionAbiVersion);
  TYPE_LAYOUT(PksSessionHandle);
  TYPE_LAYOUT(PksSessionStatus);
  TYPE_LAYOUT(PksSessionUtf8);
  TYPE_LAYOUT(PksExtensionAbiVersion);
  TYPE_LAYOUT(PksExtensionDescriptor);
  TYPE_LAYOUT(PksExtensionPort);
  TYPE_LAYOUT(PksExtensionSignalView);
  TYPE_LAYOUT(PksExtensionSignalBuffer);
  TYPE_LAYOUT(PksExtensionCallbacks);
  TYPE_LAYOUT(PksExtensionPipelineDeclaration);
  TYPE_LAYOUT(PksExtensionMetricsSnapshot);
  TYPE_LAYOUT(PksSessionEngineConfig);
  TYPE_LAYOUT(PksSessionAppMicDeclaration);
  TYPE_LAYOUT(PksSessionEvent);
  TYPE_LAYOUT(PksSessionMetricsSnapshot);
  TYPE_LAYOUT(PksSessionSourceMetrics);
  TYPE_LAYOUT(PksSessionRouteMetrics);
  TYPE_LAYOUT(PksSessionAudioBatch);
  TYPE_LAYOUT(PksSessionAudioFrame);

  FIELD_OFFSET(PksExtensionCallbacks, struct_size_bytes);
  FIELD_OFFSET(PksExtensionCallbacks, abi_major);
  FIELD_OFFSET(PksExtensionCallbacks, abi_minor);
  FIELD_OFFSET(PksExtensionCallbacks, registration_context);
  FIELD_OFFSET(PksExtensionCallbacks, max_payload_bytes);
  FIELD_OFFSET(PksExtensionCallbacks, reserved);
  FIELD_OFFSET(PksExtensionCallbacks, validate_configuration);
  FIELD_OFFSET(PksExtensionCallbacks, create);
  FIELD_OFFSET(PksExtensionCallbacks, prepare);
  FIELD_OFFSET(PksExtensionCallbacks, source_next);
  FIELD_OFFSET(PksExtensionCallbacks, operator_process);
  FIELD_OFFSET(PksExtensionCallbacks, endpoint_consume);
  FIELD_OFFSET(PksExtensionCallbacks, request_stop);
  FIELD_OFFSET(PksExtensionCallbacks, finish);
  FIELD_OFFSET(PksExtensionCallbacks, destroy_instance);
  FIELD_OFFSET(PksExtensionCallbacks, destroy_registration);
  return 0;
}
