pub mod surface;

pub use surface::ComponentName as SurfaceComponentName;
pub use surface::ComponentRange as SurfaceComponentRange;
pub use surface::Id as SurfaceId;
pub use surface::LockOptions as SurfaceLockOptions;
pub use surface::Subsampling as SurfaceSubsampling;
pub use surface::Surface;

#[link(name = "IOSurface", kind = "framework")]
extern "C" {}
