use crate::{api, arc, cg, define_obj_type, ns, objc, ut};

#[cfg(feature = "blocks")]
use crate::blocks;

#[cfg(feature = "blocks")]
use crate::sc;

#[cfg(all(feature = "blocks", feature = "cm"))]
use crate::cm;

define_obj_type!(
    /// Configuration for capturing a still screenshot.
    #[doc(alias = "SCScreenshotConfiguration")]
    pub ScreenshotCfg(ns::Id),
    SC_SCREENSHOT_CONFIGURATION,
    #[api::available(macos = 26.0, maccatalyst = 26.0)]
);

#[doc(alias = "SCScreenshotDisplayIntent")]
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
#[repr(isize)]
pub enum DisplayIntent {
    /// Capture using the canonical display rendering intent.
    #[doc(alias = "SCScreenshotDisplayIntentCanonical")]
    Canonical,
    /// Capture using the local display rendering intent.
    #[doc(alias = "SCScreenshotDisplayIntentLocal")]
    Local,
}

#[doc(alias = "SCScreenshotDynamicRange")]
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
#[repr(isize)]
pub enum DynamicRange {
    /// Capture standard dynamic range output.
    #[doc(alias = "SCScreenshotDynamicRangeSDR")]
    Sdr,
    /// Capture high dynamic range output.
    #[doc(alias = "SCScreenshotDynamicRangeHDR")]
    Hdr,
    /// Capture both standard and high dynamic range output.
    #[doc(alias = "SCScreenshotDynamicRangeSDRAndHDR")]
    SdrAndHdr,
}

impl ScreenshotCfg {
    /// The output width in pixels.
    #[objc::msg_send(width)]
    #[api::available(macos = 26.0, maccatalyst = 26.0)]
    pub fn width(&self) -> ns::Integer;

    /// Sets the output width in pixels.
    #[objc::msg_send(setWidth:)]
    #[api::available(macos = 26.0, maccatalyst = 26.0)]
    pub fn set_width(&mut self, val: ns::Integer);

    /// The output height in pixels.
    #[objc::msg_send(height)]
    #[api::available(macos = 26.0, maccatalyst = 26.0)]
    pub fn height(&self) -> ns::Integer;

    /// Sets the output height in pixels.
    #[objc::msg_send(setHeight:)]
    #[api::available(macos = 26.0, maccatalyst = 26.0)]
    pub fn set_height(&mut self, val: ns::Integer);

    /// Whether the screenshot includes the pointer cursor.
    #[objc::msg_send(showsCursor)]
    #[api::available(macos = 26.0, maccatalyst = 26.0)]
    pub fn shows_cursor(&self) -> bool;

    /// Sets whether the screenshot includes the pointer cursor.
    #[objc::msg_send(setShowsCursor:)]
    #[api::available(macos = 26.0, maccatalyst = 26.0)]
    pub fn set_shows_cursor(&mut self, val: bool);

    /// The source rectangle to capture.
    #[objc::msg_send(sourceRect)]
    #[api::available(macos = 26.0, maccatalyst = 26.0)]
    pub fn src_rect(&self) -> cg::Rect;

    /// Sets the source rectangle to capture.
    #[objc::msg_send(setSourceRect:)]
    #[api::available(macos = 26.0, maccatalyst = 26.0)]
    pub fn set_src_rect(&mut self, val: cg::Rect);

    /// The destination rectangle in the output image.
    #[objc::msg_send(destinationRect)]
    #[api::available(macos = 26.0, maccatalyst = 26.0)]
    pub fn dst_rect(&self) -> cg::Rect;

    /// Sets the destination rectangle in the output image.
    #[objc::msg_send(setDestinationRect:)]
    #[api::available(macos = 26.0, maccatalyst = 26.0)]
    pub fn set_dst_rect(&mut self, val: cg::Rect);

    /// Whether window shadows are omitted from the screenshot.
    #[objc::msg_send(ignoreShadows)]
    #[api::available(macos = 26.0, maccatalyst = 26.0)]
    pub fn ignore_shadows(&self) -> bool;

    /// Sets whether window shadows are omitted from the screenshot.
    #[objc::msg_send(setIgnoreShadows:)]
    #[api::available(macos = 26.0, maccatalyst = 26.0)]
    pub fn set_ignore_shadows(&mut self, val: bool);

    /// Whether window clipping is ignored when capturing.
    #[objc::msg_send(ignoreClipping)]
    #[api::available(macos = 26.0, maccatalyst = 26.0)]
    pub fn ignore_clipping(&self) -> bool;

    /// Sets whether window clipping is ignored when capturing.
    #[objc::msg_send(setIgnoreClipping:)]
    #[api::available(macos = 26.0, maccatalyst = 26.0)]
    pub fn set_ignore_clipping(&mut self, val: bool);

    /// Whether child windows are included with their parent content.
    #[objc::msg_send(includeChildWindows)]
    #[api::available(macos = 26.0, maccatalyst = 26.0)]
    pub fn include_child_windows(&self) -> bool;

    /// Sets whether child windows are included with their parent content.
    #[objc::msg_send(setIncludeChildWindows:)]
    #[api::available(macos = 26.0, maccatalyst = 26.0)]
    pub fn set_include_child_windows(&mut self, val: bool);

    /// The display rendering intent used for HDR screenshots.
    #[objc::msg_send(displayIntent)]
    #[api::available(macos = 26.0, maccatalyst = 26.0)]
    pub fn display_intent(&self) -> DisplayIntent;

    /// Sets the display rendering intent used for HDR screenshots.
    #[objc::msg_send(setDisplayIntent:)]
    #[api::available(macos = 26.0, maccatalyst = 26.0)]
    pub fn set_display_intent(&mut self, val: DisplayIntent);

    /// The dynamic range outputs to produce.
    #[objc::msg_send(dynamicRange)]
    #[api::available(macos = 26.0, maccatalyst = 26.0)]
    pub fn dynamic_range(&self) -> DynamicRange;

    /// Sets the dynamic range outputs to produce.
    #[objc::msg_send(setDynamicRange:)]
    #[api::available(macos = 26.0, maccatalyst = 26.0)]
    pub fn set_dynamic_range(&mut self, val: DynamicRange);

    /// The content type for file output.
    #[objc::msg_send(contentType)]
    #[api::available(macos = 26.0, maccatalyst = 26.0)]
    pub fn content_type(&self) -> arc::R<ut::Type>;

    /// Sets the content type for file output.
    #[objc::msg_send(setContentType:)]
    #[api::available(macos = 26.0, maccatalyst = 26.0)]
    pub fn set_content_type(&mut self, val: &ut::Type);

    /// The optional destination URL for writing a screenshot file.
    #[objc::msg_send(fileURL)]
    #[api::available(macos = 26.0, maccatalyst = 26.0)]
    pub fn file_url(&self) -> Option<arc::R<ns::Url>>;

    /// Sets the optional destination URL for writing a screenshot file.
    #[objc::msg_send(setFileURL:)]
    #[api::available(macos = 26.0, maccatalyst = 26.0)]
    pub fn set_file_url(&mut self, val: Option<&ns::Url>);

    /// The content types supported for screenshot file output.
    #[objc::msg_send(supportedContentTypes)]
    #[api::available(macos = 26.0, maccatalyst = 26.0)]
    pub fn supported_content_types() -> arc::R<ns::Array<ut::Type>>;
}

define_obj_type!(
    /// The result of a screenshot capture.
    #[doc(alias = "SCScreenshotOutput")]
    pub ScreenshotOutput(ns::Id),
    SC_SCREENSHOT_OUTPUT,
    #[api::available(macos = 26.0, maccatalyst = 26.0)]
);

impl ScreenshotOutput {
    /// The standard dynamic range image, when requested.
    #[objc::msg_send(sdrImage)]
    #[api::available(macos = 26.0, maccatalyst = 26.0)]
    pub fn sdr_image(&self) -> Option<&cg::Image>;

    /// Sets the standard dynamic range image.
    #[objc::msg_send(setSdrImage:)]
    #[api::available(macos = 26.0, maccatalyst = 26.0)]
    pub fn set_sdr_image(&mut self, val: Option<&cg::Image>);

    /// The high dynamic range image, when requested.
    #[objc::msg_send(hdrImage)]
    #[api::available(macos = 26.0, maccatalyst = 26.0)]
    pub fn hdr_image(&self) -> Option<&cg::Image>;

    /// Sets the high dynamic range image.
    #[objc::msg_send(setHdrImage:)]
    #[api::available(macos = 26.0, maccatalyst = 26.0)]
    pub fn set_hdr_image(&mut self, val: Option<&cg::Image>);

    /// The file URL written by the screenshot capture, when file output was requested.
    #[objc::msg_send(fileURL)]
    #[api::available(macos = 26.0, maccatalyst = 26.0)]
    pub fn file_url(&self) -> Option<arc::R<ns::Url>>;
}

define_obj_type!(
    /// A utility object for one-shot ScreenCaptureKit captures.
    #[doc(alias = "SCScreenshotManager")]
    pub ScreenshotManager(ns::Id)
);

impl ScreenshotManager {
    #[api::available(macos = 14.0)]
    crate::define_cls!(SC_SCREENSHOT_MANAGER);

    #[cfg(all(feature = "blocks", feature = "cm"))]
    /// Captures one sample buffer using a content filter and stream configuration.
    #[objc::msg_send(captureSampleBufferWithFilter:configuration:completionHandler:)]
    #[api::available(macos = 14.0)]
    pub fn capture_sample_buf_ch(
        filter: &sc::ContentFilter,
        cfg: &sc::StreamCfg,
        handler: Option<&mut blocks::ResultCh<cm::SampleBuf>>,
    );

    #[cfg(all(feature = "blocks", feature = "async", feature = "cm"))]
    /// Captures one sample buffer using a content filter and stream configuration.
    #[api::available(macos = 14.0)]
    pub async fn capture_sample_buf(
        filter: &sc::ContentFilter,
        cfg: &sc::StreamCfg,
    ) -> Result<arc::R<cm::SampleBuf>, arc::R<ns::Error>> {
        let (future, mut block) = blocks::result();
        Self::capture_sample_buf_ch(filter, cfg, Some(&mut block));
        future.await
    }

    #[cfg(feature = "blocks")]
    /// Captures one Core Graphics image using a content filter and stream configuration.
    #[objc::msg_send(captureImageWithFilter:configuration:completionHandler:)]
    #[api::available(macos = 14.0)]
    pub fn capture_image_ch(
        filter: &sc::ContentFilter,
        cfg: &sc::StreamCfg,
        handler: Option<&mut blocks::ResultCh<cg::Image>>,
    );

    #[cfg(all(feature = "blocks", feature = "async"))]
    /// Captures one Core Graphics image using a content filter and stream configuration.
    #[api::available(macos = 14.0)]
    pub async fn capture_image(
        filter: &sc::ContentFilter,
        cfg: &sc::StreamCfg,
    ) -> Result<arc::R<cg::Image>, arc::R<ns::Error>> {
        let (future, mut block) = blocks::result();
        Self::capture_image_ch(filter, cfg, Some(&mut block));
        future.await
    }

    /// Returns an image containing the contents of the rectangle in points, specified in display space
    #[cfg(feature = "blocks")]
    #[objc::msg_send(captureImageInRect:completionHandler:)]
    #[api::available(macos = 15.2)]
    pub fn capture_image_in_rect_ch(
        rect: cg::Rect,
        handler: Option<&mut blocks::ResultCh<cg::Image>>,
    );

    /// Returns an image containing the contents of the rectangle in points, specified in display space
    #[cfg(all(feature = "blocks", feature = "async"))]
    #[api::available(macos = 15.2)]
    #[allow(unused_unsafe)]
    pub async fn capture_image_in_rect(
        rect: cg::Rect,
    ) -> Result<arc::R<cg::Image>, arc::R<ns::Error>> {
        let (future, mut block) = blocks::result();
        unsafe { Self::capture_image_in_rect_ch(rect, Some(&mut block)) };
        future.await
    }

    #[cfg(feature = "blocks")]
    /// Captures a screenshot output using a content filter and screenshot configuration.
    #[objc::msg_send(captureScreenshotWithFilter:configuration:completionHandler:)]
    #[api::available(macos = 26.0, maccatalyst = 26.0)]
    pub fn capture_screenshot_ch(
        filter: &sc::ContentFilter,
        cfg: &ScreenshotCfg,
        handler: Option<&mut blocks::ResultCh<ScreenshotOutput>>,
    );

    #[cfg(all(feature = "blocks", feature = "async"))]
    /// Captures a screenshot output using a content filter and screenshot configuration.
    #[api::available(macos = 26.0, maccatalyst = 26.0)]
    pub async fn capture_screenshot(
        filter: &sc::ContentFilter,
        cfg: &ScreenshotCfg,
    ) -> Result<arc::R<ScreenshotOutput>, arc::R<ns::Error>> {
        let (future, mut block) = blocks::result();
        #[cfg(any(feature = "macos_26_0", feature = "maccatalyst_26_0"))]
        {
            Self::capture_screenshot_ch(filter, cfg, Some(&mut block));
        }

        #[cfg(not(any(feature = "macos_26_0", feature = "maccatalyst_26_0")))]
        {
            unsafe { Self::capture_screenshot_ch(filter, cfg, Some(&mut block)) };
        }
        future.await
    }

    #[cfg(feature = "blocks")]
    /// Captures a screenshot output for a rectangle in display-space points.
    #[objc::msg_send(captureScreenshotWithRect:configuration:completionHandler:)]
    #[api::available(macos = 26.0, maccatalyst = 26.0)]
    pub fn capture_screenshot_in_rect_ch(
        rect: cg::Rect,
        cfg: &ScreenshotCfg,
        handler: Option<&mut blocks::ResultCh<ScreenshotOutput>>,
    );

    #[cfg(all(feature = "blocks", feature = "async"))]
    /// Captures a screenshot output for a rectangle in display-space points.
    #[api::available(macos = 26.0, maccatalyst = 26.0)]
    pub async fn capture_screenshot_in_rect(
        rect: cg::Rect,
        cfg: &ScreenshotCfg,
    ) -> Result<arc::R<ScreenshotOutput>, arc::R<ns::Error>> {
        let (future, mut block) = blocks::result();
        #[cfg(any(feature = "macos_26_0", feature = "maccatalyst_26_0"))]
        {
            Self::capture_screenshot_in_rect_ch(rect, cfg, Some(&mut block));
        }

        #[cfg(not(any(feature = "macos_26_0", feature = "maccatalyst_26_0")))]
        {
            unsafe { Self::capture_screenshot_in_rect_ch(rect, cfg, Some(&mut block)) };
        }
        future.await
    }
}

unsafe extern "C" {
    static SC_SCREENSHOT_MANAGER: &'static objc::Class<ScreenshotManager>;
    static SC_SCREENSHOT_CONFIGURATION: &'static objc::Class<ScreenshotCfg>;
    static SC_SCREENSHOT_OUTPUT: &'static objc::Class<ScreenshotOutput>;
}

#[cfg(test)]
mod tests {
    use crate::{ns, sc};

    #[tokio::test]
    async fn basics() {
        let content = sc::ShareableContent::current_process().await.unwrap();
        let displays = content.displays();
        let display = displays.first().unwrap();
        let filter = sc::ContentFilter::with_display_excluding_windows(&display, &ns::Array::new());
        let cfg = sc::StreamCfg::new();
        let buf = sc::ScreenshotManager::capture_sample_buf(&filter, &cfg)
            .await
            .unwrap();
        let image_buf = buf.image_buf().unwrap();
        assert!(image_buf.width() > 0);
        assert!(image_buf.height() > 0);

        let image = sc::ScreenshotManager::capture_image(&filter, &cfg)
            .await
            .unwrap();
        assert!(image.width() > 0);
        assert!(image.height() > 0);
    }
}
