use crate::{arc, cg, define_obj_type, ns, objc, ui};

#[doc(alias = "UIBarButtonItemStyle")]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[repr(isize)]
#[non_exhaustive]
pub enum Style {
    Plain = 0,

    /// A prominent button, such as one that completes a task: not grouped with
    /// the other items of its bar, and styled for its prominence.
    #[doc(alias = "UIBarButtonItemStyleProminent")]
    Prominent = 2,
}

define_obj_type!(
    #[doc(alias = "UIBarButtonItem")]
    pub BarButtonItem(ns::Id),
    UI_BAR_BUTTON_ITEM
);

impl BarButtonItem {
    #[objc::init(initWithCustomView:)]
    pub fn init_with_custom_view(self, view: &ui::View) -> arc::R<BarButtonItem>;

    /// `action` with no target is sent up the responder chain.
    #[objc::init(initWithImage:style:target:action:)]
    pub fn init_with_image_style_target_action(
        self,
        image: Option<&ui::Image>,
        style: Style,
        target: Option<&ns::Id>,
        action: Option<&objc::Sel>,
    ) -> arc::R<BarButtonItem>;

    pub fn with_image_style_target_action(
        image: Option<&ui::Image>,
        style: Style,
        target: Option<&ns::Id>,
        action: Option<&objc::Sel>,
    ) -> arc::R<Self> {
        Self::alloc().init_with_image_style_target_action(image, style, target, action)
    }

    pub fn with_custom_view(view: &ui::View) -> arc::R<Self> {
        Self::alloc().init_with_custom_view(view)
    }

    #[objc::init(initWithImage:menu:)]
    #[objc::available(ios = 14.0, tvos = 14.0)]
    pub fn init_with_image_menu(
        self,
        image: Option<&ui::Image>,
        menu: Option<&ui::Menu>,
    ) -> arc::R<BarButtonItem>;

    #[objc::available(ios = 14.0, tvos = 14.0)]
    pub fn with_image_menu(image: Option<&ui::Image>, menu: Option<&ui::Menu>) -> arc::R<Self> {
        Self::alloc().init_with_image_menu(image, menu)
    }

    #[objc::init(initWithTitle:menu:)]
    #[objc::available(ios = 14.0, tvos = 14.0)]
    pub fn init_with_title_menu(
        self,
        title: Option<&ns::String>,
        menu: Option<&ui::Menu>,
    ) -> arc::R<BarButtonItem>;

    #[objc::available(ios = 14.0, tvos = 14.0)]
    pub fn with_title_menu(title: Option<&ns::String>, menu: Option<&ui::Menu>) -> arc::R<Self> {
        Self::alloc().init_with_title_menu(title, menu)
    }

    /// Creates a plain-style bar button item from the properties of the action.
    #[objc::init(initWithPrimaryAction:)]
    #[objc::available(ios = 14.0, tvos = 14.0)]
    pub fn init_with_primary_action(self, action: Option<&ui::Action>) -> arc::R<BarButtonItem>;

    #[objc::available(ios = 14.0, tvos = 14.0)]
    pub fn with_primary_action(action: Option<&ui::Action>) -> arc::R<Self> {
        Self::alloc().init_with_primary_action(action)
    }

    /// Creates a plain-style bar button item from the properties of the action,
    /// with a menu shown on long press or tap when there is no primary action.
    #[objc::init(initWithPrimaryAction:menu:)]
    #[objc::available(ios = 16.0, tvos = 16.0)]
    pub fn init_with_primary_action_menu(
        self,
        action: Option<&ui::Action>,
        menu: Option<&ui::Menu>,
    ) -> arc::R<BarButtonItem>;

    #[objc::available(ios = 16.0, tvos = 16.0)]
    pub fn with_primary_action_menu(
        action: Option<&ui::Action>,
        menu: Option<&ui::Menu>,
    ) -> arc::R<Self> {
        Self::alloc().init_with_primary_action_menu(action, menu)
    }

    /// Creates a fixed-space bar button item of the given width.
    #[objc::msg_send(fixedSpaceItemOfWidth:)]
    #[objc::available(ios = 14.0, tvos = 14.0)]
    pub fn fixed_space_of_width(width: cg::Float) -> arc::R<Self>;

    /// Creates a flexible-space bar button item.
    #[objc::msg_send(flexibleSpaceItem)]
    #[objc::available(ios = 14.0, tvos = 14.0)]
    pub fn flexible_space() -> arc::R<Self>;

    #[objc::msg_send(title)]
    pub fn title(&self) -> Option<arc::R<ns::String>>;

    #[objc::msg_send(setTitle:)]
    pub fn set_title(&mut self, val: Option<&ns::String>);

    #[objc::msg_send(image)]
    pub fn image(&self) -> Option<arc::R<ui::Image>>;

    #[objc::msg_send(setImage:)]
    pub fn set_image(&mut self, val: Option<&ui::Image>);

    #[objc::msg_send(isEnabled)]
    pub fn is_enabled(&self) -> bool;

    #[objc::msg_send(setEnabled:)]
    pub fn set_enabled(&mut self, val: bool);

    #[objc::msg_send(tag)]
    pub fn tag(&self) -> ns::Integer;

    #[objc::msg_send(setTag:)]
    pub fn set_tag(&mut self, val: ns::Integer);

    #[objc::msg_send(width)]
    pub fn width(&self) -> cg::Float;

    #[objc::msg_send(setWidth:)]
    pub fn set_width(&mut self, val: cg::Float);

    #[objc::msg_send(customView)]
    pub fn custom_view(&self) -> Option<arc::R<ui::View>>;

    #[objc::msg_send(setCustomView:)]
    pub fn set_custom_view(&mut self, val: Option<&ui::View>);

    /// Set the primary action on this item, updating the title, image, and target/action
    /// of the item to match the action.
    #[objc::msg_send(primaryAction)]
    #[objc::available(ios = 14.0, tvos = 14.0)]
    pub fn primary_action(&self) -> Option<arc::R<ui::Action>>;

    #[objc::msg_send(setPrimaryAction:)]
    #[objc::available(ios = 14.0, tvos = 14.0)]
    pub fn set_primary_action(&mut self, val: Option<&ui::Action>);

    #[objc::msg_send(menu)]
    #[objc::available(ios = 14.0, tvos = 14.0)]
    pub fn menu(&self) -> Option<arc::R<ui::Menu>>;

    #[objc::msg_send(setMenu:)]
    #[objc::available(ios = 14.0, tvos = 14.0)]
    pub fn set_menu(&mut self, val: Option<&ui::Menu>);

    /// Whether the item is drawn outside the shared glass background of its bar.
    /// Default is false.
    #[objc::msg_send(hidesSharedBackground)]
    #[objc::available(ios = 26.0)]
    pub fn hides_shared_background(&self) -> bool;

    #[objc::msg_send(setHidesSharedBackground:)]
    #[objc::available(ios = 26.0)]
    pub fn set_hides_shared_background(&mut self, val: bool);

    /// Whether the item shares a background with its neighbours. Default is true.
    #[objc::msg_send(sharesBackground)]
    #[objc::available(ios = 26.0)]
    pub fn shares_background(&self) -> bool;

    #[objc::msg_send(setSharesBackground:)]
    #[objc::available(ios = 26.0)]
    pub fn set_shares_background(&mut self, val: bool);
}

unsafe extern "C" {
    static UI_BAR_BUTTON_ITEM: &'static objc::Class<BarButtonItem>;
}
