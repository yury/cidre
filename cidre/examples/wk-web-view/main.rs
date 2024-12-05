#[cfg(target_os = "macos")]
mod macos {
    use std::ffi::c_void;

    use cidre::{
        arc, define_obj_type, dispatch, ns, objc, objc::Obj, wk, wk::NavigationDelegate,
        wk::NavigationDelegateImpl,
    };

    define_obj_type!(AppD + ns::AppDelegateImpl, (), APP_DELEGATE_CLS);

    impl ns::AppDelegate for AppD {}

    #[objc::add_methods]
    impl ns::AppDelegateImpl for AppD {}

    define_obj_type!(
        NavD + wk::NavigationDelegateImpl,
        arc::R<wk::WebView>,
        NAV_DELEGATE_CLS
    );

    impl NavigationDelegate for NavD {}

    #[objc::add_methods]
    impl NavigationDelegateImpl for NavD {
        extern "C" fn impl_web_view_did_finish_navigation(
            &mut self,
            _cmd: Option<&objc::Sel>,
            web_view: &mut wk::WebView,
            navigation: Option<&wk::Navigation>,
        ) {
            let nav = navigation.map(wk::Navigation::retained);

            let js = ns::str!(c"document.body.innerHTML");
            web_view.eval_js_ch(&js, move |res, _err| {
                eprintln!("{:?}", nav);
                res.map(|id| {
                    if let Some(str) = id.try_cast(ns::String::cls()) {
                        println!("{}", str);
                    }
                });
            });
        }

        extern "C" fn impl_web_view_did_fail_navigation_err(
            &mut self,
            _cmd: Option<&objc::Sel>,
            _web_view: &mut wk::WebView,
            navigation: Option<&wk::Navigation>,
            error: &ns::Error,
        ) {
            eprintln!("navigation: {:?}", navigation);
            eprintln!("error: {:?}", error);
        }
    }

    impl NavD {
        fn new() -> arc::R<Self> {
            let mut web_view = wk::WebView::new();
            web_view.set_inpectable(true);
            let res = Self::with(web_view.retained());
            web_view.set_nav_delegate(Some(res.as_ref()));

            res
        }

        fn load(&mut self, request: &ns::UrlRequest) {
            self.inner_mut().load_request(request);
        }

        extern "C-unwind" fn start_on_main(_ctx: *mut c_void) {
            let mut navd = NavD::new();
            let url = ns::Url::with_str("https://twitch.com").unwrap();
            let request = ns::UrlRequest::with_url(&url);
            navd.load(&request);
            unsafe {
                NAV_DELEGATE = Some(navd);
            }
        }
    }

    static mut NAV_DELEGATE: Option<arc::R<NavD>> = None;

    impl AppD {
        fn run() {
            let appd = Self::new();
            let mut app = ns::App::shared();
            app.set_delegate(Some(appd.as_ref()));
            app.run();
        }
    }

    pub fn main() {
        dispatch::Queue::main().async_f(std::ptr::null_mut(), NavD::start_on_main);
        AppD::run()
    }
}

#[cfg(target_os = "macos")]
pub use macos::main;

#[cfg(not(target_os = "macos"))]
fn main() {
    todo!()
}
