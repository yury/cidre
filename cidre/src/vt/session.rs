use crate::{arc, cf, define_cf_type, os};

define_cf_type!(Session(cf::Type));

impl Session {
    #[inline]
    pub fn property_in(
        &self,
        key: &cf::String,
        allocator: Option<&cf::Allocator>,
    ) -> Result<Option<arc::R<cf::PropList>>, os::Status> {
        let mut value = None;
        unsafe { VTSessionCopyProperty(self, key, allocator, &mut value).to_result_option(value) }
    }

    #[inline]
    pub fn property(&self, key: &cf::String) -> Result<Option<arc::R<cf::PropList>>, os::Status> {
        self.property_in(key, None)
    }

    /// # Safety
    /// use `set_prop`
    #[inline]
    pub unsafe fn set_property(
        &mut self,
        key: &cf::String,
        value: Option<&cf::Type>,
    ) -> os::Status {
        VTSessionSetProperty(self, key, value)
    }

    #[inline]
    pub unsafe fn set_properties(&mut self, dict: &cf::Dictionary) -> os::Status {
        VTSessionSetProperties(self, dict)
    }

    #[inline]
    pub fn set_props(&mut self, props: &cf::Dictionary) -> Result<(), os::Status> {
        unsafe { self.set_properties(props).result() }
    }

    #[inline]
    pub fn set_prop(
        &mut self,
        key: &cf::String,
        value: Option<&cf::Type>,
    ) -> Result<(), os::Status> {
        unsafe { self.set_property(key, value).result() }
    }

    #[doc(alias = "VTSessionCopySupportedPropertyDictionary")]
    pub unsafe fn copy_supported_property_dictionary(
        &self,
        supported_property_dictionary_out: *mut Option<arc::R<cf::Dictionary>>,
    ) -> os::Status {
        VTSessionCopySupportedPropertyDictionary(self, supported_property_dictionary_out)
    }

    #[doc(alias = "VTSessionCopySupportedPropertyDictionary")]
    pub fn supported_props(&self) -> Result<arc::R<cf::Dictionary>, os::Status> {
        unsafe {
            let mut supported_property_dictionary_out = None;
            self.copy_supported_property_dictionary(&mut supported_property_dictionary_out)
                .to_result_unchecked(supported_property_dictionary_out)
        }
    }

    pub unsafe fn copy_serializable_properties(
        &self,
        allocator: Option<&cf::Allocator>,
        dictionary_out: *mut Option<arc::R<cf::Dictionary>>,
    ) -> os::Status {
        unsafe { VTSessionCopySerializableProperties(self, allocator, dictionary_out) }
    }

    pub fn serializable_properties_in(
        &self,
        allocator: Option<&cf::Allocator>,
    ) -> Result<Option<arc::R<cf::Dictionary>>, os::Status> {
        unsafe {
            let mut dictionary_out = None;
            let res = self.copy_serializable_properties(allocator, &mut dictionary_out);
            if res.is_ok() {
                Ok(dictionary_out)
            } else {
                Err(res)
            }
        }
    }

    pub fn serializable_properties(&self) -> Result<Option<arc::R<cf::Dictionary>>, os::Status> {
        self.serializable_properties_in(None)
    }
}

#[link(name = "VideoToolbox", kind = "framework")]
extern "C" {
    fn VTSessionSetProperty(
        session: &mut Session,
        property_key: &cf::String,
        property_value: Option<&cf::Type>,
    ) -> os::Status;

    fn VTSessionCopyProperty(
        session: &Session,
        property_key: &cf::String,
        allocator: Option<&cf::Allocator>,
        property_value_out: *mut Option<arc::R<cf::PropList>>,
    ) -> os::Status;

    fn VTSessionSetProperties(
        session: &mut Session,
        property_dictionary: &cf::Dictionary,
    ) -> os::Status;

    fn VTSessionCopySupportedPropertyDictionary(
        session: &Session,
        supported_property_dictionary_out: *mut Option<arc::R<cf::Dictionary>>,
    ) -> os::Status;

    fn VTSessionCopySerializableProperties(
        session: &Session,
        allocator: Option<&cf::Allocator>,
        dictionary_out: *mut Option<arc::R<cf::Dictionary>>,
    ) -> os::Status;
}
