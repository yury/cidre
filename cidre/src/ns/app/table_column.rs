use crate::{arc, cg, define_obj_type, ns, objc};

define_obj_type!(
    #[doc(alias = "NSTableColumn")]
    pub TableColumn(ns::Id),
    NS_TABLE_COLUMN
);

impl TableColumn {
    #[objc::init(initWithIdentifier:)]
    pub fn init_with_id(self, id: &ns::String) -> arc::R<TableColumn>;

    pub fn with_id(id: &ns::String) -> arc::R<Self> {
        Self::alloc().init_with_id(id)
    }

    #[objc::msg_send(width)]
    pub fn width(&self) -> cg::Float;

    #[objc::msg_send(setWidth:)]
    pub fn set_width(&mut self, val: cg::Float);

    #[objc::msg_send(setMinWidth:)]
    pub fn set_min_width(&mut self, val: cg::Float);

    #[objc::msg_send(setEditable:)]
    pub fn set_editable(&mut self, val: bool);
}

unsafe extern "C" {
    static NS_TABLE_COLUMN: &'static objc::Class<TableColumn>;
}
