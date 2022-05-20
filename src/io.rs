pub mod surface;

pub use surface::ComponentName as SurfaceComponentName;
pub use surface::ComponentRange as SurfaceComponentRange;
pub use surface::LockOptions as SurfaceLockOptions;
pub use surface::Subsampling as SurfaceSubsampling;
pub use surface::Surface;
pub use surface::SurfaceId;

#[link(name = "IOSurface", kind = "framework")]
extern "C" {}
