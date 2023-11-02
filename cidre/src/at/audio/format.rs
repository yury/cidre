use std::{ffi::c_void, mem::size_of, ptr::NonNull};

use crate::{cat::audio, os};

#[doc(alias = "AudioFormatPropertyID")]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[repr(transparent)]
pub struct Property(pub u32);

impl Property {
    pub const fn from_be_bytes(bytes: [u8; 4]) -> Self {
        Self(u32::from_be_bytes(bytes))
    }
}

/// AudioPanningMode
#[derive(Debug, PartialEq, Eq, Copy, Clone)]
#[repr(u32)]
pub enum PanningMode {
    None = 0,

    /// Sound field panning algorithm
    SoundField = 3,

    /// Vector based panning algorithm
    VectorBasedPanning = 4,
}

#[doc(alias = "AudioPanningInfo")]
#[repr(C)]
pub struct PanningInfo<const N: usize> {
    /// the PanningMode to be used for the pan
    pub panning_mode: PanningMode,

    /// the coordinates are specified as in the AudioChannelDescription struct in CoreAudioTypes.h
    pub coordinate_flags: u32,
    /// the coordinates are specified as in the AudioChannelDescription struct in CoreAudioTypes.h
    pub coordinats: [f32; 3],
    /// mGainScale is used to multiply the panning values.
    /// In typical usage you are applying an existing volume.
    /// value in 0 -> 1 (where 1 is unity gain) to the panned values.
    /// 1 would give you panning at unity.
    /// 0 would give you back a matrix of zeroes.
    pub gain_scale: f32,
    /// This is the channel map that is going to be used to determine channel volumes for this pan.
    pub output_channel_map: *const audio::ChannelLayout<N>,
}

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
#[repr(u32)]
pub enum BalanceFadeType {
    /// the gain value never exceeds 1.0, the opposite channel fades out.
    /// This can reduce overall loudness when the balance or fade is not in the center.
    MaxUnityGain = 0,

    /// The overall loudness remains constant, but gain can exceed 1.0.
    /// the gain value is 1.0 when the balance and fade are in the center.
    /// From there they can increase to +3dB (1.414) and decrease to -inf dB (0.0).
    EqualPower = 1,
}

/// this struct is used with kAudioFormatProperty_BalanceFade
#[doc(alias = "AudioBalanceFade")]
#[repr(C)]
pub struct BalanceFade<const N: usize> {
    /// -1 is full left, 0 is center, +1 is full right
    pub left_right_balance: f32,
    /// -1 is full rear, 0 is center, +1 is full front
    pub back_front_fade: f32,
    /// max unity gain, or equal power.
    pub r#type: BalanceFadeType,
    pub channel_layout: *const audio::ChannelLayout<N>,
}

#[doc(alias = "AudioFormatInfo")]
#[repr(C)]
pub struct FormatInfo {
    pub asbd: audio::StreamBasicDesc,
    pub magic_cookie: NonNull<c_void>,
    pub magic_cookie_size: u32,
}

/// This struct is used as a specifier for the kAudioFormatProperty_FormatList property
#[doc(alias = "ExtendedAudioFormatInfo")]
#[repr(C)]
pub struct ExtendedFormatInfo {
    pub asbd: audio::StreamBasicDesc,
    pub magic_cookie: *const c_void,
    pub magic_cookie_size: u32,
    pub class_desc: audio::ClassDesc,
}

pub mod asbd_props {
    use super::Property;

    /// Retrieves general information about a format. The specifier is a
    /// magic cookie, or NULL.
    /// On input, the property value is an AudioStreamBasicDescription which
    /// should have at least the 'format' filled out. On output it will be filled out
    /// as much as possible given the information known about the format
    /// and the contents of the magic cookie (if any is given).
    /// If multiple formats can be described by the AudioStreamBasicDescription and the associated magic cookie,
    /// this property will return the base level format.
    pub const FORMAT_INFO: Property = Property::from_be_bytes(*b"fmti");

    /// Returns a name for a given format. The specifier is an
    /// AudioStreamBasicDescription describing the format to ask about.
    /// The value is a cf::String. The caller is responsible for releasing the
    /// returned string. For some formats (eg, Linear PCM) you will get back a
    /// descriptive string (e.g. 16-bit, interleaved, etc...)
    pub const FORMAT_NAME: Property = Property::from_be_bytes(*b"fnam");

    /// No specifier needed. Must be set to NULL.
    /// Returns an array of u32 format IDs for formats that are valid output formats
    /// for a converter.
    pub const ENCODE_FORMAT_IDS: Property = Property::from_be_bytes(*b"acof");

    /// No specifier needed. Must be set to NULL.
    /// Returns an array of UInt32 format IDs for formats that are valid input formats
    pub const DECODE_FORMAT_IDS: Property = Property::from_be_bytes(*b"acif");

    /// Returns a list of AudioFormatListItem structs describing the audio formats contained within the compressed bit stream
    /// as described by the magic cookie. The specifier is an AudioFormatInfo struct. The 'format' member of the
    ///
    /// ASBD struct must filled in. Formats are returned in order from the most to least 'rich', with
    /// channel count taking the highest precedence followed by sample rate. The kAudioFormatProperty_FormatList property
    /// is the preferred method for discovering format information of the audio data. If the audio data can only be described
    /// by a single AudioFormatListItem, this property would be equivalent to using the kAudioFormatProperty_FormatInfo property,
    /// which should be used by the application as a fallback case, to ensure backward compatibility with existing systems
    /// when kAudioFormatProperty_FormatList is not present on the running system.
    pub const FORMAT_LIST: Property = Property::from_be_bytes(*b"flst");

    /// Returns an audio stream description for a given ESDS. The specifier is an ESDS.
    /// The value is a AudioStreamBasicDescription. If multiple formats can be described
    /// by the ESDS this property will return the base level format.
    pub const ASBD_FROM_ESDS: Property = Property::from_be_bytes(*b"essd");

    /// Returns an audio channel layout for a given ESDS. The specifier is an
    /// ESDS. The value is a AudioChannelLayout.
    pub const CHANNEL_LAYOUT_FROM_ESDS: Property = Property::from_be_bytes(*b"escl");

    /// Returns a list of AudioFormatListItem structs describing the audio formats which may be obtained by decoding the format
    /// described by the specifier.
    ///
    /// The specifier is an AudioFormatInfo struct. At a minimum formatID member of the ASBD struct must filled in. Other fields
    /// may be filled in. If there is no magic cookie, then the number of channels and sample rate should be filled in.
    pub const OUTPUT_FORMAT_LIST: Property = Property::from_be_bytes(*b"ofls");

    /// The specifier is a list of 1 or more AudioFormatListItem. Generally it is the list of these items returned from kAudioFormatProperty_FormatList. The property value retrieved is an UInt32 that specifies an index into that list. The list that the caller provides is generally sorted with the first item as the best format (most number of channels, highest sample rate), and the returned index represents the first item in that list that can be played by the system.
    /// Thus, the property is typically used to determine the best playable format for a given (layered) audio stream
    pub const FIRST_PLAYABLE_FORMAT_FROM_LIST: Property = Property::from_be_bytes(*b"fpfl");

    /// Returns whether or not a format has a variable number of bytes per
    /// packet. The specifier is an AudioStreamBasicDescription describing
    /// the format to ask about. The value is a u32 where non-zero means
    /// the format is variable bytes per packet.
    pub const FORMAT_IS_VBR: Property = Property::from_be_bytes(*b"fvbr");

    /// Returns whether or not a format requires external framing information,
    /// i.e. AudioStreamPacketDescriptions.
    /// The specifier is an AudioStreamBasicDescription describing
    /// the format to ask about. The value is a UInt32 where non-zero means
    /// the format is externally framed. Any format which has variable byte sized packets
    /// requires AudioStreamPacketDescriptions.
    pub const FORMAT_IS_EXTERNALLY_FRAMED: Property = Property::from_be_bytes(*b"fexf");

    /// Returns whether or not a format is capable of combining independently
    /// decodable packets with dependent packets. The specifier is an
    /// AudioStreamBasicDescription describing the format to ask about.
    /// The value is a UInt32 where zero means that all packets in streams
    /// of the specified format are independently decodable and non-zero means
    /// that streams of the specified format may include dependent packets.
    pub const FORMAT_EMPLOYS_DEPENDENT_PACKETS: Property = Property::from_be_bytes(*b"fdep");

    /// Returns whether or not a format is encrypted. The specifier is a u32 format ID.
    /// The value is a u32 where non-zero means the format is encrypted.
    pub const FORMAT_IS_ENCRYPTED: Property = Property::from_be_bytes(*b"cryp");

    /// The specifier is the format that you are interested in, e.g. 'aac '
    /// Returns an array of AudioClassDescriptions for all installed encoders for the given format
    pub const ENCODERS: Property = Property::from_be_bytes(*b"aven");

    /// The specifier is the format that you are interested in, e.g. 'aac '
    /// Returns an array of AudioClassDescriptions for all installed decoders for the given format
    pub const DECODERS: Property = Property::from_be_bytes(*b"avde");

    /// The specifier is a u32 'audio::Format'.
    /// The property value is an array of AudioValueRange describing all available bit rates.
    pub const AVAILABLE_ENCODE_BIT_RATES: Property = Property::from_be_bytes(*b"aebr");

    /// The specifier is a u32 format ID.
    /// The property value is an array of AudioValueRange describing all available sample rates.
    pub const AVAILABLE_ENCODE_SAMPLE_RATES: Property = Property::from_be_bytes(*b"aesr");

    /// The specifier is an AudioStreamBasicDescription with at least the format_id
    /// and channels_per_frame fields set.
    /// The property value is an array of AudioChannelLayoutTags for the format and number of
    /// channels specified. If mChannelsPerFrame is zero, then all layouts supported by
    /// the format are returned.
    pub const AVAILABLE_ENCODE_CHANNEL_LAYOUT_TAGS: Property = Property::from_be_bytes(*b"aecl");

    /// The specifier is an AudioStreamBasicDescription with at least the format_id field set.
    /// The property value is an array of UInt32 indicating the number of channels that can be encoded.
    /// A value of 0xFFFFFFFF indicates that any number of channels may be encoded.
    pub const AVAILABLE_ENCODE_NUMBER_CHANNELS: Property = Property::from_be_bytes(*b"avnc");

    /// The specifier is an AudioStreamBasicDescription with at least the format_id field set.
    /// The property value is an array of UInt32 indicating the number of channels that can be decoded.
    pub const AVAILABLE_DECODE_NUMBER_CHANNELS: Property = Property::from_be_bytes(*b"adnc");

    /// Returns an audio stream description for a given MPEG Packet. The specifier is an MPEG Packet.
    /// The value is a AudioStreamBasicDescription.
    pub const ASBD_FROM_MPEG_PACKET: Property = Property::from_be_bytes(*b"admp");
}

pub mod channel_layout_props {
    use super::Property;

    /// Returns a bitmap for an AudioChannelLayoutTag, if there is one.
    /// The specifier is a AudioChannelLayoutTag  containing the layout tag.
    /// The value is an u32 bitmap. The bits are as defined in CoreAudioTypes.h.
    /// To go in the other direction, i.e. get a layout tag for a bitmap,
    /// use kAudioFormatProperty_TagForChannelLayout where your layout tag
    /// is kAudioChannelLayoutTag_UseChannelBitmap and the bitmap is filled in.
    pub const BITMAP_FOR_LAYOUT_TAG: Property = Property::from_be_bytes(*b"bmtg");
    pub const MATRIX_MIX_MAP: Property = Property::from_be_bytes(*b"mmap");
    pub const CHANNEL_MAP: Property = Property::from_be_bytes(*b"chmp");
    pub const NUMBER_OF_CHANNELS_FOR_LAYOUT: Property = Property::from_be_bytes(*b"nchm");
    pub const ARE_CHANNEL_LAYOUTS_EQUIVALENT: Property = Property::from_be_bytes(*b"cheq");
    pub const CHANNEL_LAYOUT_HASH: Property = Property::from_be_bytes(*b"chha");

    pub const VALIDATE_CHANNEL_LAYOUT: Property = Property::from_be_bytes(*b"vacl");
    pub const CHANNEL_LAYOUT_FOR_TAG: Property = Property::from_be_bytes(*b"cmpl");
    pub const TAG_FOR_CHANNEL_LAYOUT: Property = Property::from_be_bytes(*b"cmpt");
    pub const CHANNEL_LAYOUT_NAME: Property = Property::from_be_bytes(*b"lonm");
    pub const CHANNEL_LAYOUT_SIMPLE_NAME: Property = Property::from_be_bytes(*b"lsnm");
    pub const CHANNEL_LAYOUT_FOR_BITMAP: Property = Property::from_be_bytes(*b"cmpb");
    pub const CHANNEL_NAME: Property = Property::from_be_bytes(*b"cnam");
    pub const CHANNEL_SHORT_NAME: Property = Property::from_be_bytes(*b"csnm");

    pub const TAGS_FOR_NUMBER_OF_CHANNELS: Property = Property::from_be_bytes(*b"tagc");
    pub const PANNING_MATRIX: Property = Property::from_be_bytes(*b"panm");
    pub const BALANCE_FADE: Property = Property::from_be_bytes(*b"balf");
}

pub mod id3_props {
    use super::Property;
    pub const ID3_TAG_SIZE: Property = Property::from_be_bytes(*b"id3s");
    pub const D3_TAG_TO_DICTIONARY: Property = Property::from_be_bytes(*b"id3d");
}

impl Property {
    pub unsafe fn info(
        &self,
        in_specifier_size: u32,
        in_specifier: *const c_void,
    ) -> Result<u32, os::Status> {
        let mut out_property_data_size = 0;
        let res = AudioFormatGetPropertyInfo(
            *self,
            in_specifier_size,
            in_specifier,
            &mut out_property_data_size,
        );
        if res.is_ok() {
            Ok(out_property_data_size)
        } else {
            Err(res)
        }
    }

    pub unsafe fn value(
        &self,
        in_specifier_size: u32,
        in_specifier: *const c_void,
        io_property_data_size: *mut u32,
        out_property_data: *mut c_void,
    ) -> Result<(), os::Status> {
        let res = AudioFormatGetProperty(
            *self,
            in_specifier_size,
            in_specifier,
            io_property_data_size,
            out_property_data,
        );
        res.result()
    }

    pub unsafe fn fill<T: Sized>(&self, val: &mut T) -> Result<(), os::Status> {
        let mut size = size_of::<T>() as u32;
        self.value(0, std::ptr::null(), &mut size, val as *mut _ as *mut _)
    }

    pub unsafe fn fill_with<S: Sized, T: Sized>(
        &self,
        val: &mut T,
        specifier: &S,
    ) -> Result<(), os::Status> {
        let mut size = size_of::<T>() as u32;
        let spec_size = size_of::<S>() as u32;
        self.value(
            spec_size,
            specifier as *const _ as _,
            &mut size,
            val as *mut _ as *mut _,
        )
    }

    pub unsafe fn get_vec<T: Sized>(&self) -> Result<Vec<T>, os::Status> {
        let mut size = self.info(0, std::ptr::null())?;
        let len = size as usize / size_of::<T>();
        let mut result = Vec::with_capacity(len);
        self.value(0, std::ptr::null(), &mut size, result.as_mut_ptr() as _)?;
        result.set_len(len);
        Ok(result)
    }

    pub unsafe fn get_vec_with<S: Sized, T: Sized>(
        &self,
        specifier: &S,
    ) -> Result<Vec<T>, os::Status> {
        let spec_size = size_of::<S>() as u32;
        let mut size = self.info(spec_size, specifier as *const _ as _)?;
        let len = size as usize / size_of::<T>();
        let mut result = Vec::with_capacity(len);
        self.value(
            spec_size,
            specifier as *const _ as _,
            &mut size,
            result.as_mut_ptr() as _,
        )?;
        result.set_len(len);
        Ok(result)
    }

    /// ```
    /// use cidre::at;
    ///
    /// let formats = at::AudioFormatProperty::encode_formats().unwrap();
    /// println!("{:?}", formats.len());
    /// assert!(formats.len() > 0);
    /// ```
    pub fn encode_formats() -> Result<Vec<audio::Format>, os::Status> {
        unsafe { asbd_props::ENCODE_FORMAT_IDS.get_vec() }
    }

    /// ```
    /// use cidre::at::audio;
    ///
    /// let mut asbd = audio::StreamBasicDesc {
    ///     format: audio::Format::LINEAR_PCM,
    ///     ..Default::default()
    /// };
    /// audio::FormatProperty::format_info(&mut asbd).unwrap();
    ///
    /// ```
    pub fn format_info(asbd: &mut audio::StreamBasicDesc) -> Result<(), os::Status> {
        unsafe { asbd_props::FORMAT_INFO.fill(asbd) }
    }

    /// ```
    /// use cidre::at::audio;
    ///
    /// let encoders = audio::FormatProperty::encoders(audio::Format::MPEG4_AAC).unwrap();
    /// println!("encoders: {:?}", encoders);
    /// assert!(encoders.len() > 0);
    /// ```
    pub fn encoders(format: audio::Format) -> Result<Vec<audio::ClassDesc>, os::Status> {
        unsafe { asbd_props::ENCODERS.get_vec_with(&format) }
    }

    pub fn decoders(format: audio::Format) -> Result<Vec<audio::ClassDesc>, os::Status> {
        unsafe { asbd_props::DECODERS.get_vec_with(&format) }
    }

    /// ```
    /// use cidre::at::audio;
    ///
    /// let rates = audio::FormatProperty::available_encode_bit_rates(audio::Format::MPEG4_AAC).unwrap();
    /// println!("{:?}", rates);
    /// assert!(rates.len() > 0);
    /// ```
    pub fn available_encode_bit_rates(
        format: audio::Format,
    ) -> Result<Vec<audio::ValueRange>, os::Status> {
        unsafe { asbd_props::AVAILABLE_ENCODE_BIT_RATES.get_vec_with(&format) }
    }

    /// ```
    /// use cidre::at::audio;
    ///
    /// let rates = audio::FormatProperty::available_encode_sample_rates(audio::Format::MPEG4_AAC).unwrap();
    /// println!("{:?}", rates);
    /// assert!(rates.len() > 0);
    /// ```
    pub fn available_encode_sample_rates(
        format: audio::Format,
    ) -> Result<Vec<audio::ValueRange>, os::Status> {
        unsafe { asbd_props::AVAILABLE_ENCODE_SAMPLE_RATES.get_vec_with(&format) }
    }
}

#[link(name = "AudioToolbox", kind = "framework")]
extern "C" {
    fn AudioFormatGetPropertyInfo(
        property_id: Property,
        in_specifier_size: u32,
        in_specifier: *const c_void,
        out_property_data_size: *mut u32,
    ) -> os::Status;

    fn AudioFormatGetProperty(
        property_id: Property,
        in_specifier_size: u32,
        in_specifier: *const c_void,
        io_property_data_size: *mut u32,
        out_property_data: *mut c_void,
    ) -> os::Status;
}
