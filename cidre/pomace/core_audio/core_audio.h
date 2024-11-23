//
//  core_audio.h
//  core_audio
//
//  Created by Yury Korolev on 11/23/24.
//

#import <CoreAudio/CoreAudio.h>

NS_ASSUME_NONNULL_BEGIN

Class CA_TAP_DESCRIPTION;

__attribute__((constructor))
static void core_audio_initializer(void)
{
    static int initialized = 0;
    if (!initialized) {
        initialized = 1;

#if TARGET_OS_OSX
        CA_TAP_DESCRIPTION = NSClassFromString(@"CATapDescription");
#endif

    }
}

NS_ASSUME_NONNULL_END
