use crate::{cf::Retained, cm, cv, define_obj_type, objc::Id, os};

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
    /// use cidre::{sc, cm, cv};
    ///
    /// let mut cfg = sc::StreamConfiguration::new();
    ///
    /// cfg.set_width(200);
    /// assert_eq!(200, cfg.width());
    /// cfg.set_height(300);
    /// assert_eq!(300, cfg.height());
    ///
    /// cfg.set_minimum_frame_interval(cm::Time::new(1, 60));
    /// cfg.set_pixel_format(cv::PixelFormatType::_32BGRA);
    /// cfg.set_scales_to_fit(false);
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
        unsafe { rsel_minimumFrameInterval(self) }
    }

    pub fn set_minimum_frame_interval(&mut self, value: cm::Time) {
        unsafe { wsel_setMinimumFrameInterval(self, value) }
    }

    /// 'BGRA': Packed Little Endian ARGB8888
    /// 'l10r': Packed Little Endian ARGB2101010
    /// '420v': 2-plane "video" range YCbCr 4:2:0
    /// '420f': 2-plane "full" range YCbCr 4:2:0
    pub fn pixel_format(&self) -> cv::PixelFormatType {
        unsafe { cv::PixelFormatType(sc_rsel_pixelFormat(self)) }
    }

    pub fn set_pixel_format(&mut self, value: cv::PixelFormatType) {
        unsafe { sc_wsel_setPixelFormat(self, value.0) }
    }

    pub fn scales_to_fit(&self) -> bool {
        unsafe { rsel_scalesToFit(self) }
    }

    pub fn set_scales_to_fit(&mut self, value: bool) {
        unsafe { wsel_setScalesToFit(self, value) }
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
    fn sc_rsel_pixelFormat(id: &Id) -> os::Type;
    fn sc_wsel_setPixelFormat(id: &mut Id, value: os::Type);

    fn rsel_scalesToFit(id: &Id) -> bool;
    fn wsel_setScalesToFit(id: &Id, value: bool);
}
