use crate::{api, arc, define_obj_type, ns, objc};

#[cfg(target_os = "ios")]
use crate::av;

define_obj_type!(
    /// An iOS stream output that applies the system camera video effect experience.
    #[doc(alias = "SCVideoEffectOutput")]
    pub VideoEffectOutput(ns::Id),
    SC_VIDEO_EFFECT_OUTPUT,
    #[api::available(ios = 27.0)]
);

impl arc::A<VideoEffectOutput> {
    /// Initializes a video effect output for the given camera capture device.
    #[objc::msg_send(initWithCameraDevice:)]
    #[api::available(ios = 27.0)]
    pub fn init_with_camera_device(
        self,
        device: &av::CaptureDevice,
    ) -> arc::Retained<VideoEffectOutput>;
}

impl VideoEffectOutput {
    /// The camera capture device used by the video effect output.
    #[objc::msg_send(cameraDevice)]
    #[api::available(ios = 27.0)]
    pub fn camera_device(&self) -> arc::R<av::CaptureDevice>;

    /// Sets the camera capture device used by the video effect output.
    #[objc::msg_send(setCameraDevice:)]
    #[api::available(ios = 27.0)]
    pub fn set_camera_device(&mut self, val: &av::CaptureDevice);

    #[inline]
    /// Creates a video effect output for the given camera capture device.
    #[api::available(ios = 27.0)]
    pub fn with_camera_device(device: &av::CaptureDevice) -> Option<arc::R<Self>> {
        #[cfg(feature = "ios_27_0")]
        {
            Some(Self::alloc().init_with_camera_device(device))
        }

        #[cfg(not(feature = "ios_27_0"))]
        {
            Some(unsafe { Self::alloc()?.init_with_camera_device(device) })
        }
    }
}

#[cfg(target_os = "ios")]
unsafe extern "C" {
    static SC_VIDEO_EFFECT_OUTPUT: &'static objc::Class<VideoEffectOutput>;
}
