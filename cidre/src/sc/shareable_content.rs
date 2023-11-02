use std::ffi::c_void;

use crate::{arc, blocks, cg, define_cls, define_obj_type, ns, objc, sys};

define_obj_type!(RunningApp(ns::Id));

impl RunningApp {
    #[objc::msg_send(bundleIdentifier)]
    pub fn bundle_id(&self) -> &ns::String;

    #[objc::msg_send(applicationName)]
    pub fn app_name(&self) -> &ns::String;

    #[objc::msg_send(processID)]
    pub fn process_id(&self) -> sys::Pid;
}

define_obj_type!(Display(ns::Id));

impl Display {
    #[objc::msg_send(displayID)]
    pub fn display_id(&self) -> cg::DirectDisplayId;

    #[objc::msg_send(width)]
    pub fn width(&self) -> isize;

    #[objc::msg_send(height)]
    pub fn height(&self) -> isize;

    #[objc::msg_send(frame)]
    pub fn frame(&self) -> cg::Rect;
}

define_obj_type!(Window(ns::Id));

impl Window {
    #[objc::msg_send(windowID)]
    pub fn id(&self) -> cg::WindowId;

    #[objc::msg_send(frame)]
    pub fn frame(&self) -> cg::Rect;

    #[objc::msg_send(title)]
    pub fn title(&self) -> Option<&ns::String>;

    #[objc::msg_send(windowLayer)]
    pub fn window_layer(&self) -> ns::Integer;

    #[objc::msg_send(owningApplication)]
    pub fn owning_app(&self) -> Option<&RunningApp>;

    #[objc::msg_send(isOnScreen)]
    pub fn is_on_screen(&self) -> bool;

    #[objc::msg_send(isActive)]
    pub fn is_active(&self) -> bool;
}

impl std::fmt::Display for Window {
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

#[link(name = "sc", kind = "static")]
extern "C" {
    static SC_SHAREABLE_CONTENT: &'static objc::Class<ShareableContent>;
}

define_obj_type!(ShareableContent(ns::Id));

unsafe impl Send for ShareableContent {}

impl ShareableContent {
    define_cls!(SC_SHAREABLE_CONTENT);

    #[objc::msg_send(windows)]
    pub fn windows(&self) -> &ns::Array<Window>;

    #[objc::msg_send(displays)]
    pub fn displays(&self) -> &ns::Array<Display>;

    #[objc::msg_send(applications)]
    pub fn apps(&self) -> &ns::Array<RunningApp>;

    #[objc::cls_msg_send(getShareableContentWithCompletionHandler:)]
    pub fn get_shareable_content_with_ch(block: *mut c_void);

    pub fn current_with_ch<'ar, F>(b: &'static mut blocks::Block<F>)
    where
        F: FnOnce(Option<&'ar ShareableContent>, Option<&'ar ns::Error>),
    {
        Self::get_shareable_content_with_ch(b.as_mut_ptr());
    }

    #[cfg(all(feature = "blocks", feature = "async"))]
    pub async fn current() -> Result<arc::R<Self>, arc::R<ns::Error>> {
        let (future, block) = blocks::result();

        Self::current_with_ch(block.escape());

        future.await
    }
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

        let signal_guard = sema.signal_guard();
        let bl = blocks::once2(move |content, error| {
            signal_guard.consume();
            println!("nice {:?} {:?}", content, error);
        });

        dispatch::Queue::global(0).unwrap().async_once(move || {
            ShareableContent::current_with_ch(bl.escape());
        });

        sema.wait_forever();
    }
}
