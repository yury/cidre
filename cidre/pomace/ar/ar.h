//
//  ar.h
//  ar
//
//  Created by Yury Korolev on 2/15/26.
//

#import <ARKit/ARKit.h>

NS_ASSUME_NONNULL_BEGIN

Class AR_SESSION;
Class AR_CONFIGURATION;
Class AR_WORLD_TRACKING_CONFIGURATION;
Class AR_ANCHOR;
Class AR_PLANE_ANCHOR;
Class AR_RAYCAST_QUERY;

__attribute__((constructor))
static void ar_initializer(void)
{
    static int initialized = 0;
    if (!initialized) {
        initialized = 1;

        AR_SESSION = NSClassFromString(@"ARSession");
        AR_CONFIGURATION = NSClassFromString(@"ARConfiguration");
        AR_WORLD_TRACKING_CONFIGURATION = NSClassFromString(@"ARWorldTrackingConfiguration");
        AR_ANCHOR = NSClassFromString(@"ARAnchor");
        AR_PLANE_ANCHOR = NSClassFromString(@"ARPlaneAnchor");
        AR_RAYCAST_QUERY = NSClassFromString(@"ARRaycastQuery");
    }
}

NS_ASSUME_NONNULL_END
