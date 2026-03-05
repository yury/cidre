use crate::{arc, define_obj_type, define_opts, ns, objc};

#[doc(alias = "NSPasteboardType")]
pub type PasteboardType = ns::String;

#[doc(alias = "NSPasteboardName")]
pub type PasteboardName = ns::String;

#[doc(alias = "NSPasteboardDetectionPattern")]
pub type PasteboardDetectionPattern = ns::String;

#[doc(alias = "NSPasteboardMetadataType")]
pub type PasteboardMetadataType = ns::String;

#[doc(alias = "NSPasteboardReadingOptionKey")]
pub type PasteboardReadingOptionKey = ns::String;

define_opts!(
    #[doc(alias = "NSPasteboardContentsOptions")]
    pub PasteboardContentsOpts(usize)
);

impl PasteboardContentsOpts {
    pub const CURRENT_HOST_ONLY: Self = Self(1 << 0);
}

#[doc(alias = "NSPasteboardAccessBehavior")]
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
#[repr(isize)]
pub enum PasteboardAccessBehavior {
    Default = 0,
    Ask = 1,
    AlwaysAllow = 2,
    AlwaysDeny = 3,
}

define_obj_type!(
    #[doc(alias = "NSPasteboard")]
    pub Pasteboard(ns::Id),
    NS_PASTEBOARD
);

impl Pasteboard {
    #[objc::msg_send(generalPasteboard)]
    pub fn general() -> arc::R<Self>;

    #[objc::msg_send(pasteboardWithName:)]
    pub fn with_name(name: &PasteboardName) -> arc::R<Self>;

    #[objc::msg_send(pasteboardWithUniqueName)]
    pub fn with_unique_name() -> arc::R<Self>;

    #[objc::msg_send(name)]
    pub fn name(&self) -> arc::R<PasteboardName>;

    #[objc::msg_send(changeCount)]
    pub fn change_count(&self) -> ns::Integer;

    #[objc::msg_send(releaseGlobally)]
    pub fn release_globally(&self);

    #[objc::msg_send(accessBehavior)]
    #[objc::available(macos = 15.4)]
    pub fn access_behavior(&self) -> PasteboardAccessBehavior;

    #[objc::msg_send(prepareForNewContentsWithOptions:)]
    #[objc::available(macos = 10.12)]
    pub fn prepare_for_new_contents_with_opts(
        &mut self,
        options: PasteboardContentsOpts,
    ) -> ns::Integer;

    #[objc::msg_send(clearContents)]
    #[objc::available(macos = 10.6)]
    pub fn clear_contents(&mut self) -> ns::Integer;

    #[objc::msg_send(declareTypes:owner:)]
    pub fn declare_types_owner(
        &mut self,
        new_types: &ns::Array<PasteboardType>,
        new_owner: Option<&ns::Id>,
    ) -> ns::Integer;

    #[objc::msg_send(addTypes:owner:)]
    pub fn add_types_owner(
        &mut self,
        new_types: &ns::Array<PasteboardType>,
        new_owner: Option<&ns::Id>,
    ) -> ns::Integer;

    #[objc::msg_send(types)]
    pub fn types(&self) -> Option<arc::R<ns::Array<PasteboardType>>>;

    #[objc::msg_send(availableTypeFromArray:)]
    pub fn available_type_from_array(
        &self,
        types: &ns::Array<PasteboardType>,
    ) -> Option<arc::R<PasteboardType>>;

    #[objc::msg_send(setData:forType:)]
    pub fn set_data_for_type(
        &mut self,
        data: Option<&ns::Data>,
        data_type: &PasteboardType,
    ) -> bool;

    #[objc::msg_send(setPropertyList:forType:)]
    pub fn set_property_list_for_type(
        &mut self,
        plist: Option<&ns::Id>,
        data_type: &PasteboardType,
    ) -> bool;

    #[objc::msg_send(setString:forType:)]
    pub fn set_string_for_type(&mut self, string: &ns::String, data_type: &PasteboardType) -> bool;

    #[objc::msg_send(dataForType:)]
    pub fn data_for_type(&self, data_type: &PasteboardType) -> Option<arc::R<ns::Data>>;

    #[objc::msg_send(propertyListForType:)]
    pub fn property_list_for_type(&self, data_type: &PasteboardType) -> Option<arc::R<ns::Id>>;

    #[objc::msg_send(stringForType:)]
    pub fn string_for_type(&self, data_type: &PasteboardType) -> Option<arc::R<ns::String>>;
}

pub mod types {
    use crate::{api, ns::PasteboardType};

    #[doc(alias = "NSPasteboardTypeString")]
    #[api::available(macos = 10.6)]
    #[inline]
    pub fn string() -> &'static PasteboardType {
        unsafe { NSPasteboardTypeString }
    }

    #[doc(alias = "NSPasteboardTypePDF")]
    #[api::available(macos = 10.6)]
    #[inline]
    pub fn pdf() -> &'static PasteboardType {
        unsafe { NSPasteboardTypePDF }
    }

    #[doc(alias = "NSPasteboardTypeTIFF")]
    #[api::available(macos = 10.6)]
    #[inline]
    pub fn tiff() -> &'static PasteboardType {
        unsafe { NSPasteboardTypeTIFF }
    }

    #[doc(alias = "NSPasteboardTypePNG")]
    #[api::available(macos = 10.6)]
    #[inline]
    pub fn png() -> &'static PasteboardType {
        unsafe { NSPasteboardTypePNG }
    }

    #[doc(alias = "NSPasteboardTypeRTF")]
    #[api::available(macos = 10.6)]
    #[inline]
    pub fn rtf() -> &'static PasteboardType {
        unsafe { NSPasteboardTypeRTF }
    }

    #[doc(alias = "NSPasteboardTypeRTFD")]
    #[api::available(macos = 10.6)]
    #[inline]
    pub fn rtfd() -> &'static PasteboardType {
        unsafe { NSPasteboardTypeRTFD }
    }

    #[doc(alias = "NSPasteboardTypeHTML")]
    #[api::available(macos = 10.6)]
    #[inline]
    pub fn html() -> &'static PasteboardType {
        unsafe { NSPasteboardTypeHTML }
    }

    #[doc(alias = "NSPasteboardTypeTabularText")]
    #[api::available(macos = 10.6)]
    #[inline]
    pub fn tabular_text() -> &'static PasteboardType {
        unsafe { NSPasteboardTypeTabularText }
    }

    #[doc(alias = "NSPasteboardTypeFont")]
    #[api::available(macos = 10.6)]
    #[inline]
    pub fn font() -> &'static PasteboardType {
        unsafe { NSPasteboardTypeFont }
    }

    #[doc(alias = "NSPasteboardTypeRuler")]
    #[api::available(macos = 10.6)]
    #[inline]
    pub fn ruler() -> &'static PasteboardType {
        unsafe { NSPasteboardTypeRuler }
    }

    #[doc(alias = "NSPasteboardTypeColor")]
    #[api::available(macos = 10.6)]
    #[inline]
    pub fn color() -> &'static PasteboardType {
        unsafe { NSPasteboardTypeColor }
    }

    #[doc(alias = "NSPasteboardTypeSound")]
    #[api::available(macos = 10.6)]
    #[inline]
    pub fn sound() -> &'static PasteboardType {
        unsafe { NSPasteboardTypeSound }
    }

    #[doc(alias = "NSPasteboardTypeMultipleTextSelection")]
    #[api::available(macos = 10.6)]
    #[inline]
    pub fn multiple_text_selection() -> &'static PasteboardType {
        unsafe { NSPasteboardTypeMultipleTextSelection }
    }

    #[doc(alias = "NSPasteboardTypeTextFinderOptions")]
    #[api::available(macos = 10.7)]
    #[inline]
    pub fn text_finder_opts() -> &'static PasteboardType {
        unsafe { NSPasteboardTypeTextFinderOptions }
    }

    #[doc(alias = "NSPasteboardTypeURL")]
    #[api::available(macos = 10.13)]
    #[inline]
    pub fn url() -> &'static PasteboardType {
        unsafe { NSPasteboardTypeURL }
    }

    #[doc(alias = "NSPasteboardTypeFileURL")]
    #[api::available(macos = 10.13)]
    #[inline]
    pub fn file_url() -> &'static PasteboardType {
        unsafe { NSPasteboardTypeFileURL }
    }

    #[api::weak]
    unsafe extern "C" {
        #[api::available(macos = 10.6)]
        static NSPasteboardTypeString: &'static PasteboardType;
        #[api::available(macos = 10.6)]
        static NSPasteboardTypePDF: &'static PasteboardType;
        #[api::available(macos = 10.6)]
        static NSPasteboardTypeTIFF: &'static PasteboardType;
        #[api::available(macos = 10.6)]
        static NSPasteboardTypePNG: &'static PasteboardType;
        #[api::available(macos = 10.6)]
        static NSPasteboardTypeRTF: &'static PasteboardType;
        #[api::available(macos = 10.6)]
        static NSPasteboardTypeRTFD: &'static PasteboardType;
        #[api::available(macos = 10.6)]
        static NSPasteboardTypeHTML: &'static PasteboardType;
        #[api::available(macos = 10.6)]
        static NSPasteboardTypeTabularText: &'static PasteboardType;
        #[api::available(macos = 10.6)]
        static NSPasteboardTypeFont: &'static PasteboardType;
        #[api::available(macos = 10.6)]
        static NSPasteboardTypeRuler: &'static PasteboardType;
        #[api::available(macos = 10.6)]
        static NSPasteboardTypeColor: &'static PasteboardType;
        #[api::available(macos = 10.6)]
        static NSPasteboardTypeSound: &'static PasteboardType;
        #[api::available(macos = 10.6)]
        static NSPasteboardTypeMultipleTextSelection: &'static PasteboardType;
        #[api::available(macos = 10.7)]
        static NSPasteboardTypeTextFinderOptions: &'static PasteboardType;
        #[api::available(macos = 10.13)]
        static NSPasteboardTypeURL: &'static PasteboardType;
        #[api::available(macos = 10.13)]
        static NSPasteboardTypeFileURL: &'static PasteboardType;
    }
}

pub mod name {
    use crate::{api, ns::PasteboardName};

    #[doc(alias = "NSPasteboardNameGeneral")]
    #[api::available(macos = 10.13)]
    #[inline]
    pub fn general() -> &'static PasteboardName {
        unsafe { NSPasteboardNameGeneral }
    }

    #[doc(alias = "NSPasteboardNameFont")]
    #[api::available(macos = 10.13)]
    #[inline]
    pub fn font() -> &'static PasteboardName {
        unsafe { NSPasteboardNameFont }
    }

    #[doc(alias = "NSPasteboardNameRuler")]
    #[api::available(macos = 10.13)]
    #[inline]
    pub fn ruler() -> &'static PasteboardName {
        unsafe { NSPasteboardNameRuler }
    }

    #[doc(alias = "NSPasteboardNameFind")]
    #[api::available(macos = 10.13)]
    #[inline]
    pub fn find() -> &'static PasteboardName {
        unsafe { NSPasteboardNameFind }
    }

    #[doc(alias = "NSPasteboardNameDrag")]
    #[api::available(macos = 10.13)]
    #[inline]
    pub fn drag() -> &'static PasteboardName {
        unsafe { NSPasteboardNameDrag }
    }

    #[api::weak]
    unsafe extern "C" {
        #[api::available(macos = 10.13)]
        static NSPasteboardNameGeneral: &'static PasteboardName;
        #[api::available(macos = 10.13)]
        static NSPasteboardNameFont: &'static PasteboardName;
        #[api::available(macos = 10.13)]
        static NSPasteboardNameRuler: &'static PasteboardName;
        #[api::available(macos = 10.13)]
        static NSPasteboardNameFind: &'static PasteboardName;
        #[api::available(macos = 10.13)]
        static NSPasteboardNameDrag: &'static PasteboardName;
    }
}

pub mod reading_option_key {
    use crate::{api, ns::PasteboardReadingOptionKey};

    #[doc(alias = "NSPasteboardURLReadingFileURLsOnlyKey")]
    #[api::available(macos = 10.6)]
    #[inline]
    pub fn url_reading_file_urls_only() -> &'static PasteboardReadingOptionKey {
        unsafe { NSPasteboardURLReadingFileURLsOnlyKey }
    }

    #[doc(alias = "NSPasteboardURLReadingContentsConformToTypesKey")]
    #[api::available(macos = 10.6)]
    #[inline]
    pub fn url_reading_contents_conform_to_types() -> &'static PasteboardReadingOptionKey {
        unsafe { NSPasteboardURLReadingContentsConformToTypesKey }
    }

    #[api::weak]
    unsafe extern "C" {
        #[api::available(macos = 10.6)]
        static NSPasteboardURLReadingFileURLsOnlyKey: &'static PasteboardReadingOptionKey;
        #[api::available(macos = 10.6)]
        static NSPasteboardURLReadingContentsConformToTypesKey: &'static PasteboardReadingOptionKey;
    }
}

unsafe extern "C" {
    static NS_PASTEBOARD: &'static objc::Class<Pasteboard>;
}
