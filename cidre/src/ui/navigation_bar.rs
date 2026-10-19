use crate::{define_obj_type, objc, ui};

define_obj_type!(
    #[doc(alias = "UINavigationBar")]
    pub NavBar(ui::View)
);

impl NavBar {
    #[objc::msg_send(prefersLargeTitles)]
    #[objc::available(ios = 11.0)]
    pub fn prefers_large_titles(&self) -> bool;

    #[objc::msg_send(setPrefersLargeTitles:)]
    #[objc::available(ios = 11.0)]
    pub fn set_prefers_large_titles(&mut self, val: bool);
}
