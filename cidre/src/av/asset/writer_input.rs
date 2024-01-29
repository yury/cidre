use crate::{arc, av::MediaType, blocks, cm, define_cls, define_obj_type, dispatch, ns, objc};

define_obj_type!(pub WriterInput(ns::Id));

impl arc::A<WriterInput> {
    #[objc::msg_send(initWithMediaType:outputSettings:)]
    pub unsafe fn init_media_type_output_settings_throws(
        self,
        media_type: &MediaType,
        output_settings: Option<&ns::Dictionary<ns::String, ns::Id>>,
    ) -> arc::R<WriterInput>;

    #[objc::msg_send(initWithMediaType:outputSettings:sourceFormatHint:)]
    pub unsafe fn with_media_type_output_settings_source_format_hint_throws(
        self,
        media_type: &MediaType,
        output_settings: Option<&ns::Dictionary<ns::String, ns::Id>>,
        source_format_hint: Option<&cm::FormatDesc>,
    ) -> arc::R<WriterInput>;
}

impl WriterInput {
    define_cls!(AV_ASSET_WRITER_INPUT);

    pub unsafe fn with_media_type_output_settings_throws(
        media_type: &MediaType,
        output_settings: Option<&ns::Dictionary<ns::String, ns::Id>>,
    ) -> arc::R<WriterInput> {
        Self::alloc().init_media_type_output_settings_throws(media_type, output_settings)
    }

    pub fn with_media_type_and_output_settings<'ar>(
        media_type: &MediaType,
        output_settings: Option<&ns::Dictionary<ns::String, ns::Id>>,
    ) -> Result<arc::R<WriterInput>, &'ar ns::Exception> {
        ns::try_catch(|| unsafe {
            Self::alloc().init_media_type_output_settings_throws(media_type, output_settings)
        })
    }

    pub unsafe fn with_media_type_output_settings_source_format_hint_throws(
        media_type: &MediaType,
        output_settings: Option<&ns::Dictionary<ns::String, ns::Id>>,
        source_format_hint: Option<&cm::FormatDesc>,
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
        source_format_hint: Option<&cm::FormatDesc>,
    ) -> Result<arc::R<WriterInput>, &'ar ns::Exception> {
        ns::try_catch(|| unsafe {
            Self::with_media_type_output_settings_source_format_hint_throws(
                media_type,
                output_settings,
                source_format_hint,
            )
        })
    }

    pub unsafe fn with_media_type_format_hint_throws(
        media_type: &MediaType,
        source_format_hint: &cm::FormatDesc,
    ) -> arc::R<WriterInput> {
        Self::alloc().with_media_type_output_settings_source_format_hint_throws(
            media_type,
            None,
            Some(source_format_hint),
        )
    }

    pub unsafe fn with_media_type_throws(media_type: &MediaType) -> arc::R<WriterInput> {
        Self::with_media_type_output_settings_throws(media_type, None)
    }

    pub fn with_media_type<'ar>(
        media_type: &MediaType,
    ) -> Result<arc::R<WriterInput>, &'ar ns::Exception> {
        ns::try_catch(|| unsafe { Self::with_media_type_throws(media_type) })
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
    pub unsafe fn append_sample_buf_throws(&mut self, buffer: &cm::SampleBuf) -> bool;

    pub fn append_sample_buf<'ar>(
        &mut self,
        buffer: &cm::SampleBuf,
    ) -> Result<bool, &'ar ns::Exception> {
        ns::try_catch(|| unsafe { self.append_sample_buf_throws(buffer) })
    }

    #[objc::msg_send(requestMediaDataWhenReadyOnQueue:usingBlock:)]
    pub unsafe fn request_media_data_when_ready_on_queue_throws(
        &self,
        queue: &dispatch::Queue,
        block: &mut blocks::CompletionBlock,
    );

    pub fn request_media_data_when_ready_on_queue<'ear>(
        &self,
        queue: &dispatch::Queue,
        block: &mut blocks::CompletionBlock,
    ) -> Result<(), &'ear ns::Exception> {
        ns::try_catch(|| unsafe {
            self.request_media_data_when_ready_on_queue_throws(queue, block)
        })
    }
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
        let input = av::asset::WriterInput::with_media_type(av::MediaType::video())
            .expect("failed to create writer");
        assert_eq!(input.media_type(), av::MediaType::video());
        av::asset::WriterInput::with_media_type(av::MediaType::muxed()).unwrap();
        av::asset::WriterInput::with_media_type(av::MediaType::audio()).unwrap();
        av::asset::WriterInput::with_media_type(av::MediaType::text()).unwrap();
        av::asset::WriterInput::with_media_type(av::MediaType::closed_caption()).unwrap();
        av::asset::WriterInput::with_media_type(av::MediaType::subtitle()).unwrap();
        av::asset::WriterInput::with_media_type(av::MediaType::depth_data()).unwrap();
        let input = av::asset::WriterInput::with_media_type(av::MediaType::timecode()).unwrap();
        println!("{:?}", input);
    }
}
