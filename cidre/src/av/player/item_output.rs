use crate::{arc, av, cf, cm, cv, define_cls, define_obj_type, ns, objc};

#[cfg(feature = "dispatch")]
use crate::dispatch;

define_obj_type!(
    #[doc(alias = "AVPlayerItemOutput")]
    pub ItemOutput(ns::Id)
);

unsafe impl Send for ItemOutput {}
unsafe impl Sync for ItemOutput {}

impl ItemOutput {
    #[objc::msg_send(itemTimeForHostTime:)]
    pub fn item_time_for_host_time(&self, host_time_in_secs: cf::TimeInterval) -> cm::Time;

    #[objc::msg_send(itemTimeForMachAbsoluteTime:)]
    pub fn item_time_for_mach_abs_time(&self, mach_abs_time: i64) -> cm::Time;

    #[cfg(target_os = "macos")]
    #[objc::msg_send(itemTimeForCVTimeStamp:)]
    pub fn item_time_for_cv_ts(&self, ts: cv::TimeStamp) -> cm::Time;

    #[objc::msg_send(suppressesPlayerRendering)]
    pub fn suppresses_player_rendering(&self) -> bool;

    #[objc::msg_send(setSuppressesPlayerRendering:)]
    pub fn set_suppresses_player_rendering(&mut self, val: bool);
}

define_obj_type!(
    #[doc(alias = "AVPlayerItemVideoOutput")]
    pub ItemVideoOutput(ItemOutput)
);

unsafe impl Send for ItemVideoOutput {}
unsafe impl Sync for ItemVideoOutput {}

impl arc::A<ItemVideoOutput> {
    #[objc::msg_send(initWithPixelBufferAttributes:)]
    pub fn init_with_pixel_buf_attrs(
        self,
        pixel_buf_attrs: Option<ns::Dictionary<ns::String, ns::Id>>,
    ) -> arc::R<ItemVideoOutput>;

    #[objc::msg_send(initWithOutputSettings:)]
    pub fn init_with_output_settings(
        self,
        output_settings: Option<ns::Dictionary<ns::String, ns::Id>>,
    ) -> arc::R<ItemVideoOutput>;
}

impl ItemVideoOutput {
    define_cls!(AV_PLAYER_ITEM_VIDEO_OUTPUT);

    pub fn new() -> arc::R<ItemVideoOutput> {
        Self::with_pixel_buf_attrs(None)
    }

    pub fn with_pixel_buf_attrs(
        pixel_buf_attrs: Option<ns::Dictionary<ns::String, ns::Id>>,
    ) -> arc::R<ItemVideoOutput> {
        Self::alloc().init_with_pixel_buf_attrs(pixel_buf_attrs)
    }

    pub fn with_output_settings(
        output_settings: Option<ns::Dictionary<ns::String, ns::Id>>,
    ) -> arc::R<ItemVideoOutput> {
        Self::alloc().init_with_output_settings(output_settings)
    }

    #[objc::msg_send(hasNewPixelBufferForItemTime:)]
    pub fn has_new_pixel_buf_for_item_time(&self, item_time: cm::Time) -> bool;

    #[objc::msg_send(copyPixelBufferForItemTime:itemTimeForDisplay:)]
    pub fn pixel_buf_for_item_time_for_display(
        &self,
        item_time: cm::Time,
        out_item_time_for_display: *mut cm::Time,
    ) -> Option<arc::Retained<cv::PixelBuf>>;

    #[inline]
    pub fn pixel_buf_for_item_time(
        &self,
        item_time: cm::Time,
    ) -> Option<arc::Retained<cv::PixelBuf>> {
        self.pixel_buf_for_item_time_for_display(item_time, std::ptr::null_mut())
    }

    #[cfg(feature = "dispatch")]
    #[objc::msg_send(setDelegate:queue:)]
    pub fn set_delegate<D: ItemOutputPullDelegate>(
        &mut self,
        delegate: Option<&D>,
        delegate_queue: Option<&dispatch::Queue>,
    );

    #[objc::msg_send(requestNotificationOfMediaDataChangeWithAdvanceInterval:)]
    pub fn request_notification_of_media_data_change_with_advance_interval(
        &mut self,
        interval: ns::TimeInterval,
    );

    #[objc::msg_send(delegate)]
    pub fn delegate(&self) -> Option<arc::R<AnyItemOutputPullDelegate>>;

    #[cfg(feature = "dispatch")]
    #[objc::msg_send(delegateQueue)]
    pub fn delegate_queue(&self) -> Option<arc::R<dispatch::Queue>>;
}

#[objc::protocol(AVPlayerItemOutputPullDelegate)]
pub trait ItemOutputPullDelegate: objc::Obj {
    #[objc::optional]
    #[objc::msg_send(outputMediaDataWillChange:)]
    fn output_media_data_will_change(&mut self, sender: &mut av::PlayerItemOutput);

    #[objc::optional]
    #[objc::msg_send(outputSequenceWasFlushed:)]
    fn output_sequence_was_flushed(&mut self, sender: &mut av::PlayerItemOutput);
}

define_obj_type!(
    pub AnyItemOutputPullDelegate(ns::Id)
);

impl ItemOutputPullDelegate for AnyItemOutputPullDelegate {}

#[link(name = "av", kind = "static")]
unsafe extern "C" {
    static AV_PLAYER_ITEM_VIDEO_OUTPUT: &'static objc::Class<ItemVideoOutput>;
}
