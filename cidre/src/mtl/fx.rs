mod temporal_scaler;
pub use temporal_scaler::FrameInterpolatableScaler;

mod frame_interpolator;
pub use frame_interpolator::FrameInterpolator;
pub use frame_interpolator::FrameInterpolatorBase;
pub use frame_interpolator::FrameInterpolatorDesc;

mod temporal_denoised_scaler;
pub use temporal_denoised_scaler::TemporalDenoisedScaler;
pub use temporal_denoised_scaler::TemporalDenoisedScalerBase;
pub use temporal_denoised_scaler::TemporalDenoisedScalerDesc;

mod spatial_scaler;
pub use spatial_scaler::SpatialScaleDesc;
pub use spatial_scaler::SpatialScaler;
pub use spatial_scaler::SpatialScalerBase;
pub use spatial_scaler::SpatialScalerColorProcessingMode;

#[link(name = "MetalFX", kind = "framework")]
unsafe extern "C" {}

#[cfg(all(feature = "mtl_fx", not(target_env = "sim")))]
unsafe extern "C" {}
