use crate::{arc, ca, cg, define_obj_type, mtl, ns, objc};

define_obj_type!(
    #[doc(alias = "CAMetalLayer")]
    MetalLayer(ca::Layer),
    CA_METAL_LAYER
);

pub trait MetalDrawable<T: objc::Obj>: mtl::Drawable<T> {
    #[objc::msg_send(texture)]
    fn texture(&self) -> &mtl::Texture;

    #[objc::msg_send(layer)]
    fn layer(&self) -> &MetalLayer;
}

define_obj_type!(AnyMetalDrawable(ns::Id));

impl mtl::Drawable<ns::Id> for AnyMetalDrawable {}
impl MetalDrawable<ns::Id> for AnyMetalDrawable {}

impl MetalLayer {
    #[objc::msg_send(device)]
    pub fn device(&self) -> Option<&mtl::Device>;

    #[objc::msg_send(preferredDevice)]
    pub fn preferred_device(&self) -> Option<&mtl::Device>;

    #[objc::msg_send(setDevice:)]
    pub fn set_device(&mut self, val: Option<&mtl::Device>);

    #[objc::msg_send(nextDrawable)]
    pub fn next_drawable(&self) -> Option<&AnyMetalDrawable>;

    /// This property controls the pixel format of the [`mtl::Texture`] objects.
    #[objc::msg_send(pixelFormat)]
    pub fn pixel_format(&self) -> mtl::PixelFormat;

    /// The default value is MTLPixelFormatBGRA8Unorm.
    ///
    /// You must use one of the following formats:
    /// MTLPixelFormatBGRA8Unorm
    /// MTLPixelFormatBGRA8Unorm_sRGB
    /// MTLPixelFormatRGBA16Float
    /// MTLPixelFormatRGB10A2Unorm (macOS only)
    /// MTLPixelFormatBGR10A2Unorm (macOS only)
    /// MTLPixelFormatBGRA10_XR
    /// MTLPixelFormatBGRA10_XR_sRGB
    /// MTLPixelFormatBGR10_XR
    /// MTLPixelFormatBGR10_XR_sRGB
    #[objc::msg_send(setPixelFormat:)]
    pub fn set_pixel_format(&mut self, val: mtl::PixelFormat);

    #[objc::msg_send(framebufferOnly)]
    pub fn framebuffer_only(&self) -> bool;

    #[objc::msg_send(setFramebufferOnly:)]
    pub fn set_framebuffer_only(&self, val: bool);

    #[objc::msg_send(drawableSize)]
    pub fn drawable_size(&self) -> cg::Size;

    #[objc::msg_send(setDrawableSize:)]
    pub fn set_drawable_size(&mut self, val: cg::Size);

    /// The number of Metal drawables in the resource pool managed by Core Animation.
    #[objc::msg_send(maximumDrawableCount)]
    pub fn maximum_drawable_count(&self) -> usize;

    /// The number of Metal drawables in the resource pool managed by Core Animation.
    ///
    /// You can set this value to 2 or 3 only; if you pass a different value, Core Animation
    /// ignores the value and throws an exception.
    ///
    /// The default value is 3.
    #[objc::msg_send(setMaximumDrawableCount:)]
    pub fn set_maximum_drawable_count(&self, val: usize);
}

#[link(name = "ca", kind = "static")]
extern "C" {
    static CA_METAL_LAYER: &'static objc::Class<MetalLayer>;
}

#[cfg(test)]
mod tests {
    use crate::{ca, ca::MetalDrawable, cg, ns};

    #[test]
    fn basics() {
        let mut metal_layer = ca::MetalLayer::new();
        metal_layer.set_name(Some(&ns::String::with_str("mtl_layer")));
        metal_layer.set_bounds(cg::Rect::with_size(100.0, 100.0));
        let device = metal_layer.preferred_device().unwrap().retained();
        metal_layer.set_device(Some(&device));
        let drawable = metal_layer.next_drawable().unwrap();
        let texture = drawable.texture();
        println!("{drawable:?} {texture:?}");

        let animation_keys = metal_layer.animation_keys();
        assert!(animation_keys.is_none());
    }
}
