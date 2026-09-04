use crate::{arc, define_obj_type, ns, objc};

define_obj_type!(
    #[doc(alias = "NSSavePanel")]
    pub SavePanel(ns::Panel),
    NS_SAVE_PANEL
);

impl SavePanel {
    /// A new save panel.
    #[objc::msg_send(savePanel)]
    pub fn save_panel() -> arc::R<Self>;

    /// The content types the panel lets the user save as (empty: any).
    #[cfg(feature = "ut")]
    #[objc::msg_send(allowedContentTypes)]
    pub fn allowed_content_types(&self) -> arc::R<ns::Array<crate::ut::Type>>;

    #[cfg(feature = "ut")]
    #[objc::msg_send(setAllowedContentTypes:)]
    pub fn set_allowed_content_types(&mut self, val: &ns::Array<crate::ut::Type>);

    #[objc::msg_send(allowsOtherFileTypes)]
    pub fn allows_other_file_types(&self) -> bool;

    #[objc::msg_send(setAllowsOtherFileTypes:)]
    pub fn set_allows_other_file_types(&mut self, val: bool);

    #[objc::msg_send(canCreateDirectories)]
    pub fn can_create_directories(&self) -> bool;

    #[objc::msg_send(setCanCreateDirectories:)]
    pub fn set_can_create_directories(&mut self, val: bool);

    #[objc::msg_send(showsHiddenFiles)]
    pub fn shows_hidden_files(&self) -> bool;

    #[objc::msg_send(setShowsHiddenFiles:)]
    pub fn set_shows_hidden_files(&mut self, val: bool);

    /// The title of the panel.
    #[objc::msg_send(title)]
    pub fn title(&self) -> Option<arc::R<ns::String>>;

    #[objc::msg_send(setTitle:)]
    pub fn set_title(&mut self, val: Option<&ns::String>);

    /// The prompt on the default button ("Open" / "Save").
    #[objc::msg_send(prompt)]
    pub fn prompt(&self) -> Option<arc::R<ns::String>>;

    #[objc::msg_send(setPrompt:)]
    pub fn set_prompt(&mut self, val: Option<&ns::String>);

    /// The message shown above the file browser.
    #[objc::msg_send(message)]
    pub fn message(&self) -> Option<arc::R<ns::String>>;

    #[objc::msg_send(setMessage:)]
    pub fn set_message(&mut self, val: Option<&ns::String>);

    #[objc::msg_send(nameFieldStringValue)]
    pub fn name_field_string_value(&self) -> arc::R<ns::String>;

    #[objc::msg_send(setNameFieldStringValue:)]
    pub fn set_name_field_string_value(&mut self, val: &ns::String);

    #[objc::msg_send(directoryURL)]
    pub fn directory_url(&self) -> Option<arc::R<ns::Url>>;

    #[objc::msg_send(setDirectoryURL:)]
    pub fn set_directory_url(&mut self, val: Option<&ns::Url>);

    /// The chosen file (valid after `run_modal` returned `ns::ModalResponse::OK`).
    #[objc::msg_send(URL)]
    pub fn url(&self) -> Option<arc::R<ns::Url>>;

    /// Runs the panel as an app-modal dialog and returns the user's choice.
    #[objc::msg_send(runModal)]
    pub fn run_modal(&mut self) -> ns::ModalResponse;
}

unsafe extern "C" {
    static NS_SAVE_PANEL: &'static objc::Class<SavePanel>;
}
