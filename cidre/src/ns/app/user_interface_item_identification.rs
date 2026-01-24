use crate::{arc, define_obj_type, ns, objc};

define_obj_type!(
    #[doc(alias = "NSUserInterfaceItemIdentifier")]
    pub UiItemId(ns::String)
);

#[objc::protocol(NSUserInterfaceItemIdentification)]
pub trait UiItemIdentification: objc::Obj {
    #[objc::optional]
    #[objc::msg_send(identifier)]
    fn id(&self) -> Option<arc::R<ns::UiItemId>>;

    #[objc::optional]
    #[objc::msg_send(setIdentifier:)]
    fn set_id(&mut self, val: Option<&ns::UiItemId>);
}
