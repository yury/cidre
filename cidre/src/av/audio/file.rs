use crate::{arc, av, define_cls, define_obj_type, ns, objc};

define_obj_type!(
    #[doc(alias = "AVAudioFile")]
    pub File(ns::Id)
);

impl arc::A<File> {
    #[objc::msg_send(initForReading:error:)]
    pub unsafe fn init_for_reading_err<'ear>(
        self,
        file_url: &ns::Url,
        err: *mut Option<&'ear ns::Error>,
    ) -> Option<arc::R<File>>;

    #[objc::msg_send(initForReading:commonFormat:interleaved:error:)]
    pub unsafe fn init_for_reading_common_format_err<'ear>(
        self,
        file_url: &ns::Url,
        common_format: av::audio::CommonFormat,
        interleaved: bool,
        err: *mut Option<&'ear ns::Error>,
    ) -> Option<arc::R<File>>;

    #[objc::msg_send(initForWriting:settings:error:)]
    pub unsafe fn init_for_writing_err<'ear>(
        self,
        file_url: &ns::Url,
        settings: &ns::Dictionary<ns::String, ns::Id>,
        err: *mut Option<&'ear ns::Error>,
    ) -> Option<arc::R<File>>;

    #[objc::msg_send(initForWriting:settings:commonFormat:interleaved:error:)]
    pub unsafe fn init_for_writing_common_format_err<'ear>(
        self,
        file_url: &ns::Url,
        settings: &ns::Dictionary<ns::String, ns::Id>,
        common_format: av::audio::CommonFormat,
        interleaved: bool,
        err: *mut Option<&'ear ns::Error>,
    ) -> Option<arc::R<File>>;
}

impl File {
    define_cls!(AV_AUDIO_FILE);

    pub fn open_read<'ear>(file_url: &ns::Url) -> ns::Result<'ear, arc::R<Self>> {
        ns::if_none(|err| unsafe { Self::alloc().init_for_reading_err(file_url, err) })
    }

    pub fn open_read_common_format<'ear>(
        file_url: &ns::Url,
        common_format: av::audio::CommonFormat,
        interleaved: bool,
    ) -> ns::Result<'ear, arc::R<Self>> {
        ns::if_none(|err| unsafe {
            Self::alloc().init_for_reading_common_format_err(
                file_url,
                common_format,
                interleaved,
                err,
            )
        })
    }

    pub fn open_write<'ear>(
        file_url: &ns::Url,
        settings: &ns::Dictionary<ns::String, ns::Id>,
    ) -> ns::Result<'ear, arc::R<Self>> {
        ns::if_none(|err| unsafe { Self::alloc().init_for_writing_err(file_url, settings, err) })
    }

    pub fn open_write_common_format<'ear>(
        file_url: &ns::Url,
        settings: &ns::Dictionary<ns::String, ns::Id>,
        common_format: av::audio::CommonFormat,
        interleaved: bool,
    ) -> ns::Result<'ear, arc::R<Self>> {
        ns::if_none(|err| unsafe {
            Self::alloc().init_for_writing_common_format_err(
                file_url,
                settings,
                common_format,
                interleaved,
                err,
            )
        })
    }

    #[objc::msg_send(close)]
    pub fn close(&mut self);

    #[objc::msg_send(readIntoBuffer:error:)]
    pub unsafe fn read_err<'ear>(
        &mut self,
        buffer: &mut av::audio::PcmBuf,
        err: *mut Option<&'ear ns::Error>,
    ) -> bool;

    pub fn read(&mut self, buffer: &mut av::audio::PcmBuf) -> ns::Result {
        ns::if_false(|err| unsafe { self.read_err(buffer, err) })
    }

    #[objc::msg_send(readIntoBuffer:frameCount:error:)]
    pub unsafe fn read_n_err<'ear>(
        &mut self,
        buffer: &mut av::audio::PcmBuf,
        frame_count: av::AudioFrameCount,
        err: *mut Option<&'ear ns::Error>,
    ) -> bool;

    pub fn read_n<'ear>(
        &mut self,
        buffer: &mut av::audio::PcmBuf,
        frame_count: av::AudioFrameCount,
    ) -> ns::Result {
        ns::if_false(|err| unsafe { self.read_n_err(buffer, frame_count, err) })
    }

    #[objc::msg_send(writeFromBuffer:error:)]
    pub unsafe fn write_err<'ear>(
        &mut self,
        buffer: &av::audio::PcmBuf,
        err: *mut Option<&'ear ns::Error>,
    ) -> bool;

    pub fn write<'ear>(&mut self, buffer: &av::audio::PcmBuf) -> ns::Result {
        ns::if_false(|err| unsafe { self.write_err(buffer, err) })
    }

    /// Whether the file is open or not.
    #[objc::msg_send(isOpen)]
    pub fn is_open(&self) -> bool;

    /// The URL the file is reading or writing.
    #[objc::msg_send(url)]
    pub fn url(&self) -> arc::R<ns::Url>;

    /// The on-disk format of the file.
    #[objc::msg_send(fileFormat)]
    pub fn file_format(&self) -> arc::R<av::audio::Format>;

    /// The processing format of the file.
    #[objc::msg_send(processingFormat)]
    pub fn processing_format(&self) -> arc::R<av::audio::Format>;

    /// The number of sample frames in the file.
    ///
    /// This can be expensive to compute for the first time.
    #[objc::msg_send(length)]
    pub fn len(&self) -> av::audio::FramePos;

    /// The position in the file at which the next read or write will occur.
    #[objc::msg_send(framePosition)]
    pub fn frame_pos(&self) -> av::audio::FramePos;

    #[objc::msg_send(setFramePosition:)]
    pub fn set_frame_pos(&mut self, val: av::audio::FramePos);
}

unsafe extern "C" {
    static AV_AUDIO_FILE: &'static objc::Class<File>;
}

#[cfg(test)]
mod tests {
    use crate::{av, ns};

    #[test]
    fn basics() {
        let _ = std::fs::remove_file("/tmp/foo.caf");
        let url = ns::Url::with_fs_path_str("/tmp/foo.caf", false);
        let _err = av::AudioFile::open_read(&url).expect_err("Should fail");

        let settings = ns::Dictionary::new();
        let mut file = av::AudioFile::open_write_common_format(
            &url,
            &settings,
            av::AudioCommonFormat::PcmF32,
            true,
        )
        .expect("Error creating file");
        assert!(file.is_open());
        assert_eq!(0, file.len());
        file.close();
        assert!(!file.is_open());
    }
}
