use crate::{api, arc, blocks, cg, cm, define_obj_type, ns, objc, sc};

define_obj_type!(
    #[doc(alias = "SCScreenshotManager")]
    pub ScreenshotManager(ns::Id)
);

impl ScreenshotManager {
    #[api::available(macos = 14.0)]
    crate::define_cls!(SC_SCREENSHOT_MANAGER);

    #[cfg(feature = "blocks")]
    #[objc::msg_send(captureSampleBufferWithFilter:configuration:completionHandler:)]
    #[api::available(macos = 14.0)]
    pub fn capture_sample_buf_ch(
        filter: &sc::ContentFilter,
        cfg: &sc::StreamCfg,
        handler: Option<&mut blocks::ResultCompletionHandler<cm::SampleBuf>>,
    );

    #[cfg(all(feature = "blocks", feature = "async"))]
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
    #[objc::msg_send(captureImageWithFilter:configuration:completionHandler:)]
    #[api::available(macos = 14.0)]
    pub fn capture_image_ch(
        filter: &sc::ContentFilter,
        cfg: &sc::StreamCfg,
        handler: Option<&mut blocks::ResultCompletionHandler<cg::Image>>,
    );

    #[cfg(all(feature = "blocks", feature = "async"))]
    #[api::available(macos = 14.0)]
    pub async fn capture_image(
        filter: &sc::ContentFilter,
        cfg: &sc::StreamCfg,
    ) -> Result<arc::R<cg::Image>, arc::R<ns::Error>> {
        let (future, mut block) = blocks::result();
        Self::capture_image_ch(filter, cfg, Some(&mut block));
        future.await
    }
}

#[link(name = "sc", kind = "static")]
extern "C" {
    static SC_SCREENSHOT_MANAGER: &'static objc::Class<ScreenshotManager>;
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
