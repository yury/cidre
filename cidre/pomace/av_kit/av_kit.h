//
//  av_kit.h
//  av_kit
//
//  Created by Yury Korolev on 1/19/24.
//

#import <AVKit/AVKit.h>

NS_ASSUME_NONNULL_BEGIN

Class AV_PLAYBACK_SPEED;
Class AV_INPUT_PICKER_INTERACTION;
Class AV_CAPTURE_DEVICE_DIRECTION_COORDINATOR;

__attribute__((constructor))
static void av_kit_initializer(void)
{
    static int initialized = 0;
    if (!initialized) {
        initialized = 1;
        
        AV_PLAYBACK_SPEED = NSClassFromString(@"AVPlaybackSpeed");
        AV_INPUT_PICKER_INTERACTION = NSClassFromString(@"AVInputPickerInteraction");
        AV_CAPTURE_DEVICE_DIRECTION_COORDINATOR = NSClassFromString(@"AVCaptureDeviceDirectionCoordinator");
    }
}

NS_ASSUME_NONNULL_END
