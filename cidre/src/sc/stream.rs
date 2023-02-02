use std::ffi::c_void;

use crate::{arc, blocks, cg, cm, cv, define_cls, define_obj_type, dispatch, ns, objc};

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

define_obj_type!(Configuration(ns::Id), SC_STREAM_CONFIGURATION);

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

#[link(name = "sc", kind = "static")]
extern "C" {
    static SC_STREAM_CONFIGURATION: &'static objc::Class<Configuration>;
    static SC_CONTENT_FILTER: &'static objc::Class<ContentFilter>;
    static SC_STREAM: &'static objc::Class<Stream>;
}

define_obj_type!(ContentFilter(ns::Id));

impl arc::A<ContentFilter> {
    #[objc::msg_send(initWithDisplay:excludingWindows:)]
    pub fn init_with_display_excluding_windows(
        self,
        display: &Display,
        windows: &ns::Array<Window>,
    ) -> arc::R<ContentFilter>;
}

impl ContentFilter {
    define_cls!(SC_CONTENT_FILTER);

    pub fn with_display_excluding_windows(
        display: &Display,
        windows: &ns::Array<Window>,
    ) -> arc::R<Self> {
        Self::alloc().init_with_display_excluding_windows(display, windows)
    }
}

define_obj_type!(Stream(ns::Id));

#[objc::obj_trait]
pub trait Output: objc::Obj {
    #[objc::optional]
    #[objc::msg_send(stream:didOutputSampleBuffer:ofType:)]
    fn stream_did_output_sample_buffer(
        &mut self,
        stream: &Stream,
        sample_bufer: &mut cm::SampleBuffer,
        kind: OutputType,
    );
}

#[objc::obj_trait]
pub trait Delegate: objc::Obj {
    #[objc::optional]
    #[objc::msg_send(stream:didStopWithError:)]
    fn stream_did_stop_with_error(&mut self, stream: &Stream, error: &ns::Error);
}

impl Delegate for objc::Any {}

impl arc::A<Stream> {
    #[objc::msg_send(initWithFilter:configuration:delegate:)]
    pub fn init_with_filter_configuration_delegate<D: Delegate>(
        self,
        filter: &ContentFilter,
        configuration: &Configuration,
        delegate: Option<&D>,
    ) -> arc::R<Stream>;
}

impl Stream {
    define_cls!(SC_STREAM);

    pub fn with_delegate<T, D: Delegate>(
        filter: &ContentFilter,
        configuration: &Configuration,
        delegate: &D,
    ) -> arc::R<Self> {
        Self::alloc().init_with_filter_configuration_delegate::<D>(
            filter,
            configuration,
            Some(&delegate),
        )
    }

    pub fn new(filter: &ContentFilter, configuration: &Configuration) -> arc::R<Self> {
        Self::alloc().init_with_filter_configuration_delegate::<objc::Any>(
            filter,
            configuration,
            objc::NONE,
        )
    }

    #[objc::msg_send(addStreamOutput:type:sampleHandlerQueue:error:)]
    fn add_stream_output_type_sample_handler_queue_error<D: Output>(
        &self,
        output: &D,
        output_type: OutputType,
        queue: Option<&dispatch::Queue>,
        error: &mut Option<&ns::Error>,
    ) -> bool;

    pub fn add_stream_output<'ar, D: Output>(
        &self,
        output: &D,
        output_type: OutputType,
        queue: Option<&dispatch::Queue>,
    ) -> Result<(), &'ar ns::Error> {
        let mut error = None;
        if self.add_stream_output_type_sample_handler_queue_error(
            output,
            output_type,
            queue,
            &mut error,
        ) {
            return Ok(());
        }
        Err(error.unwrap())
    }

    pub fn start_sync(&self) {
        unsafe { test_start(self) }
    }

    #[objc::msg_send(startCaptureWithCompletionHandler:)]
    fn _start_with_completion_handler(&self, rb: *mut c_void);

    pub fn start_with_completion_handler<F>(&self, block: &'static mut blocks::Block<F>)
    where
        F: FnOnce(Option<&'static ns::Error>),
    {
        self._start_with_completion_handler(block.as_ptr());
    }
    #[objc::msg_send(stopCaptureWithCompletionHandler:)]
    fn _stop_with_completion_handler(&self, rb: *mut c_void);

    pub fn stop_with_completion_handler<F>(&self, block: &'static mut blocks::Block<F>)
    where
        F: FnOnce(Option<&'static ns::Error>),
    {
        self._stop_with_completion_handler(block.as_ptr())
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
    fn test_start(id: &ns::Id);
}

#[cfg(test)]
mod tests {
    use std::time::Duration;

    use crate::{cm, cv, dispatch, ns, sc};

    #[repr(C)]
    struct FameCounter {
        counter: u32,
    }

    impl FameCounter {
        pub fn counter(&self) -> u32 {
            self.counter
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
