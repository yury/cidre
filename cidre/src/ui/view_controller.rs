use crate::{arc, cg, define_obj_type, ns, objc, ui};

define_obj_type!(
    #[doc(alias = "UIViewController")]
    pub ViewController(ui::Responder), UI_VIEW_CONTROLLER
);

impl ViewController {
    #[objc::msg_send(view)]
    pub fn view(&self) -> arc::R<ui::View>;

    /// Is used for any container laying out a child view controller.
    #[objc::msg_send(preferredContentSize)]
    pub fn preferred_content_size(&self) -> cg::Size;

    #[objc::msg_send(setPreferredContentSize:)]
    pub fn set_preferred_content_size(&mut self, val: cg::Size);

    #[objc::msg_send(title)]
    pub fn title(&self) -> Option<arc::R<ns::String>>;

    #[objc::msg_send(setTitle:)]
    pub fn set_title_string(&mut self, val: Option<&ns::String>);

    #[inline]
    pub fn set_title<S: AsRef<ns::String>>(&mut self, val: Option<&S>) {
        self.set_title_string(val.map(|s| s.as_ref()));
    }
}

#[link(name = "ui", kind = "static")]
unsafe extern "C" {
    static UI_VIEW_CONTROLLER: &'static objc::Class<ViewController>;
}
