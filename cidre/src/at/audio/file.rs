use std::ffi::c_void;

use crate::{cat::audio, cf, define_options, os};

#[derive(Debug)]
#[doc(alias = "AudioFileID")]
#[repr(transparent)]
pub struct FileID(&'static mut c_void);

impl FileID {
    /// Creates a new audio file (or initialises an existing file)
    ///
    /// Creates a new (or initialises an existing) audio file specified by the URL.
    /// Upon success, an AudioFileID is returned which can be used for subsequent calls
    /// to the AudioFile APIs.
    pub fn create(
        url: &cf::URL,
        type_id: FileTypeID,
        format: &audio::StreamBasicDescription,
        flags: Flags,
    ) -> Result<Self, os::Status> {
        let mut result = None;
        unsafe {
            AudioFileCreateWithURL(url, type_id, format, flags, &mut result)
                .to_result_unchecked(result)
        }
    }

    /// Open an existing audio file specified by a URL.
    /// Returns a pointer to the newly opened audio file or Err(os::Status)
    ///
    /// # Arguments
    ///
    /// * `url` - The URL of an existing audio file.
    /// * `permissions` - The read-write permissions you want to assign to the file. Use the permission constants in Permissions.
    /// * `type_hint` - A hint for the file type of the designated file. For files without filename extensions
    ///                 and with types not easily or uniquely determined from the data (such as ADTS or AC3),
    ///                 use this hint to indicate the file type. Otherwise, pass 0. Only use this hint in macOS versions 10.3.1 or greater.
    ///                 In all earlier versions, any attempt to open these files fails
    #[doc(alias = "AudioFileOpenURL")]
    pub fn open(
        url: &cf::URL,
        permissions: Permissions,
        type_hint: FileTypeID,
    ) -> Result<Self, os::Status> {
        let mut result = None;
        unsafe {
            let res = AudioFileOpenURL(url, permissions, type_hint, &mut result);
            res.to_result_unchecked(result)
        }
    }

    #[doc(alias = "AudioFileWritePackets")]
    #[inline]
    pub fn write_packets(
        &mut self,
        use_cache: bool,
        num_bytes: u32,
        packet_descriptions: Option<&audio::StreamPacketDescription>,
        starting_packet: isize,
        num_packets: *mut u32,
        buffer: *const c_void,
    ) -> Result<(), os::Status> {
        unsafe {
            let res = AudioFileWritePackets(
                self,
                use_cache,
                num_bytes,
                packet_descriptions,
                starting_packet,
                num_packets,
                buffer,
            );
            res.result()
        }
    }

    /// Close an existing audio file.
    #[doc(alias = "AudioFileClose")]
    #[inline]
    pub fn close(&mut self) -> os::Status {
        unsafe { AudioFileClose(self) }
    }
}

impl Drop for FileID {
    fn drop(&mut self) {
        let res = self.close();

        debug_assert!(res == 0 || res == errors::NOT_OPEN);
    }
}

/// Identifier for an audio file type.
#[doc(alias = "AudioFileTypeID")]
#[repr(transparent)]
pub struct FileTypeID(pub u32);

impl FileTypeID {
    /// Audio Interchange File Format (AIFF)
    #[doc(alias = "kAudioFileAIFFType")]
    pub const AIFF: Self = Self(u32::from_be_bytes(*b"AIFF"));

    /// Audio Interchange File Format Compressed (AIFF-C)
    #[doc(alias = "kAudioFileAIFCType")]
    pub const AIFC: Self = Self(u32::from_be_bytes(*b"AIFC"));

    /// Microsoft WAVE
    #[doc(alias = "kAudioFileWAVEType")]
    pub const WAVE: Self = Self(u32::from_be_bytes(*b"WAVE"));

    /// File Format specified in EBU Tech 3306
    #[doc(alias = "kAudioFileRF64Type")]
    pub const RF64: Self = Self(u32::from_be_bytes(*b"RF64"));

    /// File Format specified in ITU-R BS.2088
    #[doc(alias = "kAudioFileBW64Type")]
    pub const BW64: Self = Self(u32::from_be_bytes(*b"BW64"));

    /// Sony Pictures Digital Wave 64
    #[doc(alias = "kAudioFileWave64Type")]
    pub const WAVE64: Self = Self(u32::from_be_bytes(*b"W64f"));

    /// Sound Designer II
    #[doc(alias = "kAudioFileSoundDesigner2Type")]
    pub const SOUND_DESIGNER2: Self = Self(u32::from_be_bytes(*b"Sd2f"));

    /// NeXT / Sun
    #[doc(alias = "kAudioFileNextType")]
    pub const NEXT: Self = Self(u32::from_be_bytes(*b"NeXT"));

    /// MPEG Audio Layer 3 (.mp3)
    #[doc(alias = "kAudioFileMP3Type")]
    pub const MP3: Self = Self(u32::from_be_bytes(*b"MPG3"));

    /// MPEG Audio Layer 2 (.mp2)
    #[doc(alias = "kAudioFileMP2Type")]
    pub const MP2: Self = Self(u32::from_be_bytes(*b"MPG2"));

    /// MPEG Audio Layer 1 (.mp1)
    #[doc(alias = "kAudioFileMP1Type")]
    pub const MP1: Self = Self(u32::from_be_bytes(*b"MPG1"));

    /// AC-3
    #[doc(alias = "kAudioFileAC3Type")]
    pub const AC3: Self = Self(u32::from_be_bytes(*b"ac-3"));

    /// Advanced Audio Coding (AAC) Audio Data Transport Stream (ADTS)
    #[doc(alias = "kAudioFileAAC_ADTSType")]
    pub const AAC_ADTS: Self = Self(u32::from_be_bytes(*b"adts"));

    /// An MPEG 4 file.
    #[doc(alias = "kAudioFileMPEG4Type")]
    pub const MPEG4: Self = Self(u32::from_be_bytes(*b"mp4f"));

    /// An M4A file.
    #[doc(alias = "kAudioFileM4AType")]
    pub const M4A: Self = Self(u32::from_be_bytes(*b"m4af"));

    #[doc(alias = "kAudioFileM4BType")]
    pub const M4B: Self = Self(u32::from_be_bytes(*b"m4bf"));

    /// A Core Audio File Format file.
    #[doc(alias = "kAudioFileCAFType")]
    pub const CAF: Self = Self(u32::from_be_bytes(*b"caff"));

    /// A 3GPP file, suitable for video content on GSM mobile phones.
    #[doc(alias = "kAudioFile3GPType")]
    pub const _3GP: Self = Self(u32::from_be_bytes(*b"3gpp"));

    /// A 3GPP2 file, suitable for video content on CDMA mobile phones.
    #[doc(alias = "kAudioFile3GP2Type")]
    pub const _3GP2: Self = Self(u32::from_be_bytes(*b"3gp2"));

    /// An AMR (Adaptive Multi-Rate) file suitable for compressed speech.
    #[doc(alias = "kAudioFileAMRType")]
    pub const AMR: Self = Self(u32::from_be_bytes(*b"amrf"));

    /// Free Lossless Audio Codec
    #[doc(alias = "kAudioFileFLACType")]
    pub const FLAC: Self = Self(u32::from_be_bytes(*b"flac"));

    /// Low-overhead audio stream with low-overhead audio transport multiplex, per ISO/IEC 14496-3.
    /// Support is limited to AudioSyncStream using AudioMuxElement with mux config present.
    #[doc(alias = "kAudioFileLATMInLOASType")]
    pub const LATM_IN_LOAS: Self = Self(u32::from_be_bytes(*b"loas"));
}

pub mod errors {
    use crate::os::Status;
    /// 0x7768743F, 2003334207
    /// An unspecified error has occurred.
    #[doc(alias = "kAudioFileUnspecifiedError")]
    pub const UNSPECIFIED: Status = Status(i32::from_be_bytes(*b"wht?"));

    /// 0x7479703F, 1954115647
    /// The file type is not supported.
    #[doc(alias = "kAudioFileUnsupportedFileTypeError")]
    pub const UNSUPPORTED_FILE_TYPE: Status = Status(i32::from_be_bytes(*b"typ?"));

    /// 0x666D743F, 1718449215
    /// The data format is not supported by this file type.
    #[doc(alias = "kAudioFileUnsupportedDataFormatError")]
    pub const UNSUPPORTED_DATA_FORMAT: Status = Status(i32::from_be_bytes(*b"fmt?"));

    /// 0x7074793F, 1886681407
    /// The property is not supported.
    #[doc(alias = "kAudioFileUnsupportedPropertyError")]
    pub const UNSUPPORTED_PROPERTY: Status = Status(i32::from_be_bytes(*b"pty?"));

    /// 0x2173697A,  561211770
    /// The size of the property data was not correct.
    #[doc(alias = "kAudioFileBadPropertySizeError")]
    pub const BAD_PROPERTY_SIZE: Status = Status(i32::from_be_bytes(*b"!siz"));

    /// 0x70726D3F, 1886547263
    /// The operation violated the file permissions. For example, an attempt was made to write to a file
    /// opened with the kAudioFileReadPermission constant.
    #[doc(alias = "kAudioFilePermissionsError")]
    pub const PERMISSIONS: Status = Status(i32::from_be_bytes(*b"!prm"));

    /// 0x6F70746D, 1869640813
    /// The chunks following the audio data chunk are preventing the extension of the audio data chunk.
    ///  To write more data, you must optimize the file.
    #[doc(alias = "kAudioFileNotOptimizedError")]
    pub const NOT_OPTIMIZED: Status = Status(i32::from_be_bytes(*b"optm"));

    /// 0x63686B3F, 1667787583
    /// Either the chunk does not exist in the file or it is not supported by the file.
    #[doc(alias = "kAudioFileInvalidChunkError")]
    pub const INVALID_CHUNK: Status = Status(i32::from_be_bytes(*b"chk?"));

    /// 0x6F66663F, 1868981823
    /// The file offset was too large for the file type. The AIFF and WAVE file format types have 32-bit file size limits.
    #[doc(alias = "kAudioFileDoesNotAllow64BitDataSizeError")]
    pub const DOES_NOT_ALLOW64_BIT_DATA_SIZE: Status = Status(i32::from_be_bytes(*b"off?"));

    /// 0x70636B3F, 1885563711
    /// The file offset was too large for the file type. The AIFF and WAVE file format types have 32-bit file size limits.
    #[doc(alias = "kAudioFileInvalidPacketOffsetError")]
    pub const INVALID_PACKET_OFFSET: Status = Status(i32::from_be_bytes(*b"pck?"));

    /// 0x6465703F, 1684369471
    /// The file offset was too large for the file type. The AIFF and WAVE file format types have 32-bit file size limits.
    #[doc(alias = "kAudioFileInvalidPacketDependencyError")]
    pub const INVALID_PACKET_DEPENDENCY: Status = Status(i32::from_be_bytes(*b"dep?"));

    /// 0x6474613F, 1685348671
    /// The file is malformed, or otherwise not a valid instance of an audio file of its type.
    #[doc(alias = "kAudioFileInvalidFileError")]
    pub const INVALID_FILE: Status = Status(i32::from_be_bytes(*b"dta?"));

    /// 0x6F703F3F
    /// The file is malformed, or otherwise not a valid instance of an audio file of its type.
    #[doc(alias = "kAudioFileOperationNotSupportedError")]
    pub const OPERATION_NOT_SUPPORTED: Status = Status(i32::from_be_bytes(*b"op??"));

    /// The file is closed.
    #[doc(alias = "kAudioFileNotOpenError")]
    pub const NOT_OPEN: Status = Status(-38);

    /// End of file.
    #[doc(alias = "kAudioFileEndOfFileError")]
    pub const END_OF_FILE: Status = Status(-39);

    /// Invalid file position.
    #[doc(alias = "kAudioFilePositionError")]
    pub const POSITION: Status = Status(-40);

    /// File not found.
    #[doc(alias = "kAudioFileFileNotFoundError")]
    pub const FILE_NOT_FOUND: Status = Status(-43);
}

define_options!(Flags(u32));

/// These are flags that can be used with the create call
impl Flags {
    /// If set, then the CreateURL call will erase the contents of an existing file
    /// If not set, then the CreateURL call will fail if the file already exists
    #[doc(alias = "kAudioFileFlags_EraseFile")]
    pub const ERASE_FILE: Self = Self(1);

    /// Normally, newly created and optimized files will have padding added in order to page align
    /// the data to 4KB boundaries. This makes reading the data more efficient.
    /// When disk space is a concern, this flag can be set so that the padding will not be added.
    #[doc(alias = "kAudioFileFlags_DontPageAlignAudioData")]
    pub const DONT_PAGE_ALIGN_AUDIO_DATA: Self = Self(2);
}

#[repr(i8)]
pub enum Permissions {
    #[doc(alias = "kAudioFileReadPermission")]
    Read = 0x01,
    #[doc(alias = "kAudioFileWritePermission")]
    Write = 0x2,
    #[doc(alias = "kAudioFileReadWritePermission")]
    ReadWrite = 0x3,
}

#[link(name = "AudioToolbox", kind = "framework")]
extern "C" {
    fn AudioFileCreateWithURL(
        in_file_url: &cf::URL,
        in_file_type: FileTypeID,
        in_format: &audio::StreamBasicDescription,
        in_flags: Flags,
        out_audio_file: &mut Option<FileID>,
    ) -> os::Status;

    fn AudioFileClose(file: &mut FileID) -> os::Status;

    fn AudioFileOpenURL(
        in_file_url: &cf::URL,
        in_permissions: Permissions,
        in_file_type_hint: FileTypeID,
        out_audio_file: &mut Option<FileID>,
    ) -> os::Status;

    fn AudioFileWritePackets(
        file: &mut FileID,
        use_cache: bool,
        num_bytes: u32,
        packet_descriptions: Option<&audio::StreamPacketDescription>,
        starting_packet: isize,
        num_packets: *mut u32,
        buffer: *const c_void,
    ) -> os::Status;
}

#[cfg(test)]
mod tests {
    use crate::{at::audio, cf};

    #[test]
    fn basics() {
        let path = cf::URL::from_str("file:///tmp/mp3.mp3").unwrap();

        let asbd = audio::StreamBasicDescription {
            //sample_rate: 32_000.0,
            // sample_rate: 44_100.0,
            sample_rate: 48_000.0,
            format_id: audio::FormatID::MPEG4_AAC,
            format_flags: Default::default(),
            // format_flags: AudioFormatFlags(MPEG4ObjectID::AAC_LC.0 as _),
            bytes_per_packet: 0,
            frames_per_packet: 1024,
            bytes_per_frame: 0,
            channels_per_frame: 2,
            bits_per_channel: 0,
            reserved: 0,
        };

        audio::FileID::create(
            &path,
            audio::FileTypeID::M4A,
            &asbd,
            audio::FileFlags::ERASE_FILE,
        )
        .unwrap();

        audio::FileID::open(&path, audio::FilePermissions::Read, audio::FileTypeID::M4A)
            .expect_err("should be error");
    }
}
