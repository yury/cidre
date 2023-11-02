use std::ffi::c_void;

use crate::{arc, cat::audio, cf, define_options, os};

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
#[doc(alias = "AudioFilePropertyID")]
#[repr(transparent)]
pub struct PropertyID(pub u32);

impl PropertyID {
    /// The format of the audio data file.
    /// An `FileTypeID` that identifies the format of the file
    #[doc(alias = "kAudioFilePropertyFileFormat")]
    pub const FILE_FORMAT: Self = Self(u32::from_be_bytes(*b"ffmt"));

    /// An `audio::StreamBasicDescription` describing the format of the audio data
    #[doc(alias = "kAudioFilePropertyDataFormat")]
    pub const DATA_FORMAT: Self = Self(u32::from_be_bytes(*b"dfmt"));

    /// A `u32` indicating whether an Audio File has been optimized.
    /// Optimized means it is ready to start having sound data written to it.
    /// A value of 0 indicates the file needs to be optimized.
    /// A value of 1 indicates the file is currently optimized.
    #[doc(alias = "kAudioFilePropertyIsOptimized")]
    pub const IS_OPTIMIZED: Self = Self(u32::from_be_bytes(*b"optm"));

    /// A `*const c_void` pointing to memory set up by the caller.
    /// Some file types require that a magic cookie be provided before packets can be written
    /// to the file, so this property should be set before calling
    /// write_bytes()/write_packets() if a magic cookie exists.
    #[doc(alias = "kAudioFilePropertyMagicCookieData")]
    pub const MAGIC_COOKIE_DATA: Self = Self(u32::from_be_bytes(*b"mgic"));

    /// A `u64` that indicates the number of bytes of audio data contained in the file
    #[doc(alias = "kAudioFilePropertyAudioDataByteCount")]
    pub const DATA_BYTE_COUNT: Self = Self(u32::from_be_bytes(*b"bcnt"));

    /// A `u64` that indicates the number of packets of audio data contained in the file
    #[doc(alias = "kAudioFilePropertyAudioDataPacketCount")]
    pub const DATA_PACKET_COUNT: Self = Self(u32::from_be_bytes(*b"pcnt"));

    /// A `u32` that indicates the maximum size of a packet for the data contained in the file
    #[doc(alias = "kAudioFilePropertyMaximumPacketSize")]
    pub const MAXIMUM_PACKET_SIZE: Self = Self(u32::from_be_bytes(*b"psze"));

    /// a `s64` that indicates the byte offset in the file of the audio data.
    #[doc(alias = "kAudioFilePropertyDataOffset")]
    pub const DATA_OFFSET: Self = Self(u32::from_be_bytes(*b"doff"));

    /// An `audio::ChannelLayout` struct.
    #[doc(alias = "kAudioFilePropertyChannelLayout")]
    pub const CHANNEL_LAYOUT: Self = Self(u32::from_be_bytes(*b"cmap"));

    /// A u32. If 1, then updating the files sizes in the header is not done for every write,
    /// but deferred until the file is read, optimized or closed. This is more efficient, but less safe
    /// since, if the application crashes before the size is updated, the file may not be readable.
    /// The default value is one, it doesn't update the header.
    #[doc(alias = "kAudioFilePropertyDeferSizeUpdates")]
    pub const DEFER_SIZE_UPDATES: Self = Self(u32::from_be_bytes(*b"dszu"));

    #[doc(alias = "kAudioFilePropertyDataFormatName")]
    pub const DATA_FORMAT_NAME: Self = Self(u32::from_be_bytes(*b"fnme"));

    #[doc(alias = "kAudioFilePropertyMarkerList")]
    pub const MARKER_LIST: Self = Self(u32::from_be_bytes(*b"mkls"));

    #[doc(alias = "kAudioFilePropertyRegionList")]
    pub const REGION_LIST: Self = Self(u32::from_be_bytes(*b"rgls"));

    #[doc(alias = "kAudioFilePropertyPacketToFrame")]
    pub const PACKET_TO_FRAME: Self = Self(u32::from_be_bytes(*b"pkfr"));

    #[doc(alias = "kAudioFilePropertyFrameToPacket")]
    pub const FRAME_TO_PACKET: Self = Self(u32::from_be_bytes(*b"frpk"));

    #[doc(alias = "kAudioFilePropertyRestrictsRandomAccess")]
    pub const RESTRICTS_RANDOM_ACCESS: Self = Self(u32::from_be_bytes(*b"rrap"));

    #[doc(alias = "kAudioFilePropertyPacketToRollDistance")]
    pub const PACKET_TO_ROLL_DISTANCE: Self = Self(u32::from_be_bytes(*b"pkrl"));

    #[doc(alias = "kAudioFilePropertyPreviousIndependentPacket")]
    pub const PREVIOUS_INDEPENDENT_PACKET: Self = Self(u32::from_be_bytes(*b"pind"));

    /// Pass an AudioIndependentPacketTranslation with mPacket filled out and get mIndependentlyDecodablePacket back.
    /// A value of -1 means that no independent packet is present in the stream in the direction of interest. Otherwise,
    /// for kAudioFilePropertyPreviousIndependentPacket, mIndependentlyDecodablePacket will be less than mPacket, and
    /// for kAudioFilePropertyNextIndependentPacket, mIndependentlyDecodablePacket will be greater than mPacket.
    #[doc(alias = "kAudioFilePropertyNextIndependentPacket")]
    pub const NEXT_INDEPENDENT_PACKET: Self = Self(u32::from_be_bytes(*b"nind"));

    /// Pass an AudioPacketDependencyInfoTranslation with mPacket filled out and get mIsIndependentlyDecodable
    /// and mPrerollPacketCount back.
    /// A value of 0 for mIsIndependentlyDecodable indicates that the specified packet is not independently decodable.
    /// A value of 1 for mIsIndependentlyDecodable indicates that the specified packet is independently decodable.
    /// For independently decodable packets, mPrerollPacketCount indicates the count of packets that must be decoded
    /// after the packet with the specified number in order to refresh the decoder.
    /// If the value of kAudioFilePropertyRestrictsRandomAccess is 1, either kAudioFilePropertyPacketToRollDistance or
    /// kAudioFilePropertyPacketToDependencyInfo must be used in order to identify an appropriate random access point.
    #[doc(alias = "kAudioFilePropertyPacketToDependencyInfo")]
    pub const PACKET_TO_DEPENDENCY_INFO: Self = Self(u32::from_be_bytes(*b"pkdp"));

    /// pass an AudioBytePacketTranslation struct with mPacket filled out and get mByte back.
    /// mByteOffsetInPacket is ignored. If the mByte value is an estimate then
    /// kBytePacketTranslationFlag_IsEstimate will be set in the mFlags field.
    #[doc(alias = "kAudioFilePropertyPacketToByte")]
    pub const PACKET_TO_BYTE: Self = Self(u32::from_be_bytes(*b"pkby"));

    /// pass an AudioBytePacketTranslation struct with mByte filled out and get mPacket and
    /// mByteOffsetInPacket back. If the mPacket value is an estimate then
    /// kBytePacketTranslationFlag_IsEstimate will be set in the mFlags field
    #[doc(alias = "kAudioFilePropertyByteToPacket")]
    pub const BYTE_TO_PACKET: Self = Self(u32::from_be_bytes(*b"bypk"));

    /// returns an array of `os::Type` four char codes for each kind of chunk in the file.
    #[doc(alias = "kAudioFilePropertyChunkIDs")]
    pub const CHUNK_IDS: Self = Self(u32::from_be_bytes(*b"chid"));

    /// returns a CFDictionary filled with information about the data contained in the file.
    /// See dictionary key constants already defined for info string types.
    /// AudioFileComponents are free to add keys to the dictionaries that they return for this property...
    /// caller is responsible for releasing the CFObject
    #[doc(alias = "kAudioFilePropertyInfoDictionary")]
    pub const INFO_DICTIONARY: Self = Self(u32::from_be_bytes(*b"info"));

    /// Gets or sets an AudioFilePacketTableInfo struct for the file types that support it.
    /// When setting, the sum of mNumberValidFrames, mPrimingFrames and mRemainderFrames must be the same as the total
    /// number of frames in all packets. If not you will get a kAudio_ParamError. The best way to ensure this is to get the value of
    /// the property and make sure the sum of the three values you set has the same sum as the three values you got.
    #[doc(alias = "kAudioFilePropertyPacketTableInfo")]
    pub const PACKET_TABLE_INFO: Self = Self(u32::from_be_bytes(*b"pnfo"));

    /// In order to support formats such as AAC SBR where an encoded data stream can be decoded to
    /// multiple destination formats, this property returns an array of AudioFormatListItems (see AudioFormat.h) of those formats.
    /// The default behavior is to return the an AudioFormatListItem that has the same AudioStreamBasicDescription
    /// that kAudioFilePropertyDataFormat returns.
    #[doc(alias = "kAudioFilePropertyFormatList")]
    pub const FORMAT_LIST: Self = Self(u32::from_be_bytes(*b"flst"));

    #[doc(alias = "kAudioFilePropertyPacketSizeUpperBound")]
    pub const PACKET_SIZE_UPPER_BOUND: Self = Self(u32::from_be_bytes(*b"pkub"));

    /// Pass an AudioPacketRangeByteCountTranslation with mPacket and mPacketCount filled out
    /// and get mByteCountUpperBound back. The value of mByteCountUpperBound can be used to allocate a buffer
    /// for use with AudioFileReadPacketData in order to accommodate the entire packet range.
    /// May require scanning in order to obtain the requested information, but even if so, no scanning will occur
    /// beyond the last packet in the specified range.
    /// For file formats in which packets are directly accessible and stored both contiguously and byte-aligned,
    /// the returned upper bound will be equal to the total size of the packets in the range. Otherwise the
    /// upper bound may reflect per-packet storage overhead.
    #[doc(alias = "kAudioFilePropertyPacketRangeByteCountUpperBound")]
    pub const PACKET_RANGE_BYTE_COUNT_UPPER_BOUND: Self = Self(u32::from_be_bytes(*b"prub"));

    /// The value is a `f64` of the duration in seconds of data that is expected to be written.
    /// Setting this property before any data has been written reserves space in the file header for a packet table
    /// and/or other information so that it can appear before the audio data. Otherwise the packet table may get written at the
    /// end of the file, preventing the file from being streamable.
    #[doc(alias = "kAudioFilePropertyReserveDuration")]
    pub const RESERVE_DURATION: Self = Self(u32::from_be_bytes(*b"rsrv"));

    /// The value is a `f64` representing an estimated duration in seconds.
    /// If duration can be calculated without scanning the entire file,
    /// or all the audio data packets have been scanned, the value will
    /// accurately reflect the duration of the audio data.
    #[doc(alias = "kAudioFilePropertyEstimatedDuration")]
    pub const ESTIMATED_DURATION: Self = Self(u32::from_be_bytes(*b"edur"));

    /// Returns the bit rate for the audio data as a `u32`. For some formats this will be approximate.
    #[doc(alias = "kAudioFilePropertyBitRate")]
    pub const BIT_RATE: Self = Self(u32::from_be_bytes(*b"brat"));

    #[doc(alias = "kAudioFilePropertyID3Tag")]
    pub const ID3_TAG: Self = Self(u32::from_be_bytes(*b"id3t"));

    #[doc(alias = "kAudioFilePropertyID3TagOffset")]
    pub const ID3_TAG_OFFSET: Self = Self(u32::from_be_bytes(*b"id3o"));

    #[doc(alias = "kAudioFilePropertySourceBitDepth")]
    pub const SOURCE_BIT_DEPTH: Self = Self(u32::from_be_bytes(*b"sbtd"));

    #[doc(alias = "kAudioFilePropertyAlbumArtwork")]
    pub const ALBUM_ARTWORK: Self = Self(u32::from_be_bytes(*b"aart"));

    /// A `u32` that indicates the number of audio tracks contained in the file. (get property only)
    #[doc(alias = "kAudioFilePropertyAudioTrackCount")]
    pub const AUDIO_TRACK_COUNT: Self = Self(u32::from_be_bytes(*b"atct"));

    /// A `u32` that indicates the number of audio tracks contained in the file. (set property only)
    #[doc(alias = "kAudioFilePropertyUseAudioTrack")]
    pub const USE_AUDIO_TRACK: Self = Self(u32::from_be_bytes(*b"uatk"));
}

#[derive(Debug)]
#[doc(alias = "OpaqueAudioFileID")]
#[repr(transparent)]
pub struct OpaqueFileID(c_void);

#[derive(Debug)]
#[doc(alias = "AudioFileID")]
#[repr(transparent)]
pub struct FileID(&'static mut OpaqueFileID);

impl FileID {
    /// Creates a new audio file (or initialises an existing file)
    ///
    /// Creates a new (or initialises an existing) audio file specified by the URL.
    /// Upon success, an AudioFileID is returned which can be used for subsequent calls
    /// to the AudioFile APIs.
    pub fn create(
        url: &cf::Url,
        type_id: FileTypeID,
        format: &audio::StreamBasicDesc,
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
        url: &cf::Url,
        permissions: Permissions,
        type_hint: FileTypeID,
    ) -> Result<Self, os::Status> {
        let mut result = None;
        unsafe {
            AudioFileOpenURL(url, permissions, type_hint, &mut result).to_result_unchecked(result)
        }
    }

    #[doc(alias = "AudioFileWritePackets")]
    #[inline]
    pub fn write_packets(
        &mut self,
        use_cache: bool,
        num_bytes: u32,
        packet_descriptions: *const audio::StreamPacketDesc,
        starting_packet: isize,
        num_packets: *mut u32,
        buffer: *const u8,
    ) -> Result<(), os::Status> {
        unsafe {
            AudioFileWritePackets(
                self.0,
                use_cache,
                num_bytes,
                packet_descriptions,
                starting_packet,
                num_packets,
                buffer,
            )
            .result()
        }
    }

    #[doc(alias = "AudioFileWritePackets")]
    #[inline]
    pub fn read_packets(
        &mut self,
        use_cache: bool,
        num_bytes: &mut u32,
        packet_descriptions: *mut audio::StreamPacketDesc,
        starting_packet: isize,
        num_packets: *mut u32,
        buffer: *mut u8,
    ) -> Result<(), os::Status> {
        unsafe {
            AudioFileReadPacketData(
                self.0,
                use_cache,
                num_bytes,
                packet_descriptions,
                starting_packet,
                num_packets,
                buffer,
            )
            .result()
        }
    }

    #[doc(alias = "AudioFileGetPropertyInfo")]
    #[inline]
    pub fn property_info(&self, property_id: PropertyID) -> Result<(usize, bool), os::Status> {
        unsafe {
            let mut data_size = 0;
            let mut is_writable = 0;
            let res = AudioFileGetPropertyInfo(
                self.0,
                property_id,
                Some(&mut data_size),
                Some(&mut is_writable),
            );
            match res {
                os::Status::NO_ERR => Ok((data_size as _, is_writable == 1)),
                _ => Err(res),
            }
        }
    }

    #[doc(alias = "AudioFileGetProperty")]
    #[inline]
    pub unsafe fn get_property(
        &self,
        property_id: PropertyID,
        data_size: *mut u32,
        property_data: *mut c_void,
    ) -> os::Status {
        AudioFileGetProperty(self.0, property_id, data_size, property_data)
    }

    #[doc(alias = "AudioFileGetProperty")]
    #[inline]
    pub fn prop<T: Default>(&self, property_id: PropertyID) -> Result<T, os::Status> {
        let mut value = T::default();
        let mut size = std::mem::size_of::<T>() as u32;
        unsafe {
            self.get_property(property_id, &mut size, &mut value as *mut T as _)
                .result()?;
        }
        Ok(value)
    }

    #[doc(alias = "AudioFileSetProperty")]
    #[inline]
    pub fn set_prop<T: Sized>(
        &mut self,
        property_id: PropertyID,
        val: &T,
    ) -> Result<(), os::Status> {
        let size = std::mem::size_of::<T>() as u32;
        unsafe {
            self.set_property(property_id, size, val as *const T as _)
                .result()?;
        }
        Ok(())
    }

    #[doc(alias = "AudioFileSetProperty")]
    #[inline]
    pub unsafe fn set_property(
        &mut self,
        property_id: PropertyID,
        data_size: u32,
        property_data: *const c_void,
    ) -> os::Status {
        AudioFileSetProperty(self.0, property_id, data_size, property_data)
    }

    #[inline]
    pub unsafe fn prop_vec<T: Sized>(&self, property_id: PropertyID) -> Result<Vec<T>, os::Status> {
        let (size, _writable) = self.property_info(property_id)?;
        let mut sz: u32 = size as _;
        let mut vec = Vec::with_capacity(size / std::mem::size_of::<T>());
        self.get_property(property_id, &mut sz, vec.as_mut_ptr() as _)
            .result()?;
        vec.set_len(sz as usize / std::mem::size_of::<T>());
        Ok(vec)
    }

    #[inline]
    pub fn magic_cookie_data(&self) -> Result<Vec<u8>, os::Status> {
        unsafe { self.prop_vec(PropertyID::MAGIC_COOKIE_DATA) }
    }

    #[inline]
    pub fn maximum_packet_size(&self) -> Result<u32, os::Status> {
        self.prop(PropertyID::MAXIMUM_PACKET_SIZE)
    }

    #[inline]
    pub fn defer_size_updates(&self) -> Result<bool, os::Status> {
        Ok(self.prop::<u32>(PropertyID::DEFER_SIZE_UPDATES)? == 1)
    }

    #[inline]
    pub fn set_defer_size_updates(&mut self, val: bool) -> Result<(), os::Status> {
        let v: u32 = val as _;
        self.set_prop(PropertyID::DEFER_SIZE_UPDATES, &v)
    }

    #[inline]
    pub fn optimized(&self) -> Result<bool, os::Status> {
        Ok(self.prop::<u32>(PropertyID::IS_OPTIMIZED)? == 1)
    }

    /// Read only
    #[inline]
    pub fn file_format(&self) -> Result<FileTypeID, os::Status> {
        self.prop(PropertyID::FILE_FORMAT)
    }

    #[inline]
    pub fn data_format(&self) -> Result<audio::StreamBasicDesc, os::Status> {
        self.prop(PropertyID::DATA_FORMAT)
    }

    #[inline]
    pub fn set_data_format(&mut self, asbd: &audio::StreamBasicDesc) -> Result<(), os::Status> {
        self.set_prop(PropertyID::DATA_FORMAT, asbd)
    }

    #[inline]
    pub fn reserve_duration(&self) -> Result<f64, os::Status> {
        self.prop(PropertyID::RESERVE_DURATION)
    }

    #[inline]
    pub fn set_reserve_duration(&mut self, val: f64) -> Result<(), os::Status> {
        self.set_prop(PropertyID::RESERVE_DURATION, &val)
    }

    #[inline]
    pub fn estimated_duration(&self) -> Result<f64, os::Status> {
        self.prop(PropertyID::ESTIMATED_DURATION)
    }

    #[inline]
    pub fn info_dictionary(&self) -> Result<arc::R<cf::Dictionary>, os::Status> {
        self.prop(PropertyID::INFO_DICTIONARY)
    }

    /// Close an existing audio file.
    #[doc(alias = "AudioFileClose")]
    #[inline]
    pub fn close(&mut self) -> os::Status {
        unsafe { AudioFileClose(self.0) }
    }
}

impl Drop for FileID {
    fn drop(&mut self) {
        let res = self.close();
        debug_assert_eq!(res, 0, "failed to close audio file properly");
    }
}

/// Identifier for an audio file type.
#[derive(Debug, Default, Copy, Clone, Eq, PartialEq)]
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
        in_file_url: &cf::Url,
        in_file_type: FileTypeID,
        in_format: &audio::StreamBasicDesc,
        in_flags: Flags,
        out_audio_file: &mut Option<FileID>,
    ) -> os::Status;

    fn AudioFileClose(file: *const OpaqueFileID) -> os::Status;

    fn AudioFileOpenURL(
        file_url: &cf::Url,
        permissions: Permissions,
        file_type_hint: FileTypeID,
        out_audio_file: &mut Option<FileID>,
    ) -> os::Status;

    fn AudioFileGetPropertyInfo(
        file: *const OpaqueFileID,
        property_id: PropertyID,
        data_size: Option<&mut u32>,
        is_writable: Option<&mut u32>,
    ) -> os::Status;

    fn AudioFileGetProperty(
        file: *const OpaqueFileID,
        property_id: PropertyID,
        data_size: *mut u32,
        property_data: *mut c_void,
    ) -> os::Status;

    fn AudioFileSetProperty(
        file: *mut OpaqueFileID,
        property_id: PropertyID,
        data_size: u32,
        property_data: *const c_void,
    ) -> os::Status;

    fn AudioFileWritePackets(
        file: *mut OpaqueFileID,
        use_cache: bool,
        num_bytes: u32,
        packet_descriptions: *const audio::StreamPacketDesc,
        starting_packet: isize,
        num_packets: *mut u32,
        buffer: *const u8,
    ) -> os::Status;

    fn AudioFileReadPacketData(
        file: *mut OpaqueFileID,
        use_cache: bool,
        num_bytes: *mut u32,
        packet_descriptions: *mut audio::StreamPacketDesc,
        starting_packet: isize,
        num_packets: *mut u32,
        buffer: *mut u8,
    ) -> os::Status;
}

#[cfg(test)]
mod tests {
    use crate::{at::audio, cf};

    #[test]
    fn basics() {
        let path = cf::Url::from_str("file:///tmp/m4a.m4a").unwrap();

        let asbd = audio::StreamBasicDesc {
            format: audio::Format::MPEG4_AAC,
            format_flags: audio::FormatFlags::ALL_CLEAR,
            frames_per_packet: 1024,
            sample_rate: 48_000.0,
            channels_per_frame: 2,
            ..Default::default()
        };

        let mut file = audio::FileID::create(
            &path,
            audio::FileTypeID::M4A,
            &asbd,
            audio::FileFlags::ERASE_FILE,
        )
        .unwrap();

        let (size, writable) = file
            .property_info(audio::FilePropertyID::DEFER_SIZE_UPDATES)
            .unwrap();

        assert_eq!(size, std::mem::size_of::<u32>());
        assert_eq!(writable, true);

        let defer_size_updates = file.defer_size_updates().unwrap();

        assert!(defer_size_updates);

        file.set_defer_size_updates(false).unwrap();

        let defer_size_updates = file.defer_size_updates().unwrap();

        assert!(!defer_size_updates);

        assert_eq!(file.file_format().unwrap(), audio::FileTypeID::M4A);

        let (size, writable) = file
            .property_info(audio::FilePropertyID::FILE_FORMAT)
            .unwrap();

        assert_eq!(size, std::mem::size_of::<audio::FileTypeID>());
        assert_eq!(writable, false);

        let (size, writable) = file
            .property_info(audio::FilePropertyID::DATA_FORMAT)
            .unwrap();

        assert_eq!(size as usize, std::mem::size_of::<audio::StreamBasicDesc>());
        assert_eq!(writable, true);

        let file_asbd = file.data_format().unwrap();

        assert_eq!(file_asbd, asbd);

        file.set_data_format(&asbd).unwrap();

        let (size, writable) = file
            .property_info(audio::FilePropertyID::IS_OPTIMIZED)
            .unwrap();

        assert_eq!(size, std::mem::size_of::<u32>());
        assert_eq!(writable, false);

        assert_eq!(file.optimized().unwrap(), false);

        assert!(file
            .property_info(audio::FilePropertyID::DATA_FORMAT_NAME)
            .is_err());

        let (size, writable) = file
            .property_info(audio::FilePropertyID::RESERVE_DURATION)
            .unwrap();

        assert_eq!(size, std::mem::size_of::<f64>());
        assert_eq!(writable, true);

        file.set_reserve_duration(1000f64).unwrap();

        let (size, writable) = file
            .property_info(audio::FilePropertyID::ESTIMATED_DURATION)
            .unwrap();

        assert_eq!(size, std::mem::size_of::<f64>());
        assert_eq!(writable, false);

        assert_eq!(0f64, file.estimated_duration().unwrap());

        let (size, writable) = file
            .property_info(audio::FilePropertyID::INFO_DICTIONARY)
            .unwrap();

        assert_eq!(size, std::mem::size_of::<usize>());
        assert_eq!(writable, false);

        let info = file.info_dictionary().unwrap();

        assert!(!info.is_empty());
        info.show();

        file.close();
        std::mem::forget(file);
    }
}
