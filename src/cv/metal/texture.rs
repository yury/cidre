use crate::{cv, mtl};

pub type Texture = cv::ImageBuffer;

impl Texture {
    #[inline]
    pub fn texture(&self) -> Option<&mtl::Texture> {
        unsafe { CVMetalTextureGetTexture(self) }
    }

    #[inline]
    pub fn is_texture_flipped(&self) -> bool {
        unsafe { CVMetalTextureIsFlipped(self) }
    }
}

#[link(name = "CoreVideo", kind = "framework")]
extern "C" {
    fn CVMetalTextureGetTexture(image: &Texture) -> Option<&mtl::Texture>;
    fn CVMetalTextureIsFlipped(image: &Texture) -> bool;
}


pub mod keys {
    use crate::cf;

    #[inline]
    pub fn usage() -> &'static cf::String {
        unsafe { kCVMetalTextureUsage }
    }

    #[inline]
    pub fn storage_mode() -> &'static cf::String {
        unsafe { kCVMetalTextureStorageMode }
    }

    #[link(name = "CoreVideo", kind = "framework")]
    extern "C" {
        static kCVMetalTextureUsage: &'static cf::String;
        static kCVMetalTextureStorageMode: &'static cf::String;
    }
}