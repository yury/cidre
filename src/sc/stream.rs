use crate::{cf::{Retained, self}, cg, cm, cv, define_obj_type, objc::Id, os};

use super::{Display, Window};

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


define_obj_type!(Configuration(Id));

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
    /// cfg.set_shows_cursor(false);
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

    pub fn shows_cursor(&self) -> bool {
        unsafe { rsel_showsCursor(self) }
    }

    pub fn set_shows_cursor(&mut self, value: bool) {
        unsafe { wsel_setShowsCursor(self, value) }
    }
    pub fn background_color(&self) -> cg::Color {
        unsafe { rsel_backgroundColor(self) }
    }

    pub fn set_background_color(&mut self, value: cg::Color) {
        unsafe { wsel_setBackgroundColor(self, value) }
    }

    pub fn source_rect(&self) -> cg::Rect {
        unsafe { rsel_sourceRect(self) }
    }

    pub fn set_source_rect(&mut self, value: cg::Rect) {
        unsafe { wsel_setSourceRect(self, value) }
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

    fn rsel_showsCursor(id: &Id) -> bool;
    fn wsel_setShowsCursor(id: &Id, value: bool);

    fn rsel_backgroundColor(id: &Id) -> cg::Color;
    fn wsel_setBackgroundColor(id: &Id, value: cg::Color);

    fn rsel_sourceRect(id: &Id) -> cg::Rect;
    fn wsel_setSourceRect(id: &Id, value: cg::Rect);
}

define_obj_type!(ContentFilter(Id));

impl ContentFilter {
    pub fn with_display_excluding_windows<'a>(display: &Display, windows: &cf::ArrayOf<Window>) -> Retained<'a, ContentFilter> {
        unsafe {
            SCContentFilter_initWithDisplay_excludingWindows(display, windows)
        }
    }
}

#[link(name = "sc", kind = "static")]
extern "C" {
    fn SCContentFilter_initWithDisplay_excludingWindows<'a>(display: &Display, windows: &cf::ArrayOf<Window>) -> Retained<'a, ContentFilter>;
}

define_obj_type!(Stream(Id));