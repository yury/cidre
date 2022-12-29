//
//  mps.h
//  mps
//
//  Created by Yury Korolev on 27.12.2022.
//

#import <Foundation/Foundation.h>
#import <MetalPerformanceShaders/MetalPerformanceShaders.h>
#import "../macro.h"

NS_ASSUME_NONNULL_BEGIN

#pragma mark - MPSNDArray

#pragma mark - SELECTORS

SEL sel_readBytes_strideBytes;

__attribute__((constructor))
static void common_initializer()
{
  static int initialized = 0;
  if (!initialized) {
    sel_readBytes_strideBytes = @selector(readBytes:strideBytes:);
  }
}


NS_ASSUME_NONNULL_END
