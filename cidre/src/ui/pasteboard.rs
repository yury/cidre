use crate::{arc, define_obj_type, ns, objc, ui};

#[doc(alias = "UIPasteboardName")]
pub type PasteboardName = ns::String;

#[doc(alias = "UIPasteboardDetectionPattern")]
pub type PasteboardDetectionPattern = ns::String;

#[doc(alias = "UIPasteboardOption")]
pub type PasteboardOption = ns::String;

#[doc(alias = "UIPasteboardItem")]
pub type PasteboardItem = ns::Dictionary<ns::String, ns::Id>;

define_obj_type!(
    #[doc(alias = "UIPasteboard")]
    pub Pasteboard(ns::Id),
    UI_PASTEBOARD
);

impl Pasteboard {
    #[objc::msg_send(generalPasteboard)]
    pub fn general() -> arc::R<Self>;

    #[objc::msg_send(pasteboardWithName:create:)]
    pub fn with_name_create(name: &PasteboardName, create: bool) -> Option<arc::R<Self>>;

    #[objc::msg_send(pasteboardWithUniqueName)]
    pub fn with_unique_name() -> arc::R<Self>;

    #[objc::msg_send(removePasteboardWithName:)]
    pub fn remove_with_name(name: &PasteboardName);

    #[objc::msg_send(name)]
    pub fn name(&self) -> arc::R<PasteboardName>;

    #[objc::msg_send(changeCount)]
    pub fn change_count(&self) -> ns::Integer;

    #[objc::msg_send(numberOfItems)]
    pub fn number_of_items(&self) -> ns::Integer;

    #[objc::msg_send(pasteboardTypes)]
    pub fn pasteboard_types(&self) -> arc::R<ns::Array<ns::String>>;

    #[objc::msg_send(containsPasteboardTypes:)]
    pub fn contains_pasteboard_types(&self, pasteboard_types: &ns::Array<ns::String>) -> bool;

    #[objc::msg_send(dataForPasteboardType:)]
    pub fn data_for_pasteboard_type(
        &self,
        pasteboard_type: &ns::String,
    ) -> Option<arc::R<ns::Data>>;

    #[objc::msg_send(valueForPasteboardType:)]
    pub fn value_for_pasteboard_type(&self, pasteboard_type: &ns::String)
    -> Option<arc::R<ns::Id>>;

    #[objc::msg_send(setValue:forPasteboardType:)]
    pub fn set_value_for_pasteboard_type(&mut self, value: &ns::Id, pasteboard_type: &ns::String);

    #[objc::msg_send(setData:forPasteboardType:)]
    pub fn set_data_for_pasteboard_type(&mut self, data: &ns::Data, pasteboard_type: &ns::String);

    #[objc::msg_send(items)]
    pub fn items(&self) -> arc::R<ns::Array<PasteboardItem>>;

    #[objc::msg_send(setItems:)]
    pub fn set_items(&mut self, items: &ns::Array<PasteboardItem>);

    #[objc::msg_send(addItems:)]
    pub fn add_items(&mut self, items: &ns::Array<PasteboardItem>);

    #[objc::msg_send(setItems:options:)]
    pub fn set_items_with_opts(
        &mut self,
        items: &ns::Array<PasteboardItem>,
        options: &ns::Dictionary<PasteboardOption, ns::Id>,
    );

    #[objc::msg_send(string)]
    pub fn string(&self) -> Option<arc::R<ns::String>>;

    #[objc::msg_send(setString:)]
    pub fn set_string(&mut self, value: Option<&ns::String>);

    #[objc::msg_send(strings)]
    pub fn strings(&self) -> Option<arc::R<ns::Array<ns::String>>>;

    #[objc::msg_send(setStrings:)]
    pub fn set_strings(&mut self, value: Option<&ns::Array<ns::String>>);

    #[objc::msg_send(URL)]
    pub fn url(&self) -> Option<arc::R<ns::Url>>;

    #[objc::msg_send(setURL:)]
    pub fn set_url(&mut self, value: Option<&ns::Url>);

    #[objc::msg_send(URLs)]
    pub fn urls(&self) -> Option<arc::R<ns::Array<ns::Url>>>;

    #[objc::msg_send(setURLs:)]
    pub fn set_urls(&mut self, value: Option<&ns::Array<ns::Url>>);

    #[objc::msg_send(image)]
    pub fn image(&self) -> Option<arc::R<ui::Image>>;

    #[objc::msg_send(setImage:)]
    pub fn set_image(&mut self, value: Option<&ui::Image>);

    #[objc::msg_send(images)]
    pub fn images(&self) -> Option<arc::R<ns::Array<ui::Image>>>;

    #[objc::msg_send(setImages:)]
    pub fn set_images(&mut self, value: Option<&ns::Array<ui::Image>>);

    #[objc::msg_send(color)]
    pub fn color(&self) -> Option<arc::R<ui::Color>>;

    #[objc::msg_send(setColor:)]
    pub fn set_color(&mut self, value: Option<&ui::Color>);

    #[objc::msg_send(colors)]
    pub fn colors(&self) -> Option<arc::R<ns::Array<ui::Color>>>;

    #[objc::msg_send(setColors:)]
    pub fn set_colors(&mut self, value: Option<&ns::Array<ui::Color>>);

    #[objc::msg_send(hasStrings)]
    #[objc::available(ios = 10.0)]
    pub fn has_strings(&self) -> bool;

    #[objc::msg_send(hasURLs)]
    #[objc::available(ios = 10.0)]
    pub fn has_urls(&self) -> bool;

    #[objc::msg_send(hasImages)]
    #[objc::available(ios = 10.0)]
    pub fn has_images(&self) -> bool;

    #[objc::msg_send(hasColors)]
    #[objc::available(ios = 10.0)]
    pub fn has_colors(&self) -> bool;
}

pub mod name {
    use crate::ui::PasteboardName;

    #[doc(alias = "UIPasteboardNameGeneral")]
    #[inline]
    pub fn general() -> &'static PasteboardName {
        unsafe { UIPasteboardNameGeneral }
    }

    unsafe extern "C" {
        static UIPasteboardNameGeneral: &'static PasteboardName;
    }
}

pub mod option {
    use crate::{api, ui::PasteboardOption};

    #[doc(alias = "UIPasteboardOptionExpirationDate")]
    #[api::available(ios = 10.0)]
    #[inline]
    pub fn expiration_date() -> &'static PasteboardOption {
        unsafe { UIPasteboardOptionExpirationDate }
    }

    #[doc(alias = "UIPasteboardOptionLocalOnly")]
    #[api::available(ios = 10.0)]
    #[inline]
    pub fn local_only() -> &'static PasteboardOption {
        unsafe { UIPasteboardOptionLocalOnly }
    }

    #[api::weak]
    unsafe extern "C" {
        #[api::available(ios = 10.0)]
        static UIPasteboardOptionExpirationDate: &'static PasteboardOption;
        #[api::available(ios = 10.0)]
        static UIPasteboardOptionLocalOnly: &'static PasteboardOption;
    }
}

pub mod notifications {
    use crate::ns;

    #[doc(alias = "UIPasteboardChangedNotification")]
    #[inline]
    pub fn changed() -> &'static ns::NotificationName {
        unsafe { UIPasteboardChangedNotification }
    }

    #[doc(alias = "UIPasteboardRemovedNotification")]
    #[inline]
    pub fn removed() -> &'static ns::NotificationName {
        unsafe { UIPasteboardRemovedNotification }
    }

    #[doc(alias = "UIPasteboardChangedTypesAddedKey")]
    #[inline]
    pub fn changed_types_added_key() -> &'static ns::String {
        unsafe { UIPasteboardChangedTypesAddedKey }
    }

    #[doc(alias = "UIPasteboardChangedTypesRemovedKey")]
    #[inline]
    pub fn changed_types_removed_key() -> &'static ns::String {
        unsafe { UIPasteboardChangedTypesRemovedKey }
    }

    unsafe extern "C" {
        static UIPasteboardChangedNotification: &'static ns::NotificationName;
        static UIPasteboardRemovedNotification: &'static ns::NotificationName;
        static UIPasteboardChangedTypesAddedKey: &'static ns::String;
        static UIPasteboardChangedTypesRemovedKey: &'static ns::String;
    }
}

pub mod type_list {
    use crate::ns;

    #[doc(alias = "UIPasteboardTypeListString")]
    #[inline]
    pub fn string() -> &'static ns::Array<ns::String> {
        unsafe { UIPasteboardTypeListString }
    }

    #[doc(alias = "UIPasteboardTypeListURL")]
    #[inline]
    pub fn url() -> &'static ns::Array<ns::String> {
        unsafe { UIPasteboardTypeListURL }
    }

    #[doc(alias = "UIPasteboardTypeListImage")]
    #[inline]
    pub fn image() -> &'static ns::Array<ns::String> {
        unsafe { UIPasteboardTypeListImage }
    }

    #[doc(alias = "UIPasteboardTypeListColor")]
    #[inline]
    pub fn color() -> &'static ns::Array<ns::String> {
        unsafe { UIPasteboardTypeListColor }
    }

    unsafe extern "C" {
        static UIPasteboardTypeListString: &'static ns::Array<ns::String>;
        static UIPasteboardTypeListURL: &'static ns::Array<ns::String>;
        static UIPasteboardTypeListImage: &'static ns::Array<ns::String>;
        static UIPasteboardTypeListColor: &'static ns::Array<ns::String>;
    }
}

pub mod types {
    use crate::{api, ns};

    #[doc(alias = "UIPasteboardTypeAutomatic")]
    #[api::available(ios = 10.0)]
    #[inline]
    pub fn automatic() -> &'static ns::String {
        unsafe { UIPasteboardTypeAutomatic }
    }

    #[api::weak]
    unsafe extern "C" {
        #[api::available(ios = 10.0)]
        static UIPasteboardTypeAutomatic: &'static ns::String;
    }
}

unsafe extern "C" {
    static UI_PASTEBOARD: &'static objc::Class<Pasteboard>;
}
