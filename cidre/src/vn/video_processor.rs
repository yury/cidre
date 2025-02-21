use crate::{arc, cf, cm, define_obj_type, ns, objc, vn};

define_obj_type!(
    /// An object that defines the cadence at which the video stream is processed.
    #[doc(alias = "VNVideoProcessorCadence")]
    pub Cadence(ns::Id)
);

define_obj_type!(
    /// An object that defines a frame-based cadence for processing the video stream.
    #[doc(alias = "VNVideoProcessorFrameRateCadence")]
    pub FrameRateCadence(Cadence)
);

define_obj_type!(
    /// An object that defines a time-based cadence for processing the video stream.
    #[doc(alias = "VNVideoProcessorTimeIntervalCadence")]
    pub TimeIntervalCadence(Cadence)
);

impl arc::A<FrameRateCadence> {
    #[objc::msg_send(initWithFrameRate:)]
    pub fn init_with_frame_rate(self, frame_rate: isize) -> arc::R<FrameRateCadence>;
}

impl FrameRateCadence {
    crate::define_cls!(VN_VIDEO_PROCESSOR_FRAME_RATE_CADENCE);

    pub fn new(frame_rate: isize) -> arc::R<Self> {
        Self::alloc().init_with_frame_rate(frame_rate)
    }

    /// The frame rate at which to process video.
    #[objc::msg_send(frameRate)]
    pub fn frame_rate(&self) -> isize;
}

impl arc::A<TimeIntervalCadence> {
    #[objc::msg_send(initWithTimeInterval:)]
    pub fn init_with_time_interval(
        self,
        time_interval: cf::TimeInterval,
    ) -> arc::R<TimeIntervalCadence>;
}

impl TimeIntervalCadence {
    crate::define_cls!(VN_VIDEO_PROCESSOR_TIME_INTERVAL_CADENCE);

    pub fn new(time_interval: cf::TimeInterval) -> arc::R<Self> {
        Self::alloc().init_with_time_interval(time_interval)
    }

    /// The time interval of the cadence.
    #[objc::msg_send(timeInterval)]
    pub fn time_interval(&self) -> cf::TimeInterval;
}

define_obj_type!(
    #[doc(alias = "VNVideoProcessorRequestProcessingOptions")]
    pub RequestProcessingOpts(ns::Id),
    VN_VIDEO_PROCESSOR_REQUEST_PROCESSING_OPTIONS
);

impl RequestProcessingOpts {
    #[objc::msg_send(cadence)]
    pub fn cadence(&self) -> Option<arc::R<Cadence>>;

    #[objc::msg_send(setCadence:)]
    pub fn set_cadence(&mut self, val: Option<&Cadence>);
}

define_obj_type!(
    #[doc(alias = "VNVideoProcessor")]
    pub VideoProcessor(ns::Id)
);

impl arc::A<VideoProcessor> {
    #[objc::msg_send(initWithURL:)]
    #[objc::available(
        macos = 11.0,
        maccatalyst = 14.0,
        ios = 14.0,
        tvos = 14.0,
        visionos = 1.0
    )]
    pub fn init_with_url(self, url: &ns::Url) -> arc::R<VideoProcessor>;
}

impl VideoProcessor {
    crate::define_cls!(VN_VIDEO_PROCESSOR);

    /// Creates a video processor to perform Vision requests against the specified video asset.
    #[objc::available(
        macos = 11.0,
        maccatalyst = 14.0,
        ios = 14.0,
        tvos = 14.0,
        visionos = 1.0
    )]
    pub fn with_url(url: &ns::Url) -> arc::R<Self> {
        Self::alloc().init_with_url(url)
    }

    #[objc::msg_send(addRequest:processingOptions:error:)]
    pub fn add_request_err<'ear>(
        &mut self,
        request: &vn::Request,
        processing_opts: &RequestProcessingOpts,
        error: *mut Option<&'ear ns::Error>,
    ) -> bool;

    pub fn add_request<'ear>(
        &mut self,
        request: &vn::Request,
        processing_opts: &RequestProcessingOpts,
    ) -> ns::Result<'ear> {
        ns::if_false(|err| self.add_request_err(request, processing_opts, err))
    }

    pub fn add_request_with_cadence<'ear>(
        &mut self,
        request: &vn::Request,
        cadence: &Cadence,
    ) -> ns::Result<'ear> {
        let mut opts = RequestProcessingOpts::new();
        opts.set_cadence(Some(cadence));
        self.add_request(request, &opts)
    }

    #[objc::msg_send(removeRequest:error:)]
    pub fn remove_request_err<'ear>(
        &mut self,
        request: &vn::Request,
        error: *mut Option<&'ear ns::Error>,
    ) -> bool;

    pub fn remove_request<'ear>(&mut self, request: &vn::Request) -> ns::Result<'ear> {
        ns::if_false(|err| self.remove_request_err(request, err))
    }

    #[objc::msg_send(analyzeTimeRange:error:)]
    pub fn analyze_time_range_err<'ear>(
        &mut self,
        time_range: cm::TimeRange,
        error: *mut Option<&'ear ns::Error>,
    ) -> bool;

    pub fn analyze_time_range<'ear>(&mut self, time_range: cm::TimeRange) -> ns::Result<'ear> {
        ns::if_false(|err| self.analyze_time_range_err(time_range, err))
    }

    pub fn analyze<'ear>(&mut self) -> ns::Result<'ear> {
        self.analyze_time_range(cm::TimeRange {
            start: cm::Time::zero(),
            duration: cm::Time::indefinit(),
        })
    }

    #[objc::msg_send(cancel)]
    pub fn cancel(&mut self);
}

#[link(name = "vn", kind = "static")]
unsafe extern "C" {
    static VN_VIDEO_PROCESSOR: &'static objc::Class<VideoProcessor>;
    static VN_VIDEO_PROCESSOR_FRAME_RATE_CADENCE: &'static objc::Class<FrameRateCadence>;
    static VN_VIDEO_PROCESSOR_TIME_INTERVAL_CADENCE: &'static objc::Class<TimeIntervalCadence>;
    static VN_VIDEO_PROCESSOR_REQUEST_PROCESSING_OPTIONS:
        &'static objc::Class<RequestProcessingOpts>;
}
