use crate::{arc, define_obj_type, dispatch, ns, nw};

#[doc(alias = "nw_browser_state_t")]
#[derive(Debug, Eq, PartialEq, Copy, Clone)]
#[repr(i32)]
pub enum State {
    Invalid = 0,
    Ready = 1,
    Failed = 2,
    Cancelled = 3,
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
