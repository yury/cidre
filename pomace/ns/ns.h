//
//  ns.h
//  ns
//
//  Created by Yury Korolev on 07.07.2022.
//

#import <Foundation/Foundation.h>
#include "../macro.h"

NS_ASSUME_NONNULL_BEGIN

#pragma mark - Common

wsel_ab(, id, scheduleInRunLoop, NSRunLoop *, forMode, NSRunLoopMode)
wsel_ab(, id, removeFromRunLoop, NSRunLoop *, forMode, NSRunLoopMode)

#pragma mark - NSPort

csel(, NSPort, port, NSPort *)
rsel(, id, machPort, uint32_t)

@interface CidreMachPortDelegate : NSObject<NSMachPortDelegate> {
  @public void * _vtable[2];
}
@end

NS_RETURNS_RETAINED
CidreMachPortDelegate * make_mach_port_delegate(void * _Nonnull vtable[_Nonnull 2]) {
  CidreMachPortDelegate * result = [CidreMachPortDelegate new];
  memcpy(result->_vtable, vtable, 2 * sizeof(void *));
  return result;
}

#pragma mark - NSProcessInfo

NS_RETURNS_NOT_RETAINED
csel(, NSProcessInfo, processInfo, NSProcessInfo *)

rsel(, id, thermalState, NSProcessInfoThermalState)
rsel(, id, isLowPowerModeEnabled, BOOL)
rsel(, id, processorCount, NSUInteger)
rsel(, id, activeProcessorCount, NSUInteger)

rsel(, id, isMacCatalystApp, BOOL)
rsel(, id, isiOSAppOnMac, BOOL)

void cidre_raise_exception(NSString *message) {
  [NSException raise:NSGenericException format:@"%@", message];
}

void cidre_throw_exception(NSString *message) {
  @throw message;
}

id _Nullable cidre_try_catch(void (*during)(void *), void * context ) {
  @try {
    during(context);
    return nil;
  } @catch (id e) {
    return e;
  }
}

#pragma mark - NSURLSession

NS_RETURNS_NOT_RETAINED
csel(, NSURLSession, sharedSession, NSURLSession *)

rsel_a(, id, dataTaskWithURL, NSURL *, NSURLSessionDataTask *)

#pragma mark - NSURLSessionTask

wsel(NSURLSessionTask_, NSURLSessionTask *, resume)
wsel(NSURLSessionTask_, NSURLSessionTask *, cancel)
wsel(NSURLSessionTask_, NSURLSessionTask *, suspend)
rsel(NSURLSessionTask_, NSURLSessionTask *, state, NSURLSessionTaskState)
rsel(NSURLSessionTask_, NSURLSessionTask *, error, NSError *)

#pragma mark - NSURLRequest

NS_RETURNS_RETAINED
csel_a(, NSURLRequest, requestWithURL, NSURL *, NSURLRequest *)

NS_RETURNS_RETAINED
csel_abc(, NSURLRequest, requestWithURL, NSURL *, cachePolicy, NSURLRequestCachePolicy, timeoutInterval, NSTimeInterval, NSURLRequest *)


NS_ASSUME_NONNULL_END
