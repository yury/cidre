use crate::{cf::Retained, define_obj_type, objc::Id, cm};

#[derive(Debug, PartialEq, Eq)]
#[repr(isize)]
pub enum Status {
    Complete,
    Idle,
    Blank,
    Suspended,
    Started,
    Stopped,
}

#[derive(Debug, PartialEq, Eq)]
#[repr(isize)]
pub enum OutputType {
    Screen,
}

define_obj_type!(ContentFilter(Id));
define_obj_type!(Configuration(Id));
define_obj_type!(Stream(Id));

impl Configuration {
    /// ```
    /// use cidre::{sc, cm};
    ///
    /// let mut cfg = sc::StreamConfiguration::new();
    /// 
    /// cfg.set_width(200);
    /// assert_eq!(200, cfg.width());
    /// cfg.set_height(300);
    /// assert_eq!(300, cfg.height());
    /// 
    /// cfg.set_minimum_frame_interval(cm::Time::new(1, 60))
    ///
    /// ```
    pub fn new<'new>() -> Retained<'new, Configuration> {
        unsafe { SCStreamConfiguration_new() }
    }

    pub fn width(&self) -> usize {
        unsafe { sc_rsel_width(self) }
    }

    pub fn set_width(&mut self, value: usize) {
        unsafe { sc_wsel_setWidth(self, value) }
    }

    pub fn height(&self) -> usize {
        unsafe { sc_rsel_height(self) }
    }

    pub fn set_height(&mut self, value: usize) {
        unsafe { sc_wsel_setHeight(self, value) }
    }

    pub fn minimum_frame_interval(&self) -> cm::Time {
      unsafe {
        rsel_minimumFrameInterval(self)
      }
    }

    pub fn set_minimum_frame_interval(&mut self, value: cm::Time) {
      unsafe {
        wsel_setMinimumFrameInterval(self, value)
      }
    }
}

#[link(name = "ScreenCaptureKit", kind = "framework")]
extern "C" {}

#[link(name = "sc", kind = "static")]
extern "C" {
    fn SCStreamConfiguration_new<'new>() -> Retained<'new, Configuration>;

    fn sc_rsel_width(id: &Id) -> usize;
    fn sc_wsel_setWidth(id: &Id, value: usize);
    fn sc_rsel_height(id: &Id) -> usize;
    fn sc_wsel_setHeight(id: &Id, value: usize);

    fn rsel_minimumFrameInterval(id: &Id) -> cm::Time;
    fn wsel_setMinimumFrameInterval(id: &Id, value: cm::Time);
}
