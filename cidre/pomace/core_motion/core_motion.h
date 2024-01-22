//
//  core_motion.h
//  core_motion
//
//  Created by Yury Korolev on 1/19/24.
//

#import <Foundation/Foundation.h>
#import <CoreMotion/CoreMotion.h>

NS_ASSUME_NONNULL_BEGIN

Class CM_MOTION_MANAGER;
Class CM_ALTIMETER;
Class CM_PEDOMETER;
Class CM_HEADPHONE_MOTION_MANAGER;

__attribute__((constructor))
static void core_motion_initializer(void)
{
    static int initialized = 0;
    if (!initialized) {
        initialized = 1;
#if TARGET_OS_IOS || TARGET_OS_WATCHOS
        CM_MOTION_MANAGER = [CMMotionManager class];
#endif
#if TARGET_OS_OSX || TARGET_OS_IOS
        CM_HEADPHONE_MOTION_MANAGER = [CMHeadphoneMotionManager class];
#endif

#if TARGET_OS_OSX
#else
        CM_ALTIMETER = [CMAltimeter class];
#endif
        CM_PEDOMETER = [CMPedometer class];
    }
}

NS_ASSUME_NONNULL_END
