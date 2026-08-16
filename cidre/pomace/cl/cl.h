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
Class CL_BEACON_IDENTITY_CONSTRAINT;
Class CL_CONDITION;
#endif
Class CL_LOCATION;
Class CL_LOCATION_MANAGER;
Class CL_LOCATION_UPDATER;
Class CL_REGION;
Class CL_UPDATE;
#if TARGET_OS_OSX || TARGET_OS_IOS
Class CL_VISIT;
#endif

__attribute__((constructor))
static void cl_initializer(void)
{
    static int initialized = 0;
    if (!initialized) {
        initialized = 1;
        
#if TARGET_OS_OSX || TARGET_OS_IOS
        CL_BEACON_IDENTITY_CONDITION = [CLBeaconIdentityCondition class];
        CL_BEACON_IDENTITY_CONSTRAINT = [CLBeaconIdentityConstraint class];
        CL_CONDITION = [CLCondition class];
#endif
        CL_LOCATION = [CLLocation class];
        CL_LOCATION_MANAGER = [CLLocationManager class];
        CL_LOCATION_UPDATER = [CLLocationUpdater class];
        CL_REGION = [CLRegion class];
        CL_UPDATE = [CLUpdate class];
#if TARGET_OS_OSX || TARGET_OS_IOS
        CL_VISIT = [CLVisit class];
#endif
    }
}

NS_ASSUME_NONNULL_END
