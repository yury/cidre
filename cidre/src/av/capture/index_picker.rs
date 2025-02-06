use crate::{api, arc, av, define_obj_type, ns, objc};

#[cfg(all(feature = "blocks", feature = "dispatch"))]
use crate::{blocks, dispatch};

define_obj_type!(
    #[doc(alias = "AVCaptureIndexPicker")]
    pub IndexPicker(av::CaptureControl)
);

impl arc::A<IndexPicker> {
    #[objc::msg_send(initWithLocalizedTitle:symbolName:numberOfIndexes:)]
    pub unsafe fn init_with_indexes_n_throws(
        self,
        localized_title: &ns::String,
        symbol_name: &ns::String,
        n: isize,
    ) -> arc::R<IndexPicker>;

    pub fn init_with_indexes_n<'ear>(
        self,
        localized_title: &ns::String,
        symbol_name: &ns::String,
        n: isize,
    ) -> ns::ExResult<'ear, arc::R<IndexPicker>> {
        ns::try_catch(|| unsafe {
            self.init_with_indexes_n_throws(localized_title, symbol_name, n)
        })
    }

    #[objc::msg_send(initWithLocalizedTitle:symbolName:localizedIndexTitles:)]
    pub unsafe fn init_with_indexes_titles_throws(
        self,
        localized_title: &ns::String,
        symbol_name: &ns::String,
        titles: &ns::Array<ns::String>,
    ) -> arc::R<IndexPicker>;

    pub fn init_with_titles<'ear>(
        self,
        localized_title: &ns::String,
        symbol_name: &ns::String,
        titles: &ns::Array<ns::String>,
    ) -> ns::ExResult<'ear, arc::R<IndexPicker>> {
        ns::try_catch(|| unsafe {
            self.init_with_indexes_titles_throws(localized_title, symbol_name, titles)
        })
    }
}

impl IndexPicker {
    #[api::available(macos = 15.0, ios = 18.0, maccatalyst = 18.0, tvos = 18.0)]
    crate::define_cls!(AV_CAPTURE_INDEX_PICKER);

    #[api::available(macos = 15.0, ios = 18.0, maccatalyst = 18.0, tvos = 18.0)]
    pub fn with_indexes_n<'ear>(
        localized_title: &ns::String,
        symbol_name: &ns::String,
        n: isize,
    ) -> ns::ExResult<'ear, arc::R<IndexPicker>> {
        Self::alloc().init_with_indexes_n(localized_title, symbol_name, n)
    }

    #[api::available(macos = 15.0, ios = 18.0, maccatalyst = 18.0, tvos = 18.0)]
    pub fn with_titles<'ear>(
        localized_title: &ns::String,
        symbol_name: &ns::String,
        titles: &ns::Array<ns::String>,
    ) -> ns::ExResult<'ear, arc::R<IndexPicker>> {
        Self::alloc().init_with_titles(localized_title, symbol_name, titles)
    }

    #[objc::msg_send(selectedIndex)]
    #[api::available(macos = 15.0, ios = 18.0, maccatalyst = 18.0, tvos = 18.0)]
    pub fn selected_index(&self) -> isize;

    #[objc::msg_send(setSelectedIndex:)]
    #[api::available(macos = 15.0, ios = 18.0, maccatalyst = 18.0, tvos = 18.0)]
    pub unsafe fn set_selected_index_throws(&mut self, val: isize);

    #[api::available(macos = 15.0, ios = 18.0, maccatalyst = 18.0, tvos = 18.0)]
    pub fn set_selected_index<'ear>(&mut self, val: isize) -> ns::ExResult<'ear> {
        ns::try_catch(|| unsafe { self.set_selected_index_throws(val) })
    }

    #[objc::msg_send(localizedTitle)]
    #[api::available(macos = 15.0, ios = 18.0, maccatalyst = 18.0, tvos = 18.0)]
    pub fn localized_title(&self) -> arc::R<ns::String>;

    #[objc::msg_send(symbolName)]
    #[api::available(macos = 15.0, ios = 18.0, maccatalyst = 18.0, tvos = 18.0)]
    pub fn symbol_name(&self) -> arc::R<ns::String>;

    #[objc::msg_send(numberOfIndexes)]
    #[api::available(macos = 15.0, ios = 18.0, maccatalyst = 18.0, tvos = 18.0)]
    pub fn indexes_n(&self) -> isize;

    #[objc::msg_send(localizedIndexTitles)]
    #[api::available(macos = 15.0, ios = 18.0, maccatalyst = 18.0, tvos = 18.0)]
    pub fn localized_index_titles(&self) -> arc::R<ns::Array<ns::String>>;

    #[objc::msg_send(accessibilityIdentifier)]
    #[api::available(macos = 15.0, ios = 18.0, maccatalyst = 18.0, tvos = 18.0)]
    pub fn accessibility_id(&self) -> arc::R<ns::String>;

    #[objc::msg_send(setAccessibilityIdentifier:)]
    #[api::available(macos = 15.0, ios = 18.0, maccatalyst = 18.0, tvos = 18.0)]
    pub fn set_accessibility_id(&mut self, val: Option<&ns::String>);

    #[cfg(all(feature = "blocks", feature = "dispatch"))]
    #[objc::msg_send(setActionQueue:action:)]
    pub fn set_queue_action(
        &mut self,
        queue: &dispatch::Queue,
        action: &mut blocks::EscBlock<fn(new_value: isize)>,
    );
}

#[link(name = "av", kind = "static")]
extern "C" {
    static AV_CAPTURE_INDEX_PICKER: &'static objc::Class<IndexPicker>;
}
