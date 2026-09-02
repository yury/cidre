use crate::{arc, cg, define_obj_type, objc, ui};

define_obj_type!(
    #[doc(alias = "UICollectionViewFlowLayout")]
    pub CollectionViewFlowLayout(ui::CollectionViewLayout),
    UI_COLLECTION_VIEW_FLOW_LAYOUT
);

impl CollectionViewFlowLayout {
    #[objc::msg_send(itemSize)]
    pub fn item_size(&self) -> cg::Size;

    #[objc::msg_send(setItemSize:)]
    pub fn set_item_size(&mut self, val: cg::Size);

    #[objc::msg_send(estimatedItemSize)]
    pub fn estimated_item_size(&self) -> cg::Size;

    #[objc::msg_send(setEstimatedItemSize:)]
    pub fn set_estimated_item_size(&mut self, val: cg::Size);

    #[objc::msg_send(minimumLineSpacing)]
    pub fn min_line_spacing(&self) -> cg::Float;

    #[objc::msg_send(setMinimumLineSpacing:)]
    pub fn set_min_line_spacing(&mut self, val: cg::Float);

    #[objc::msg_send(minimumInteritemSpacing)]
    pub fn min_interitem_spacing(&self) -> cg::Float;

    #[objc::msg_send(setMinimumInteritemSpacing:)]
    pub fn set_min_interitem_spacing(&mut self, val: cg::Float);

    #[objc::msg_send(scrollDirection)]
    pub fn scroll_dir(&self) -> ui::CollectionViewScrollDir;

    #[objc::msg_send(setScrollDirection:)]
    pub fn set_scroll_dir(&mut self, val: ui::CollectionViewScrollDir);

    #[objc::msg_send(sectionInset)]
    pub fn section_inset(&self) -> ui::EdgeInsets;

    #[objc::msg_send(setSectionInset:)]
    pub fn set_section_inset(&mut self, val: ui::EdgeInsets);
}

unsafe extern "C" {
    static UI_COLLECTION_VIEW_FLOW_LAYOUT: &'static objc::Class<CollectionViewFlowLayout>;
}
