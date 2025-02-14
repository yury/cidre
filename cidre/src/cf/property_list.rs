use std::mem::transmute;

use crate::{arc, cf};

impl cf::Plist {
    pub unsafe fn from_data_err_in(
        data: &cf::Data,
        flags: cf::PlistMutabilityOpts,
        format: *mut cf::PlistFormat,
        err: *mut arc::R<cf::Error>,
        allocator: Option<&cf::Allocator>,
    ) -> Option<arc::R<cf::Plist>> {
        unsafe { CFPropertyListCreateWithData(allocator, data, flags, format, err) }
    }

    pub unsafe fn from_data_err(
        data: &cf::Data,
        flags: cf::PlistMutabilityOpts,
        format: *mut cf::PlistFormat,
        err: *mut arc::R<cf::Error>,
    ) -> Option<arc::R<cf::Plist>> {
        unsafe { Self::from_data_err_in(data, flags, format, err, None) }
    }

    pub fn from_data(
        data: &cf::Data,
        flags: cf::PlistMutabilityOpts,
    ) -> Result<arc::R<cf::Plist>, arc::R<cf::Error>> {
        cf::if_none(|err| unsafe { Self::from_data_err(data, flags, std::ptr::null_mut(), err) })
    }

    pub fn from_data_with_format(
        data: &cf::Data,
        flags: cf::PlistMutabilityOpts,
    ) -> Result<(arc::R<cf::Plist>, cf::PlistFormat), arc::R<cf::Error>> {
        let mut format = cf::PlistFormat::OpenStep;
        let res = cf::if_none(|err| unsafe { Self::from_data_err(data, flags, &mut format, err) });
        res.map(|r| (r, format))
    }

    pub unsafe fn to_cf_data_err_in(
        &self,
        format: cf::PlistFormat,
        err: *mut arc::R<cf::Error>,
        allocator: Option<&cf::Allocator>,
    ) -> Option<arc::R<cf::Data>> {
        unsafe { CFPropertyListCreateData(allocator, self, format, Default::default(), err) }
    }

    pub unsafe fn to_cf_data_err(
        &self,
        format: cf::PlistFormat,
        err: *mut arc::R<cf::Error>,
    ) -> Option<arc::R<cf::Data>> {
        unsafe { CFPropertyListCreateData(None, self, format, Default::default(), err) }
    }

    pub fn to_cf_data(
        &self,
        format: cf::PlistFormat,
    ) -> Result<arc::R<cf::Data>, Option<arc::R<cf::Error>>> {
        cf::if_none_maybe(|err| unsafe {
            CFPropertyListCreateData(None, self, format, Default::default(), err)
        })
    }
}

/// Type to mean any instance of a property list type;
/// currently, cf::String, cf::Data, cf::Number, cf::Boolean, cf::Date,
/// cf::Array, and cf::Dictionary.
impl cf::Plist {
    pub fn try_as_string(&self) -> Option<&crate::cf::String> {
        if self.get_type_id() == crate::cf::String::type_id() {
            Some(unsafe { transmute(self) })
        } else {
            None
        }
    }

    pub fn as_string(&self) -> &crate::cf::String {
        assert!(self.get_type_id() == crate::cf::String::type_id());
        unsafe { transmute(self) }
    }

    pub fn try_as_data(&self) -> Option<&crate::cf::Data> {
        if self.get_type_id() == crate::cf::Data::type_id() {
            Some(unsafe { transmute(self) })
        } else {
            None
        }
    }

    pub fn as_data(&self) -> &crate::cf::Data {
        assert!(self.get_type_id() == crate::cf::Data::type_id());
        unsafe { transmute(self) }
    }

    pub fn try_as_number(&self) -> Option<&crate::cf::Number> {
        if self.get_type_id() == crate::cf::Number::type_id() {
            Some(unsafe { transmute(self) })
        } else {
            None
        }
    }

    pub fn as_number(&self) -> &crate::cf::Number {
        assert!(self.get_type_id() == crate::cf::Number::type_id());
        unsafe { transmute(self) }
    }

    pub fn try_as_boolean(&self) -> Option<&crate::cf::Boolean> {
        if self.get_type_id() == crate::cf::Boolean::type_id() {
            Some(unsafe { transmute(self) })
        } else {
            None
        }
    }

    pub fn as_boolean(&self) -> &crate::cf::Boolean {
        debug_assert!(self.get_type_id() == crate::cf::Boolean::type_id());
        unsafe { transmute(self) }
    }

    pub fn try_as_date(&self) -> Option<&crate::cf::Date> {
        if self.get_type_id() == crate::cf::Date::type_id() {
            Some(unsafe { transmute(self) })
        } else {
            None
        }
    }

    pub fn as_date(&self) -> &crate::cf::Date {
        assert!(self.get_type_id() == crate::cf::Date::type_id());
        unsafe { transmute(self) }
    }

    pub fn try_as_raw_array(&self) -> Option<&crate::cf::Array> {
        if self.get_type_id() == crate::cf::Array::type_id() {
            Some(unsafe { transmute(self) })
        } else {
            None
        }
    }

    pub fn as_raw_array(&self) -> &crate::cf::Array {
        assert!(self.get_type_id() == crate::cf::Array::type_id());
        unsafe { transmute(self) }
    }

    pub fn try_as_array(&self) -> Option<&crate::cf::ArrayOf<cf::Plist>> {
        if self.get_type_id() == crate::cf::Array::type_id() {
            Some(unsafe { transmute(self) })
        } else {
            None
        }
    }

    pub fn as_array(&self) -> &crate::cf::ArrayOf<cf::Plist> {
        assert!(self.get_type_id() == crate::cf::Array::type_id());
        unsafe { transmute(self) }
    }

    pub fn try_as_raw_dictionary(&self) -> Option<&crate::cf::Dictionary> {
        if self.get_type_id() == crate::cf::Dictionary::type_id() {
            Some(unsafe { transmute(self) })
        } else {
            None
        }
    }

    pub fn as_raw_dictionary(&self) -> &crate::cf::Dictionary {
        assert!(self.get_type_id() == crate::cf::Dictionary::type_id());
        unsafe { transmute(self) }
    }

    pub fn try_as_dictionary(&self) -> Option<&crate::cf::DictionaryOf<cf::String, cf::Plist>> {
        if self.get_type_id() == crate::cf::Dictionary::type_id() {
            Some(unsafe { transmute(self) })
        } else {
            None
        }
    }

    pub fn as_dictionary(&self) -> &crate::cf::DictionaryOf<cf::String, cf::Plist> {
        assert!(self.get_type_id() == crate::cf::Dictionary::type_id());
        unsafe { transmute(self) }
    }

    #[inline]
    pub fn is_valid_for_format(&self, format: cf::PlistFormat) -> bool {
        unsafe { CFPropertyListIsValid(self, format) }
    }

    #[inline]
    pub fn deep_copy_in(
        &self,
        options: cf::PlistMutabilityOpts,
        alloc: Option<&cf::Allocator>,
    ) -> Option<arc::R<Self>> {
        unsafe { CFPropertyListCreateDeepCopy(alloc, self, options) }
    }

    #[inline]
    pub fn deep_copy(&self, options: cf::PlistMutabilityOpts) -> Option<arc::R<Self>> {
        unsafe { CFPropertyListCreateDeepCopy(None, self, options) }
    }
}

impl From<&cf::String> for &cf::Plist {
    fn from(value: &cf::String) -> Self {
        unsafe { std::mem::transmute(value) }
    }
}

impl From<arc::R<cf::String>> for arc::R<cf::Plist> {
    fn from(value: arc::R<cf::String>) -> Self {
        unsafe { std::mem::transmute(value) }
    }
}

impl From<&cf::Data> for &cf::Plist {
    fn from(value: &cf::Data) -> Self {
        unsafe { std::mem::transmute(value) }
    }
}

impl From<arc::R<cf::Data>> for arc::R<cf::Plist> {
    fn from(value: arc::R<cf::Data>) -> Self {
        unsafe { std::mem::transmute(value) }
    }
}

impl From<&cf::Number> for &cf::Plist {
    fn from(value: &cf::Number) -> Self {
        unsafe { std::mem::transmute(value) }
    }
}

impl From<arc::R<cf::Number>> for arc::R<cf::Plist> {
    fn from(value: arc::R<cf::Number>) -> Self {
        unsafe { std::mem::transmute(value) }
    }
}

impl From<&cf::Boolean> for &cf::Plist {
    fn from(value: &cf::Boolean) -> Self {
        unsafe { std::mem::transmute(value) }
    }
}

impl From<arc::R<cf::Boolean>> for arc::R<cf::Plist> {
    fn from(value: arc::R<cf::Boolean>) -> Self {
        unsafe { std::mem::transmute(value) }
    }
}

impl<'a, T: arc::Retain> From<&'a cf::DictionaryOf<cf::String, T>> for &'a cf::Plist
where
    &'a T: Into<&'a cf::Plist>,
{
    fn from(value: &'a cf::DictionaryOf<cf::String, T>) -> Self {
        unsafe { std::mem::transmute(value) }
    }
}

impl<'a, T: arc::Retain> From<&'a cf::ArrayOf<T>> for &'a cf::Plist
where
    &'a T: Into<&'a cf::Plist>,
{
    fn from(value: &'a cf::ArrayOf<T>) -> Self {
        unsafe { std::mem::transmute(value) }
    }
}

#[link(name = "CoreFoundation", kind = "framework")]
extern "C-unwind" {
    fn CFPropertyListCreateWithData(
        allocator: Option<&cf::Allocator>,
        data: &cf::Data,
        flags: cf::PlistMutabilityOpts,
        format: *mut cf::PlistFormat,
        err: *mut arc::R<cf::Error>,
    ) -> Option<arc::R<cf::Plist>>;

    fn CFPropertyListCreateData(
        allocator: Option<&cf::Allocator>,
        prop_list: &cf::Plist,
        format: cf::PlistFormat,
        options: cf::OptionFlags,
        err: *mut arc::R<cf::Error>,
    ) -> Option<arc::R<cf::Data>>;

    fn CFPropertyListIsValid(plist: &cf::Plist, format: cf::PlistFormat) -> bool;

    fn CFPropertyListCreateDeepCopy(
        allocator: Option<&cf::Allocator>,
        plist: &cf::Plist,
        options: cf::PlistMutabilityOpts,
    ) -> Option<arc::R<cf::Plist>>;
}

#[cfg(test)]
mod tests {
    use crate::cf;

    #[test]
    fn basics() {
        let num = cf::Number::from_i16(10);
        let prop_list_val: &cf::Plist = num.as_ref().into();
        let data = prop_list_val.to_cf_data(cf::PlistFormat::XmlV1_0).unwrap();

        assert!(!data.is_empty());

        let list = cf::Plist::from_data(&data, Default::default()).unwrap();
        assert_eq!(list.as_number().to_i32().unwrap(), 10);

        let (list, format) = cf::Plist::from_data_with_format(&data, Default::default()).unwrap();

        assert_eq!(list.as_number().to_i32().unwrap(), 10);
        assert_eq!(format, cf::PlistFormat::XmlV1_0);

        let dict = cf::DictionaryOf::with_keys_values(&[cf::str!(c"key")], &[num.as_ref()]);

        let prop_list: &cf::Plist = dict.as_ref().into();

        assert!(prop_list.is_valid_for_format(cf::PlistFormat::BinaryV1_0));
        assert!(prop_list.is_valid_for_format(cf::PlistFormat::XmlV1_0));
        assert!(!prop_list.is_valid_for_format(cf::PlistFormat::OpenStep));

        let _err = prop_list
            .to_cf_data(cf::PlistFormat::OpenStep)
            .expect_err("can't write in OpenStep format");

        let data = prop_list.to_cf_data(cf::PlistFormat::XmlV1_0).unwrap();
        assert!(!data.is_empty());

        let new_prop_list = cf::Plist::from_data(&data, Default::default()).unwrap();
        assert!(prop_list.equal(&new_prop_list));
        let dict = new_prop_list.as_dictionary();
        assert_eq!(dict.len(), 1);

        let _deep_copy = new_prop_list.deep_copy(Default::default()).unwrap();
    }
}
