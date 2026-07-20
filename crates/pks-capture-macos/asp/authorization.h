#pragma once
#include <stdint.h>

// Mirrors AVAuthorizationStatus without exposing Objective-C types over FFI.
// 0 not determined, 1 restricted, 2 denied, 3 authorized.
int32_t pks_microphone_authorization_status(void);
