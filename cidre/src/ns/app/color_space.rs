use crate::{arc, cg, define_obj_type, ns, objc};

define_obj_type!(
    #[doc(alias = "NSColorSpace")]
    pub ColorSpace(ns::Id),
    NS_COLOR_SPACE
);

unsafe impl Send for ColorSpace {}
unsafe impl Sync for ColorSpace {}

impl arc::A<ColorSpace> {
    #[objc::msg_send(initWithCGColorSpace:)]
    pub fn init_with_cg_color_space(self, val: &cg::ColorSpace) -> Option<arc::R<ColorSpace>>;
}

impl ColorSpace {
    pub fn with_cg_color_space(val: &cg::ColorSpace) -> Option<arc::R<Self>> {
        Self::alloc().init_with_cg_color_space(val)
    }

    #[objc::msg_send(CGColorSpace)]
    pub fn cg_color_space(&self) -> Option<arc::Retained<cg::ColorSpace>>;

    #[objc::msg_send(numberOfColorComponents)]
    pub fn number_of_color_components(&self) -> ns::Integer;

    #[objc::msg_send(localizedName)]
    pub fn localized_name(&self) -> Option<arc::R<ns::String>>;

    #[objc::msg_send(sRGBColorSpace)]
    pub fn srgb() -> arc::R<Self>;

    #[objc::msg_send(genericGamma22GrayColorSpace)]
    pub fn generic_gamma22_gray() -> arc::R<Self>;

    #[objc::msg_send(extendedSRGBColorSpace)]
    pub fn extended_srgb() -> arc::R<Self>;

    #[objc::msg_send(extendedGenericGamma22GrayColorSpace)]
    pub fn extended_generic_gamma22_gray(&self) -> arc::R<Self>;

    #[objc::msg_send(displayP3ColorSpace)]
    pub fn display_p3() -> arc::R<Self>;

    #[objc::msg_send(adobeRGB1998ColorSpace)]
    pub fn adobe_rgb1998() -> arc::R<Self>;

    #[objc::msg_send(genericRGBColorSpace)]
    pub fn generic_rgb() -> arc::R<Self>;

    #[objc::msg_send(genericGrayColorSpace)]
    pub fn generic_gray() -> arc::R<Self>;

    #[objc::msg_send(genericCMYKColorSpace)]
    pub fn generic_cmyk() -> arc::R<Self>;

    #[objc::msg_send(deviceRGBColorSpace)]
    pub fn device_rgb() -> arc::R<Self>;

    #[objc::msg_send(deviceGrayColorSpace)]
    pub fn device_gray() -> arc::R<Self>;

    #[objc::msg_send(deviceCMYKColorSpace)]
    pub fn device_cmyk() -> arc::R<Self>;
}

unsafe extern "C" {
    static NS_COLOR_SPACE: &'static objc::Class<ColorSpace>;
}

#[cfg(test)]
mod tests {
    use crate::{cg, ns};
    #[test]
    fn basics() {
        let cg_cspace = cg::ColorSpace::device_rgb().unwrap();
        let ns_cspace = ns::ColorSpace::with_cg_color_space(&cg_cspace).unwrap();

        let _val = ns_cspace.cg_color_space().unwrap();

        let ns_cspace = ns::ColorSpace::device_rgb();
        let _val = ns_cspace.cg_color_space().unwrap();
    }
}
