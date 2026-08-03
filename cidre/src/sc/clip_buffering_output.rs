use crate::{api, arc, define_obj_type, ns, objc};

#[cfg(feature = "blocks")]
use crate::blocks;

#[doc(alias = "SCClipBufferingOutputDelegate")]
/// Receives lifecycle callbacks for a clip buffering output.
#[objc::protocol(SCClipBufferingOutputDelegate)]
pub trait Delegate: objc::Obj {
    /// Called after clip buffering starts successfully.
    #[objc::optional]
    #[objc::msg_send(clipBufferingOutputDidStartBuffering:)]
    fn clip_buffering_output_did_start_buffering(
        &mut self,
        clip_buffering_output: &mut ClipBufferingOutput,
    );

    /// Called when clip buffering fails.
    #[objc::optional]
    #[objc::msg_send(clipBufferingOutput:didFailWithError:)]
    fn clip_buffering_output_did_fail_with_err(
        &mut self,
        clip_buffering_output: &mut ClipBufferingOutput,
        error: &ns::Error,
    );

    /// Called after clip buffering stops successfully.
    #[objc::optional]
    #[objc::msg_send(clipBufferingOutputDidStopBuffering:)]
    fn clip_buffering_output_did_stop_buffering(
        &mut self,
        clip_buffering_output: &mut ClipBufferingOutput,
    );
}

define_obj_type!(pub AnyDelegate(ns::Id));
impl Delegate for AnyDelegate {}

define_obj_type!(
    /// An output that keeps a rolling buffer of recent stream content for later export.
    #[doc(alias = "SCClipBufferingOutput")]
    pub ClipBufferingOutput(ns::Id)
);

impl arc::A<ClipBufferingOutput> {
    /// Initializes a clip buffering output with an optional delegate.
    #[objc::msg_send(initWithDelegate:)]
    #[api::available(
        macos = 27.0,
        maccatalyst = 27.0,
        ios = 27.0,
        visionos = 27.0,
        tvos = 27.0
    )]
    pub fn init_with_delegate<D: Delegate>(
        self,
        delegate: Option<&D>,
    ) -> arc::Retained<ClipBufferingOutput>;
}

impl ClipBufferingOutput {
    #[api::available(
        macos = 27.0,
        maccatalyst = 27.0,
        ios = 27.0,
        visionos = 27.0,
        tvos = 27.0
    )]
    crate::define_cls!(SC_CLIP_BUFFERING_OUTPUT);

    #[inline]
    /// Creates a clip buffering output and installs the given delegate.
    #[api::available(
        macos = 27.0,
        maccatalyst = 27.0,
        ios = 27.0,
        visionos = 27.0,
        tvos = 27.0
    )]
    pub fn with_delegate(delegate: &impl Delegate) -> Option<arc::R<Self>> {
        #[cfg(any(
            feature = "macos_27_0",
            feature = "maccatalyst_27_0",
            feature = "ios_27_0",
            feature = "tvos_27_0",
            feature = "visionos_27_0"
        ))]
        {
            Some(Self::alloc().init_with_delegate(Some(delegate)))
        }

        #[cfg(not(any(
            feature = "macos_27_0",
            feature = "maccatalyst_27_0",
            feature = "ios_27_0",
            feature = "tvos_27_0",
            feature = "visionos_27_0"
        )))]
        {
            Some(unsafe { Self::alloc()?.init_with_delegate(Some(delegate)) })
        }
    }

    #[inline]
    /// Creates a clip buffering output without a delegate.
    #[api::available(
        macos = 27.0,
        maccatalyst = 27.0,
        ios = 27.0,
        visionos = 27.0,
        tvos = 27.0
    )]
    pub fn without_delegate() -> Option<arc::R<Self>> {
        #[cfg(any(
            feature = "macos_27_0",
            feature = "maccatalyst_27_0",
            feature = "ios_27_0",
            feature = "tvos_27_0",
            feature = "visionos_27_0"
        ))]
        {
            Some(Self::alloc().init_with_delegate::<AnyDelegate>(None))
        }

        #[cfg(not(any(
            feature = "macos_27_0",
            feature = "maccatalyst_27_0",
            feature = "ios_27_0",
            feature = "tvos_27_0",
            feature = "visionos_27_0"
        )))]
        {
            Some(unsafe { Self::alloc()?.init_with_delegate::<AnyDelegate>(None) })
        }
    }

    #[cfg(feature = "blocks")]
    /// Exports the most recent buffered content to a movie file.
    ///
    /// The output must be attached to a stream before export. The requested
    /// duration is capped by the available buffer and by ScreenCaptureKit.
    #[objc::msg_send(exportClipToURL:duration:completionHandler:)]
    #[api::available(
        macos = 27.0,
        maccatalyst = 27.0,
        ios = 27.0,
        visionos = 27.0,
        tvos = 27.0
    )]
    pub fn export_clip_to_url_ch(
        &self,
        url: &ns::Url,
        duration: ns::TimeInterval,
        handler: Option<&mut blocks::ErrCh>,
    );

    #[cfg(all(feature = "blocks", feature = "async"))]
    /// Exports the most recent buffered content to a movie file.
    ///
    /// This does not stop ongoing buffering.
    #[api::available(
        macos = 27.0,
        maccatalyst = 27.0,
        ios = 27.0,
        visionos = 27.0,
        tvos = 27.0
    )]
    pub async fn export_clip_to_url(
        &self,
        url: &ns::Url,
        duration: ns::TimeInterval,
    ) -> Result<(), arc::R<ns::Error>> {
        let (future, mut block) = blocks::ok();
        #[cfg(any(
            feature = "macos_27_0",
            feature = "maccatalyst_27_0",
            feature = "ios_27_0",
            feature = "tvos_27_0",
            feature = "visionos_27_0"
        ))]
        {
            self.export_clip_to_url_ch(url, duration, Some(&mut block));
        }

        #[cfg(not(any(
            feature = "macos_27_0",
            feature = "maccatalyst_27_0",
            feature = "ios_27_0",
            feature = "tvos_27_0",
            feature = "visionos_27_0"
        )))]
        {
            unsafe { self.export_clip_to_url_ch(url, duration, Some(&mut block)) };
        }
        future.await
    }
}

unsafe extern "C" {
    static SC_CLIP_BUFFERING_OUTPUT: &'static objc::Class<ClipBufferingOutput>;
}
