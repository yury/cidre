//
//  vn.h
//  vn
//
//  Created by Yury Korolev on 13.10.2022.
//

#import <Vision/Vision.h>

NS_ASSUME_NONNULL_BEGIN

Class VN_CALCULATE_IMAGE_AESTHETICS_SCORES_REQUEST;
Class VN_GENERATE_FOREGROUND_INSTANCE_MASK_REQUEST;

__attribute__((constructor))
static void vn_initializer(void)
{
    static int initialized = 0;
    if (!initialized) {
        initialized = 1;
        
        VN_CALCULATE_IMAGE_AESTHETICS_SCORES_REQUEST = NSClassFromString(@"VNCalculateImageAestheticsScoresRequest");

        VN_GENERATE_FOREGROUND_INSTANCE_MASK_REQUEST = NSClassFromString(@"VNGenerateForegroundInstanceMaskRequest");

    }
}

NS_ASSUME_NONNULL_END

