use crate::{arc, define_obj_type, ns, objc};

define_obj_type!(
    #[doc(alias = "NSTableCellView")]
    pub TableCellView(ns::View),
    NS_TABLE_CELL_VIEW
);

impl TableCellView {
    #[objc::init(initWithFrame:)]
    pub fn init_with_frame(self, frame: ns::Rect) -> arc::R<TableCellView>;

    pub fn with_frame(frame: ns::Rect) -> arc::R<Self> {
        Self::alloc().init_with_frame(frame)
    }

    #[objc::msg_send(objectValue)]
    pub fn obj_value(&self) -> Option<arc::R<ns::Id>>;

    #[objc::msg_send(setObjectValue:)]
    pub fn set_obj_value(&mut self, val: Option<&ns::Id>);

    /// The primary text field, whose colors follow the row's selection. Not retained.
    #[objc::msg_send(textField)]
    pub fn text_field(&self) -> Option<arc::R<ns::TextField>>;

    #[objc::msg_send(setTextField:)]
    pub fn set_text_field(&mut self, val: Option<&ns::TextField>);

    /// The primary image view. Not retained.
    #[objc::msg_send(imageView)]
    pub fn image_view(&self) -> Option<arc::R<ns::ImageView>>;

    #[objc::msg_send(setImageView:)]
    pub fn set_image_view(&mut self, val: Option<&ns::ImageView>);

    #[objc::msg_send(backgroundStyle)]
    pub fn bg_style(&self) -> ns::BackgroundStyle;

    #[objc::msg_send(setBackgroundStyle:)]
    pub fn set_bg_style(&mut self, val: ns::BackgroundStyle);

    #[objc::msg_send(rowSizeStyle)]
    pub fn row_size_style(&self) -> ns::TableViewRowSizeStyle;

    #[objc::msg_send(setRowSizeStyle:)]
    pub fn set_row_size_style(&mut self, val: ns::TableViewRowSizeStyle);
}

unsafe extern "C" {
    static NS_TABLE_CELL_VIEW: &'static objc::Class<TableCellView>;
}
