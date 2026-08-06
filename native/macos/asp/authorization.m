#import "authorization.h"

#import <AVFoundation/AVFoundation.h>

int32_t pks_microphone_authorization_status(void) {
    return (int32_t)[AVCaptureDevice authorizationStatusForMediaType:AVMediaTypeAudio];
}
