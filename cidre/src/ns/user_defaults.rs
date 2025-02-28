use crate::{
    arc, define_obj_type, ns,
    objc::{self, Obj},
};

define_obj_type!(
    #[doc(alias = "NSUserDefaults")]
    pub UserDefaults(ns::Id),
    NS_USER_DEFAULTS
);

impl arc::A<UserDefaults> {
    #[objc::msg_send(initWithSuiteName:)]
    pub fn init_with_suite_name(self, name: Option<&ns::String>) -> Option<arc::R<UserDefaults>>;
}

impl UserDefaults {
    #[objc::msg_send(standardUserDefaults)]
    pub fn standard() -> arc::R<Self>;

    #[objc::msg_send(resetStandardUserDefaults)]
    pub fn reset();

    pub fn with_suite_name(name: Option<&ns::String>) -> Option<arc::R<Self>> {
        Self::alloc().init_with_suite_name(name)
    }

    #[objc::msg_send(objectForKey:)]
    pub fn get_obj(&self, key: &ns::String) -> Option<arc::R<ns::Id>>;

    #[objc::msg_send(setObject:forKey:)]
    pub fn set_obj_for_key(&mut self, val: Option<&ns::Id>, key: &ns::String);

    #[inline]
    pub fn set_obj(&mut self, key: &ns::String, val: Option<&ns::Id>) {
        self.set_obj_for_key(val, key)
    }

    #[objc::msg_send(stringForKey:)]
    pub fn get_string(&self, key: &ns::String) -> Option<arc::R<ns::String>>;

    #[objc::msg_send(arrayForKey:)]
    pub fn get_array(&self, key: &ns::String) -> Option<arc::R<ns::Array<ns::Id>>>;

    #[objc::msg_send(dictionaryForKey:)]
    pub fn get_dictionary(
        &self,
        key: &ns::String,
    ) -> Option<arc::R<ns::Dictionary<ns::String, ns::Id>>>;

    #[objc::msg_send(dataForKey:)]
    pub fn get_data(&self, key: &ns::String) -> Option<arc::R<ns::Data>>;

    #[objc::msg_send(stringArrayForKey:)]
    pub fn get_string_array(&self, key: &ns::String) -> Option<arc::R<ns::Array<ns::String>>>;

    #[objc::msg_send(integerForKey:)]
    pub fn get_integer(&self, key: &ns::String) -> ns::Integer;

    #[objc::msg_send(floatForKey:)]
    pub fn get_f32(&self, key: &ns::String) -> f32;

    #[objc::msg_send(doubleForKey:)]
    pub fn get_f64(&self, key: &ns::String) -> f64;

    #[objc::msg_send(boolForKey:)]
    pub fn get_bool(&self, key: &ns::String) -> bool;

    #[objc::msg_send(URLForKey:)]
    pub fn get_url(&self, key: &ns::String) -> Option<arc::R<ns::Url>>;

    #[objc::msg_send(setInteger:forKey:)]
    pub fn set_integer_for_key(&mut self, val: ns::Integer, key: &ns::String);

    #[inline]
    pub fn set_integer(&mut self, key: &ns::String, val: ns::Integer) {
        self.set_integer_for_key(val, key);
    }

    #[objc::msg_send(setFloat:forKey:)]
    pub fn set_f32_for_key(&mut self, val: f32, key: &ns::String);

    #[inline]
    pub fn set_f32(&mut self, key: &ns::String, val: f32) {
        self.set_f32_for_key(val, key);
    }

    #[objc::msg_send(setDouble:forKey:)]
    pub fn set_f64_for_key(&mut self, val: f64, key: &ns::String);

    #[inline]
    pub fn set_f64(&mut self, key: &ns::String, val: f64) {
        self.set_f64_for_key(val, key);
    }

    #[objc::msg_send(setURL:forKey:)]
    pub fn set_url_for_key(&mut self, val: &ns::Url, key: &ns::String);

    #[inline]
    pub fn set_url(&mut self, key: &ns::String, val: &ns::Url) {
        self.set_url_for_key(val, key);
    }

    pub fn set_u64(&mut self, key: &ns::String, val: u64) {
        let val = ns::Number::with_u64(val);
        self.set_obj(key, Some(&val));
    }

    pub fn get_u64(&self, key: &ns::String) -> u64 {
        let Some(obj) = self.get_obj(key) else {
            return 0;
        };

        let Some(num) = obj.try_cast(ns::Number::cls()) else {
            return 0;
        };

        num.as_u64()
    }
}

impl ns::KvObserverRegistration for UserDefaults {}

unsafe impl Send for UserDefaults {}
unsafe impl Sync for UserDefaults {}

#[link(name = "ns", kind = "static")]
unsafe extern "C" {
    static NS_USER_DEFAULTS: &'static objc::Class<UserDefaults>;
}

#[cfg(test)]
mod tests {
    use crate::ns;

    #[test]
    fn basics() {
        let mut ud = ns::UserDefaults::standard();
        let key = ns::str!(c"unknown");

        assert!(ud.get_obj(key).is_none());
        assert!(ud.get_string(key).is_none());
        assert!(ud.get_array(key).is_none());
        assert!(ud.get_data(key).is_none());
        assert!(ud.get_string_array(key).is_none());
        assert!(ud.get_url(key).is_none());

        assert_eq!(ud.get_bool(key), false);
        assert_eq!(ud.get_f32(key), 0.0);
        assert_eq!(ud.get_f64(key), 0.0);
        assert_eq!(ud.get_integer(key), 0);

        let key = ns::str!(c"u64");
        ud.set_u64(key, u64::MAX);
        assert_eq!(ud.get_u64(key), u64::MAX);
        ud.set_u64(key, u64::MIN);
        assert_eq!(ud.get_u64(key), u64::MIN);
    }
}
