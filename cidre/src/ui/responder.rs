use crate::{arc, define_obj_type, ns, objc};

#[objc::protocol(UIResponderStandardEditActions)]
pub trait ResponderStandardEditActions: objc::Obj {
    #[cfg(not(target_os = "watchos"))]
    #[objc::optional]
    #[objc::msg_send(cut:)]
    fn cut(&mut self, sender: Option<&ns::Id>);

    #[cfg(not(target_os = "watchos"))]
    #[objc::optional]
    #[objc::msg_send(copy:)]
    fn copy(&mut self, sender: Option<&ns::Id>);

    #[cfg(not(target_os = "watchos"))]
    #[objc::optional]
    #[objc::msg_send(copy:)]
    fn paste(&mut self, sender: Option<&ns::Id>);

    #[cfg(not(target_os = "watchos"))]
    #[objc::optional]
    #[objc::msg_send(pasteAndMatchStyle:)]
    fn paste_and_match_style(&mut self, sender: Option<&ns::Id>);

    #[cfg(not(target_os = "watchos"))]
    #[objc::optional]
    #[objc::msg_send(pasteAndGo:)]
    fn paste_and_go(&mut self, sender: Option<&ns::Id>);

    #[cfg(not(target_os = "watchos"))]
    #[objc::optional]
    #[objc::msg_send(newFromPasteboard:)]
    fn new_from_pasteboard(&mut self, sender: Option<&ns::Id>);

    #[cfg(not(target_os = "watchos"))]
    #[objc::optional]
    #[objc::msg_send(select:)]
    fn select(&mut self, sender: Option<&ns::Id>);

    #[cfg(not(target_os = "watchos"))]
    #[objc::optional]
    #[objc::msg_send(selectAll:)]
    fn select_all(&mut self, sender: Option<&ns::Id>);

    #[cfg(not(target_os = "watchos"))]
    #[objc::optional]
    #[objc::msg_send(delete:)]
    fn delete(&mut self, sender: Option<&ns::Id>);

    #[cfg(not(target_os = "watchos"))]
    #[objc::optional]
    #[objc::msg_send(makeTextWritingDirectionLeftToRight:)]
    fn make_text_writing_ltr(&mut self, sender: Option<&ns::Id>);

    #[cfg(not(target_os = "watchos"))]
    #[objc::optional]
    #[objc::msg_send(makeTextWritingDirectionRightToLeft:)]
    fn make_text_writing_rtl(&mut self, sender: Option<&ns::Id>);

    #[cfg(not(target_os = "watchos"))]
    #[objc::optional]
    #[objc::msg_send(toggleBoldface:)]
    fn toggle_bold(&mut self, sender: Option<&ns::Id>);

    #[cfg(not(target_os = "watchos"))]
    #[objc::optional]
    #[objc::msg_send(toggleItalics:)]
    fn toggle_italics(&mut self, sender: Option<&ns::Id>);

    #[cfg(not(target_os = "watchos"))]
    #[objc::optional]
    #[objc::msg_send(toggleUnderline:)]
    fn toggle_underline(&mut self, sender: Option<&ns::Id>);

    #[cfg(not(target_os = "watchos"))]
    #[objc::optional]
    #[objc::msg_send(increaseSize:)]
    fn increase_size(&mut self, sender: Option<&ns::Id>);

    #[cfg(not(target_os = "watchos"))]
    #[objc::optional]
    #[objc::msg_send(decreaseSize:)]
    fn decrease_size(&mut self, sender: Option<&ns::Id>);

    #[cfg(not(target_os = "watchos"))]
    #[objc::optional]
    #[objc::msg_send(alignLeft:)]
    fn align_left(&mut self, sender: Option<&ns::Id>);

    #[cfg(not(target_os = "watchos"))]
    #[objc::optional]
    #[objc::msg_send(alignCenter:)]
    fn align_center(&mut self, sender: Option<&ns::Id>);

    #[cfg(not(target_os = "watchos"))]
    #[objc::optional]
    #[objc::msg_send(alignJustified:)]
    fn align_justified(&mut self, sender: Option<&ns::Id>);

    #[cfg(not(target_os = "watchos"))]
    #[objc::optional]
    #[objc::msg_send(alignRight:)]
    fn align_right(&mut self, sender: Option<&ns::Id>);

    #[cfg(not(target_os = "watchos"))]
    #[objc::optional]
    #[objc::msg_send(find:)]
    fn find(&mut self, sender: Option<&ns::Id>);

    #[cfg(not(target_os = "watchos"))]
    #[objc::optional]
    #[objc::msg_send(findAndReplace:)]
    fn find_and_replace(&mut self, sender: Option<&ns::Id>);

    #[cfg(not(target_os = "watchos"))]
    #[objc::optional]
    #[objc::msg_send(findNext:)]
    fn find_next(&mut self, sender: Option<&ns::Id>);

    #[cfg(not(target_os = "watchos"))]
    #[objc::optional]
    #[objc::msg_send(findPrevious:)]
    fn find_previous(&mut self, sender: Option<&ns::Id>);

    #[cfg(not(target_os = "watchos"))]
    #[objc::optional]
    #[objc::msg_send(useSelectionForFind:)]
    fn use_selection_for_find(&mut self, sender: Option<&ns::Id>);

    #[cfg(not(target_os = "watchos"))]
    #[objc::optional]
    #[objc::msg_send(print:)]
    fn print(&mut self, sender: Option<&ns::Id>);

    #[cfg(not(target_os = "watchos"))]
    #[objc::optional]
    #[objc::msg_send(rename:)]
    fn rename(&mut self, sender: Option<&ns::Id>);

    #[cfg(not(target_os = "watchos"))]
    #[objc::optional]
    #[objc::msg_send(duplicate:)]
    fn duplicate(&mut self, sender: Option<&ns::Id>);

    #[cfg(not(target_os = "watchos"))]
    #[objc::optional]
    #[objc::msg_send(move_:)]
    fn move_(&mut self, sender: Option<&ns::Id>);

    #[cfg(not(target_os = "watchos"))]
    #[objc::optional]
    #[objc::msg_send(export:)]
    fn export(&mut self, sender: Option<&ns::Id>);

    #[cfg(not(target_os = "watchos"))]
    #[objc::optional]
    #[objc::msg_send(toggleSidebar:)]
    fn toggle_sidebar(&mut self, sender: Option<&ns::Id>);

    #[cfg(not(target_os = "watchos"))]
    #[objc::optional]
    #[objc::msg_send(toggleInspector:)]
    fn toggle_inspector(&mut self, sender: Option<&ns::Id>);

    #[cfg(not(target_os = "watchos"))]
    #[objc::optional]
    #[objc::msg_send(performClose:)]
    fn perform_close(&mut self, sender: Option<&ns::Id>);

    #[cfg(not(target_os = "watchos"))]
    #[objc::optional]
    #[objc::msg_send(showWritingTools:)]
    fn show_writing_tools(&mut self, sender: Option<&ns::Id>);
}

define_obj_type!(
    #[doc(alias = "UIResponder")]
    pub Responder(ns::Id),
    UI_RESPONDER
);

impl ResponderStandardEditActions for Responder {}

impl Responder {
    #[objc::msg_send(nextResponder)]
    pub fn next_responder(&self) -> Option<arc::R<Self>>;
}

#[link(name = "ui", kind = "static")]
unsafe extern "C" {
    static UI_RESPONDER: &'static objc::Class<Responder>;
}
