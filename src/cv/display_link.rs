use crate::{cf, cg, cv, define_cf_type};

define_cf_type!(DisplayLink(cf::Type));

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
}

extern "C" {
    fn CVDisplayLinkGetTypeID() -> cf::TypeId;

    fn CVDisplayLinkCreateWithCGDisplay<'a>(
        display_id: cg::DirectDisplayID,
        display_link_out: &mut Option<cf::Retained<'a, DisplayLink>>,
    ) -> cv::Return;

    fn CVDisplayLinkStart(link: &DisplayLink) -> cv::Return;
    fn CVDisplayLinkStop(link: &DisplayLink) -> cv::Return;
}
