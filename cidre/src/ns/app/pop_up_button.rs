use crate::{arc, define_obj_type, ns, objc};

define_obj_type!(
    #[doc(alias = "NSPopUpButton")]
    pub PopUpButton(ns::Button),
    NS_POP_UP_BUTTON
);

impl PopUpButton {
    #[objc::init(initWithFrame:pullsDown:)]
    pub fn init_with_frame_pulls_down(
        self,
        frame: ns::Rect,
        pulls_down: bool,
    ) -> arc::R<PopUpButton>;

    pub fn with_frame_pulls_down(frame: ns::Rect, pulls_down: bool) -> arc::R<Self> {
        Self::alloc().init_with_frame_pulls_down(frame, pulls_down)
    }

    #[objc::msg_send(menu)]
    pub fn menu(&self) -> Option<arc::R<ns::Menu>>;

    #[objc::msg_send(setMenu:)]
    pub fn set_menu(&mut self, val: Option<&ns::Menu>);

    #[objc::msg_send(pullsDown)]
    pub fn pulls_down(&self) -> bool;

    #[objc::msg_send(setPullsDown:)]
    pub fn set_pulls_down(&mut self, val: bool);

    #[objc::msg_send(addItemWithTitle:)]
    pub fn add_item_with_title(&mut self, title: &ns::String);

    #[objc::msg_send(removeAllItems)]
    pub fn remove_all_items(&mut self);

    #[objc::msg_send(itemArray)]
    pub fn item_array(&self) -> arc::R<ns::Array<ns::MenuItem>>;

    #[objc::msg_send(selectItemAtIndex:)]
    pub fn select_item_at(&mut self, index: ns::Integer);

    #[objc::msg_send(indexOfSelectedItem)]
    pub fn index_of_selected_item(&self) -> ns::Integer;

    #[objc::msg_send(selectedItem)]
    pub fn selected_item(&self) -> Option<arc::R<ns::MenuItem>>;

    #[objc::msg_send(synchronizeTitleAndSelectedItem)]
    pub fn synchronize_title_and_selected_item(&mut self);
}

unsafe extern "C" {
    static NS_POP_UP_BUTTON: &'static objc::Class<PopUpButton>;
}
