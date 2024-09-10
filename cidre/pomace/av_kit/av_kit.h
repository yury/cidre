//
//  av_kit.h
//  av_kit
//
//  Created by Yury Korolev on 1/19/24.
//

#import <AVKit/AVKit.h>

NS_ASSUME_NONNULL_BEGIN

Class AV_PICTURE_IN_PICTURE_CONTROLLER;
Class AV_PICTURE_IN_PICTURE_CONTROLLER_CONTENT_SOURCE;

__attribute__((constructor))
static void av_kit_initializer(void)
{
    static int initialized = 0;
    if (!initialized) {
        initialized = 1;
        
        AV_PICTURE_IN_PICTURE_CONTROLLER = [AVPictureInPictureController class];
        AV_PICTURE_IN_PICTURE_CONTROLLER_CONTENT_SOURCE = [AVPictureInPictureControllerContentSource class];
    }
}

NS_ASSUME_NONNULL_END
