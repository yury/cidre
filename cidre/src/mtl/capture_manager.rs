use crate::{arc, define_cls, define_obj_type, ns, objc};

#[doc(alias = "MTLCaptureDestination")]
#[derive(Eq, PartialEq, Debug, Copy, Clone)]
#[repr(isize)]
pub enum CaptureDst {
    DeveloperTools = 1,
    GpuTraceDocument,
}

define_obj_type!(pub CaptureDesc(ns::Id), MTL_CAPTURE_DESCRIPTOR);

impl CaptureDesc {
    #[objc::msg_send(captureObject)]
    pub fn capture_object(&self) -> Option<arc::R<ns::Id>>;

    #[objc::msg_send(setCaptureObject:)]
    pub fn set_capture_object(&mut self, val: &ns::Id);

    #[objc::msg_send(destination)]
    pub fn dst(&self) -> CaptureDst;

    #[objc::msg_send(setDestination:)]
    pub fn set_dst(&self, val: CaptureDst);

    #[objc::msg_send(outputURL)]
    pub fn output_url(&self) -> Option<arc::R<ns::Url>>;

    #[objc::msg_send(setOutputURL:)]
    pub fn set_output_url(&mut self, val: Option<&ns::Url>);
}

define_obj_type!(pub CaptureManager(ns::Id));

impl CaptureManager {
    define_cls!(MTL_CAPTURE_MANAGER);
}

// #[api::available(macos = 10.15, ios = 13.0)]
impl CaptureManager {
    #[objc::msg_send(sharedCaptureManager)]
    pub fn shared() -> &'static mut Self;

    //- (BOOL)startCaptureWithDescriptor:(MTLCaptureDescriptor *)descriptor error:(__autoreleasing NSError **)error API_AVAILABLE(macos(10.15), ios(13.0));
    // #[objc::optional]
    #[objc::msg_send(startCaptureWithDescriptor:error:)]
    pub unsafe fn start_with_descriptor_err<'ar>(
        &mut self,
        desc: &CaptureDesc,
        error: *mut Option<&'ar ns::Error>,
    ) -> bool;

    // #[api::available(macos = 10.15, ios = 13.0)]
    #[inline]
    pub fn start(&mut self, desc: &CaptureDesc) -> ns::Result {
        ns::if_false(|err| unsafe { self.start_with_descriptor_err(desc, err) })
    }

    // #[api::available(macos = 10.15, ios = 13.0)]
    #[objc::msg_send(stopCapture)]
    pub fn stop(&mut self);

    // #[api::available(macos = 10.15, ios = 13.0)]
    #[objc::msg_send(isCapturing)]
    pub fn is_capturing(&self) -> bool;
}

#[link(name = "mtl", kind = "static")]
extern "C" {
    static MTL_CAPTURE_DESCRIPTOR: &'static objc::Class<CaptureDesc>;
    static MTL_CAPTURE_MANAGER: &'static objc::Class<CaptureManager>;
}
