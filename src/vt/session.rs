use crate::{
    cf,
    os::{self, NO_ERR},
};

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
    pub unsafe fn set_properties(&mut self, dict: &cf::Dictionary) -> os::Status {
        VTSessionSetProperties(self, dict)
    }

    #[inline]
    pub fn set_props(&mut self, props: &cf::Dictionary) -> Result<(), os::Status> {
        let res = unsafe { self.set_properties(props) };
        match res {
            NO_ERR => Ok(()),
            _ => Err(res),
        }
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
    pub fn set_prop(
        &mut self,
        key: &cf::String,
        value: Option<&cf::Type>,
    ) -> Result<(), os::Status> {
        let res = unsafe { self.set_property(key, value) };
        match res {
            NO_ERR => Ok(()),
            _ => Err(res),
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
