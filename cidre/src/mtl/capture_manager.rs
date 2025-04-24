use crate::{arc, define_cls, define_obj_type, mtl, ns, objc};

/// The destination where you want the GPU trace to be captured to.
#[doc(alias = "MTLCaptureDestination")]
#[derive(Eq, PartialEq, Debug, Copy, Clone)]
#[repr(isize)]
pub enum CaptureDst {
    /// Capture to Developer Tools (Xcode) and stop the execution after capturing.
    DeveloperTools = 1,

    /// Capture to a GPU Trace document and continue execution after capturing.
    GpuTraceDocument,
}

define_obj_type!(pub CaptureDesc(ns::Id), MTL_CAPTURE_DESCRIPTOR);

impl CaptureDesc {
    /// The object that is captured.
    #[objc::msg_send(captureObject)]
    pub fn capture_obj(&self) -> Option<arc::R<ns::Id>>;

    /// The object that is captured.
    ///
    /// Must be one of the following:
    /// - mtl::Device captures all command queues of the device.
    /// - mtl::CmdQueue captures a single command queue.
    /// - mtl::CaptureScope captures between the next begin and end of the scope.
    #[objc::msg_send(setCaptureObject:)]
    pub fn set_capture_obj(&mut self, val: &ns::Id);

    #[objc::msg_send(destination)]
    pub fn dst(&self) -> CaptureDst;

    #[objc::msg_send(setDestination:)]
    pub fn set_dst(&self, val: CaptureDst);

    /// URL the GPU Trace document will be captured to.
    #[objc::msg_send(outputURL)]
    pub fn output_url(&self) -> Option<arc::R<ns::Url>>;

    /// URL the GPU Trace document will be captured to.
    ///
    /// Must be specified when destiation is mtl::CaptureDst::GpuTraceDocument.
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
    pub fn shared() -> arc::R<Self>;

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

    #[objc::msg_send(supportsDestination:)]
    pub fn supports_dst(&self, val: CaptureDst) -> bool;

    #[objc::msg_send(newCaptureScopeWithDevice:)]
    pub fn new_capture_scope_with_device(&self, device: &mtl::Device) -> arc::R<mtl::CaptureScope>;

    #[objc::msg_send(newCaptureScopeWithCommandQueue:)]
    pub fn new_capture_scope_with_cmd_queue(
        &self,
        cmd_queue: &mtl::CmdQueue,
    ) -> arc::R<mtl::CaptureScope>;

    #[objc::msg_send(defaultCaptureScope)]
    pub fn default_capture_scope(&self) -> Option<arc::R<mtl::CaptureScope>>;
}

#[link(name = "mtl", kind = "static")]
unsafe extern "C" {
    static MTL_CAPTURE_DESCRIPTOR: &'static objc::Class<CaptureDesc>;
    static MTL_CAPTURE_MANAGER: &'static objc::Class<CaptureManager>;
}

#[cfg(test)]
mod tests {
    use crate::{mtl, ns};
    #[test]
    fn basics() {
        let mut desc = mtl::CaptureDesc::new();
        desc.set_capture_obj(ns::str!(c"invalid obj"));
        let mut manager = mtl::CaptureManager::shared();
        manager.start(&desc).expect_err("invalid");
    }
}
