//
//  cl.h
//  cl
//
//  Created by Yury Korolev on 1/21/24.
//

#import <CoreLocation/CoreLocation.h>

NS_ASSUME_NONNULL_BEGIN

#if TARGET_OS_OSX || TARGET_OS_IOS
Class CL_BEACON_IDENTITY_CONDITION;
Class CL_CONDITION;
#endif
Class CL_LOCATION_UPDATER;
Class CL_UPDATE;

__attribute__((constructor))
static void cl_initializer(void)
{
    static int initialized = 0;
    if (!initialized) {
        initialized = 1;
        
#if TARGET_OS_OSX || TARGET_OS_IOS
        CL_BEACON_IDENTITY_CONDITION = [CLBeaconIdentityCondition class];
        CL_CONDITION = [CLCondition class];
#endif
        CL_LOCATION_UPDATER = [CLLocationUpdater class];
        CL_UPDATE = [CLUpdate class];
    }
}

NS_ASSUME_NONNULL_END
