//
//  at.h
//  AudioToolbox
//
//  Created by Yury Korolev on 15.08.2025.
//

#import <AudioToolbox/AudioToolbox.h>

NS_ASSUME_NONNULL_BEGIN

Class AU_AUDIO_UNIT;

__attribute__((constructor))
static void at_initializer(void)
{
    static int initialized = 0;
    if (!initialized) {
        initialized = 1;

        AU_AUDIO_UNIT = [AUAudioUnit class];
    }
    
}


NS_ASSUME_NONNULL_END
