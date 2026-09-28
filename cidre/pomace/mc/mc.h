//
//  mc.h
//  mc
//
//  Created by Yury Korolev on 1/18/24.
//

#import <MultipeerConnectivity/MultipeerConnectivity.h>

NS_ASSUME_NONNULL_BEGIN

__attribute__((constructor))
static void mc_initializer(void)
{
    static int initialized = 0;
    if (!initialized) {
        initialized = 1;
        
    }
}

NS_ASSUME_NONNULL_END
