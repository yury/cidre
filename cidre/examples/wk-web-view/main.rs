use std::ffi::c_void;

use cidre::{
    arc, blocks, define_obj_type, dispatch, ns, objc, objc::Obj, wk, wk::NavigationDelegate,
    wk::NavigationDelegateImpl,
};

define_obj_type!(
    AppDelegate + ns::ApplicationDelegateImpl,
    usize,
    APP_DELEGATE_CLS
);

impl ns::ApplicationDelegate for AppDelegate {}

#[objc::add_methods]
impl ns::ApplicationDelegateImpl for AppDelegate {}

define_obj_type!(
    NavDelegate + wk::NavigationDelegateImpl,
    usize,
    NAV_DELEGATE_CLS
);

impl NavigationDelegate for NavDelegate {}

#[objc::add_methods]
impl NavigationDelegateImpl for NavDelegate {
    extern "C" fn impl_web_view_did_finish_navigation(
        &mut self,
        _cmd: Option<&objc::Sel>,
        web_view: &mut wk::WebView,
        _navigation: Option<&wk::Navigation>,
    ) {
        let mut block = blocks::mut2(|result: Option<&ns::Id>, _error| {
            let r = result.unwrap();
            if let Some(str) = r.try_cast(ns::String::cls()) {
                println!("{}", str);
            }
        });
        let js = ns::String::with_str("document.body.innerHTML");
        web_view.eval_js_completion(&js, block.escape());
    }
}

extern "C" fn in_main(_ctx: *mut c_void) {
    let nav_delegate = unsafe { NAV_DELEGATE.as_ref() };
    eprintln!("{:?}", nav_delegate);
    let mut view = wk::WebView::new();
    view.set_inpectable(true);
    view.set_nav_delegate(nav_delegate.map(|f| f.as_ref()));
    let url = ns::Url::with_str("https://twitch.com").unwrap();
    let request = ns::UrlRequest::with_url(&url);
    view.load_request(&request);
    unsafe {
        WEB_VIEW = Some(view);
    }
}

static mut NAV_DELEGATE: Option<arc::R<NavDelegate>> = None;
static mut WEB_VIEW: Option<arc::R<wk::WebView>> = None;

fn main() {
    let app = ns::Application::shared();
    let appd = AppDelegate::with(0);
    let navd = NavDelegate::with(0);
    unsafe {
        NAV_DELEGATE = Some(navd);
    }
    dispatch::Queue::main().async_f(std::ptr::null_mut(), in_main);
    app.set_delegate(Some(appd.as_ref()));
    app.run();
}
