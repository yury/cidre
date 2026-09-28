//
//  ca.h
//  ca
//
//  Created by Yury Korolev on 22.05.2022.
//

#import <QuartzCore/QuartzCore.h>

NS_ASSUME_NONNULL_BEGIN

Class CA_DISPLAY_LINK;
Class CA_METAL_DISPLAY_LINK;
Class CA_EDR_METADATA;

__attribute__((constructor))
static void ca_initializer(void)
{
    static int initialized = 0;
    if (!initialized) {
        initialized = 1;
        
        CA_DISPLAY_LINK = NSClassFromString(@"CADisplayLink");
        CA_METAL_DISPLAY_LINK = NSClassFromString(@"CAMetalDisplayLink");
        CA_EDR_METADATA = NSClassFromString(@"CAEDRMetadata");
    }
}

NS_ASSUME_NONNULL_END

