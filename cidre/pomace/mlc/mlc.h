//
//  mlc.h
//  mlc
//
//  Created by Yury Korolev on 27.02.2022.
//

#import <Foundation/Foundation.h>

// MLC is not supported on simulators
#if TARGET_OS_SIMULATOR
#else
#import <MLCompute/MLCompute.h>
#endif

NS_ASSUME_NONNULL_BEGIN

//Class MLC_LAYER;

__attribute__((constructor))
static void mlc_initializer(void)
{
    static int initialized = 0;
    if (!initialized) {
        initialized = 1;
        
#if TARGET_OS_SIMULATOR
#else
        //      MLC_LAYER = [MLCLayer class];
#endif
    }
}

NS_ASSUME_NONNULL_END
