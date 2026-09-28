//
//  ar.h
//  ar
//
//  Created by Yury Korolev on 2/15/26.
//

#import <ARKit/ARKit.h>

NS_ASSUME_NONNULL_BEGIN

__attribute__((constructor))
static void ar_initializer(void)
{
    static int initialized = 0;
    if (!initialized) {
        initialized = 1;

    }
}

NS_ASSUME_NONNULL_END
