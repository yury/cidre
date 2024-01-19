//
//  core_motion.h
//  core_motion
//
//  Created by Yury Korolev on 1/19/24.
//

#import <Foundation/Foundation.h>
#import <CoreMotion/CoreMotion.h>

NS_ASSUME_NONNULL_BEGIN

Class CM_PEDOMETER;

__attribute__((constructor))
static void core_motion_initializer(void)
{
    static int initialized = 0;
    if (!initialized) {
        initialized = 1;

        CM_PEDOMETER = [CMPedometer class];
    }
}

NS_ASSUME_NONNULL_END
