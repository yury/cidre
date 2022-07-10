//
//  ns.h
//  ns
//
//  Created by Yury Korolev on 07.07.2022.
//

#import <Foundation/Foundation.h>
#include "../macro.h"

NS_ASSUME_NONNULL_BEGIN

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


BOOL cidre_try_catch(
  void (*during)(void *),
  void * context,
// CF_RETURNS_RETAINED
  id _Nullable  * _Nonnull exception

) {
  @try {
    during(context);
    *exception = nil;
    return YES;
  } @catch (id e) {
    *exception = e;
    return NO;
  }
}


NS_ASSUME_NONNULL_END
