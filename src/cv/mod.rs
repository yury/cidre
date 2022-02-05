pub mod buffer;

pub use buffer::AttachmentMode;
pub use buffer::Buffer;

pub mod image_buffer;
pub use image_buffer::ImageBuffer;

// pub mod surface;

// pub use surface::Surface;
// pub use surface::SurfaceComponentName;
// pub use surface::SurfaceComponentRange;
// pub use surface::SurfaceId;
// pub use surface::SurfaceLockOptions;
// pub use surface::SurfaceSubsampling;

// pub use surface::keys as surface_keys;

#[link(name = "CoreVideo", kind = "framework")]
extern "C" {}
