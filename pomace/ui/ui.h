//
//  ui.h
//  ui
//
//  Created by Yury Korolev on 25.05.2022.
//

#import <Foundation/Foundation.h>
#include "../macro.h"

#import <UIKit/UIKit.h>

NS_ASSUME_NONNULL_BEGIN

#pragma mark - UIDevice

NS_RETURNS_NOT_RETAINED
csel(, UIDevice, currentDevice, UIDevice *)

// name should be in common

NS_RETURNS_NOT_RETAINED
rsel(, id, model, NSString *)

NS_RETURNS_NOT_RETAINED
rsel(, id, systemName, NSString *)

NS_RETURNS_NOT_RETAINED
rsel(, UIDevice *, systemVersion, NSString *)

rsel(, UIDevice *, orientation, UIDeviceOrientation)

NS_RETURNS_NOT_RETAINED
rsel(, id, identifierForVendor,  NSUUID * _Nullable)

rwsel(, id, isBatteryMonitoringEnabled, setBatteryMonitoringEnabled, BOOL)

rsel(, id, batteryState, UIDeviceBatteryState)
rsel(, id, batteryLevel, float)

rwsel(, id, isProximityMonitoringEnabled, setProximityMonitoringEnabled, BOOL)
rsel(, id, proximityState, BOOL)
rsel(, id, isMultitaskingSupported, BOOL)
rsel(, id, userInterfaceIdiom, UIUserInterfaceIdiom)



NS_ASSUME_NONNULL_END
