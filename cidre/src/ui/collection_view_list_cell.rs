use crate::{arc, cg, define_obj_type, ns, objc, ui};

define_obj_type!(
    #[doc(alias = "UICollectionViewListCell")]
    pub CollectionViewListCell(ui::CollectionViewCell),
    UI_COLLECTION_VIEW_LIST_CELL
);

impl CollectionViewListCell {
    /// The amount of indentation applied to the cell's content and accessories, in levels.
    #[objc::msg_send(indentationLevel)]
    #[objc::available(ios = 14.0, tvos = 14.0)]
    pub fn indentation_level(&self) -> ns::Integer;

    #[objc::msg_send(setIndentationLevel:)]
    #[objc::available(ios = 14.0, tvos = 14.0)]
    pub fn set_indentation_level(&mut self, val: ns::Integer);

    /// The width of one level of indentation. Default is 20.
    #[objc::msg_send(indentationWidth)]
    #[objc::available(ios = 14.0, tvos = 14.0)]
    pub fn indentation_width(&self) -> cg::Float;

    #[objc::msg_send(setIndentationWidth:)]
    #[objc::available(ios = 14.0, tvos = 14.0)]
    pub fn set_indentation_width(&mut self, val: cg::Float);

    /// Whether accessories on the leading side are indented as well. Default is true.
    #[objc::msg_send(indentsAccessories)]
    #[objc::available(ios = 14.0, tvos = 14.0)]
    pub fn indents_accessories(&self) -> bool;

    #[objc::msg_send(setIndentsAccessories:)]
    #[objc::available(ios = 14.0, tvos = 14.0)]
    pub fn set_indents_accessories(&mut self, val: bool);

    /// All the accessories decorating the cell.
    #[objc::msg_send(accessories)]
    #[objc::available(ios = 14.0, tvos = 14.0)]
    pub fn accessories(&self) -> arc::R<ns::Array<ui::CellAccessory>>;

    #[objc::msg_send(setAccessories:)]
    #[objc::available(ios = 14.0, tvos = 14.0)]
    pub fn set_accessories(&mut self, val: &ns::Array<ui::CellAccessory>);
}

unsafe extern "C" {
    static UI_COLLECTION_VIEW_LIST_CELL: &'static objc::Class<CollectionViewListCell>;
}
