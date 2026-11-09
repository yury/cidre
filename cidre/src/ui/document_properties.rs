use crate::{arc, define_cls, define_obj_type, lp, ns, objc};

define_obj_type!(
    #[doc(alias = "UIDocumentProperties")]
    pub DocumentProps(ns::Id)
);

impl DocumentProps {
    define_cls!(sym UIDocumentProperties);

    #[objc::init(initWithMetadata:)]
    fn init_with_metadata(self, metadata: &lp::LinkMetadata) -> arc::R<DocumentProps>;

    #[objc::available(ios = 16.0)]
    pub fn with_metadata(metadata: &lp::LinkMetadata) -> arc::R<Self> {
        Self::alloc().init_with_metadata(metadata)
    }

    #[objc::msg_send(wantsIconRepresentation)]
    pub fn wants_icon_representation(&self) -> bool;

    #[objc::msg_send(setWantsIconRepresentation:)]
    pub fn set_wants_icon_representation(&mut self, val: bool);
}
