use crate::{arc, cg, define_cls, define_obj_type, ns, objc, sc, sys};

#[cfg(feature = "blocks")]
use crate::blocks;

#[doc(alias = "SCShareableContentStyle")]
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
#[repr(isize)]
pub enum Style {
    /// No specific shareable content style.
    None,
    /// Window content.
    Window,
    /// Display content.
    Display,
    /// Application content.
    Application,
}

define_obj_type!(
    /// A running application that can own shareable content.
    #[doc(alias = "SCRunningApplication")]
    pub RunningApp(ns::Id)
);

impl RunningApp {
    /// The application's bundle identifier.
    #[objc::msg_send(bundleIdentifier)]
    pub fn bundle_id(&self) -> arc::R<ns::String>;

    /// The localized application name.
    #[objc::msg_send(applicationName)]
    pub fn app_name(&self) -> arc::R<ns::String>;

    /// The application's process identifier.
    #[objc::msg_send(processID)]
    pub fn process_id(&self) -> sys::Pid;
}

define_obj_type!(
    /// A display that can be shared or captured.
    #[doc(alias = "SCDisplay")]
    pub Display(ns::Id)
);

impl Display {
    /// The Core Graphics display identifier.
    #[objc::msg_send(displayID)]
    pub fn display_id(&self) -> cg::DirectDisplayId;

    /// The display width in pixels.
    #[objc::msg_send(width)]
    pub fn width(&self) -> isize;

    /// The display height in pixels.
    #[objc::msg_send(height)]
    pub fn height(&self) -> isize;

    /// The display frame in points.
    #[objc::msg_send(frame)]
    pub fn frame(&self) -> cg::Rect;
}

define_obj_type!(
    /// A window that can be shared or captured.
    #[doc(alias = "SCWindow")]
    pub Window(ns::Id)
);

impl Window {
    /// The Core Graphics window identifier.
    #[objc::msg_send(windowID)]
    pub fn id(&self) -> cg::WindowId;

    /// The window frame in points.
    #[objc::msg_send(frame)]
    pub fn frame(&self) -> cg::Rect;

    /// The window title, if available.
    #[objc::msg_send(title)]
    pub fn title(&self) -> Option<arc::R<ns::String>>;

    /// The Core Graphics window layer.
    #[objc::msg_send(windowLayer)]
    pub fn window_layer(&self) -> ns::Integer;

    /// The application that owns the window, if available.
    #[objc::msg_send(owningApplication)]
    pub fn owning_app(&self) -> Option<arc::R<RunningApp>>;

    /// Whether the window is currently onscreen.
    #[objc::msg_send(isOnScreen)]
    pub fn is_on_screen(&self) -> bool;

    /// Whether the window is active.
    #[objc::msg_send(isActive)]
    #[objc::available(macos = 13.1)]
    pub fn is_active(&self) -> bool;
}

impl std::fmt::Display for Window {
    #[cfg(not(all(target_os = "macos", feature = "macos_13_1")))]
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Window")
            .field("id", &self.id())
            .field("frame", &self.frame())
            .field("title", &self.title())
            .field("is_on_screen", &self.is_on_screen())
            .finish()
    }
    #[cfg(all(target_os = "macos", feature = "macos_13_1"))]
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Window")
            .field("id", &self.id())
            .field("frame", &self.frame())
            .field("title", &self.title())
            .field("is_on_screen", &self.is_on_screen())
            .field("is_active", &self.is_active())
            .finish()
    }
}

unsafe extern "C" {
    static SC_SHAREABLE_CONTENT: &'static objc::Class<ShareableContent>;
}

define_obj_type!(
    /// A snapshot of content that ScreenCaptureKit can share or capture.
    #[doc(alias = "SCShareableContent")]
    pub ShareableContent(ns::Id)
);

unsafe impl Send for ShareableContent {}

impl ShareableContent {
    define_cls!(SC_SHAREABLE_CONTENT);

    /// The shareable windows in this snapshot.
    #[objc::msg_send(windows)]
    pub fn windows(&self) -> arc::R<ns::Array<Window>>;

    /// The shareable displays in this snapshot.
    #[objc::msg_send(displays)]
    pub fn displays(&self) -> arc::R<ns::Array<Display>>;

    /// The shareable applications in this snapshot.
    #[objc::msg_send(applications)]
    pub fn apps(&self) -> arc::R<ns::Array<RunningApp>>;

    #[cfg(feature = "blocks")]
    /// Retrieves the current shareable content snapshot.
    #[objc::msg_send(getShareableContentWithCompletionHandler:)]
    pub fn current_with_ch_block(block: &mut blocks::ResultCh<Self>);

    #[cfg(feature = "blocks")]
    /// Retrieves the current shareable content snapshot.
    pub fn current_with_ch(f: impl FnMut(Option<&Self>, Option<&ns::Error>) + 'static) {
        let mut block = blocks::ResultCh::new2(f);
        Self::current_with_ch_block(&mut block);
    }

    #[cfg(all(feature = "blocks", feature = "async"))]
    /// Retrieves the current shareable content snapshot.
    pub async fn current() -> Result<arc::R<Self>, arc::R<ns::Error>> {
        let (future, mut block) = blocks::result();
        Self::current_with_ch_block(&mut block);
        future.await
    }

    #[cfg(feature = "blocks")]
    /// Retrieves shareable content for the current process.
    #[objc::msg_send(getCurrentProcessShareableContentWithCompletionHandler:)]
    pub fn current_process_with_ch(block: &mut blocks::ResultCh<Self>);

    #[cfg(all(feature = "blocks", feature = "async"))]
    /// Retrieves shareable content for the current process.
    pub async fn current_process() -> Result<arc::R<Self>, arc::R<ns::Error>> {
        let (future, mut block) = blocks::result();
        Self::current_process_with_ch(&mut block);
        future.await
    }

    /// Returns metadata for a content filter.
    #[objc::msg_send(infoForFilter:)]
    pub fn info_for_filter(filter: &sc::ContentFilter) -> arc::R<Info>;
}

define_obj_type!(
    /// Metadata describing the content represented by a filter.
    #[doc(alias = "SCShareableContentInfo")]
    pub Info(ns::Id)
);

impl Info {
    /// The style of content represented by the filter.
    #[objc::msg_send(style)]
    pub fn style(&self) -> Style;

    /// The point-to-pixel scale for the content.
    #[objc::msg_send(pointPixelScale)]
    pub fn point_pixel_scale(&self) -> f32;

    /// The content rectangle in points.
    #[objc::msg_send(contentRect)]
    pub fn content_rect(&self) -> cg::Rect;
}

#[cfg(test)]
mod tests {

    use crate::{
        blocks, define_obj_type, dispatch, objc,
        sc::{
            self,
            stream::{Delegate, DelegateImpl, Output, OutputImpl},
        },
    };

    use super::ShareableContent;

    define_obj_type!(OutputObj + OutputImpl, usize, OUTPUT_CLS);

    impl Output for OutputObj {}

    #[objc::add_methods]
    impl OutputImpl for OutputObj {}

    define_obj_type!(DelegateObj + DelegateImpl, usize, OUTPUT_CLS);

    impl Delegate for DelegateObj {}

    #[objc::add_methods]
    impl DelegateImpl for DelegateObj {}

    #[tokio::test]
    pub async fn current() {
        let f = sc::ShareableContent::current().await.expect("result");
        assert!(!f.windows().is_empty());
        println!(
            "current retain count {:?} {:?}",
            f.as_type_ref().retain_count(),
            f.windows().len()
        );
    }

    #[tokio::test]
    pub async fn current2() {
        let f = sc::ShareableContent::current().await.expect("result");
        assert!(!f.windows().is_empty());
        println!(
            "current retain count {:?} {:?}",
            f.as_type_ref().retain_count(),
            f.windows().len()
        );
    }

    #[test]
    pub fn current_ch() {
        let sema = dispatch::Semaphore::new(0);

        let signal_guard = sema.guard();
        let mut bl = blocks::ResultCh::new2(move |content, error| {
            _ = &signal_guard;
            println!("nice {:?} {:?}", content, error);
        });

        dispatch::Queue::global(0).unwrap().async_mut(move || {
            ShareableContent::current_with_ch_block(&mut bl);
        });

        sema.wait_forever();
    }
}
