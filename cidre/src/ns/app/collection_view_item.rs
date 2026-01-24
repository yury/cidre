use crate::{arc, define_obj_type, ns, objc};

define_obj_type!(
    #[doc(alias = "NSCollectionViewItem")]
    pub CollectionViewItem(ns::ViewController),
    NS_COLLECTION_VIEW_ITEM
);

impl CollectionViewItem {
    #[objc::msg_send(collectionView)]
    pub fn collection_view(&self) -> Option<arc::R<ns::CollectionView>>;

    #[objc::msg_send(isSelected)]
    pub fn is_selected(&self) -> bool;

    #[objc::msg_send(setSelected:)]
    pub fn set_selected(&mut self, val: bool);

    #[objc::msg_send(highlightState)]
    pub fn highlight_state(&self) -> ns::CollectionViewItemHighlightState;

    #[objc::msg_send(setHighlightState:)]
    pub fn set_highlight_state(&mut self, val: ns::CollectionViewItemHighlightState);

    #[objc::msg_send(imageView)]
    pub fn image_view(&self) -> Option<arc::R<ns::View>>;

    #[objc::msg_send(setImageView:)]
    pub fn set_image_view(&mut self, val: Option<&ns::View>);

    #[objc::msg_send(textField)]
    pub fn text_field(&self) -> Option<arc::R<ns::TextField>>;

    #[objc::msg_send(setTextField:)]
    pub fn set_text_field(&mut self, val: Option<&ns::TextField>);

    #[objc::msg_send(draggingImageComponents)]
    pub fn dragging_image_components(&self) -> arc::R<ns::Array<ns::Id>>;
}

#[link(name = "app", kind = "static")]
unsafe extern "C" {
    static NS_COLLECTION_VIEW_ITEM: &'static objc::Class<CollectionViewItem>;
}
