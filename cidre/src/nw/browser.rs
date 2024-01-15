use std::ffi::c_void;

use crate::{arc, define_obj_type, dispatch, ns, nw};

#[cfg(feature = "blocks")]
use crate::blocks;

#[doc(alias = "nw_browser_state_t")]
#[derive(Debug, Eq, PartialEq, Copy, Clone)]
#[repr(i32)]
pub enum State {
    /// The state of the browser is not valid.
    ///
    /// This state will never be delivered in the browser's state update
    /// handler and can be treated as an unexpected value.
    #[doc(alias = "nw_browser_state_invalid")]
    Invalid = 0,

    /// The browser is ready and able to receive
    /// endpoint updates. All callbacks from the browse_results_changed_handler
    /// occur when the browser is in this state.
    #[doc(alias = "nw_browser_state_ready")]
    Ready = 1,

    /// The browser has irrecoverably failed.
    ///
    /// You should not try to call [`nw::Browser::start()`] on the browser to restart
    /// it. Instead, cancel the browser and create a new browser object.
    #[doc(alias = "nw_browser_state_failed")]
    Failed = 2,

    /// The browser has been cancelled by
    /// the caller. You should not try to call [`nw::Browser::start()`] on the
    /// browser to restart it. Instead, create a new browser object.
    #[doc(alias = "nw_browser_state_cancelled")]
    Cancelled = 3,

    /// The browser is waiting for connectivity.
    ///
    /// Results will not be delivered until the browser moves into the ready
    /// state. A browser can move from the ready state into the waiting state.
    /// The associated error indicates why the browser is unable to browse.
    #[doc(alias = "nw_browser_state_waiting")]
    Waiting = 4,
}

define_obj_type!(
    #[doc(alias = "nw_browser")]
    #[doc(alias = "nw_browser_t")]
    pub Browser(ns::Id)
);

unsafe impl Send for Browser {}
unsafe impl Sync for Browser {}

impl Browser {
    #[doc(alias = "nw_browser_create")]
    #[inline]
    pub fn with_desc(desc: &nw::BrowseDesc, params: Option<&nw::Params>) -> arc::R<Self> {
        unsafe { nw_browser_create(desc, params) }
    }

    #[doc(alias = "nw_browser_set_queue")]
    #[inline]
    fn set_queue(&mut self, val: &dispatch::Queue) {
        unsafe { nw_browser_set_queue(self, val) }
    }

    #[inline]
    pub fn start(&mut self, queue: &dispatch::Queue) {
        self.set_queue(queue);
        unsafe { nw_browser_start(self) }
    }

    #[doc(alias = "nw_browser_cancel")]
    #[inline]
    pub fn cancel(&mut self) {
        unsafe { nw_browser_cancel(self) }
    }

    #[doc(alias = "nw_browser_copy_parameters")]
    #[inline]
    pub fn params(&self) -> arc::R<nw::Params> {
        unsafe { nw_browser_copy_parameters(self) }
    }

    #[doc(alias = "nw_browser_copy_browse_descriptor")]
    #[inline]
    pub fn desc(&self) -> arc::R<nw::BrowseDesc> {
        unsafe { nw_browser_copy_browse_descriptor(self) }
    }

    #[doc(alias = "nw_browser_set_state_changed_handler")]
    #[cfg(feature = "blocks")]
    #[inline]
    pub fn set_state_changed_handler<'a, F>(
        &mut self,
        handler: Option<&'static mut blocks::Block<F>>,
    ) where
        F: FnMut(nw::BrowserState, Option<&'a nw::Error>),
    {
        unsafe {
            nw_browser_set_state_changed_handler(
                self,
                handler.map_or(std::ptr::null_mut(), |b| b.as_mut_ptr()),
            );
        }
    }

    #[doc(alias = "nw_browser_set_browse_results_changed_handler")]
    #[cfg(feature = "blocks")]
    #[inline]
    pub fn set_results_changed_handler<'a, F>(
        &mut self,
        handler: Option<&'static mut blocks::Block<F>>,
    ) where
        F: FnMut(&'a nw::BrowseResult, &'a nw::BrowseResult, bool),
    {
        unsafe {
            nw_browser_set_browse_results_changed_handler(
                self,
                handler.map_or(std::ptr::null_mut(), |b| b.as_mut_ptr()),
            );
        }
    }
}

#[link(name = "Network", kind = "framework")]
extern "C" {
    fn nw_browser_create(
        descriptor: &nw::BrowseDesc,
        params: Option<&nw::Params>,
    ) -> arc::R<Browser>;
    fn nw_browser_set_queue(browser: &mut Browser, val: &dispatch::Queue);
    fn nw_browser_start(browser: &mut Browser);
    fn nw_browser_cancel(browser: &mut Browser);
    fn nw_browser_copy_parameters(browser: &Browser) -> arc::R<nw::Params>;
    fn nw_browser_copy_browse_descriptor(browser: &Browser) -> arc::R<nw::BrowseDesc>;
    #[cfg(feature = "blocks")]
    fn nw_browser_set_browse_results_changed_handler(browser: &mut Browser, handler: *mut c_void);
    #[cfg(feature = "blocks")]
    fn nw_browser_set_state_changed_handler(browser: &mut Browser, handler: *mut c_void);
}

#[cfg(test)]
mod tests {
    use std::ffi::CString;

    use crate::{blocks, dispatch, nw};

    #[test]
    fn basics() {
        let queue = dispatch::Queue::new();
        let type_ = CString::new("_service._udp").unwrap();
        let desc = nw::BrowseDesc::create_bonjour_service(&type_, None);
        let mut browser = nw::Browser::with_desc(&desc, None);
        let mut state_handler = blocks::mut2(|state, error| {
            eprintln!("------ {:?} {:?}", state, error);
        });
        browser.set_state_changed_handler(Some(state_handler.escape()));
        let mut changes_handler = blocks::mut3(|old, new, _complete| {
            eprintln!("------ {:?} {:?}", old, new);
        });
        browser.set_results_changed_handler(Some(changes_handler.escape()));

        browser.start(&queue);
        eprintln!("browser {:?}", browser);
    }
}
