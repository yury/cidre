use crate::{arc, cg, define_obj_type, define_opts, ns, objc};

define_opts!(
    #[doc(alias = "NSTableColumnResizingOptions")]
    pub TableColumnResizingOpts(usize)
);

impl TableColumnResizingOpts {
    pub const NO_RESIZING: Self = Self(0);
    /// The column resizes with the table, per the table's column autoresizing style.
    pub const AUTORESIZING: Self = Self(1 << 0);
    /// The user can resize the column.
    pub const USER_RESIZING: Self = Self(1 << 1);
}

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

    #[objc::msg_send(maxWidth)]
    pub fn max_width(&self) -> cg::Float;

    #[objc::msg_send(setMaxWidth:)]
    pub fn set_max_width(&mut self, val: cg::Float);

    #[objc::msg_send(resizingMask)]
    pub fn resizing_mask(&self) -> TableColumnResizingOpts;

    #[objc::msg_send(setResizingMask:)]
    pub fn set_resizing_mask(&mut self, val: TableColumnResizingOpts);

    #[objc::msg_send(setEditable:)]
    pub fn set_editable(&mut self, val: bool);
}

unsafe extern "C" {
    static NS_TABLE_COLUMN: &'static objc::Class<TableColumn>;
}
