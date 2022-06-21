use std::{ffi::c_void, intrinsics::transmute};

use crate::{cf, cg, cv, define_cf_type};

define_cf_type!(DisplayLink(cf::Type));

pub type OutputCallback<T> = extern "C" fn(
    link: &DisplayLink,
    in_now: &cv::TimeStamp,
    in_output_time: &cv::TimeStamp,
    flags_in: cv::OptionFlags,
    flags_out: &mut cv::OptionFlags,
    user_info: *mut T,
) -> cv::Return;

impl DisplayLink {
    pub fn type_id() -> cf::TypeId {
        unsafe { CVDisplayLinkGetTypeID() }
    }

    #[inline]
    pub unsafe fn create_with_cg_display<'a>(
        display_id: cg::DirectDisplayID,
        display_link_out: &mut Option<cf::Retained<'a, DisplayLink>>,
    ) -> cv::Return {
        CVDisplayLinkCreateWithCGDisplay(display_id, display_link_out)
    }

    /// ```
    /// use cidre::{cv, cg};
    ///
    /// let display_id = cg::main_display_id();
    /// let link = cv::DisplayLink::with_cg_display(display_id).unwrap();
    ///
    /// let err = link.start().unwrap_err();
    /// assert_eq!(err, cv::Return::DISPLAY_LINK_CALLBACKS_NOT_SET);
    ///
    /// let time = link.nominal_output_video_refresh_period();
    /// println!("time {:?}", time);
    ///
    /// assert_eq!(false, link.is_running());
    ///
    /// let err = link.current_time().unwrap_err();
    /// assert_eq!(err, cv::Return::DISPLAY_LINK_NOT_RUNNING);
    ///
    /// ```
    pub fn with_cg_display<'a>(
        display_id: cg::DirectDisplayID,
    ) -> Result<cf::Retained<'a, DisplayLink>, cv::Return> {
        unsafe {
            let mut display_link_out = None;
            Self::create_with_cg_display(display_id, &mut display_link_out)
                .to_result(display_link_out)
        }
    }

    pub fn start(&self) -> Result<(), cv::Return> {
        unsafe { CVDisplayLinkStart(self).into() }
    }

    pub fn stop(&self) -> Result<(), cv::Return> {
        unsafe { CVDisplayLinkStop(self).into() }
    }

    pub fn nominal_output_video_refresh_period(&self) -> cv::Time {
        unsafe { CVDisplayLinkGetNominalOutputVideoRefreshPeriod(self) }
    }

    pub fn output_video_latency(&self) -> cv::Time {
        unsafe { CVDisplayLinkGetOutputVideoLatency(self) }
    }

    pub fn is_running(&self) -> bool {
        unsafe { CVDisplayLinkIsRunning(self) }
    }

    pub unsafe fn get_current_time(&self, out_time: &mut cv::TimeStamp) -> cv::Return {
        CVDisplayLinkGetCurrentTime(self, out_time)
    }

    pub fn current_time(&self) -> Result<cv::TimeStamp, cv::Return> {
        unsafe {
            let mut out_time = cv::TimeStamp::default();
            let res = self.get_current_time(&mut out_time);
            if res.is_ok() {
                Ok(out_time)
            } else {
                Err(res)
            }
        }
    }

    pub unsafe fn set_callback<T>(
        &self,
        callback: OutputCallback<T>,
        user_info: *mut T,
    ) -> cv::Return {
        CVDisplayLinkSetOutputCallback(self, transmute(callback), user_info as _)
    }
}

extern "C" {
    fn CVDisplayLinkGetTypeID() -> cf::TypeId;

    fn CVDisplayLinkCreateWithCGDisplay<'a>(
        display_id: cg::DirectDisplayID,
        display_link_out: &mut Option<cf::Retained<'a, DisplayLink>>,
    ) -> cv::Return;

    fn CVDisplayLinkStart(link: &DisplayLink) -> cv::Return;
    fn CVDisplayLinkStop(link: &DisplayLink) -> cv::Return;
    fn CVDisplayLinkIsRunning(link: &DisplayLink) -> bool;
    fn CVDisplayLinkGetNominalOutputVideoRefreshPeriod(link: &DisplayLink) -> cv::Time;
    fn CVDisplayLinkGetOutputVideoLatency(link: &DisplayLink) -> cv::Time;
    fn CVDisplayLinkGetCurrentTime(link: &DisplayLink, out_time: &mut cv::TimeStamp) -> cv::Return;
    fn CVDisplayLinkSetOutputCallback(
        link: &DisplayLink,
        callback: OutputCallback<c_void>,
        user_info: *mut c_void,
    ) -> cv::Return;
}
