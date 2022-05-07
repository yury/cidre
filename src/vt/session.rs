use crate::{cf, os};

pub type Session = cf::Type;

impl Session {
    #[inline]
    pub fn copy_property<'a>(
        &self,
        key: &cf::String,
        allocator: Option<&cf::Allocator>,
    ) -> Option<cf::Retained<'a, cf::Type>> {
        unsafe { VTSessionCopyProperty(self, key, allocator) }
    }

    #[inline]
    pub fn set_properties(&mut self, dict: &cf::Dictionary) -> os::Status {
        unsafe { VTSessionSetProperties(self, dict) }
    }

    #[inline]
    pub fn set_property(&mut self, key: &cf::String, value: Option<&cf::Type>) -> os::Status {
        unsafe { VTSessionSetProperty(self, key, value) }
    }
}

#[link(name = "VideoToolbox", kind = "framework")]
extern "C" {
    fn VTSessionSetProperty(
        session: &mut Session,
        property_key: &cf::String,
        property_value: Option<&cf::Type>,
    ) -> os::Status;

    fn VTSessionCopyProperty<'a>(
        session: &Session,
        property_key: &cf::String,
        allocator: Option<&cf::Allocator>,
    ) -> Option<cf::Retained<'a, cf::Type>>;

    fn VTSessionSetProperties(
        session: &mut Session,
        property_dictionary: &cf::Dictionary,
    ) -> os::Status;
}
