pub mod display_link;
pub use display_link::DisplayLink;
pub use display_link::Target as DisplayLinkTarget;
pub use display_link::TargetImpl as DisplayLinkTargetImpl;

mod edr_metadata;
pub use edr_metadata::EdrMetadata;

mod frame_rate_range;
pub use frame_rate_range::FrameRateRange;

mod base;
pub use base::current_media_time;

mod animation;
pub use animation::Animation;

mod media_timing_function;
pub use media_timing_function::MediaTimingFn;
pub use media_timing_function::Name as MediaTimingFnName;

mod transform3d;
pub use transform3d::Transform3d;

mod layer;
pub use layer::Action;
pub use layer::AnyAction;
pub use layer::AnyLayerDelegate;
pub use layer::AutoresizingMask;
pub use layer::ContentsFilter as LayerContentsFilter;
pub use layer::ContentsFormat as LayerContentsFormat;
pub use layer::ContentsGravity as LayerContentsGravity;
pub use layer::CornerCurve as LayerCornerCurve;
pub use layer::CornerMask;
pub use layer::EdgeAntialiasingMask;
pub use layer::Layer;
pub use layer::LayerDelegate;
pub use layer::ToneMapMode;

mod gradient_layer;
pub use gradient_layer::GradientLayer;
pub use gradient_layer::GradientLayerType;

mod shape_layer;
pub use shape_layer::FillRule;
pub use shape_layer::LineCap;
pub use shape_layer::LineJoin;
pub use shape_layer::ShapeLayer;

mod text_layer;
pub use text_layer::AlignmentMode;
pub use text_layer::TextLayer;
pub use text_layer::TruncationMode;

#[cfg(feature = "mtl")]
mod metal_layer;
#[cfg(feature = "mtl")]
pub use metal_layer::AnyMetalDrawable;
#[cfg(feature = "mtl")]
pub use metal_layer::MetalDrawable;
#[cfg(feature = "mtl")]
pub use metal_layer::MetalLayer;

#[cfg(feature = "mtl")]
mod metal_display_link;
#[cfg(feature = "mtl")]
pub use metal_display_link::AnyMetalDisplayLinkDelegate;
#[cfg(feature = "mtl")]
pub use metal_display_link::MetalDisplayLink;
#[cfg(feature = "mtl")]
pub use metal_display_link::MetalDisplayLinkDelegate;
#[cfg(feature = "mtl")]
pub use metal_display_link::MetalDisplayLinkDelegateImpl;
#[cfg(feature = "mtl")]
pub use metal_display_link::MetalDisplayLinkUpdate;

mod renderer;
pub use renderer::OptKey as RendererOptKey;
pub use renderer::Renderer;

mod transaction;
pub use transaction::Transaction;

#[link(name = "QuartzCore", kind = "framework")]
unsafe extern "C" {}

#[link(name = "ca", kind = "static")]
unsafe extern "C" {}
