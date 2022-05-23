use std::{ffi::c_void, ops::Deref};

use crate::{
    cf::{self, Retained},
    cg, cm, cv, define_obj_type, dispatch, msg_send,
    objc::{block::Completion, Id, Delegate},
    os,
};

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
    /// cfg.set_pixel_format(cv::PixelFormatType::_32_BGRA);
    /// cfg.set_scales_to_fit(false);
    /// cfg.set_shows_cursor(false);
    ///
    /// ```
    pub fn new<'new>() -> Retained<'new, Configuration> {
        unsafe { SCStreamConfiguration_new() }
    }

    pub fn width(&self) -> usize {
        msg_send!("common", self, sel_width)
    }

    pub fn set_width(&mut self, value: usize) {
        msg_send!("common", self, sel_setWidth, value)
    }

    pub fn height(&self) -> usize {
        msg_send!("common", self, sel_height)
    }

    pub fn set_height(&mut self, value: usize) {
        msg_send!("common", self, sel_setHeight, value)
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
    pub fn with_display_excluding_windows<'a>(
        display: &Display,
        windows: &cf::ArrayOf<Window>,
    ) -> Retained<'a, ContentFilter> {
        unsafe { SCContentFilter_initWithDisplay_excludingWindows(display, windows) }
    }
}

#[link(name = "sc", kind = "static")]
extern "C" {
    fn SCContentFilter_initWithDisplay_excludingWindows<'a>(
        display: &Display,
        windows: &cf::ArrayOf<Window>,
    ) -> Retained<'a, ContentFilter>;
}

define_obj_type!(Stream(Id));

pub trait StreamDelegate {
    extern "C" fn stream_did_stop_with_error(&mut self, stream: &Stream, error: Option<&cf::Error>);

    fn delegate(self) -> Delegate<Self>
    where
        Self: Sized,
    {
        let b = Box::new(self);
        let table: [*const c_void; 2] = [
            b.as_ref() as *const _ as _,
            Self::stream_did_stop_with_error as _,
        ];

        let ptr = table.as_ptr();

        let obj = unsafe { make_stream_delegate(ptr as _) };

        Delegate { delegate: b, obj }
    }
}

pub trait StreamOutput {
    extern "C" fn stream_did_output_sample_buffer_of_type(
        &mut self,
        stream: &Stream,
        sample_buffer: &cm::SampleBuffer,
        of_type: OutputType,
    );

    fn delegate(self) -> Delegate<Self>
    where
        Self: Sized,
    {
        let b = Box::new(self);
        let table: [*const c_void; 2] = [
            b.as_ref() as *const _ as _,
            // &self as *const _ as *const _,
            Self::stream_did_output_sample_buffer_of_type as _,
        ];

        let ptr = table.as_ptr();
        let obj = unsafe { make_stream_out(ptr as _) };

        Delegate { delegate: b, obj }
    }
}

#[link(name = "sc", kind = "static")]
extern "C" {
    fn make_stream_out<'a>(vtable: *const *const c_void) -> Retained<'a, Id>;
    fn make_stream_delegate<'a>(vtable: *const *const c_void) -> Retained<'a, Id>;
}


impl Stream {
    pub fn with_delegate<'a, T>(
        filter: &ContentFilter,
        configuration: &Configuration,
        delegate: &Delegate<T>,
    ) -> Retained<'a, Self>
    where
        T: StreamDelegate,
    {
        let delegate = delegate.obj.deref();
        unsafe {
            SCStream_initWithFilter_configuration_delegate(filter, configuration, Some(delegate))
        }
    }

    pub fn new<'a>(filter: &ContentFilter, configuration: &Configuration) -> Retained<'a, Self> {
        unsafe { SCStream_initWithFilter_configuration_delegate(filter, configuration, None) }
    }

    pub fn add_stream_output<T>(
        &self,
        delegate: &Delegate<T>,
        output_type: OutputType,
        queue: Option<&dispatch::Queue>,
        error: &mut Option<&cf::Error>,
    ) -> bool
    where
        T: StreamOutput,
    {
        unsafe {
            rsel_addStreamOutput_type_sampleHandlerQueue_error(
                self,
                delegate.obj.deref(),
                output_type,
                queue,
                error,
            )
        }
    }

    pub fn start_sync(&self) {
        unsafe { test_start(self) }
    }

    pub async fn start<'a>(&self) -> Result<(), Retained<'a, cf::Error>> {
        let (future, block_ptr) = Completion::ok_or_error();
        unsafe {
            sel_startCaptureWithCompletionHandler(self, block_ptr);
        }
        future.await
    }

    pub async fn stop<'a>(&self) -> Result<(), Retained<'a, cf::Error>> {
        let (future, block_ptr) = Completion::ok_or_error();
        unsafe {
            sel_stopCaptureWithCompletionHandler(self, block_ptr);
        }
        future.await
    }
}

#[link(name = "sc", kind = "static")]
extern "C" {
    fn SCStream_initWithFilter_configuration_delegate<'a>(
        filter: &ContentFilter,
        configuration: &Configuration,
        delegate: Option<&Id>,
    ) -> Retained<'a, Stream>;

    fn rsel_addStreamOutput_type_sampleHandlerQueue_error(
        id: &Id,
        output: &Id,
        output_type: OutputType,
        queue: Option<&dispatch::Queue>,
        error: &mut Option<&cf::Error>,
    ) -> bool;

    fn test_start(id: &Id);

    fn sel_startCaptureWithCompletionHandler(id: &Id, rb: *const c_void);
    fn sel_stopCaptureWithCompletionHandler(id: &Id, rb: *const c_void);

}

#[cfg(test)]
mod tests {
    use std::time::Duration;

    use crate::{cf, dispatch, sc};

    use super::StreamOutput;

    #[repr(C)]
    struct FameCounter {
        counter: u32,
    }

    impl FameCounter {
        pub fn counter(&self) -> u32 {
            self.counter
        }
    }

    impl StreamOutput for FameCounter {
        extern "C" fn stream_did_output_sample_buffer_of_type(
            &mut self,
            stream: &sc::Stream,
            sample_buffer: &crate::cm::SampleBuffer,
            of_type: sc::OutputType,
        ) {
            self.counter += 1;
            // why without println is not working well?
            println!("frame {:?}", self.counter);
        }
    }

    #[tokio::test]
    async fn test_start_fails() {
        let q = dispatch::Queue::serial_with_autoreleasepool();
        let content = sc::ShareableContent::current().await.expect("content");
        let ref display = content.displays()[0];
        let mut cfg = sc::StreamConfiguration::new();
        cfg.set_width(display.width() as usize * 2);
        cfg.set_height(display.height() as usize * 2);

        let windows = cf::ArrayOf::<sc::Window>::new().unwrap();
        let filter = sc::ContentFilter::with_display_excluding_windows(display, &windows);
        let stream = sc::Stream::new(&filter, &cfg);
        let delegate = FameCounter { counter: 0 };
        let d = delegate.delegate();
        let mut error = None;
        stream.add_stream_output(&d, sc::OutputType::Screen, Some(&q), &mut error);
        assert!(error.is_none());
        stream.start().await.expect("started");
        stream.start().await.expect_err("already started");

        tokio::time::sleep(Duration::from_secs(2)).await;

        stream.stop().await.expect("stopped");
        stream.stop().await.expect_err("already stopped");
        println!(
            "------- {:?} {:?}",
            d.obj.as_type_ref(),
            d.delegate.counter()
        );

        assert!(d.delegate.counter() > 10, "{:?}", d.delegate.counter);
    }
}
