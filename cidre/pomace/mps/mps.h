//
//  mps.h
//  mps
//
//  Created by Yury Korolev on 27.12.2022.
//

#import <Foundation/Foundation.h>
#if TARGET_OS_SIMULATOR
#else
#import <MetalPerformanceShaders/MetalPerformanceShaders.h>
#endif
#import "../macro.h"

NS_ASSUME_NONNULL_BEGIN

Class MPS_NDARRAY_DESCRIPTOR;
Class MPS_NDARRAY;

__attribute__((constructor))
static void mps_initializer(void)
{
    static int initialized = 0;
    if (!initialized) {

#if TARGET_OS_SIMULATOR
#else
        MPS_NDARRAY_DESCRIPTOR = [MPSNDArrayDescriptor class];
        MPS_NDARRAY = [MPSNDArray class];
#endif
        
        initialized = 1;
    }
}


NS_ASSUME_NONNULL_END
