use crate::{
    at::{AudioBufList, AudioChannelLayout, audio},
    cat::AudioStreamBasicDesc,
    cf, os,
};

#[derive(Copy, Clone, Eq, PartialEq)]
#[repr(transparent)]
pub struct ExtAudioFilePropId(pub u32);

impl ExtAudioFilePropId {
    /// AudioStreamBasicDesc
    #[doc(alias = "kExtAudioFileProperty_FileDataFormat")]
    pub const FILE_DATA_FORMAT: Self = Self(u32::from_be_bytes(*b"ffmt"));

    /// AudioChannelLayout
    #[doc(alias = "kExtAudioFileProperty_FileChannelLayout")]
    pub const FILE_CHANNEL_LAYOUT: Self = Self(u32::from_be_bytes(*b"fclo"));

    /// AudioStreamBasicDesc
    #[doc(alias = "kExtAudioFileProperty_ClientDataFormat")]
    pub const CLIENT_DATA_FORMAT: Self = Self(u32::from_be_bytes(*b"cfmt"));

    /// AudioChannelLayout
    #[doc(alias = "kExtAudioFileProperty_ClientChannelLayout")]
    pub const CLIENT_CHANNEL_LAYOUT: Self = Self(u32::from_be_bytes(*b"cclo"));

    /// u32
    #[doc(alias = "kExtAudioFileProperty_CodecManufacturer")]
    pub const CODEC_MANUFACTURER: Self = Self(u32::from_be_bytes(*b"cman"));
}

/// Readonly
impl ExtAudioFilePropId {
    /// AudioConverterRef
    #[doc(alias = "kExtAudioFileProperty_AudioConverter")]
    pub const AUDIO_CONVERTER: Self = Self(u32::from_be_bytes(*b"acnv"));

    /// AudioFileID
    #[doc(alias = "kExtAudioFileProperty_AudioFile")]
    pub const AUDIO_FILE: Self = Self(u32::from_be_bytes(*b"afil"));

    #[doc(alias = "kExtAudioFileProperty_FileMaxPacketSize")]
    pub const FILE_MAX_PACKET_SIZE: Self = Self(u32::from_be_bytes(*b"fmps"));

    #[doc(alias = "kExtAudioFileProperty_ClientMaxPacketSize")]
    pub const CLIENT_MAX_PACKET_SIZE: Self = Self(u32::from_be_bytes(*b"cmps"));

    #[doc(alias = "kExtAudioFileProperty_FileLengthFrames")]
    pub const FILE_LEN_FRAMES: Self = Self(u32::from_be_bytes(*b"#frm"));
}
/// Writable
impl ExtAudioFilePropId {
    #[doc(alias = "kExtAudioFileProperty_ConverterConfig")]
    pub const CONVERTER_CFG: Self = Self(u32::from_be_bytes(*b"accf"));

    #[doc(alias = "kExtAudioFileProperty_IOBufferSizeBytes")]
    pub const IO_BUF_SIZE_BYTES: Self = Self(u32::from_be_bytes(*b"iobs"));

    /// *mut c_void
    #[doc(alias = "kExtAudioFileProperty_IOBuffer")]
    pub const IO_BUF: Self = Self(u32::from_be_bytes(*b"iobf"));

    /// AudioFilePacketTableInfo
    #[doc(alias = "kExtAudioFileProperty_PacketTable")]
    pub const PACKET_TABLE: Self = Self(u32::from_be_bytes(*b"xpti"));
}

impl std::fmt::Debug for ExtAudioFilePropId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        crate::four_cc_fmt_debug(self.0, "ExtAudioFilePropId", f)
    }
}

pub mod err {
    use crate::os::Error;

    pub const INVALID_PROP: Error = Error::new_unchecked(-66561);
    pub const INVALID_PROP_SIZE: Error = Error::new_unchecked(-66562);
    pub const NON_PCM_CLIENT_FORMAT: Error = Error::new_unchecked(-66563);
    pub const INVALID_CHANNEL_MAP: Error = Error::new_unchecked(-66564);
    pub const INVALID_OP_ORDER: Error = Error::new_unchecked(-66565);
    pub const INVALID_DATA_FORMAT: Error = Error::new_unchecked(-66566);
    pub const MAX_PACKET_SIZE_UNKNOWN: Error = Error::new_unchecked(-66567);
    pub const INVALID_SEEK: Error = Error::new_unchecked(-66568);
    pub const ASYNC_WRITE_TOO_LARGE: Error = Error::new_unchecked(-66569);
    pub const ASYNC_WRITE_BUF_OVERFLOW: Error = Error::new_unchecked(-66570);
}

// define_cf_type!(
//     #[doc(alias = "ExtAudioFileRef")]
//     ExtAudioFile(cf::Type)
// );

#[derive(Debug)]
#[repr(transparent)]
pub struct ExtAudioFile(std::ffi::c_void);

#[derive(Debug)]
#[repr(transparent)]
pub struct ExtAudioFileRef(&'static mut ExtAudioFile);

impl std::ops::Deref for ExtAudioFileRef {
    type Target = ExtAudioFile;

    fn deref(&self) -> &Self::Target {
        self.0
    }
}

impl std::ops::DerefMut for ExtAudioFileRef {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.0
    }
}

impl Drop for ExtAudioFileRef {
    fn drop(&mut self) {
        let res = unsafe { self.0.dispose() };
        debug_assert!(res.is_ok());
    }
}

impl ExtAudioFile {
    pub fn open(url: &cf::Url) -> os::Result<ExtAudioFileRef> {
        unsafe {
            let mut res = None;
            ExtAudioFileOpenURL(url, &mut res).result()?;
            Ok(res.unwrap_unchecked())
        }
    }

    pub fn create(
        url: &cf::Url,
        file_type: audio::FileTypeId,
        stream_desc: &AudioStreamBasicDesc,
        channel_layout: *const AudioChannelLayout<1>,
        flags: audio::FileFlags,
    ) -> os::Result<ExtAudioFileRef> {
        unsafe {
            let mut res = None;
            ExtAudioFileCreateWithURL(url, file_type, stream_desc, channel_layout, flags, &mut res)
                .result()?;
            Ok(res.unwrap_unchecked())
        }
    }

    pub unsafe fn dispose(&mut self) -> os::Result {
        unsafe { ExtAudioFileDispose(self).result() }
    }

    pub fn read<const N: usize>(
        &mut self,
        frames_n: &mut u32,
        buf_list: &mut AudioBufList<N>,
    ) -> os::Result {
        unsafe { ExtAudioFileRead(self, frames_n, std::mem::transmute(buf_list)).result() }
    }

    pub fn write<const N: usize>(
        &mut self,
        frames_n: u32,
        buf_list: &AudioBufList<N>,
    ) -> os::Result {
        unsafe { ExtAudioFileWrite(self, frames_n, std::mem::transmute(buf_list)).result() }
    }

    pub fn write_async<const N: usize>(
        &mut self,
        frames_n: u32,
        buf_list: Option<&AudioBufList<N>>,
    ) -> os::Result {
        unsafe { ExtAudioFileWriteAsync(self, frames_n, std::mem::transmute(buf_list)).result() }
    }

    pub fn seek(&mut self, frames_offset: i64) -> os::Result {
        unsafe { ExtAudioFileSeek(self, frames_offset).result() }
    }

    pub fn tell(&mut self, frames_offset: &mut i64) -> os::Result {
        unsafe { ExtAudioFileTell(self, frames_offset).result() }
    }

    pub fn prop_info(
        &self,
        prop_id: ExtAudioFilePropId,
        out_size: *mut u32,
        writable: *mut bool,
    ) -> os::Result {
        unsafe { ExtAudioFileGetPropertyInfo(self, prop_id, out_size, writable).result() }
    }
}

unsafe extern "C" {
    fn ExtAudioFileOpenURL(url: &cf::Url, audio_file: *mut Option<ExtAudioFileRef>) -> os::Status;

    fn ExtAudioFileCreateWithURL(
        url: &cf::Url,
        file_type: audio::FileTypeId,
        stream_desc: &AudioStreamBasicDesc,
        channel_layout: *const AudioChannelLayout<1>,
        flags: audio::FileFlags,
        audio_file: *mut Option<ExtAudioFileRef>,
    ) -> os::Status;

    fn ExtAudioFileDispose(audio_file: &mut ExtAudioFile) -> os::Status;

    fn ExtAudioFileRead(
        audio_file: &mut ExtAudioFile,
        frames_n: &mut u32,
        buf_list: &mut AudioBufList,
    ) -> os::Status;

    fn ExtAudioFileWrite(
        audio_file: &mut ExtAudioFile,
        frames_n: u32,
        buf_list: &AudioBufList,
    ) -> os::Status;

    fn ExtAudioFileWriteAsync(
        audio_file: &mut ExtAudioFile,
        frames_n: u32,
        buf_list: Option<&AudioBufList>,
    ) -> os::Status;

    fn ExtAudioFileSeek(audio_file: &mut ExtAudioFile, frames_offset: i64) -> os::Status;

    fn ExtAudioFileTell(audio_file: &ExtAudioFile, frames_offset: &mut i64) -> os::Status;

    fn ExtAudioFileGetPropertyInfo(
        audio_file: &ExtAudioFile,
        prop_id: ExtAudioFilePropId,
        out_size: *mut u32,
        writable: *mut bool,
    ) -> os::Status;

}
