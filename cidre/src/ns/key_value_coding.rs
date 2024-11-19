use crate::{arc, ns, objc};

impl ns::Id {
    #[objc::msg_send(valueForKey:)]
    pub unsafe fn value_for_key_throws(&self, key: &ns::String) -> Option<arc::R<Self>>;

    pub fn value_for_key<'ear>(
        &self,
        key: &ns::String,
    ) -> ns::ExResult<'ear, Option<arc::R<Self>>> {
        ns::try_catch(|| unsafe { self.value_for_key_throws(key) })
    }

    #[objc::msg_send(setValue:forKey:)]
    pub unsafe fn set_value_for_key_throws(&mut self, val: Option<&Self>, key: &ns::String);

    pub fn set_value_for_key<'ear>(
        &mut self,
        val: Option<&Self>,
        key: &ns::String,
    ) -> ns::ExResult<'ear> {
        ns::try_catch(|| unsafe { self.set_value_for_key_throws(val, key) })
    }

    #[objc::msg_send(valueForKeyPath:)]
    pub unsafe fn value_for_key_path_throws(&self, key_path: &ns::String) -> Option<arc::R<Self>>;

    pub fn value_for_key_path<'ear>(
        &self,
        key_path: &ns::String,
    ) -> ns::ExResult<'ear, Option<arc::R<Self>>> {
        ns::try_catch(|| unsafe { self.value_for_key_path_throws(key_path) })
    }

    #[objc::msg_send(setValue:forKeyPath:)]
    pub unsafe fn set_value_for_key_path_throws(
        &mut self,
        val: Option<&Self>,
        key_path: &ns::String,
    );

    pub fn set_value_for_key_path<'ear>(
        &mut self,
        val: Option<&Self>,
        key_path: &ns::String,
    ) -> ns::ExResult<'ear> {
        ns::try_catch(|| unsafe { self.set_value_for_key_path_throws(val, key_path) })
    }
}

#[cfg(test)]
mod tests {
    use crate::{ns, objc::Obj};

    #[test]
    fn basics() {
        let s = ns::str!(c"nice");
        let v = s.value_for_key(ns::str!(c"length")).expect("should be ok");
        let v = v.expect("number");
        let num = v.try_cast(ns::Number::cls()).unwrap();
        assert_eq!(num.as_i8(), 4);

        let _ = s
            .value_for_key(ns::str!(c"invalid_key"))
            .expect_err("should be ns::Exception");
    }
}
