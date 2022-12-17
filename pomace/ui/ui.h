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
csel0(, UIDevice, currentDevice, UIDevice *)

// name should be in common

NS_RETURNS_NOT_RETAINED
rsel0(, id, model, NSString *)

NS_RETURNS_NOT_RETAINED
rsel0(UIDevice_, id, systemName, NSString *)

NS_RETURNS_NOT_RETAINED
rsel0(UIDevice_, UIDevice *, systemVersion, NSString *)

rsel0(, UIDevice *, orientation, UIDeviceOrientation)

NS_RETURNS_NOT_RETAINED
rsel0(, id, identifierForVendor,  NSUUID * _Nullable)

rwsel(, id, isBatteryMonitoringEnabled, setBatteryMonitoringEnabled, BOOL)

rsel0(, id, batteryState, UIDeviceBatteryState)
rsel0(, id, batteryLevel, float)

rwsel(, id, isProximityMonitoringEnabled, setProximityMonitoringEnabled, BOOL)
rsel0(, id, proximityState, BOOL)
rsel0(, id, isMultitaskingSupported, BOOL)
rsel0(, id, userInterfaceIdiom, UIUserInterfaceIdiom)



NS_ASSUME_NONNULL_END
