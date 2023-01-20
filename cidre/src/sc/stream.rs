use std::{ffi::c_void, ops::Deref};

use crate::{
    arc, blocks, cg, cm, cv, define_obj_type, dispatch, ns,
    objc::{self, Delegate, Id},
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
    Audio,
}

define_obj_type!(Configuration(Id));

impl Configuration {
    /// ```no_run
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
    pub fn new() -> arc::R<Self> {
        unsafe { SCStreamConfiguration_new() }
    }

    #[objc::msg_send(width)]
    pub fn width(&self) -> usize;

    #[objc::msg_send(setWidth:)]
    pub fn set_width(&mut self, value: usize);

    #[objc::msg_send(height)]
    pub fn height(&self) -> usize;

    #[objc::msg_send(setHeight:)]
    pub fn set_height(&mut self, value: usize);

    #[objc::msg_send(minimumFrameInterval)]
    pub fn minimum_frame_interval(&self) -> cm::Time;

    #[objc::msg_send(setMinimumFrameInterval:)]
    pub fn set_minimum_frame_interval(&mut self, value: cm::Time);

    /// 'BGRA': Packed Little Endian ARGB8888
    /// 'l10r': Packed Little Endian ARGB2101010
    /// '420v': 2-plane "video" range YCbCr 4:2:0
    /// '420f': 2-plane "full" range YCbCr 4:2:0
    #[objc::msg_send(pixelFormat)]
    pub fn pixel_format(&self) -> cv::PixelFormatType;

    #[objc::msg_send(setPixelFormat:)]
    pub fn set_pixel_format(&mut self, value: cv::PixelFormatType);

    #[objc::msg_send(scalesToFit)]
    pub fn scales_to_fit(&self) -> bool;

    #[objc::msg_send(setScalesToFit:)]
    pub fn set_scales_to_fit(&mut self, value: bool);

    #[objc::msg_send(showsCursor)]
    pub fn shows_cursor(&self) -> bool;

    #[objc::msg_send(setShowsCursor:)]
    pub fn set_shows_cursor(&mut self, value: bool);

    #[objc::msg_send(backgroundColor)]
    pub fn background_color(&self) -> cg::Color;

    #[objc::msg_send(setBackgroundColor:)]
    pub fn set_background_color(&mut self, value: cg::Color);

    #[objc::msg_send(sourceRect)]
    pub fn source_rect(&self) -> cg::Rect;

    #[objc::msg_send(setSourceRect:)]
    pub fn set_source_rect(&mut self, value: cg::Rect);

    /// Specifies whether the audio will be captured.  By default audio is not captured.
    #[objc::msg_send(capturesAudio)]
    pub fn captures_audio(&self) -> bool;

    #[objc::msg_send(setCapturesAudio:)]
    pub fn set_captures_audio(&mut self, value: bool);

    /// The sample rate for audio. Default is set to 48000.
    #[objc::msg_send(sampleRate)]
    pub fn sample_rate(&self) -> i64;

    #[objc::msg_send(setSampleRate:)]
    pub fn set_sample_rate(&mut self, value: i64);

    /// Channel count. Default is set to two.
    #[objc::msg_send(channelCount)]
    pub fn channel_count(&self) -> i64;

    #[objc::msg_send(setChannelCount:)]
    pub fn set_channel_count(&mut self, value: i64);

    /// Whether to exclude audio from current process. Default is set to false.
    #[objc::msg_send(excludesCurrentProcessAudio)]
    pub fn excludes_current_process_audio(&self) -> bool;

    #[objc::msg_send(setExcludesCurrentProcessAudio:)]
    pub fn set_excludes_current_process_audio(&mut self, value: bool);
}

#[link(name = "ScreenCaptureKit", kind = "framework")]
extern "C" {}

#[link(name = "sc", kind = "static")]
extern "C" {
    fn SCStreamConfiguration_new() -> arc::R<Configuration>;

}

define_obj_type!(ContentFilter(Id));

impl ContentFilter {
    pub fn with_display_excluding_windows(
        display: &Display,
        windows: &ns::Array<Window>,
    ) -> arc::R<Self> {
        unsafe { SCContentFilter_initWithDisplay_excludingWindows(display, windows) }
    }
}

#[link(name = "sc", kind = "static")]
extern "C" {
    fn SCContentFilter_initWithDisplay_excludingWindows(
        display: &Display,
        windows: &ns::Array<Window>,
    ) -> arc::R<ContentFilter>;
}

define_obj_type!(Stream(Id));

pub trait StreamDelegate {
    extern "C" fn stream_did_stop_with_error(&mut self, stream: &Stream, error: Option<&ns::Error>);

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
        sample_buffer: &mut cm::SampleBuffer,
        of_type: OutputType,
    );

    fn delegate(self) -> Delegate<Self>
    where
        Self: Sized,
    {
        let b = Box::new(self);
        let table: [*const c_void; 2] = [
            b.as_ref() as *const _ as _,
            Self::stream_did_output_sample_buffer_of_type as _,
        ];

        let ptr = table.as_ptr();
        let obj = unsafe { make_stream_out(ptr as _) };

        Delegate { delegate: b, obj }
    }
}

#[link(name = "sc", kind = "static")]
extern "C" {
    fn make_stream_out(vtable: *const *const c_void) -> arc::R<Id>;
    fn make_stream_delegate(vtable: *const *const c_void) -> arc::R<Id>;
}

impl Stream {
    pub fn with_delegate<T>(
        filter: &ContentFilter,
        configuration: &Configuration,
        delegate: &Delegate<T>,
    ) -> arc::R<Self>
    where
        T: StreamDelegate,
    {
        let delegate = delegate.obj.deref();
        unsafe {
            SCStream_initWithFilter_configuration_delegate(filter, configuration, Some(delegate))
        }
    }

    pub fn new(filter: &ContentFilter, configuration: &Configuration) -> arc::R<Self> {
        unsafe { SCStream_initWithFilter_configuration_delegate(filter, configuration, None) }
    }

    pub fn add_stream_output<T>(
        &self,
        delegate: &Delegate<T>,
        output_type: OutputType,
        queue: Option<&dispatch::Queue>,
        error: &mut Option<&ns::Error>,
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

    pub fn start_with_completion_handler<F>(&self, block: &'static mut blocks::Block<F>)
    where
        F: FnOnce(Option<&'static ns::Error>),
    {
        unsafe {
            wsel_startCaptureWithCompletionHandler(self, block.as_ptr());
        }
    }

    pub fn stop_with_completion_handler<F>(&self, block: &'static mut blocks::Block<F>)
    where
        F: FnOnce(Option<&'static ns::Error>),
    {
        unsafe {
            wsel_stopCaptureWithCompletionHandler(self, block.as_ptr());
        }
    }

    pub async fn start(&self) -> Result<(), arc::R<ns::Error>> {
        let (future, block) = blocks::ok();
        self.start_with_completion_handler(block.escape());
        future.await
    }

    pub async fn stop(&self) -> Result<(), arc::R<ns::Error>> {
        let (future, block) = blocks::ok();
        self.stop_with_completion_handler(block.escape());
        future.await
    }
}

#[link(name = "sc", kind = "static")]
extern "C" {
    fn SCStream_initWithFilter_configuration_delegate(
        filter: &ContentFilter,
        configuration: &Configuration,
        delegate: Option<&Id>,
    ) -> arc::R<Stream>;

    fn rsel_addStreamOutput_type_sampleHandlerQueue_error(
        id: &Id,
        output: &Id,
        output_type: OutputType,
        queue: Option<&dispatch::Queue>,
        error: &mut Option<&ns::Error>,
    ) -> bool;

    fn test_start(id: &Id);

    fn wsel_startCaptureWithCompletionHandler(id: &Id, rb: *mut c_void);
    fn wsel_stopCaptureWithCompletionHandler(id: &Id, rb: *mut c_void);

}

#[cfg(test)]
mod tests {
    use std::time::Duration;

    use crate::{cm, cv, dispatch, ns, sc};

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
            _stream: &sc::Stream,
            _sample_buffer: &mut crate::cm::SampleBuffer,
            _of_type: sc::OutputType,
        ) {
            self.counter += 1;
            // why without println is not working well?
            println!("frame {:?}", self.counter);
        }
    }

    #[test]
    fn basics() {
        let mut cfg = sc::StreamConfiguration::new();

        cfg.set_width(200);
        assert_eq!(200, cfg.width());
        cfg.set_height(300);
        assert_eq!(300, cfg.height());

        cfg.set_minimum_frame_interval(cm::Time::new(1, 60));
        cfg.set_pixel_format(cv::PixelFormatType::_32_BGRA);
        cfg.set_scales_to_fit(false);
        cfg.set_shows_cursor(false);
    }

    #[tokio::test]
    async fn test_start_fails() {
        let q = dispatch::Queue::serial_with_autoreleasepool();
        let content = sc::ShareableContent::current().await.expect("content");
        let ref display = content.displays()[0];
        let mut cfg = sc::StreamConfiguration::new();
        cfg.set_width(display.width() as usize * 2);
        cfg.set_height(display.height() as usize * 2);

        let windows = ns::Array::new();
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
