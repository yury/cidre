use crate::{
    arc,
    av::MediaType,
    blocks::Block,
    cm, define_cls, define_obj_type, dispatch,
    ns::{self, try_catch},
    objc,
};

define_obj_type!(WriterInput(ns::Id));

impl arc::A<WriterInput> {
    #[objc::msg_send(initWithMediaType:outputSettings:)]
    pub fn init_media_type_output_settings_throws(
        self,
        media_type: &MediaType,
        output_settings: Option<&ns::Dictionary<ns::String, ns::Id>>,
    ) -> arc::R<WriterInput>;

    #[objc::msg_send(initWithMediaType:outputSettings:sourceFormatHint:)]
    pub fn with_media_type_output_settings_source_format_hint_throws(
        self,
        media_type: &MediaType,
        output_settings: Option<&ns::Dictionary<ns::String, ns::Id>>,
        source_format_hint: Option<&cm::FormatDescription>,
    ) -> arc::R<WriterInput>;
}

impl WriterInput {
    define_cls!(AV_ASSET_WRITER_INPUT);

    pub fn with_media_type_output_settings_throws(
        media_type: &MediaType,
        output_settings: Option<&ns::Dictionary<ns::String, ns::Id>>,
    ) -> arc::R<WriterInput> {
        Self::alloc().init_media_type_output_settings_throws(media_type, output_settings)
    }

    pub fn with_media_type_and_output_settings<'ar>(
        media_type: &MediaType,
        output_settings: Option<&ns::Dictionary<ns::String, ns::Id>>,
    ) -> Result<arc::R<WriterInput>, &'ar ns::Exception> {
        try_catch(|| {
            Self::alloc().init_media_type_output_settings_throws(media_type, output_settings)
        })
    }

    pub fn with_media_type_output_settings_source_format_hint_throws(
        media_type: &MediaType,
        output_settings: Option<&ns::Dictionary<ns::String, ns::Id>>,
        source_format_hint: Option<&cm::FormatDescription>,
    ) -> arc::R<WriterInput> {
        Self::alloc().with_media_type_output_settings_source_format_hint_throws(
            media_type,
            output_settings,
            source_format_hint,
        )
    }

    pub fn with_media_type_output_settings_source_format_hint<'ar>(
        media_type: &MediaType,
        output_settings: Option<&ns::Dictionary<ns::String, ns::Id>>,
        source_format_hint: Option<&cm::FormatDescription>,
    ) -> Result<arc::R<WriterInput>, &'ar ns::Exception> {
        ns::try_catch(|| {
            Self::with_media_type_output_settings_source_format_hint_throws(
                media_type,
                output_settings,
                source_format_hint,
            )
        })
    }

    pub fn with_media_type_format_hint_throws(
        media_type: &MediaType,
        source_format_hint: &cm::FormatDescription,
    ) -> arc::R<WriterInput> {
        Self::alloc().with_media_type_output_settings_source_format_hint_throws(
            media_type,
            None,
            Some(source_format_hint),
        )
    }

    pub fn with_media_type(media_type: &MediaType) -> arc::R<WriterInput> {
        Self::with_media_type_output_settings_throws(media_type, None)
    }

    #[objc::msg_send(mediaType)]
    pub fn media_type(&self) -> &MediaType;

    #[objc::msg_send(outputSettings)]
    pub fn output_settings(&self) -> Option<&ns::Dictionary<ns::String, ns::Id>>;

    #[objc::msg_send(isReadyForMoreMediaData)]
    pub fn is_ready_for_more_media_data(&self) -> bool;

    #[objc::msg_send(expectsMediaDataInRealTime)]
    pub fn expects_media_data_in_real_time(&self) -> bool;

    #[objc::msg_send(setExpectsMediaDataInRealTime:)]
    pub fn set_expects_media_data_in_real_time(&mut self, value: bool);

    #[objc::msg_send(markAsFinished)]
    pub fn mark_as_finished(&mut self);

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
    ///
    /// This method throws an exception if the sample buffer's media type does not match the asset writer input's media type.
    #[objc::msg_send(appendSampleBuffer:)]
    pub fn append_sample_buffer_throws(&mut self, buffer: &cm::SampleBuf) -> bool;

    pub fn append_sample_buffer<'ar>(
        &mut self,
        buffer: &cm::SampleBuf,
    ) -> Result<bool, &'ar ns::Exception> {
        try_catch(|| self.append_sample_buffer_throws(buffer))
    }

    #[objc::msg_send(requestMediaDataWhenReadyOnQueue:usingBlock:)]
    pub fn request_media_data_when_ready_on_queue_throws<F>(
        &self,
        queue: &dispatch::Queue,
        block: &'static Block<F>,
    ) where
        F: FnMut();
}

#[link(name = "av", kind = "static")]
extern "C" {
    static AV_ASSET_WRITER_INPUT: &'static objc::Class<WriterInput>;
}

#[cfg(test)]
mod tests {
    use crate::av;

    #[test]
    fn basics() {
        let input = av::asset::WriterInput::with_media_type(av::MediaType::video());
        assert_eq!(input.media_type(), av::MediaType::video());
        av::asset::WriterInput::with_media_type(av::MediaType::muxed());
        av::asset::WriterInput::with_media_type(av::MediaType::audio());
        av::asset::WriterInput::with_media_type(av::MediaType::text());
        av::asset::WriterInput::with_media_type(av::MediaType::closed_caption());
        av::asset::WriterInput::with_media_type(av::MediaType::subtitle());
        av::asset::WriterInput::with_media_type(av::MediaType::depth_data());
        let input = av::asset::WriterInput::with_media_type(av::MediaType::timecode());
        println!("{:?}", input);
    }
}
