#[cfg(not(target_os = "watchos"))]
use crate::{FourCharCode, arc, cf, cm};

#[cfg(not(target_os = "watchos"))]
impl cm::MediaType {
    pub fn localized_name(&self) -> arc::R<cf::String> {
        unsafe { MTCopyLocalizedNameForMediaType(*self) }
    }
    pub fn sub_type_localized_name(&self, fcc: FourCharCode) -> arc::R<cf::String> {
        unsafe { MTCopyLocalizedNameForMediaSubType(*self, fcc) }
    }
}

#[cfg(not(target_os = "watchos"))]
#[link(name = "MediaToolbox", kind = "framework")]
unsafe extern "C-unwind" {
    fn MTCopyLocalizedNameForMediaType(media_type: cm::MediaType) -> arc::R<cf::String>;
    fn MTCopyLocalizedNameForMediaSubType(
        media_type: cm::MediaType,
        sub_type: FourCharCode,
    ) -> arc::R<cf::String>;
}

#[cfg(test)]
mod tests {
    use crate::cm;

    #[test]
    fn basics() {
        let name = cm::MediaType(1).localized_name();
        assert_eq!(name.to_string(), "");
        let name = cm::MediaType::AUDIO.sub_type_localized_name(u32::from_be_bytes(*b"aac "));
        assert_eq!(name.to_string(), "MPEG-4 AAC");
    }
}
