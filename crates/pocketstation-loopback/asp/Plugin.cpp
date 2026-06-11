// AudioServerPlugin entry point — compiled only when `asp` feature is enabled.
// Requires vendor/libASPL submodule; see asp/README.md for operator instructions.
#include "bridge.h"
#include <CoreAudio/AudioServerPlugIn.h>
#include <CoreFoundation/CoreFoundation.h>

// Check for plugin installation by probing the HAL plug-ins directory.
int pks_asp_is_installed(void) {
    CFURLRef url = CFURLCreateWithFileSystemPath(
        kCFAllocatorDefault,
        CFSTR("/Library/Audio/Plug-Ins/HAL/PocketStation.driver"),
        kCFURLPOSIXPathStyle,
        true);
    if (\!url) return 0;
    Boolean exists = CFURLResourceIsReachable(url, NULL);
    CFRelease(url);
    return exists ? 1 : 0;
}
