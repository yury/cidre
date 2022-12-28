//
//  mpsg.h
//  mpsg
//
//  Created by Yury Korolev on 27.12.2022.
//

#import <Foundation/Foundation.h>
#import <MetalPerformanceShadersGraph/MetalPerformanceShadersGraph.h>
#import "../macro.h"

NS_ASSUME_NONNULL_BEGIN

#pragma mark - MPSGraphDevice

NS_RETURNS_RETAINED
csel1(, MPSGraphDevice, deviceWithMTLDevice, id, MPSGraphDevice *)

rsel0(, id, metalDevice, id<MTLDevice>)

#pragma mark - MPSGraphExecutableExecutionDescriptor

rwsel(, id, scheduledHandler, setScheduledHandler, id)
rwsel(, id, completionHandler, setCompletionHandler, id)

SEL sel_type;
SEL sel_waitUntilCompleted;
SEL sel_setWaitUntilCompleted;

__attribute__((constructor))
static void common_initializer()
{
  static int initialized = 0;
  if (!initialized) {
    sel_type = @selector(type);
    sel_waitUntilCompleted = @selector(waitUntilCompleted);
    sel_setWaitUntilCompleted = @selector(setWaitUntilCompleted:);
  }
}

NS_ASSUME_NONNULL_END
