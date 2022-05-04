use crate::{cg, define_obj_type, msg_send, objc::Id};

define_obj_type!(MetadataObject(Id));

impl MetadataObject {
    pub fn bounds(&self) -> cg::Rect {
        msg_send!("common", self, sel_bounds)
    }
}
