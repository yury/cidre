//
//  ca.h
//  ca
//
//  Created by Yury Korolev on 22.05.2022.
//

#import <Foundation/Foundation.h>
#include "../macro.h"
#include "TargetConditionals.h"

#import <QuartzCore/QuartzCore.h>

NS_ASSUME_NONNULL_BEGIN

Class CA_DISPLAY_LINK;
Class CA_ANIMATION;
Class CA_MEDIA_TIMING_FUNCTION;
Class CA_LAYER;
Class CA_METAL_LAYER;
Class CA_RENDERER;
Class CA_TRANSACTION;

__attribute__((constructor))
static void ca_initializer(void)
{
    static int initialized = 0;
    if (!initialized) {
        CA_ANIMATION = [CAAnimation class];
        CA_MEDIA_TIMING_FUNCTION = [CAMediaTimingFunction class];
        CA_DISPLAY_LINK = [CADisplayLink class];
        CA_LAYER = [CALayer class];
        CA_METAL_LAYER = [CAMetalLayer class];
        CA_RENDERER = [CARenderer class];
        CA_TRANSACTION = [CATransaction class];
        
        initialized = 1;
    }
}


NS_ASSUME_NONNULL_END

