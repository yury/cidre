use crate::{arc, cg, define_obj_type, ns, objc};

#[doc(alias = "NSBoxType")]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[repr(usize)]
pub enum BoxType {
    /// A group with a title, drawn by the system.
    Primary = 0,
    /// A horizontal or vertical line, by the box's longer side.
    Separator = 2,
    /// Drawn with the box's own fill and border colours, corner radius and width.
    Custom = 4,
}

#[doc(alias = "NSTitlePosition")]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[repr(usize)]
pub enum TitlePos {
    No = 0,
    AboveTop = 1,
    AtTop = 2,
    BelowTop = 3,
    AboveBottom = 4,
    AtBottom = 5,
    BelowBottom = 6,
}

define_obj_type!(
    #[doc(alias = "NSBox")]
    pub Box(ns::View),
    NS_BOX
);

impl Box {
    #[objc::msg_send(boxType)]
    pub fn box_type(&self) -> BoxType;

    #[objc::msg_send(setBoxType:)]
    pub fn set_box_type(&mut self, val: BoxType);

    #[objc::msg_send(title)]
    pub fn title(&self) -> arc::R<ns::String>;

    #[objc::msg_send(setTitle:)]
    pub fn set_title(&mut self, val: &ns::String);

    #[objc::msg_send(titlePosition)]
    pub fn title_pos(&self) -> TitlePos;

    #[objc::msg_send(setTitlePosition:)]
    pub fn set_title_pos(&mut self, val: TitlePos);

    #[objc::msg_send(isTransparent)]
    pub fn is_transparent(&self) -> bool;

    #[objc::msg_send(setTransparent:)]
    pub fn set_transparent(&mut self, val: bool);

    #[objc::msg_send(contentView)]
    pub fn content_view(&self) -> Option<arc::R<ns::View>>;

    #[objc::msg_send(setContentView:)]
    pub fn set_content_view(&mut self, val: Option<&ns::View>);

    #[objc::msg_send(contentViewMargins)]
    pub fn content_view_margins(&self) -> ns::Size;

    #[objc::msg_send(setContentViewMargins:)]
    pub fn set_content_view_margins(&mut self, val: ns::Size);

    /// For [`BoxType::Custom`].
    #[objc::msg_send(fillColor)]
    pub fn fill_color(&self) -> arc::R<ns::Color>;

    #[objc::msg_send(setFillColor:)]
    pub fn set_fill_color(&mut self, val: &ns::Color);

    #[objc::msg_send(borderColor)]
    pub fn border_color(&self) -> arc::R<ns::Color>;

    #[objc::msg_send(setBorderColor:)]
    pub fn set_border_color(&mut self, val: &ns::Color);

    #[objc::msg_send(borderWidth)]
    pub fn border_width(&self) -> cg::Float;

    #[objc::msg_send(setBorderWidth:)]
    pub fn set_border_width(&mut self, val: cg::Float);

    #[objc::msg_send(cornerRadius)]
    pub fn corner_radius(&self) -> cg::Float;

    #[objc::msg_send(setCornerRadius:)]
    pub fn set_corner_radius(&mut self, val: cg::Float);
}

impl Box {
    /// A line along the longer side of `frame`.
    pub fn separator(frame: ns::Rect) -> arc::R<Self> {
        let mut val = Self::new();
        val.set_box_type(BoxType::Separator);
        val.set_frame(frame);
        val
    }
}

unsafe extern "C" {
    static NS_BOX: &'static objc::Class<Box>;
}

#[cfg(test)]
mod tests {
    use crate::ns;

    #[test]
    fn basics() {
        let mut separator = ns::Box::separator(ns::Rect::new(0.0, 0.0, 100.0, 1.0));
        assert_eq!(separator.box_type(), ns::BoxType::Separator);
        separator.set_box_type(ns::BoxType::Custom);
        separator.set_corner_radius(8.0);
        assert_eq!(separator.corner_radius(), 8.0);
    }
}
