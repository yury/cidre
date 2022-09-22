use crate::{
    cf::{self, Retained},
    define_cf_type, os,
};

define_cf_type!(Session(cf::Type));

impl Session {
    #[inline]
    pub fn property(
        &self,
        key: &cf::String,
        allocator: Option<&cf::Allocator>,
    ) -> Option<cf::Retained<cf::Type>> {
        unsafe { VTSessionCopyProperty(self, key, allocator) }
    }

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

    pub unsafe fn copy_supported_property_dictionary(
        &self,
        supported_property_dictionary_out: &mut Option<Retained<cf::Dictionary>>,
    ) -> os::Status {
        VTSessionCopySupportedPropertyDictionary(self, supported_property_dictionary_out)
    }

    pub fn supported_properties(&self) -> Result<Retained<cf::Dictionary>, os::Status> {
        unsafe {
            let mut supported_property_dictionary_out = None;
            self.copy_supported_property_dictionary(&mut supported_property_dictionary_out)
                .to_result_unchecked(supported_property_dictionary_out)
        }
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
    ) -> Option<cf::Retained<cf::Type>>;

    fn VTSessionSetProperties(
        session: &mut Session,
        property_dictionary: &cf::Dictionary,
    ) -> os::Status;

    fn VTSessionCopySupportedPropertyDictionary(
        session: &Session,
        supported_property_dictionary_out: &mut Option<Retained<cf::Dictionary>>,
    ) -> os::Status;
}
