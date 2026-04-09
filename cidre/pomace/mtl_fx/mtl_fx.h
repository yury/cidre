//
//  mtl_fx.h
//  mtl_fx
//
//  Created by Yury Korolev on 4/9/26.
//

#import <MetalFX/MetalFX.h>

NS_ASSUME_NONNULL_BEGIN

Class MTLFX_SPATIAL_SCALE_DESCRIPTOR;
Class MTLFX_FRAME_INTERPOLATOR_DESCRIPTOR;
Class MTLFX_TEMPORAL_DENOISED_SCALER_DESCRIPTOR;

__attribute__((constructor))
static void mtl_fx_initializer(void)
{
    static int initialized = 0;
    if (!initialized) {
        initialized = 1;
        
        MTLFX_SPATIAL_SCALE_DESCRIPTOR = NSClassFromString(@"MTLFXSpatialScalerDescriptor");
        MTLFX_FRAME_INTERPOLATOR_DESCRIPTOR = NSClassFromString(@"MTLFXFrameInterpolatorDescriptor");
        MTLFX_TEMPORAL_DENOISED_SCALER_DESCRIPTOR = NSClassFromString(@"MTLFXTemporalDenoisedScalerDescriptor");

    }
}

NS_ASSUME_NONNULL_END
