use crate::{arc, define_cls, define_obj_type, ns, objc};

#[derive(Eq, PartialEq, Debug, Copy, Clone)]
#[repr(isize)]
pub enum CaptureDestination {
    DeveloperTools = 1,
    GPUTraceDocument,
}

define_obj_type!(CaptureDescriptor(ns::Id), MTL_CAPTURE_DESCRIPTOR);

impl CaptureDescriptor {
    #[objc::msg_send(captureObject)]
    pub fn capture_object(&self) -> Option<&ns::Id>;

    #[objc::msg_send(setCaptureObject:)]
    pub fn set_capture_object(&mut self, value: &ns::Id);

    #[objc::msg_send(destination)]
    pub fn dst(&self) -> CaptureDestination;

    #[objc::msg_send(setDestination:)]
    pub fn set_dst(&self, value: CaptureDestination);
}

define_obj_type!(CaptureManager(ns::Id));

impl CaptureManager {
    define_cls!(MTL_CAPTURE_MANAGER);

    #[objc::cls_msg_send(sharedCaptureManager)]
    pub fn shared() -> &'static mut Self;

    //- (BOOL)startCaptureWithDescriptor:(MTLCaptureDescriptor *)descriptor error:(__autoreleasing NSError **)error API_AVAILABLE(macos(10.15), ios(13.0));
    #[objc::msg_send(startCaptureWithDescriptor:error:)]
    pub unsafe fn start_with_descriptor_err<'ar>(
        &mut self,
        desc: &CaptureDescriptor,
        error: *mut Option<&'ar ns::Error>,
    ) -> bool;

    #[inline]
    pub fn start<'ar>(&mut self, desc: &CaptureDescriptor) -> Result<(), &'ar ns::Error> {
        let mut error = None;
        unsafe {
            if self.start_with_descriptor_err(desc, &mut error) {
                Ok(())
            } else {
                Err(error.unwrap_unchecked())
            }
        }
    }

    #[objc::msg_send(stopCapture)]
    pub fn stop(&mut self);

    #[objc::msg_send(isCapturing)]
    pub fn is_capturing(&self) -> bool;
}

#[link(name = "mtl", kind = "static")]
extern "C" {
    static MTL_CAPTURE_DESCRIPTOR: &'static objc::Class<CaptureDescriptor>;
    static MTL_CAPTURE_MANAGER: &'static objc::Class<CaptureManager>;
}
