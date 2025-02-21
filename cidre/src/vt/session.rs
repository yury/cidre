use crate::{arc, cf, define_cf_type, os};

define_cf_type!(Session(cf::Type));

impl Session {
    #[inline]
    pub fn property_in(
        &self,
        key: &cf::String,
        allocator: Option<&cf::Allocator>,
    ) -> os::Result<Option<arc::R<cf::Plist>>> {
        let mut value = None;
        unsafe { VTSessionCopyProperty(self, key, allocator, &mut value).result()? };
        Ok(value)
    }

    #[inline]
    pub fn property(&self, key: &cf::String) -> os::Result<Option<arc::R<cf::Plist>>> {
        self.property_in(key, None)
    }

    /// # Safety
    /// use `set_prop`
    #[inline]
    pub unsafe fn set_property(
        &mut self,
        key: &cf::String,
        value: Option<&cf::Type>,
    ) -> os::Result {
        unsafe { VTSessionSetProperty(self, key, value).result() }
    }

    #[inline]
    pub unsafe fn set_properties(&mut self, dict: &cf::Dictionary) -> os::Result {
        unsafe { VTSessionSetProperties(self, dict).result() }
    }

    #[inline]
    pub fn set_props(&mut self, props: &cf::Dictionary) -> os::Result {
        unsafe { self.set_properties(props) }
    }

    #[doc(alias = "VTSessionSetProperty")]
    #[inline]
    pub fn set_prop(&mut self, key: &cf::String, value: Option<&cf::Type>) -> os::Result {
        unsafe { self.set_property(key, value) }
    }

    #[doc(alias = "VTSessionCopySupportedPropertyDictionary")]
    pub unsafe fn copy_supported_property_dictionary(
        &self,
        supported_property_dictionary_out: *mut Option<arc::R<cf::Dictionary>>,
    ) -> os::Result {
        unsafe {
            VTSessionCopySupportedPropertyDictionary(self, supported_property_dictionary_out)
                .result()
        }
    }

    #[doc(alias = "VTSessionCopySupportedPropertyDictionary")]
    pub fn supported_props(&self) -> os::Result<arc::R<cf::Dictionary>> {
        unsafe { os::result_unchecked(|res| self.copy_supported_property_dictionary(res)) }
    }

    pub unsafe fn copy_serializable_properties(
        &self,
        allocator: Option<&cf::Allocator>,
        dictionary_out: *mut Option<arc::R<cf::Dictionary>>,
    ) -> os::Result {
        unsafe { VTSessionCopySerializableProperties(self, allocator, dictionary_out).result() }
    }

    pub fn serializable_properties_in(
        &self,
        allocator: Option<&cf::Allocator>,
    ) -> os::Result<Option<arc::R<cf::Dictionary>>> {
        unsafe {
            let mut dictionary_out = None;
            self.copy_serializable_properties(allocator, &mut dictionary_out)?;
            Ok(dictionary_out)
        }
    }

    pub fn serializable_properties(&self) -> os::Result<Option<arc::R<cf::Dictionary>>> {
        self.serializable_properties_in(None)
    }
}

#[link(name = "VideoToolbox", kind = "framework")]
unsafe extern "C-unwind" {
    fn VTSessionSetProperty(
        session: &mut Session,
        property_key: &cf::String,
        property_value: Option<&cf::Type>,
    ) -> os::Status;

    fn VTSessionCopyProperty(
        session: &Session,
        property_key: &cf::String,
        allocator: Option<&cf::Allocator>,
        property_value_out: *mut Option<arc::R<cf::Plist>>,
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
