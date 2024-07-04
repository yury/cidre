use crate::{arc, define_cls, define_obj_type, ns, objc};

define_obj_type!(
    #[doc(alias = "NSKeyedArchiver")]
    pub KeyedArchiver(ns::Coder)
);

impl KeyedArchiver {
    define_cls!(NS_KEYED_ARCHIVER);

    #[objc::msg_send(archivedDataWithRootObject:requiringSecureCoding:error:)]
    pub unsafe fn archived_data_with_root_obj_err<'ear>(
        obj: &ns::Id,
        secure_coding: bool,
        err: *mut Option<&'ear ns::Error>,
    ) -> Option<arc::R<ns::Data>>;

    #[inline]
    pub fn archived_data_with_root_obj<'ear>(
        obj: &ns::Id,
        secure_coding: bool,
    ) -> Result<arc::R<ns::Data>, &'ear ns::Error> {
        ns::if_none(|err| unsafe { Self::archived_data_with_root_obj_err(obj, secure_coding, err) })
    }
}

#[link(name = "ns", kind = "static")]
extern "C" {
    static NS_KEYED_ARCHIVER: &'static objc::Class<KeyedArchiver>;
}

#[cfg(test)]
mod tests {
    use crate::ns;

    #[test]
    fn basics() {
        let s = ns::str!(c"value");
        let data = ns::KeyedArchiver::archived_data_with_root_obj(s, false).unwrap();
        assert!(!data.is_empty());
    }
}
