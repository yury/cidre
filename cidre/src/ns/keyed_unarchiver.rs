use crate::{arc, define_cls, define_obj_type, ns, objc};

define_obj_type!(
    #[doc(alias = "NSKeyedUnarchiver")]
    pub KeyedUnarchiver(ns::Coder)
);

impl KeyedUnarchiver {
    define_cls!(NS_KEYED_UNARCHIVER);

    #[objc::cls_msg_send(unarchivedObjectOfClass:fromData:error:)]
    pub unsafe fn unarchived_obj_of_cls_err_ar<'a, 'ear, T: objc::Obj>(
        cls: &'a objc::Class<T>,
        data: &'a ns::Data,
        err: *mut Option<&'ear ns::Error>,
    ) -> Option<arc::Rar<T>>;

    #[objc::cls_rar_retain]
    pub unsafe fn unarchived_obj_of_cls_err<'a, 'ear, T: objc::Obj>(
        cls: &'a objc::Class<T>,
        data: &'a ns::Data,
        err: *mut Option<&'ear ns::Error>,
    ) -> Option<arc::R<T>>;

    #[inline]
    pub fn unarchived_obj_of_cls<'a, 'ear, T: objc::Obj>(
        cls: &'a objc::Class<T>,
        data: &'a ns::Data,
    ) -> Result<arc::R<T>, &'ear ns::Error> {
        ns::if_none(|err| unsafe { Self::unarchived_obj_of_cls_err(cls, data, err) })
    }
}

#[link(name = "ns", kind = "static")]
extern "C" {
    static NS_KEYED_UNARCHIVER: &'static objc::Class<KeyedUnarchiver>;
}

#[cfg(test)]
mod tests {
    use crate::ns;

    #[test]
    fn basics() {
        let a = ns::String::with_str("data");
        let data = ns::KeyedArchiver::archived_data_with_root_obj(&a, true).unwrap();
        let b = ns::KeyedUnarchiver::unarchived_obj_of_cls(ns::String::cls(), &data).unwrap();
        assert_eq!(&a, &b);
    }
}
