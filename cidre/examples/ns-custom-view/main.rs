//! A custom `NSView` subclass defined from Rust.
//!
//! `View` overrides `layout` (re-positions a label and shows the current size),
//! `viewDidMoveToWindow` and `isFlipped`, calling the `NSView` implementations
//! through the generated `super_*` methods, and declares `initWithFrame:`.
#[cfg(target_os = "macos")]
mod macos {
    use cidre::{arc, cg, define_obj_type, ns, ns::AppDelegate, objc};

    struct ViewInner {
        label: arc::R<ns::TextField>,
        layouts: usize,
    }

    define_obj_type!(View(ns::View), ViewInner, CIDRE_CUSTOM_VIEW);

    #[objc::add_methods]
    impl View {
        /// `initWithFrame:`, callable on `arc::A<View>` through the generated `ViewInit`
        #[objc::init(initWithFrame:)]
        fn init_with_frame(self, frame: ns::Rect) -> arc::R<Self>;

        fn with_frame(frame: ns::Rect) -> arc::R<Self> {
            let label = ns::TextField::label(ns::str!(c""));
            let inner = ViewInner { label, layouts: 0 };
            let mut view = Self::alloc_with(inner).init_with_frame(frame);
            let label = view.inner().label.retained();
            view.add_subview(&label);
            view
        }

        #[objc::overrides(layout)]
        fn layout(&mut self) {
            self.super_layout();

            let bounds = self.bounds();
            let inner = self.inner_mut();
            inner.layouts += 1;
            let text = format!(
                "{} x {} (layout #{})",
                bounds.size.width, bounds.size.height, inner.layouts
            );
            inner.label.set_string_value(&ns::String::with_str(&text));
            let size = cg::Size::new(bounds.size.width, 24.0);
            let origin = cg::Point::new(0.0, (bounds.size.height - size.height) * 0.5);
            inner.label.set_frame(cg::Rect { origin, size });
        }

        #[objc::overrides(viewDidMoveToWindow)]
        fn view_did_move_to_window(&mut self) {
            self.super_view_did_move_to_window();
            println!("view did move to window: {:?}", self.window());
        }

        #[objc::overrides(isFlipped)]
        fn is_flipped(&self) -> bool {
            true
        }
    }

    define_obj_type!(
        AppD + ns::AppDelegateImpl,
        (),
        CIDRE_CUSTOM_VIEW_APP_DELEGATE
    );

    impl ns::AppDelegate for AppD {}

    #[objc::add_methods]
    impl ns::AppDelegateImpl for AppD {
        extern "C" fn impl_app_did_finish_launching(
            &mut self,
            _cmd: Option<&objc::Sel>,
            _n: &ns::Notification,
        ) {
            let rect = ns::Rect::new(0.0, 0.0, 480.0, 320.0);
            let mut window = ns::Window::with_content_rect_style_mask_backing_defer(
                rect,
                ns::WindowStyleMask::TITLED
                    | ns::WindowStyleMask::CLOSABLE
                    | ns::WindowStyleMask::RESIZABLE,
                ns::BackingStoreType::Buffered,
                false,
            );
            window.set_title(ns::str!(c"cidre custom NSView"));
            let view = View::with_frame(rect);
            window.set_content_view(Some(&view));
            window.center();
            window.make_key_and_order_front(None);

            let mut app = ns::App::shared();
            app.activate_ignoring_other_apps(true);

            unsafe {
                WINDOW = Some(window);
            }
        }
    }

    static mut WINDOW: Option<arc::R<ns::Window>> = None;

    pub fn main() {
        let delegate = AppD::new();
        let mut app = ns::App::shared();
        app.set_activation_policy(ns::AppActivationPolicy::Regular);
        app.set_delegate(Some(delegate.as_ref()));
        app.run();
    }
}

#[cfg(target_os = "macos")]
pub use macos::main;

#[cfg(not(target_os = "macos"))]
fn main() {
    todo!()
}
