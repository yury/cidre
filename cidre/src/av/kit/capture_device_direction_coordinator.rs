use crate::{api, arc, av, define_obj_type, ns, objc, ui};

#[cfg(feature = "blocks")]
use crate::blocks;

define_obj_type!(
    /// The subset of a capture device's properties that identifies it.
    #[doc(alias = "AVCaptureDeviceDescriptor")]
    pub CaptureDeviceDescriptor(ns::Id)
);

impl CaptureDeviceDescriptor {
    #[objc::msg_send(deviceType)]
    #[objc::available(ios = 27.1, maccatalyst = 27.1)]
    pub fn device_type(&self) -> arc::R<av::CaptureDeviceType>;

    #[objc::msg_send(mediaTypes)]
    #[objc::available(ios = 27.1, maccatalyst = 27.1)]
    pub fn media_types(&self) -> arc::R<ns::Set<av::MediaType>>;

    #[objc::msg_send(position)]
    #[objc::available(ios = 27.1, maccatalyst = 27.1)]
    pub fn pos(&self) -> av::CaptureDevicePos;

    /// Matches the capture device's unique id.
    #[objc::msg_send(uniqueID)]
    #[objc::available(ios = 27.1, maccatalyst = 27.1)]
    pub fn unique_id(&self) -> arc::R<ns::String>;

    #[objc::msg_send(localizedName)]
    #[objc::available(ios = 27.1, maccatalyst = 27.1)]
    pub fn localized_name(&self) -> arc::R<ns::String>;
}

define_obj_type!(
    /// Which cameras face forward and which backward, relative to a view.
    #[doc(alias = "AVCaptureDeviceDirectionMap")]
    pub CaptureDeviceDirectionMap(ns::Id)
);

impl CaptureDeviceDirectionMap {
    /// The cameras that capture the scene in front of the display. Empty
    /// when none are available or applicable.
    #[objc::msg_send(forwardFacingDeviceDescriptors)]
    #[objc::available(ios = 27.1, maccatalyst = 27.1)]
    pub fn forward_facing_device_descriptors(&self) -> arc::R<ns::Array<CaptureDeviceDescriptor>>;

    /// The cameras that capture the scene behind the display. Empty when
    /// none are available or applicable.
    #[objc::msg_send(backwardFacingDeviceDescriptors)]
    #[objc::available(ios = 27.1, maccatalyst = 27.1)]
    pub fn backward_facing_device_descriptors(&self) -> arc::R<ns::Array<CaptureDeviceDescriptor>>;
}

define_obj_type!(
    /// Monitors the direction capture devices face in relation to a view.
    #[doc(alias = "AVCaptureDeviceDirectionCoordinator")]
    pub CaptureDeviceDirectionCoordinator(ns::Id)
);

impl CaptureDeviceDirectionCoordinator {
    #[api::available(ios = 27.1, maccatalyst = 27.1)]
    crate::define_cls!(AV_CAPTURE_DEVICE_DIRECTION_COORDINATOR);

    /// Initialize on the main thread only. The change handler is called on
    /// the main queue whenever the set of forward or backward facing video
    /// devices changes.
    #[cfg(feature = "blocks")]
    #[objc::init(initWithView:deviceTypes:changeHandler:)]
    pub fn init_with_view(
        self,
        view: &ui::View,
        device_types: &ns::Array<av::CaptureDeviceType>,
        change_handler: Option<&mut blocks::EscBlock<fn(&CaptureDeviceDirectionMap)>>,
    ) -> arc::R<CaptureDeviceDirectionCoordinator>;

    #[cfg(feature = "blocks")]
    #[api::available(ios = 27.1, maccatalyst = 27.1)]
    pub fn with_view(
        view: &ui::View,
        device_types: &ns::Array<av::CaptureDeviceType>,
        change_handler: Option<&mut blocks::EscBlock<fn(&CaptureDeviceDirectionMap)>>,
    ) -> arc::R<Self> {
        Self::alloc().init_with_view(view, device_types, change_handler)
    }

    /// The current directions of all cameras in relation to the view; an
    /// empty map until the first callback.
    #[objc::msg_send(deviceDirections)]
    #[objc::available(ios = 27.1, maccatalyst = 27.1)]
    pub fn device_directions(&self) -> arc::R<CaptureDeviceDirectionMap>;
}

unsafe extern "C" {
    static AV_CAPTURE_DEVICE_DIRECTION_COORDINATOR:
        &'static objc::Class<CaptureDeviceDirectionCoordinator>;
}
