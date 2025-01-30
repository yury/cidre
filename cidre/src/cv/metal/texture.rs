use crate::{cv, mtl};

#[doc(alias = "CVMetalTextureRef")]
pub type Texture = cv::ImageBuf;

impl Texture {
    #[doc(alias = "CVMetalTextureGetTexture")]
    #[inline]
    pub fn texture(&self) -> Option<&mtl::Texture> {
        unsafe { CVMetalTextureGetTexture(self) }
    }

    #[doc(alias = "CVMetalTextureIsFlipped")]
    #[inline]
    pub fn is_texture_flipped(&self) -> bool {
        unsafe { CVMetalTextureIsFlipped(self) }
    }
}

#[link(name = "CoreVideo", kind = "framework")]
extern "C-unwind" {
    fn CVMetalTextureGetTexture(image: &Texture) -> Option<&mtl::Texture>;
    fn CVMetalTextureIsFlipped(image: &Texture) -> bool;
}

pub mod keys {
    use crate::cf;

    #[doc(alias = "kCVMetalTextureUsage")]
    #[inline]
    pub fn usage() -> &'static cf::String {
        unsafe { kCVMetalTextureUsage }
    }

    #[doc(alias = "kCVMetalTextureStorageMode")]
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
