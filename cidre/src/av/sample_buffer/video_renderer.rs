use crate::{api, arc, av, blocks, cm, cv, define_obj_type, ns, objc};

define_obj_type!(
    #[doc(alias = "AVSampleBufferVideoRenderer")]
    pub VideoRenderer(ns::Id),
    AV_SAMPLE_BUFFER_VIDEO_RENDERER,
    #[api::available(macos = 14.0, ios = 17.0, maccatalyst = 17.0, tvos = 17.0, visionos = 1.0)]
);

impl av::QueuedSampleBufRendering for VideoRenderer {}

impl VideoRenderer {
    #[objc::msg_send(status)]
    pub fn status(&self) -> av::QueuedSampleBufRenderingStatus;

    #[objc::msg_send(error)]
    pub fn error(&self) -> Option<arc::R<ns::Error>>;

    #[objc::msg_send(requiresFlushToResumeDecoding)]
    pub fn requires_flush_to_resume_decoding(&self) -> bool;

    /// Instructs the video renderer to discard pending enqueued sample buffers and call the provided block when complete.
    #[objc::msg_send(removeDisplayedImage:completionHandler:)]
    pub fn flush_ch_block(
        &mut self,
        remove_displayed_image: bool,
        ch: Option<&mut blocks::CompletionBlock>,
    );

    /// Instructs the video renderer to discard pending enqueued sample buffers and call the provided closure when complete.
    #[doc(alias = "removeDisplayedImage:completionHandler:")]
    pub fn flush_ch(&mut self, remove_displayed_image: bool, ch: impl FnMut() + 'static) {
        let mut block = blocks::CompletionBlock::new0(ch);
        self.flush_ch_block(remove_displayed_image, Some(&mut block));
    }

    #[doc(alias = "removeDisplayedImage:completionHandler:")]
    #[cfg(feature = "async")]
    pub async fn flush(&mut self, remove_displayed_image: bool) {
        let (f, mut block) = blocks::comp0();
        self.flush_ch_block(remove_displayed_image, Some(&mut block));
        f.await
    }
}

/// AVSampleBufferVideoRendererPixelBufferOutput
impl VideoRenderer {
    #[objc::msg_send(copyDisplayedPixelBuffer)]
    #[objc::available(
        macos = 14.4,
        maccatalyst = 17.4,
        ios = 17.4,
        tvos = 17.4,
        visionos = 1.1
    )]
    pub fn displayed_pixel_buf(&self) -> Option<arc::Retained<cv::PixelBuf>>;
}

/// AVSampleBufferVideoRendererPowerOptimization
impl VideoRenderer {
    /// Promises, for the purpose of enabling power optimizations,
    /// that future sample buffers will have PTS values no less than a specified lower-bound PTS.
    #[objc::msg_send(minimumUpcomingPresentationTime:)]
    #[objc::available(
        macos = 14.4,
        maccatalyst = 17.4,
        ios = 17.4,
        tvos = 17.4,
        visionos = 1.1
    )]
    pub fn expect_minimum_upcoming_sample_buf_pts(&mut self, minimum_upcoming_pts: cm::Time);

    /// Promises, for the purpose of enabling power optimizations,
    /// that future sample buffers will have monotonically increasing PTS values.
    #[objc::msg_send(expectMonotonicallyIncreasingUpcomingSampleBufferPresentationTimes)]
    #[objc::available(
        macos = 14.4,
        maccatalyst = 17.4,
        ios = 17.4,
        tvos = 17.4,
        visionos = 1.1
    )]
    pub fn expect_monotonically_increasing_upcoming_sample_buf_pts(&mut self);

    /// Resets previously-promised expectations about upcoming sample buffer PTSs.
    #[objc::msg_send(resetUpcomingSampleBufferPresentationTimeExpectations)]
    #[objc::available(
        macos = 14.4,
        maccatalyst = 17.4,
        ios = 17.4,
        tvos = 17.4,
        visionos = 1.1
    )]
    pub fn reset_upcoming_sample_buf_pts_expectations(&mut self);

    #[api::available(
        macos = 14.0,
        maccatalyst = 17.0,
        ios = 17.0,
        tvos = 17.0,
        visionos = 1.0
    )]
    pub fn did_fail_to_decode_notification() -> &'static ns::NotificationName {
        unsafe { AVSampleBufferVideoRendererDidFailToDecodeNotification }
    }

    #[api::available(
        macos = 14.0,
        maccatalyst = 17.0,
        ios = 17.0,
        tvos = 17.0,
        visionos = 1.0
    )]
    pub fn did_fail_to_decode_notification_err_key() -> &'static ns::String {
        unsafe { AVSampleBufferVideoRendererDidFailToDecodeNotificationErrorKey }
    }

    #[api::available(
        macos = 14.0,
        maccatalyst = 17.0,
        ios = 17.0,
        tvos = 17.0,
        visionos = 1.0
    )]
    pub fn requires_flush_to_resume_decoding_did_change_notification(
    ) -> &'static ns::NotificationName {
        unsafe { AVSampleBufferVideoRendererRequiresFlushToResumeDecodingDidChangeNotification }
    }
}

#[link(name = "ca", kind = "static")]
extern "C" {
    static AV_SAMPLE_BUFFER_VIDEO_RENDERER: &'static objc::Class<VideoRenderer>;
}

#[link(name = "AVFoundation", kind = "framework")]
#[api::weak]
extern "C" {
    #[api::available(
        macos = 14.0,
        maccatalyst = 17.0,
        ios = 17.0,
        tvos = 17.0,
        visionos = 1.0
    )]
    static AVSampleBufferVideoRendererDidFailToDecodeNotification: &'static ns::NotificationName;

    #[api::available(
        macos = 14.0,
        maccatalyst = 17.0,
        ios = 17.0,
        tvos = 17.0,
        visionos = 1.0
    )]
    static AVSampleBufferVideoRendererDidFailToDecodeNotificationErrorKey: &'static ns::String;

    #[api::available(
        macos = 14.0,
        maccatalyst = 17.0,
        ios = 17.0,
        tvos = 17.0,
        visionos = 1.0
    )]
    static AVSampleBufferVideoRendererRequiresFlushToResumeDecodingDidChangeNotification:
        &'static ns::NotificationName;
}
