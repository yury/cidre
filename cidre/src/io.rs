pub mod surface;

pub use surface::ComponentName as SurfComponentName;
pub use surface::ComponentRange as SurfComponentRange;
pub use surface::LockOpts as SurfLockOptions;
pub use surface::Subsampling as SurfSubsampling;
pub use surface::Surf;
pub use surface::SurfId;

#[link(name = "IOSurface", kind = "framework")]
extern "C" {}
