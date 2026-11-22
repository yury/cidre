use crate::{arc, ns, objc, ut};

impl ns::ItemProvider {
    /// Offers a file representation that the recipient can copy or open in place.
    #[objc::init(initWithContentsOfURL:contentType:openInPlace:coordinated:visibility:)]
    #[objc::available(macos = 13.0, ios = 16.0, watchos = 9.0, tvos = 16.0)]
    pub fn init_with_file_url(
        self,
        url: &ns::Url,
        content_type: Option<&ut::Type>,
        open_in_place: bool,
        coordinated: bool,
        visibility: ns::ItemProviderRepresentationVisibility,
    ) -> arc::R<ns::ItemProvider>;
}
