use crate::{arc, define_obj_type, objc, ui};

define_obj_type!(
    #[doc(alias = "UICollectionViewCell")]
    pub CollectionViewCell(ui::CollectionReusableView)
);

#[doc(alias = "UICollectionViewCellDragState")]
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
#[repr(isize)]
pub enum CollectionViewCellDragState {
    None,
    Lifting,
    Dragging,
}

impl CollectionViewCell {
    #[objc::msg_send(contentView)]
    pub fn content_view(&self) -> arc::R<ui::View>;

    #[objc::msg_send(isSelected)]
    pub fn is_selected(&self) -> bool;

    #[objc::msg_send(setSelected:)]
    pub fn set_selected(&mut self, val: bool);

    #[objc::msg_send(isHighlighted)]
    pub fn is_highlighted(&self) -> bool;

    #[objc::msg_send(setHighlighted:)]
    pub fn set_highlighted(&mut self, val: bool);

    #[objc::msg_send(backgroundView)]
    pub fn bg_view(&self) -> Option<arc::R<ui::View>>;

    #[objc::msg_send(setBackgroundView:)]
    pub fn set_bg_view(&mut self, val: Option<&ui::View>);

    #[objc::msg_send(selectedBackgroundView)]
    pub fn selected_bg_view(&self) -> Option<arc::R<ui::View>>;

    #[objc::msg_send(setSelectedBackgroundView:)]
    pub fn set_selected_bg_view(&mut self, val: Option<&ui::View>);

    #[objc::msg_send(dragStateDidChange:)]
    pub fn drag_state_did_change(&self, drag_state: CollectionViewCellDragState);
}
