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

    /// Returns convenient normalized texture coordinates for the part of the image that should be displayed
    ///
    /// This function automatically takes into account whether or not the texture is flipped.
    #[doc(alias = "CVMetalTextureGetCleanTexCoords")]
    #[inline]
    pub fn clean_tex_coords(
        &self,
        lower_left: &mut [f32; 2],
        lower_right: &mut [f32; 2],
        upper_right: &mut [f32; 2],
        upper_left: &mut [f32; 2],
    ) {
        unsafe {
            CVMetalTextureGetCleanTexCoords(self, lower_left, lower_right, upper_right, upper_left);
        }
    }
}

#[link(name = "CoreVideo", kind = "framework")]
unsafe extern "C-unwind" {
    fn CVMetalTextureGetTexture(image: &Texture) -> Option<&mtl::Texture>;
    fn CVMetalTextureIsFlipped(image: &Texture) -> bool;
    fn CVMetalTextureGetCleanTexCoords(
        image: &Texture,
        lower_left: &mut [f32; 2],
        lower_right: &mut [f32; 2],
        upper_right: &mut [f32; 2],
        upper_left: &mut [f32; 2],
    );
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
    unsafe extern "C" {
        static kCVMetalTextureUsage: &'static cf::String;
        static kCVMetalTextureStorageMode: &'static cf::String;
    }
}
