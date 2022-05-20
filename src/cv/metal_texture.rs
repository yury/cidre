use crate::{cv, mtl};

pub type MetalTexture = cv::ImageBuffer;

impl MetalTexture {
    #[inline]
    pub fn texture(&self) -> Option<&mtl::Texture> {
        unsafe { CVMetalTextureGetTexture(self) }
    }

    #[inline]
    pub fn is_texture_flipped(&self) -> bool {
        unsafe { CVMetalTextureIsFlipped(self) }
    }
}

extern "C" {
    fn CVMetalTextureGetTexture(image: &MetalTexture) -> Option<&mtl::Texture>;
    fn CVMetalTextureIsFlipped(image: &MetalTexture) -> bool;
}
