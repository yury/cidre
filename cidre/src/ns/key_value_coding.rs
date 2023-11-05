use crate::{arc, ns, objc};

impl ns::Id {
    #[objc::msg_send(valueForKey:)]
    pub fn value_for_key_throws_ar(&self, key: &ns::String) -> Option<arc::Rar<Self>>;

    #[objc::rar_retain]
    pub fn value_for_key_throws(&self, key: &ns::String) -> Option<arc::R<Self>>;

    pub fn value_for_key<'ar>(
        &self,
        key: &ns::String,
    ) -> Result<Option<arc::R<Self>>, &'ar ns::Exception> {
        ns::try_catch(|| self.value_for_key_throws(key))
    }

    #[objc::msg_send(setValue:forKey:)]
    pub fn set_value_for_key_throws(&mut self, val: Option<&Self>, key: &ns::String);

    pub fn set_value_for_key<'ar>(
        &mut self,
        val: Option<&Self>,
        key: &ns::String,
    ) -> Result<(), &'ar ns::Exception> {
        ns::try_catch(|| self.set_value_for_key_throws(val, key))
    }

    #[objc::msg_send(valueForKeyPath:)]
    pub fn value_for_key_path_throws_ar(&self, key_path: &ns::String) -> Option<arc::Rar<Self>>;

    #[objc::rar_retain]
    pub fn value_for_key_path_throws(&self, key_path: &ns::String) -> Option<arc::R<Self>>;

    pub fn value_for_key_path<'ar>(
        &self,
        key_path: &ns::String,
    ) -> Result<Option<arc::R<Self>>, &'ar ns::Exception> {
        ns::try_catch(|| self.value_for_key_path_throws(key_path))
    }

    #[objc::msg_send(setValue:forKeyPath:)]
    pub fn set_value_for_key_path_throws(&mut self, val: Option<&Self>, key_path: &ns::String);

    pub fn set_value_for_key_path<'ar>(
        &mut self,
        val: Option<&Self>,
        key_path: &ns::String,
    ) -> Result<(), &'ar ns::Exception> {
        ns::try_catch(|| self.set_value_for_key_path_throws(val, key_path))
    }
}

#[cfg(test)]
mod tests {
    use crate::{ns, objc::Obj};

    #[test]
    fn basics() {
        let s = ns::String::with_str("nice");
        let v = s
            .value_for_key(ns::String::with_str("length").as_ref())
            .expect("should be ok");
        let v = v.expect("number");
        let num = v.try_cast(ns::Number::cls()).unwrap();
        assert_eq!(num.as_i8(), 4);

        let _ = s
            .value_for_key(ns::String::with_str("invalid_key").as_ref())
            .expect_err("should be ns::Exception");
    }
}
