use crate::{arc, cg, define_obj_type, ns, objc, ui};

define_obj_type!(
    #[doc(alias = "UIToolbar")]
    pub Toolbar(ui::View),
    UI_TOOLBAR
);

impl Toolbar {
    #[objc::init(initWithFrame:)]
    pub fn init_with_frame(self, frame: cg::Rect) -> arc::R<Toolbar>;

    pub fn with_frame(frame: cg::Rect) -> arc::R<Self> {
        Self::alloc().init_with_frame(frame)
    }

    #[objc::msg_send(barStyle)]
    pub fn bar_style(&self) -> ui::BarStyle;

    #[objc::msg_send(setBarStyle:)]
    pub fn set_bar_style(&mut self, val: ui::BarStyle);

    /// Visible `ui::BarButtonItem`s, shown in order. Default is `None`. Changes are not animated.
    #[objc::msg_send(items)]
    pub fn items(&self) -> Option<arc::R<ns::Array<ui::BarButtonItem>>>;

    #[objc::msg_send(setItems:)]
    pub fn set_items(&mut self, val: Option<&ns::Array<ui::BarButtonItem>>);

    /// Will fade in or out or reorder and adjust spacing.
    #[objc::msg_send(setItems:animated:)]
    pub fn set_items_animated(
        &mut self,
        val: Option<&ns::Array<ui::BarButtonItem>>,
        animated: bool,
    );

    #[objc::msg_send(isTranslucent)]
    pub fn is_translucent(&self) -> bool;

    #[objc::msg_send(setTranslucent:)]
    pub fn set_translucent(&mut self, val: bool);

    #[objc::msg_send(barTintColor)]
    pub fn bar_tint_color(&self) -> Option<arc::R<ui::Color>>;

    #[objc::msg_send(setBarTintColor:)]
    pub fn set_bar_tint_color(&mut self, val: Option<&ui::Color>);
}

unsafe extern "C" {
    static UI_TOOLBAR: &'static objc::Class<Toolbar>;
}
