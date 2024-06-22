//
//  cl.h
//  cl
//
//  Created by Yury Korolev on 1/21/24.
//

#import <CoreLocation/CoreLocation.h>

NS_ASSUME_NONNULL_BEGIN

Class CL_LOCATION;
Class CL_LOCATION_MANAGER;

__attribute__((constructor))
static void cl_initializer(void)
{
    static int initialized = 0;
    if (!initialized) {
        initialized = 1;
        
        CL_LOCATION = [CLLocation class];
        CL_LOCATION_MANAGER = [CLLocationManager class];
    }
}

NS_ASSUME_NONNULL_END
