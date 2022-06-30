use crate::{av::MediaType, cf, cm, define_obj_type, ns};

define_obj_type!(WriterInput(ns::Id));

impl WriterInput {
    pub fn with_media_type_and_output_settings<'a>(
        media_type: MediaType,
        output_settings: Option<&cf::DictionaryOf<cf::String, ns::Id>>,
    ) -> cf::Retained<'a, WriterInput> {
        unsafe {
            AVAssetWriterInput_assetWriterInputWithMediaType_outputSettings(
                media_type,
                output_settings,
            )
        }
    }

    pub fn with_media_type<'a>(media_type: MediaType) -> cf::Retained<'a, WriterInput> {
        Self::with_media_type_and_output_settings(media_type, None)
    }

    pub fn media_type(&self) -> MediaType {
        unsafe { rsel_mediaType(self) }
    }

    pub fn output_settings(&self) -> Option<&cf::DictionaryOf<cf::String, ns::Id>> {
        unsafe { rsel_outputSettings(self) }
    }

    #[inline]
    pub fn is_ready_for_more_media_data(&self) -> bool {
        unsafe { rsel_isReadyForMoreMediaData(self) }
    }

    pub fn expects_media_data_in_real_time(&self) -> bool {
        unsafe { rsel_expectsMediaDataInRealTime(self) }
    }

    pub fn set_expects_media_data_in_real_time(&self, value: bool) {
        unsafe { wsel_setExpectsMediaDataInRealTime(self, value) }
    }

    pub fn mark_as_finished(&self) {
        unsafe { wsel_markAsFinished(self) }
    }

    /// Order the samples you append according to storage requirements.
    /// For example, if you’re working with sample buffers containing compressed video,
    /// order and append them according to their decode timestamp. The system uses the
    /// timing information in the sample buffer relative to the time you set in the
    /// call to startSessionAtSourceTime: to determine the timing of samples in the
    /// output file.
    ///
    /// If this method returns NO,
    /// check the value of the asset writer’s status property to determine whether
    /// the writing operation’s status is complete, failed, or canceled. 
    /// If the status is AVAssetWriterStatusFailed, the asset writer’s error property
    /// contains an error object that describes the failure.
    #[inline]
    pub fn append_sample_buffer(&self, buffer: &cm::SampleBuffer) -> bool {
        unsafe { rsel_appendSampleBuffer(self, buffer) }
    }
}

#[link(name = "av", kind = "static")]
extern "C" {
    fn AVAssetWriterInput_assetWriterInputWithMediaType_outputSettings<'a>(
        media_type: MediaType,
        output_settings: Option<&cf::DictionaryOf<cf::String, ns::Id>>,
    ) -> cf::Retained<'a, WriterInput>;

    fn rsel_mediaType(id: &ns::Id) -> MediaType;
    fn rsel_outputSettings(id: &ns::Id) -> Option<&cf::DictionaryOf<cf::String, ns::Id>>;
    fn rsel_isReadyForMoreMediaData(id: &ns::Id) -> bool;
    fn rsel_expectsMediaDataInRealTime(id: &ns::Id) -> bool;
    fn wsel_setExpectsMediaDataInRealTime(id: &ns::Id, value: bool);

    fn wsel_markAsFinished(id: &ns::Id);

    fn rsel_appendSampleBuffer(id: &ns::Id, buffer: &cm::SampleBuffer) -> bool;
}
