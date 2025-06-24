use crate::{api, arc, cv, define_obj_type, ml, ns, objc};

define_obj_type!(
    #[doc(alias = "MLFeatureValue")]
    pub FeatureValue(ns::Id),
    ML_FEATURE_VALUE,
    #[api::available(macos = 10.13, ios = 11.0, watchos = 4.0, tvos = 11.0)]
);

impl FeatureValue {
    #[objc::msg_send(type)]
    pub fn type_(&self) -> ml::FeatureType;

    #[objc::msg_send(isUndefined)]
    pub fn is_undefined(&self) -> bool;

    #[objc::msg_send(int64Value)]
    pub fn i64(&self) -> i64;

    #[objc::msg_send(doubleValue)]
    pub fn f64(&self) -> f64;

    #[objc::msg_send(stringValue)]
    pub fn string(&self) -> Option<arc::R<ns::String>>;

    #[objc::msg_send(dictionaryValue)]
    pub fn dictionary(&self) -> Option<ns::Dictionary<ns::Id, ns::Number>>;

    #[objc::msg_send(imageBufferValue)]
    pub fn image_buf(&self) -> Option<&cv::ImageBuf>;

    #[objc::msg_send(featureValueWithInt64:)]
    pub fn with_i64(val: i64) -> arc::R<Self>;

    #[objc::msg_send(featureValueWithDouble:)]
    pub fn with_f64(val: f64) -> arc::R<Self>;

    #[objc::msg_send(featureValueWithString:)]
    pub fn with_ns_string(val: &ns::String) -> arc::R<Self>;

    pub fn with_string<S: AsRef<ns::String>>(val: S) -> arc::R<Self> {
        Self::with_ns_string(val.as_ref())
    }

    #[objc::msg_send(featureValueWithPixelBuffer:)]
    pub fn with_pixel_buf(val: &cv::PixelBuf) -> arc::R<Self>;

    #[objc::msg_send(undefinedFeatureValueWithType:)]
    pub fn undefined_with_type(type_: ml::FeatureType) -> arc::R<Self>;

    #[objc::msg_send(featureValueWithDictionary:error:)]
    pub fn with_dictionary_err<'ear>(
        val: &ns::Dictionary<ns::Id, ns::Number>,
        err: *mut Option<&'ear ns::Error>,
    ) -> Option<arc::R<Self>>;

    pub fn with_dictionary<'ear>(
        val: &ns::Dictionary<ns::Id, ns::Number>,
    ) -> ns::Result<'ear, arc::R<Self>> {
        ns::if_none(|err| Self::with_dictionary_err(val, err))
    }

    #[objc::msg_send(isEqualToFeatureValue:)]
    pub fn is_equal_to_feature_value(&self, val: &Self) -> bool;
}

#[link(name = "ml", kind = "static")]
unsafe extern "C" {
    static ML_FEATURE_VALUE: &'static objc::Class<FeatureValue>;
}

#[cfg(test)]
mod test {

    use crate::{ml, ns};

    #[test]
    fn basics() {
        let fval = ml::FeatureValue::new();
        assert_eq!(ml::FeatureType::Invalid, fval.type_());
        assert_eq!(0, fval.i64());
        assert_eq!(0.0, fval.f64());
        assert_eq!(None, fval.string());
        assert_eq!(true, fval.dictionary().is_none());
        assert_eq!(true, fval.image_buf().is_none());

        let fval = ml::FeatureValue::with_string(ns::str!(c"Hello"));
        assert_eq!(true, fval.string().is_some());

        let fval = ml::FeatureValue::with_dictionary(&ns::Dictionary::new()).unwrap();
        assert_eq!(true, fval.dictionary().is_some());
    }
}
