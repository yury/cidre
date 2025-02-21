use crate::{arc, av, cm, define_cls, define_obj_type, ns, objc, ut};

use super::WriterInput;

#[doc(alias = "AVAssetWriterStatus")]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(isize)]
pub enum Status {
    /// Indicates that the status of the asset writer is not currently known.
    Unknown = 0,
    /// Indicates that the asset writer is successfully writing samples to its output file.
    Writing = 1,
    /// Indicates that the asset writer has successfully written all samples following a call to finishWriting.
    Completed = 2,
    /// Indicates that the asset writer can no longer write samples to its output file because of an error. The error is described by the value of the asset writer's error property.
    Failed = 3,
    /// Indicates that the asset writer can no longer write samples because writing was canceled with the cancelWriting method.
    Cancelled = 4,
}

define_obj_type!(
    #[doc(alias = "AVAssetWriter")]
    pub Writer(ns::Id)
);

impl arc::A<Writer> {
    #[objc::msg_send(initWithURL:fileType:error:)]
    pub fn init_with_url_file_type_err<'ear>(
        self,
        url: &ns::Url,
        file_type: &av::FileType,
        error: *mut Option<&'ear ns::Error>,
    ) -> Option<arc::R<Writer>>;

    #[objc::msg_send(initWithContentType:)]
    pub unsafe fn init_with_content_type_throws(
        self,
        output_content_type: &ut::Type,
    ) -> arc::R<Writer>;
}

impl Writer {
    define_cls!(AV_ASSET_WRITER);

    #[objc::msg_send(shouldOptimizeForNetworkUse)]
    pub fn should_optimize_for_network_use(&self) -> bool;

    #[objc::msg_send(setShouldOptimizeForNetworkUse:)]
    pub fn set_should_optimize_for_network_use(&mut self, value: bool);

    #[objc::msg_send(canAddInput:)]
    pub fn can_add_input(&self, input: &WriterInput) -> bool;

    #[objc::msg_send(addInput:)]
    pub unsafe fn add_input_throws(&mut self, input: &WriterInput);

    pub fn add_input(&mut self, input: &WriterInput) -> ns::ExResult {
        ns::try_catch(|| unsafe { self.add_input_throws(input) })
    }

    /// Prepares the receiver for accepting input and for writing its output to its output file.
    #[objc::msg_send(startWriting)]
    pub fn start_writing(&mut self) -> bool;

    #[objc::msg_send(startSessionAtSourceTime:)]
    pub fn start_session_at_src_time(&mut self, start_time: cm::Time);

    #[objc::msg_send(endSessionAtSourceTime:)]
    pub fn end_session_at_src_time(&mut self, end_time: cm::Time);

    #[objc::msg_send(finishWriting)]
    pub fn finish_writing(&mut self);

    #[objc::msg_send(cancelWriting)]
    pub fn cancel_writing(&mut self);

    #[objc::msg_send(status)]
    pub fn status(&self) -> Status;

    #[objc::msg_send(error)]
    pub fn error(&self) -> Option<arc::R<ns::Error>>;

    #[objc::msg_send(inputs)]
    pub fn inputs(&self) -> arc::R<ns::Array<WriterInput>>;

    /// ```
    /// use cidre::{av, ns};
    /// let url = ns::Url::with_str("file://tmp/bla.mp4").unwrap();
    ///
    /// let writer = av::AssetWriter::with_url_and_file_type(&url, av::FileType::mp4()).unwrap();
    /// assert_eq!(writer.inputs().len(), 0);
    /// ```
    pub fn with_url_and_file_type<'ear>(
        url: &ns::Url,
        file_type: &av::FileType,
    ) -> ns::Result<'ear, arc::R<Self>> {
        ns::if_none(|err| Self::alloc().init_with_url_file_type_err(url, file_type, err))
    }

    pub fn with_content_type<'ear>(
        output_content_type: &ut::Type,
    ) -> ns::ExResult<'ear, arc::R<Self>> {
        ns::try_catch(|| unsafe {
            Self::alloc().init_with_content_type_throws(output_content_type)
        })
    }
}

/// AVAssetWriterSegmentation
impl Writer {
    #[objc::msg_send(preferredOutputSegmentInterval)]
    pub fn preferred_output_segment_interval(&self) -> cm::Time;

    /// Specifies preferred segment interval.
    #[objc::msg_send(setPreferredOutputSegmentInterval:)]
    pub fn set_preferred_output_segment_interval(&mut self, val: cm::Time);

    #[objc::msg_send(initialSegmentStartTime)]
    pub fn initial_segment_start_time(&self) -> cm::Time;

    /// Specifies start time of initial segment.
    ///
    /// A numeric time must be set if the value of preferredOutputSegmentInterval property
    /// is positive numeric. If not, this property is irrelevant.
    ///
    /// This property cannot be set after writing has started.
    /// TODO: check throws
    #[objc::msg_send(setInitialSegmentStartTime:)]
    pub fn set_initial_segment_start_time(&mut self, val: cm::Time);

    #[objc::msg_send(outputFileTypeProfile)]
    pub fn output_file_type_profile(&self) -> Option<arc::R<av::FileTypeProfile>>;

    /// TODO: check throws
    #[objc::msg_send(setOutputFileTypeProfile:)]
    pub fn set_output_file_type_profile(&mut self, val: Option<&av::FileTypeProfile>);

    #[objc::msg_send(delegate)]
    pub fn delegate(&self) -> Option<arc::R<AnyDelegate>>;

    #[objc::msg_send(setDelegate:)]
    pub fn set_delegate<D: Delegate>(&mut self, val: Option<&D>);

    /// This method throws an exception if the delegate method to output segment data is not implemented,
    /// or if the value of the preferredOutputSegmentInterval property is not kCMTimeIndefinite.
    #[objc::msg_send(flushSegment)]
    pub unsafe fn flush_segment_throws(&mut self);

    /// Closes the current segment and outputs it to the -assetWriter:didOutputSegmentData:segmentType:segmentReport:
    /// or -assetWriter:didOutputSegmentData:segmentType: delegate method.
    pub fn flush_segment(&mut self) -> ns::ExResult {
        ns::try_catch(|| unsafe { self.flush_segment_throws() })
    }
}

/// AVAssetWriterFileTypeSpecificProperties
impl Writer {
    /// The interval at which to write movie fragments.
    ///
    /// Some container formats, such as QuickTime movies, support writing movies
    /// in fragments. Using this feature enables you to open and play a partially
    /// written movie in the event that an unexpected error or interruption occurs.
    #[objc::msg_send(movieFragmentInterval)]
    pub fn movie_fragment_interval(&self) -> cm::Time;

    /// This property cannot be set after writing has started.
    #[objc::msg_send(setMovieFragmentInterval:)]
    pub fn set_movie_fragment_interval(&mut self, val: cm::Time);

    /// For file types that support movie fragments, specifies the interval at which initial movie
    /// fragment should be written.
    ///
    /// This property is irrelevant if the movieFragmentInterval property is not set.
    /// The default value is kCMTimeInvalid, which indicates that the interval for
    /// initial movie fragment is same as the one specified by movie_fragment_interval
    /// property.
    #[objc::msg_send(initialMovieFragmentInterval)]
    pub fn initial_movie_fragment_interval(&self) -> cm::Time;

    /// This property cannot be set after writing has started.
    #[objc::msg_send(setInitialMovieFragmentInterval:)]
    pub fn set_initial_movie_fragment_interval(&mut self, val: cm::Time);

    #[objc::msg_send(initialMovieFragmentSequenceNumber)]
    pub fn initial_movie_fragment_sequence_num(&self) -> isize;

    /// This property cannot be set after writing has started.
    #[objc::msg_send(setInitialMovieFragmentSequenceNumber:)]
    pub fn set_initial_movie_fragment_sequence_num(&mut self, val: isize);

    #[objc::msg_send(producesCombinableFragments)]
    pub fn produces_combinable_fragments(&self) -> bool;

    /// This property cannot be set after writing has started.
    #[objc::msg_send(setProducesCombinableFragments:)]
    pub fn set_produces_combinable_fragments(&mut self, val: bool);

    #[objc::msg_send(overallDurationHint)]
    pub fn overall_duration_hint(&self) -> cm::Time;

    #[objc::msg_send(setOverallDurationHint:)]
    pub fn set_overall_duration_hint(&mut self, val: cm::Time);

    #[objc::msg_send(movieTimeScale)]
    pub fn movie_time_scale(&self) -> cm::TimeScale;

    #[objc::msg_send(setMovieTimeScale:)]
    pub fn set_movie_time_scale(&self, val: cm::TimeScale);
}

#[objc::protocol(AVAssetWriterDelegate)]
pub trait Delegate: objc::Obj {
    #[objc::optional]
    #[objc::msg_send(assetWriter:didOutputSegmentData:segmentType:segmentReport:)]
    fn asset_writer_did_output_segment_data_with_report(
        &mut self,
        writer: &av::AssetWriter,
        segment_data: &ns::Data,
        segment_type: av::AssetSegmentType,
        segment_report: Option<&av::AssetSegmentReport>,
    );

    #[objc::optional]
    #[objc::msg_send(assetWriter:didOutputSegmentData:segmentType:)]
    fn asset_writer_did_output_segment_data(
        &mut self,
        writer: &av::AssetWriter,
        segment_data: &ns::Data,
        segment_type: av::AssetSegmentType,
    );
}

define_obj_type!(pub AnyDelegate(ns::Id));
impl Delegate for AnyDelegate {}

#[link(name = "av", kind = "static")]
unsafe extern "C" {
    static AV_ASSET_WRITER: &'static objc::Class<Writer>;
}

#[cfg(test)]
mod tests {
    use crate::ut;

    #[test]
    fn basics() {
        use crate::{av, ns};

        let url = ns::Url::with_str("file://tmp/bla.mp4").unwrap();

        let writer = av::AssetWriter::with_url_and_file_type(&url, av::FileType::mp4()).unwrap();
        assert_eq!(writer.inputs().len(), 0);

        av::AssetWriter::with_content_type(&ut::Type::pdf())
            .expect_err("Can't create writer for pdf");

        av::AssetWriter::with_content_type(&ut::Type::mpeg4movie()).expect("Can't create viedeo");
    }
}
