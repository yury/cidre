use crate::{arc, define_cls, define_obj_type, ns, objc};

define_obj_type!(
    #[doc(alias = "CAEDRMetadata")]
    pub EdrMetadata(ns::Id)
);

impl EdrMetadata {
    define_cls!(CA_EDR_METADATA);

    #[objc::msg_send(HDR10MetadataWithDisplayInfo:contentInfo:opticalOutputScale:)]
    pub fn hdr_10_with_display_info(
        display_data: Option<&ns::Data>,
        content_data: Option<&ns::Data>,
        optical_output_scale: f32,
    ) -> arc::R<Self>;

    #[objc::msg_send(HDR10MetadataWithDisplayInfo:contentInfo:opticalOutputScale:)]
    pub fn hdr_10_with_lum(min_nits: f32, max_nits: f32, optical_output_scale: f32)
    -> arc::R<Self>;

    #[objc::msg_send(HLGMetadata)]
    pub fn hlg() -> arc::R<Self>;

    #[objc::msg_send(HLGMetadataWithAmbientViewingEnvironment:)]
    #[objc::available(macos = 14.0, ios = 17.0)]
    pub fn hlg_with_ambient_viewing_env(env: &ns::Data) -> arc::R<Self>;

    #[objc::msg_send(isAvailable)]
    pub fn is_available() -> bool;
}

#[link(name = "ca", kind = "static")]
unsafe extern "C" {
    static CA_EDR_METADATA: &'static objc::Class<EdrMetadata>;
}

#[cfg(test)]
mod tests {
    use crate::ca;

    #[test]
    fn basics() {
        let meta = ca::EdrMetadata::hdr_10_with_display_info(None, None, 1.0);
        dbg!(meta);
        let hlg = ca::EdrMetadata::hlg();
        dbg!(hlg);
    }
}
