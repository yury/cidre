use crate::{arc, define_obj_type, dispatch, ns, nw};

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

impl Browser {
    #[doc(alias = "nw_browser_set_queue")]
    #[inline]
    pub fn set_queue(&mut self, val: &dispatch::Queue) {
        unsafe { nw_browser_set_queue(self, val) }
    }

    #[doc(alias = "nw_browser_start")]
    #[inline]
    pub fn start(&mut self) {
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
}

#[link(name = "Network", kind = "framework")]
extern "C" {
    fn nw_browser_set_queue(browser: &mut Browser, val: &dispatch::Queue);
    fn nw_browser_start(browser: &mut Browser);
    fn nw_browser_cancel(browser: &mut Browser);
    fn nw_browser_copy_parameters(browser: &Browser) -> arc::R<nw::Params>;
    fn nw_browser_copy_browse_descriptor(browser: &Browser) -> arc::R<nw::BrowseDesc>;
}
