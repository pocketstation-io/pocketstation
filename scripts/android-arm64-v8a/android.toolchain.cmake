if(NOT DEFINED ENV{PKS_ANDROID_NDK_DIR})
  message(FATAL_ERROR "PKS_ANDROID_NDK_DIR is required")
endif()

set(ANDROID_ABI "arm64-v8a" CACHE STRING "PocketStation Android ABI" FORCE)
set(ANDROID_PLATFORM "android-29" CACHE STRING "PocketStation minimum Android API" FORCE)

include("$ENV{PKS_ANDROID_NDK_DIR}/build/cmake/android.toolchain.cmake")
