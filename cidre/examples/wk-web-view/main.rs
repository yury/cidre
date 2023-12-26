use std::ffi::c_void;

use cidre::{
    arc, blocks, define_obj_type, dispatch, ns, objc, objc::Obj, wk, wk::NavigationDelegate,
    wk::NavigationDelegateImpl,
};

define_obj_type!(
    AppDelegate + ns::ApplicationDelegateImpl,
    (),
    APP_DELEGATE_CLS
);

impl ns::ApplicationDelegate for AppDelegate {}

#[objc::add_methods]
impl ns::ApplicationDelegateImpl for AppDelegate {}

define_obj_type!(
    NavDelegate + wk::NavigationDelegateImpl,
    arc::R<wk::WebView>,
    NAV_DELEGATE_CLS
);

impl NavigationDelegate for NavDelegate {}

#[objc::add_methods]
impl NavigationDelegateImpl for NavDelegate {
    extern "C" fn impl_web_view_did_finish_navigation(
        &mut self,
        _cmd: Option<&objc::Sel>,
        web_view: &mut wk::WebView,
        navigation: Option<&wk::Navigation>,
    ) {
        let nav = navigation.map(wk::Navigation::retained);
        let mut block = blocks::mut2(move |res: Option<&ns::Id>, _error| {
            eprintln!("{:?}", nav);
            res.map(|id| {
                if let Some(str) = id.try_cast(ns::String::cls()) {
                    println!("{}", str);
                }
            });
        });

        let js = ns::String::with_str("document.body.innerHTML");
        web_view.eval_js_ch(&js, block.escape());
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

impl NavDelegate {
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

    extern "C" fn start_on_main(_ctx: *mut c_void) {
        let mut navd = NavDelegate::new();
        let url = ns::Url::with_str("https://twitch.com").unwrap();
        let request = ns::UrlRequest::with_url(&url);
        navd.load(&request);
        unsafe {
            NAV_DELEGATE = Some(navd);
        }
    }
}

static mut NAV_DELEGATE: Option<arc::R<NavDelegate>> = None;

impl AppDelegate {
    fn run() {
        let appd = Self::new();
        let app = ns::Application::shared();
        app.set_delegate(Some(appd.as_ref()));
        app.run();
    }
}

fn main() {
    dispatch::Queue::main().async_f(std::ptr::null_mut(), NavDelegate::start_on_main);
    AppDelegate::run()
}
