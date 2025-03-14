//
//  ca.h
//  ca
//
//  Created by Yury Korolev on 22.05.2022.
//

#import <QuartzCore/QuartzCore.h>

NS_ASSUME_NONNULL_BEGIN

Class CA_DISPLAY_LINK;
Class CA_ANIMATION;
Class CA_MEDIA_TIMING_FUNCTION;
Class CA_LAYER;
Class CA_METAL_LAYER;
Class CA_RENDERER;
Class CA_TRANSACTION;
Class CA_EDR_METADATA;

__attribute__((constructor))
static void ca_initializer(void)
{
    static int initialized = 0;
    if (!initialized) {
        initialized = 1;
        
        CA_ANIMATION = [CAAnimation class];
        CA_MEDIA_TIMING_FUNCTION = [CAMediaTimingFunction class];
        CA_DISPLAY_LINK = NSClassFromString(@"CADisplayLink");
        CA_LAYER = [CALayer class];
        CA_METAL_LAYER = [CAMetalLayer class];
        CA_RENDERER = [CARenderer class];
        CA_TRANSACTION = [CATransaction class];
        CA_EDR_METADATA = NSClassFromString(@"CAEDRMetadata");
    }
}


NS_ASSUME_NONNULL_END

