use crate::{arc, cg, define_obj_type, ns, objc};

define_obj_type!(
    #[doc(alias = "NSTitlebarAccessoryViewController")]
    pub TitlebarAccessoryViewController(ns::ViewController),
    NS_TITLEBAR_ACCESSORY_VIEW_CONTROLLER
);

impl TitlebarAccessoryViewController {
    #[objc::msg_send(layoutAttribute)]
    pub fn layout_attr(&self) -> ns::LayoutAttr;

    #[objc::msg_send(setLayoutAttribute:)]
    pub fn set_layout_attr(&mut self, val: ns::LayoutAttr);

    #[objc::msg_send(fullScreenMinHeight)]
    pub fn full_screen_min_height(&self) -> cg::Float;

    #[objc::msg_send(setFullScreenMinHeight:)]
    pub fn set_full_screen_min_height(&mut self, val: cg::Float);

    #[objc::msg_send(isHidden)]
    pub fn is_hidden(&self) -> bool;

    #[objc::msg_send(setHidden:)]
    pub fn set_hidden(&mut self, val: bool);

    #[objc::msg_send(automaticallyAdjustsSize)]
    pub fn automatically_adjusts_size(&self) -> bool;

    #[objc::msg_send(setAutomaticallyAdjustsSize:)]
    pub fn set_automatically_adjusts_size(&mut self, val: bool);

    #[objc::msg_send(viewWillAppear)]
    pub fn view_will_appear(&mut self);

    #[objc::msg_send(viewDidAppear)]
    pub fn view_did_appear(&mut self);

    #[objc::msg_send(viewDidDisappear)]
    pub fn view_did_disappear(&mut self);
}

unsafe extern "C" {
    static NS_TITLEBAR_ACCESSORY_VIEW_CONTROLLER:
        &'static objc::Class<TitlebarAccessoryViewController>;
}

#[cfg(test)]
mod test {
    use crate::ns;

    #[test]
    fn basics() {
        let titlebar_vc = ns::TitlebarAccessoryViewController::new();
        let attr = titlebar_vc.layout_attr();
        assert_eq!(attr, ns::LayoutAttr::Bottom);
        assert_eq!(titlebar_vc.full_screen_min_height(), 0.0);
        assert_eq!(titlebar_vc.is_hidden(), false);
        assert_eq!(titlebar_vc.automatically_adjusts_size(), true);
    }
}
