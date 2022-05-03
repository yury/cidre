//
//  av.h
//  av
//
//  Created by Yury Korolev on 02.05.2022.
//

#import <Foundation/Foundation.h>
#import <AVFoundation/AVFoundation.h>

#include "../macro.h"

NS_ASSUME_NONNULL_BEGIN

#pragma mark - AVCaptureSystemPressureState

#if TARGET_OS_IPHONE
//@property(atomic, readonly) AVCaptureSystemPressureLevel level;
rsel(, AVCaptureSystemPressureState *, level, AVCaptureSystemPressureLevel)
rsel(, AVCaptureSystemPressureState *, factors, AVCaptureSystemPressureFactors)

#endif

#pragma mark - AVCaptureDevice

NS_RETURNS_RETAINED
csel_abc(, AVCaptureDevice, defaultDeviceWithDeviceType, AVCaptureDeviceType, mediaType, AVMediaType _Nullable, position, AVCaptureDevicePosition, AVCaptureDevice * _Nullable)
// + (nullable AVCaptureDevice *)defaultDeviceWithDeviceType:(AVCaptureDeviceType)deviceType mediaType:(nullable AVMediaType)mediaType position:(AVCaptureDevicePosition)position API_AVAILABLE(macos(10.15), ios(10.0), macCatalyst(14.0)) API_UNAVAILABLE(tvos);

NS_ASSUME_NONNULL_END
