use crate::{arc, ca, cg, define_obj_type, mtl, ns, objc};

define_obj_type!(
    #[doc(alias = "CAMetalLayer")]
    pub MetalLayer(ca::Layer),
    CA_METAL_LAYER
);

pub trait MetalDrawable<T: objc::Obj>: mtl::Drawable<T> {
    #[objc::msg_send(texture)]
    fn texture(&self) -> arc::R<mtl::Texture>;

    #[objc::msg_send(layer)]
    fn layer(&self) -> arc::R<MetalLayer>;
}

define_obj_type!(pub AnyMetalDrawable(ns::Id));

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
    pub fn next_drawable(&self) -> Option<arc::R<AnyMetalDrawable>>;

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

    #[objc::msg_send(presentsWithTransaction)]
    pub fn presents_with_transaction(&self) -> bool;

    #[objc::msg_send(setPresentsWithTransaction:)]
    pub fn set_presents_with_transaction(&mut self, val: bool);

    #[objc::msg_send(colorspace)]
    pub fn colorspace(&self) -> Option<&cg::ColorSpace>;

    #[objc::msg_send(setColorspace:)]
    pub fn set_colorspace(&mut self, val: Option<&cg::ColorSpace>);

    #[objc::msg_send(wantsExtendedDynamicRangeContent)]
    #[objc::available(macos = 10.11, ios = 16.0, maccatalyst = 16.0)]
    pub fn wants_extended_dynamic_range_content(&self) -> bool;

    #[objc::msg_send(setWantsExtendedDynamicRangeContent:)]
    #[objc::available(macos = 10.11, ios = 16.0, maccatalyst = 16.0)]
    pub fn set_wants_extended_dynamic_range_content(&mut self, val: bool);

    #[objc::msg_send(EDRMetadata)]
    #[objc::available(macos = 10.15, ios = 16.0)]
    pub fn edr_metadata(&self) -> Option<arc::R<ca::EdrMetadata>>;

    #[objc::msg_send(setEDRMetadata:)]
    #[objc::available(macos = 10.15, ios = 16.0)]
    pub fn set_edr_metadata(&mut self, val: Option<&ca::EdrMetadata>);

    #[objc::msg_send(displaySyncEnabled)]
    #[objc::available(macos = 10.13)]
    pub fn display_sync_enabled(&self) -> bool;

    #[objc::msg_send(setDisplaySyncEnabled:)]
    #[objc::available(macos = 10.13)]
    pub fn set_display_sync_enabled(&mut self, val: bool);

    #[objc::msg_send(allowsNextDrawableTimeout)]
    #[objc::available(macos = 10.13, ios = 11.0, tvos = 11.0)]
    pub fn allows_next_drawable_timeout(&self) -> bool;

    #[objc::msg_send(setAllowsNextDrawableTimeout:)]
    #[objc::available(macos = 10.13, ios = 11.0, tvos = 11.0)]
    pub fn set_allows_next_drawable_timeout(&mut self, val: bool);

    #[objc::msg_send(developerHUDProperties)]
    #[objc::available(macos = 13.0, ios = 16.0, tvos = 16.0)]
    pub fn developer_hud_props(&self) -> Option<arc::R<ns::Dictionary<ns::String, ns::Id>>>;

    #[objc::msg_send(setDeveloperHUDProperties:)]
    #[objc::available(macos = 13.0, ios = 16.0, tvos = 16.0)]
    pub fn set_developer_hud_props(&mut self, val: Option<&ns::Dictionary<ns::String, ns::Id>>);

    #[objc::msg_send(residencySet)]
    #[objc::available(macos = 26.0, ios = 26.0, tvos = 26.0, visionos = 26.0)]
    pub fn residency_set(&self) -> arc::R<mtl::ResidencySet>;
}

#[link(name = "ca", kind = "static")]
unsafe extern "C" {
    static CA_METAL_LAYER: &'static objc::Class<MetalLayer>;
}

#[cfg(test)]
mod tests {
    use crate::{
        ca::{self, MetalDrawable},
        cg, ns,
    };

    #[test]
    fn basics() {
        let mut metal_layer = ca::MetalLayer::new();
        metal_layer.set_name(Some(ns::str!(c"mtl_layer")));
        metal_layer.set_bounds(cg::Rect::with_size(100.0, 100.0));
        let device = metal_layer.preferred_device().unwrap().retained();
        metal_layer.set_device(Some(&device));
        let drawable = metal_layer.next_drawable().unwrap();
        let texture = drawable.texture();
        println!("{drawable:?} {texture:?}");
        println!("colorspace: {:?}", metal_layer.colorspace());
        assert_eq!(3, metal_layer.maximum_drawable_count());
        assert_eq!(false, metal_layer.wants_extended_dynamic_range_content());
        assert_eq!(true, metal_layer.framebuffer_only());
        assert_eq!(cg::Size::new(100.0, 100.0), metal_layer.drawable_size());

        let animation_keys = metal_layer.animation_keys();
        assert!(animation_keys.is_none());
    }
}
