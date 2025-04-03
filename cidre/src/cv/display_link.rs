use std::{ffi::c_void, mem::transmute};

use crate::{arc, cf, cv, define_cf_type, os};

#[cfg(feature = "cg")]
use crate::cg;

define_cf_type!(DisplayLink(cf::Type));

#[doc(alias = "CVDisplayLinkOutputCallback")]
pub type OutputCb<T> = extern "C" fn(
    link: &DisplayLink,
    in_now: &cv::TimeStamp,
    in_output_time: &cv::TimeStamp,
    flags_in: cv::OptionFlags,
    flags_out: &mut cv::OptionFlags,
    user_info: *mut T,
) -> cv::Return;

impl DisplayLink {
    #[doc(alias = "CVDisplayLinkGetTypeID")]
    #[inline]
    pub fn type_id() -> cf::TypeId {
        unsafe { CVDisplayLinkGetTypeID() }
    }

    #[doc(alias = "CVDisplayLinkCreateWithCGDisplay")]
    #[cfg(feature = "cg")]
    #[inline]
    pub unsafe fn create_with_cg_display(
        display_id: cg::DirectDisplayId,
        display_link_out: *mut Option<arc::R<DisplayLink>>,
    ) -> cv::Return {
        unsafe { CVDisplayLinkCreateWithCGDisplay(display_id, display_link_out) }
    }

    /// ```
    /// use cidre::{cv, cg};
    ///
    /// let display_id = cg::main_display_id();
    /// let link = cv::DisplayLink::with_cg_display(display_id).unwrap();
    ///
    /// let err = link.start().unwrap_err();
    /// assert_eq!(err, cv::err::DISPLAY_LINK_CALLBACKS_NOT_SET);
    ///
    /// let time = link.nominal_output_video_refresh_period();
    /// println!("time {:?}", time);
    ///
    /// assert_eq!(false, link.is_running());
    ///
    /// let err = link.current_time().unwrap_err();
    /// assert_eq!(err, cv::err::DISPLAY_LINK_NOT_RUNNING);
    ///
    /// ```
    #[cfg(feature = "cg")]
    #[doc(alias = "CVDisplayLinkCreateWithCGDisplay")]
    pub fn with_cg_display(display_id: cg::DirectDisplayId) -> os::Result<arc::R<DisplayLink>> {
        unsafe { os::result_unchecked(|res| Self::create_with_cg_display(display_id, res)) }
    }

    #[doc(alias = "CVDisplayLinkStart")]
    pub fn start(&self) -> os::Result {
        unsafe { CVDisplayLinkStart(self).result() }
    }

    #[doc(alias = "CVDisplayLinkStop")]
    pub fn stop(&self) -> os::Result {
        unsafe { CVDisplayLinkStop(self).result() }
    }

    #[doc(alias = "CVDisplayLinkGetNominalOutputVideoRefreshPeriod")]
    pub fn nominal_output_video_refresh_period(&self) -> cv::Time {
        unsafe { CVDisplayLinkGetNominalOutputVideoRefreshPeriod(self) }
    }

    #[doc(alias = "CVDisplayLinkGetOutputVideoLatency")]
    pub fn output_video_latency(&self) -> cv::Time {
        unsafe { CVDisplayLinkGetOutputVideoLatency(self) }
    }

    #[doc(alias = "CVDisplayLinkIsRunning")]
    pub fn is_running(&self) -> bool {
        unsafe { CVDisplayLinkIsRunning(self) }
    }

    #[doc(alias = "CVDisplayLinkGetCurrentTime")]
    pub unsafe fn get_current_time(&self, out_time: *mut cv::TimeStamp) -> cv::Return {
        unsafe { CVDisplayLinkGetCurrentTime(self, out_time) }
    }

    #[doc(alias = "CVDisplayLinkGetCurrentTime")]
    pub fn current_time(&self) -> os::Result<cv::TimeStamp> {
        os::result_init(|res| unsafe { self.get_current_time(res) })
    }

    #[doc(alias = "CVDisplayLinkSetOutputCallback")]
    #[must_use]
    pub unsafe fn set_callback<T>(&self, callback: OutputCb<T>, user_info: *mut T) -> cv::Return {
        unsafe { CVDisplayLinkSetOutputCallback(self, transmute(callback), user_info as _) }
    }
}

unsafe extern "C-unwind" {
    fn CVDisplayLinkGetTypeID() -> cf::TypeId;

    #[cfg(feature = "cg")]
    fn CVDisplayLinkCreateWithCGDisplay(
        display_id: cg::DirectDisplayId,
        display_link_out: *mut Option<arc::R<DisplayLink>>,
    ) -> cv::Return;

    fn CVDisplayLinkStart(link: &DisplayLink) -> cv::Return;
    fn CVDisplayLinkStop(link: &DisplayLink) -> cv::Return;
    fn CVDisplayLinkIsRunning(link: &DisplayLink) -> bool;
    fn CVDisplayLinkGetNominalOutputVideoRefreshPeriod(link: &DisplayLink) -> cv::Time;
    fn CVDisplayLinkGetOutputVideoLatency(link: &DisplayLink) -> cv::Time;
    fn CVDisplayLinkGetCurrentTime(link: &DisplayLink, out_time: *mut cv::TimeStamp) -> cv::Return;
    fn CVDisplayLinkSetOutputCallback(
        link: &DisplayLink,
        callback: OutputCb<c_void>,
        user_info: *mut c_void,
    ) -> cv::Return;
}
