use crate::{arc, define_obj_type, ns, objc, ui};

define_obj_type!(
    #[doc(alias = "UIDocumentPickerViewController")]
    pub DocumentPickerViewController(ui::ViewController),
    UI_DOCUMENT_PICKER_VIEW_CONTROLLER
);

impl DocumentPickerViewController {
    /// A picker for opening documents of the given content types. With `as_copy` the picked
    /// documents are copied into the app's temporary directory (no security-scoped access is
    /// needed for the returned URLs).
    #[cfg(feature = "ut")]
    #[objc::init(initForOpeningContentTypes:asCopy:)]
    pub fn init_for_opening_content_types_as_copy(
        self,
        content_types: &ns::Array<crate::ut::Type>,
        as_copy: bool,
    ) -> arc::R<DocumentPickerViewController>;

    #[cfg(feature = "ut")]
    pub fn for_opening_content_types(
        content_types: &ns::Array<crate::ut::Type>,
        as_copy: bool,
    ) -> arc::R<Self> {
        Self::alloc().init_for_opening_content_types_as_copy(content_types, as_copy)
    }

    #[objc::msg_send(delegate)]
    pub fn delegate(&self) -> Option<arc::R<AnyDocumentPickerDelegate>>;

    #[objc::msg_send(setDelegate:)]
    pub fn set_delegate<D: DocumentPickerDelegate>(&mut self, val: Option<&D>);

    #[objc::msg_send(allowsMultipleSelection)]
    pub fn allows_multiple_selection(&self) -> bool;

    #[objc::msg_send(setAllowsMultipleSelection:)]
    pub fn set_allows_multiple_selection(&mut self, val: bool);

    #[objc::msg_send(shouldShowFileExtensions)]
    pub fn should_show_file_extensions(&self) -> bool;

    #[objc::msg_send(setShouldShowFileExtensions:)]
    pub fn set_should_show_file_extensions(&mut self, val: bool);

    #[objc::msg_send(directoryURL)]
    pub fn directory_url(&self) -> Option<arc::R<ns::Url>>;

    #[objc::msg_send(setDirectoryURL:)]
    pub fn set_directory_url(&mut self, val: Option<&ns::Url>);
}

#[objc::protocol(UIDocumentPickerDelegate)]
pub trait DocumentPickerDelegate: objc::Obj {
    #[objc::optional]
    #[objc::msg_send(documentPicker:didPickDocumentsAtURLs:)]
    fn document_picker_did_pick_documents_at_urls(
        &mut self,
        picker: &mut DocumentPickerViewController,
        urls: &ns::Array<ns::Url>,
    );

    #[objc::optional]
    #[objc::msg_send(documentPickerWasCancelled:)]
    fn document_picker_was_cancelled(&mut self, picker: &mut DocumentPickerViewController);
}

define_obj_type!(
    pub AnyDocumentPickerDelegate(ns::Id)
);

impl DocumentPickerDelegate for AnyDocumentPickerDelegate {}

unsafe extern "C" {
    static UI_DOCUMENT_PICKER_VIEW_CONTROLLER: &'static objc::Class<DocumentPickerViewController>;
}
