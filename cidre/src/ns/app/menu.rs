use crate::{arc, define_obj_type, ns, objc};

define_obj_type!(
    #[doc(alias = "NSMenu")]
    pub Menu(ns::Id),
    NS_MENU
);

impl arc::A<Menu> {
    #[objc::msg_send(initWithTitle:)]
    pub fn init_with_title(self, title: &ns::String) -> arc::R<Menu>;
}

impl Menu {
    pub fn with_title(title: &ns::String) -> arc::R<Self> {
        Self::alloc().init_with_title(title)
    }

    #[objc::msg_send(title)]
    pub fn title(&self) -> arc::R<ns::String>;

    #[objc::msg_send(setTitle:)]
    pub fn set_title(&mut self, val: &ns::String);
}

unsafe extern "C" {
    static NS_MENU: &'static objc::Class<Menu>;
}

#[cfg(test)]
mod tests {
    use crate::ns;

    #[test]
    fn basics() {
        let mut menu = ns::Menu::with_title(ns::str!(c"Copy"));
        assert_eq!(menu.title().as_ref(), "Copy");

        menu.set_title(ns::str!(c""));
        assert!(menu.title().is_empty());
    }
}
