mod texture_loader;
pub use texture_loader::TextureLoader;
#[cfg(feature = "blocks")]
pub use texture_loader::TextureLoaderArrayCb;
#[cfg(feature = "blocks")]
pub use texture_loader::TextureLoaderCb;
pub use texture_loader::TextureLoaderCubeLayout;
pub use texture_loader::TextureLoaderOpt;
pub use texture_loader::TextureLoaderOrigin;

#[link(name = "MetalKit", kind = "framework")]
unsafe extern "C" {}

#[link(name = "mtk", kind = "static")]
unsafe extern "C" {}
