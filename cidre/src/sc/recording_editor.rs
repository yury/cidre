use crate::{api, arc, define_obj_type, ns, objc};

#[cfg(all(
    feature = "blocks",
    any(
        all(feature = "app", target_os = "macos"),
        all(
            feature = "ui",
            any(target_os = "ios", target_os = "tvos", target_os = "visionos")
        )
    )
))]
use crate::blocks;
#[cfg(all(
    feature = "ui",
    any(target_os = "ios", target_os = "tvos", target_os = "visionos")
))]
use crate::ui;

#[doc(alias = "SCRecordingEditorMode")]
#[cfg(target_os = "tvos")]
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
#[repr(isize)]
/// The presentation mode for the tvOS recording editor.
pub enum Mode {
    /// Present a preview and trimming interface.
    #[doc(alias = "SCRecordingEditorModePreview")]
    Preview,
    /// Present the system sharing interface.
    #[doc(alias = "SCRecordingEditorModeShare")]
    Share,
}

#[doc(alias = "SCRecordingEditorDelegate")]
/// Receives callbacks from the recording editor presentation.
#[objc::protocol(SCRecordingEditorDelegate)]
pub trait Delegate: objc::Obj {
    /// Called after the editor is dismissed.
    #[objc::optional]
    #[objc::msg_send(recordingEditorDidDismiss:)]
    fn recording_editor_did_dismiss(&mut self, editor: &mut RecordingEditor);

    /// Called when the editor fails to present or complete its work.
    #[objc::optional]
    #[objc::msg_send(recordingEditor:didFailWithError:)]
    fn recording_editor_did_fail_with_err(
        &mut self,
        editor: &mut RecordingEditor,
        error: &ns::Error,
    );
}

define_obj_type!(pub AnyDelegate(ns::Id));
impl Delegate for AnyDelegate {}

define_obj_type!(
    /// A system editor for previewing, trimming, and sharing a recording file.
    #[doc(alias = "SCRecordingEditor")]
    pub RecordingEditor(ns::Id)
);

impl arc::A<RecordingEditor> {
    /// Initializes an editor for a recording file URL.
    #[objc::msg_send(initWithURL:)]
    #[api::available(
        macos = 27.0,
        maccatalyst = 27.0,
        ios = 27.0,
        visionos = 27.0,
        tvos = 27.0
    )]
    pub fn init_with_url(self, url: &ns::Url) -> arc::Retained<RecordingEditor>;
}

impl RecordingEditor {
    #[api::available(
        macos = 27.0,
        maccatalyst = 27.0,
        ios = 27.0,
        visionos = 27.0,
        tvos = 27.0
    )]
    crate::define_cls!(SC_RECORDING_EDITOR);

    #[inline]
    /// Creates an editor for a recording file URL.
    ///
    /// The URL is typically produced by [`crate::sc::RecordingOutput`] or
    /// [`crate::sc::ClipBufferingOutput`].
    #[api::available(
        macos = 27.0,
        maccatalyst = 27.0,
        ios = 27.0,
        visionos = 27.0,
        tvos = 27.0
    )]
    pub fn with_url(url: &ns::Url) -> Option<arc::R<Self>> {
        #[cfg(any(
            feature = "macos_27_0",
            feature = "maccatalyst_27_0",
            feature = "ios_27_0",
            feature = "tvos_27_0",
            feature = "visionos_27_0"
        ))]
        {
            Some(Self::alloc().init_with_url(url))
        }

        #[cfg(not(any(
            feature = "macos_27_0",
            feature = "maccatalyst_27_0",
            feature = "ios_27_0",
            feature = "tvos_27_0",
            feature = "visionos_27_0"
        )))]
        {
            Some(unsafe { Self::alloc()?.init_with_url(url) })
        }
    }

    /// The recording editor delegate.
    #[objc::msg_send(delegate)]
    #[api::available(
        macos = 27.0,
        maccatalyst = 27.0,
        ios = 27.0,
        visionos = 27.0,
        tvos = 27.0
    )]
    pub fn delegate(&self) -> Option<arc::R<AnyDelegate>>;

    /// Sets the recording editor delegate.
    #[objc::msg_send(setDelegate:)]
    #[api::available(
        macos = 27.0,
        maccatalyst = 27.0,
        ios = 27.0,
        visionos = 27.0,
        tvos = 27.0
    )]
    pub fn set_delegate<D: Delegate>(&mut self, val: Option<&D>);

    #[cfg(all(feature = "blocks", feature = "app", target_os = "macos"))]
    /// Presents the editor from a macOS window.
    #[objc::msg_send(presentFromWindow:completionHandler:)]
    #[api::available(macos = 27.0)]
    pub fn present_from_window_ch(
        &mut self,
        window: &ns::Window,
        handler: Option<&mut blocks::ErrCh>,
    );

    #[cfg(all(
        feature = "blocks",
        feature = "async",
        feature = "app",
        target_os = "macos"
    ))]
    /// Presents the editor from a macOS window.
    #[api::available(macos = 27.0)]
    pub async fn present_from_window(
        &mut self,
        window: &ns::Window,
    ) -> Result<(), arc::R<ns::Error>> {
        let (future, mut block) = blocks::ok();
        #[cfg(feature = "macos_27_0")]
        {
            self.present_from_window_ch(window, Some(&mut block));
        }

        #[cfg(not(feature = "macos_27_0"))]
        {
            unsafe { self.present_from_window_ch(window, Some(&mut block)) };
        }
        future.await
    }

    #[cfg(all(
        feature = "blocks",
        feature = "ui",
        any(target_os = "ios", target_os = "tvos", target_os = "visionos")
    ))]
    /// Presents the editor from a UIKit window scene.
    #[objc::msg_send(presentFromWindowScene:completionHandler:)]
    #[api::available(ios = 27.0, maccatalyst = 27.0, visionos = 27.0, tvos = 27.0)]
    pub fn present_from_window_scene_ch(
        &mut self,
        window_scene: &ui::WindowScene,
        handler: Option<&mut blocks::ErrCh>,
    );

    #[cfg(all(
        feature = "blocks",
        feature = "async",
        feature = "ui",
        any(target_os = "ios", target_os = "tvos", target_os = "visionos")
    ))]
    /// Presents the editor from a UIKit window scene.
    #[api::available(ios = 27.0, maccatalyst = 27.0, visionos = 27.0, tvos = 27.0)]
    pub async fn present_from_window_scene(
        &mut self,
        window_scene: &ui::WindowScene,
    ) -> Result<(), arc::R<ns::Error>> {
        let (future, mut block) = blocks::ok();
        unsafe { self.present_from_window_scene_ch(window_scene, Some(&mut block)) };
        future.await
    }

    #[cfg(all(feature = "blocks", feature = "ui", target_os = "tvos"))]
    /// Presents the editor from a tvOS window scene using the given mode.
    #[objc::msg_send(presentFromWindowScene:mode:completionHandler:)]
    #[api::available(tvos = 27.0)]
    pub fn present_from_window_scene_mode_ch(
        &mut self,
        window_scene: &ui::WindowScene,
        mode: Mode,
        handler: Option<&mut blocks::ErrCh>,
    );

    #[cfg(all(
        feature = "blocks",
        feature = "async",
        feature = "ui",
        target_os = "tvos"
    ))]
    /// Presents the editor from a tvOS window scene using the given mode.
    #[api::available(tvos = 27.0)]
    pub async fn present_from_window_scene_mode(
        &mut self,
        window_scene: &ui::WindowScene,
        mode: Mode,
    ) -> Result<(), arc::R<ns::Error>> {
        let (future, mut block) = blocks::ok();
        unsafe { self.present_from_window_scene_mode_ch(window_scene, mode, Some(&mut block)) };
        future.await
    }
}

unsafe extern "C" {
    static SC_RECORDING_EDITOR: &'static objc::Class<RecordingEditor>;
}
