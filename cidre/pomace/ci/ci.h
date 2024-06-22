//
//  ci.h
//  ci
//
//  Created by Yury Korolev on 22.05.2022.
//

#import <CoreImage/CoreImage.h>

NS_ASSUME_NONNULL_BEGIN

Class CI_IMAGE;
Class CI_CONTEXT;

__attribute__((constructor))
static void ci_initializer(void)
{
    static int initialized = 0;
    if (!initialized) {
        initialized = 1;
        
        CI_IMAGE = [CIImage class];
        CI_CONTEXT = [CIContext class];
    }
}

NS_ASSUME_NONNULL_END

