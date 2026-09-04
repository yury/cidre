use crate::{arc, define_obj_type, ns, objc};

define_obj_type!(
    #[doc(alias = "NSOpenPanel")]
    pub OpenPanel(ns::SavePanel),
    NS_OPEN_PANEL
);

impl OpenPanel {
    /// A new open panel.
    #[objc::msg_send(openPanel)]
    pub fn open_panel() -> arc::R<Self>;

    /// The chosen files (valid after `run_modal` returned `ns::ModalResponse::OK`).
    #[objc::msg_send(URLs)]
    pub fn urls(&self) -> arc::R<ns::Array<ns::Url>>;

    #[objc::msg_send(resolvesAliases)]
    pub fn resolves_aliases(&self) -> bool;

    #[objc::msg_send(setResolvesAliases:)]
    pub fn set_resolves_aliases(&mut self, val: bool);

    #[objc::msg_send(canChooseDirectories)]
    pub fn can_choose_directories(&self) -> bool;

    #[objc::msg_send(setCanChooseDirectories:)]
    pub fn set_can_choose_directories(&mut self, val: bool);

    #[objc::msg_send(allowsMultipleSelection)]
    pub fn allows_multiple_selection(&self) -> bool;

    #[objc::msg_send(setAllowsMultipleSelection:)]
    pub fn set_allows_multiple_selection(&mut self, val: bool);

    #[objc::msg_send(canChooseFiles)]
    pub fn can_choose_files(&self) -> bool;

    #[objc::msg_send(setCanChooseFiles:)]
    pub fn set_can_choose_files(&mut self, val: bool);
}

unsafe extern "C" {
    static NS_OPEN_PANEL: &'static objc::Class<OpenPanel>;
}
