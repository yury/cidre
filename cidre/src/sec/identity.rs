use crate::{cf, sec};

impl sec::Identity {
    pub fn type_id() -> cf::TypeId {
        unsafe { SecIdentityGetTypeID() }
    }
}

unsafe extern "C" {
    fn SecIdentityGetTypeID() -> cf::TypeId;
}
