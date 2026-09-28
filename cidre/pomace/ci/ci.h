//
//  ci.h
//  ci
//
//  Created by Yury Korolev on 22.05.2022.
//

#import <CoreImage/CoreImage.h>

NS_ASSUME_NONNULL_BEGIN

__attribute__((constructor))
static void ci_initializer(void)
{
    static int initialized = 0;
    if (!initialized) {
        initialized = 1;
        
    }
}

NS_ASSUME_NONNULL_END

