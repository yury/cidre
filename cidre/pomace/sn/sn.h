//
//  sn.h
//  sn
//
//  Created by Yury Korolev on 25.12.2022.
//

#import <SoundAnalysis/SoundAnalysis.h>

NS_ASSUME_NONNULL_BEGIN

Class SN_AUDIO_STREAM_ANALYZER;
Class SN_AUDIO_FILE_ANALYZER;
Class SN_TIME_DURATION_CONSTRAINT;
Class SN_CLASSIFY_SOUND_REQUEST;
Class SN_CLASSIFICATION_RESULT;

__attribute__((constructor))
static void sn_initializer(void)
{
    static int initialized = 0;
    if (!initialized) {
        initialized = 1;
        
        SN_AUDIO_STREAM_ANALYZER = [SNAudioStreamAnalyzer class];
        SN_AUDIO_FILE_ANALYZER = [SNAudioFileAnalyzer class];
        SN_TIME_DURATION_CONSTRAINT = [SNTimeDurationConstraint class];
        SN_CLASSIFY_SOUND_REQUEST = [SNClassifySoundRequest class];
        SN_CLASSIFICATION_RESULT = [SNClassificationResult class];
    }
}


NS_ASSUME_NONNULL_END
