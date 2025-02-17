use crate::{
    arc, cf, cv, define_cf_type, os, vt,
    vt::pixel_transfer_properties::{keys, scaling_mode},
};

define_cf_type!(
    #[doc(alias = "VTPixelTransferSessionRef")]
    Session(vt::Session)
);

impl Session {
    #[doc(alias = "VTPixelTransferSessionGetTypeID")]
    #[inline]
    pub fn type_id() -> cf::TypeId {
        unsafe { VTPixelTransferSessionGetTypeID() }
    }

    #[doc(alias = "VTPixelTransferSessionCreate")]
    #[inline]
    pub fn new() -> os::Result<arc::R<Self>> {
        Self::new_in(None)
    }

    #[doc(alias = "VTPixelTransferSessionCreate")]
    #[inline]
    pub fn new_in(allocator: Option<&cf::Allocator>) -> os::Result<arc::R<Self>> {
        unsafe { os::result_unchecked(|res| VTPixelTransferSessionCreate(allocator, res)) }
    }

    pub fn set_realtime(&mut self, value: bool) -> os::Result {
        let value: &'static cf::Boolean = value.into();
        self.set_prop(keys::real_time(), Some(value))
    }

    #[inline]
    pub fn set_scaling_normal(&mut self) -> os::Result {
        self.set_prop(keys::scaling_mode(), Some(scaling_mode::normal()))
    }

    #[inline]
    pub fn set_scaling_crop_src_to_clean_aperture(&mut self) -> os::Result {
        self.set_prop(
            keys::scaling_mode(),
            Some(scaling_mode::crop_src_to_clean_aperture()),
        )
    }

    #[inline]
    pub fn set_scaling_letter_box(&mut self) -> os::Result {
        self.set_prop(keys::scaling_mode(), Some(scaling_mode::letter_box()))
    }

    #[inline]
    pub fn set_scaling_trim(&mut self) -> os::Result {
        self.set_prop(keys::scaling_mode(), Some(scaling_mode::trim()))
    }

    #[doc(alias = "VTPixelTransferSessionInvalidate")]
    #[inline]
    pub fn invalidate(&mut self) {
        unsafe { VTPixelTransferSessionInvalidate(self) }
    }

    #[doc(alias = "VTPixelTransferSessionTransferImage")]
    #[inline]
    pub fn transfer(&self, src: &cv::PixelBuf, dst: &cv::PixelBuf) -> os::Result {
        unsafe { VTPixelTransferSessionTransferImage(self, src, dst).result() }
    }
}

extern "C-unwind" {
    fn VTPixelTransferSessionGetTypeID() -> cf::TypeId;
    fn VTPixelTransferSessionCreate(
        allocator: Option<&cf::Allocator>,
        pixel_transfer_session_out: *mut Option<arc::R<Session>>,
    ) -> os::Status;
    fn VTPixelTransferSessionInvalidate(session: &mut Session);
    fn VTPixelTransferSessionTransferImage(
        session: &Session,
        source_buffer: &cv::PixelBuf,
        destination_buffer: &cv::PixelBuf,
    ) -> os::Status;
}
