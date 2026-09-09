use crate::{arc, define_obj_type, define_opts, ns, objc, ui};

define_opts!(
    #[doc(alias = "UIMenuOptions")]
    pub MenuOpts(usize)
);

impl MenuOpts {
    /// Show children inline in parent, instead of hierarchically.
    pub const DISPLAY_INLINE: Self = Self(1 << 0);

    /// Indicates whether the menu should be rendered with a destructive appearance in its parent.
    pub const DESTRUCTIVE: Self = Self(1 << 1);

    /// Indicates whether the menu should only allow a single "on" menu item.
    pub const SINGLE_SELECTION: Self = Self(1 << 5);

    /// Indicates whether the menu should be rendered as a palette.
    pub const DISPLAY_AS_PALETTE: Self = Self(1 << 7);
}

define_obj_type!(
    #[doc(alias = "UIMenu")]
    pub Menu(ui::MenuElement),
    UI_MENU
);

impl Menu {
    #[objc::msg_send(menuWithTitle:children:)]
    #[objc::available(ios = 13.0, tvos = 13.0)]
    pub fn with_title_children(
        title: &ns::String,
        children: &ns::Array<ui::MenuElement>,
    ) -> arc::R<Self>;

    #[objc::msg_send(menuWithChildren:)]
    #[objc::available(ios = 14.0, tvos = 14.0)]
    pub fn with_children(children: &ns::Array<ui::MenuElement>) -> arc::R<Self>;

    #[objc::msg_send(menuWithTitle:image:identifier:options:children:)]
    #[objc::available(ios = 13.0, tvos = 13.0)]
    pub fn with_title_image_id_opts_children(
        title: &ns::String,
        image: Option<&ui::Image>,
        id: Option<&ns::String>,
        opts: MenuOpts,
        children: &ns::Array<ui::MenuElement>,
    ) -> arc::R<Self>;

    #[objc::msg_send(title)]
    pub fn title(&self) -> arc::R<ns::String>;

    #[objc::msg_send(identifier)]
    pub fn id(&self) -> arc::R<ns::String>;

    #[objc::msg_send(options)]
    pub fn opts(&self) -> MenuOpts;

    #[objc::msg_send(children)]
    pub fn children(&self) -> arc::R<ns::Array<ui::MenuElement>>;

    #[objc::msg_send(menuByReplacingChildren:)]
    pub fn by_replacing_children(&self, children: &ns::Array<ui::MenuElement>) -> arc::R<Self>;
}

unsafe extern "C" {
    static UI_MENU: &'static objc::Class<Menu>;
}
