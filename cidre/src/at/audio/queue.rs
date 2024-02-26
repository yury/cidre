use std::{ffi::c_void, ptr::NonNull};

use crate::{at::audio, blocks, define_opts};

#[doc(alias = "AudioQueuePropertyID")]
#[repr(transparent)]
pub struct QueueProp(pub u32);

#[doc(alias = "AudioQueueParameterID")]
#[repr(transparent)]
pub struct QueueParam(pub u32);

#[doc(alias = "AudioQueueParameterValue")]
pub type QueueParamValue = f32;

#[doc(alias = "AudioQueueRef")]
#[repr(transparent)]
pub struct Queue(NonNull<c_void>);

#[repr(transparent)]
pub struct QueueTimeline(NonNull<c_void>);

pub mod err {
    use crate::os::Status;

    pub const INVALID_BUFFER: Status = Status(-66687);
    pub const BUFFER_EMPTY: Status = Status(-66686);
    pub const DISPOSAL_PENDING: Status = Status(-66685);
    pub const INVALID_PROPERTY: Status = Status(-66684);
    pub const INVALID_PROPERTY_SIZE: Status = Status(-66683);
    pub const INVALID_PARAMETER: Status = Status(-66682);
    pub const CANNOT_START: Status = Status(-66681);
    pub const INVALID_DEVICE: Status = Status(-66680);
    pub const BUFFER_IN_QUEUE: Status = Status(-66679);
    pub const INVALID_RUN_STATE: Status = Status(-66678);
    pub const INVALID_QUEUE_TYPE: Status = Status(-66677);
    pub const PERMISSIONS: Status = Status(-66676);
    pub const INVALID_PROPERTY_VALUE: Status = Status(-66675);
    pub const PRIME_TIMED_OUT: Status = Status(-66674);
    pub const CODEC_NOT_FOUND: Status = Status(-66673);
    pub const INVALID_CODEC_ACCESS: Status = Status(-66672);
    pub const QUEUE_INVALIDATED: Status = Status(-66671);
    pub const TOO_MANY_TAPS: Status = Status(-66670);
    pub const INVALID_TAP_CONTEXT: Status = Status(-66669);
    pub const RECORD_UNDERRUN: Status = Status(-66668);
    pub const INVALID_TAP_TYPE: Status = Status(-66667);
    pub const BUFFER_ENQUEUED_TWICE: Status = Status(-66666);
    pub const CANNOT_START_YET: Status = Status(-66665);
    pub const ENQUEUE_DURING_RESET: Status = Status(-66632);
    pub const INVALID_OFFLINE_MODE: Status = Status(-66626);
}

impl QueueProp {
    /// A read-only property whose value is a u32 that indicates whether or not the queue is
    /// running. A notification is sent when the audio device starts or stops, which is not
    /// necessarily when the start or stop function is called.
    #[doc(alias = "kAudioQueueProperty_IsRunning")]
    pub const IS_RUNNING: Self = Self(u32::from_be_bytes(*b"aqrn"));

    /// A read-only property whose value is a f64 that indicates the sampling rate of the
    /// associated audio device.
    #[doc(alias = "kAudioQueueDeviceProperty_SampleRate")]
    pub const SAMPLE_RATE: Self = Self(u32::from_be_bytes(*b"aqsr"));

    /// A read-only property whose value is a u32 that indicates the number of channels in
    /// the associated audio device.
    #[doc(alias = "kAudioQueueDeviceProperty_NumberChannels")]
    pub const NUMBER_CHANNELS: Self = Self(u32::from_be_bytes(*b"aqdc"));

    /// A read/write property whose value is a cf::String that contains the unique identifier
    /// (UID) of the associated audio device.
    /// If the audio queue is tracking the default system device and the device changes, it will
    /// generate a property changed notification for this property. You can then query the HAL
    /// for info on the new default system device.
    #[doc(alias = "kAudioQueueProperty_CurrentDevice")]
    pub const CURRENT_DEVICE: Self = Self(u32::from_be_bytes(*b"aqcd"));

    /// A read/write property whose value is an audio format magic cookie. If the audio format
    /// requires a magic cookie, you must set this property before enqueuing any buffers.
    #[doc(alias = "kAudioQueueProperty_MagicCookie")]
    pub const MAGIC_COOKIE: Self = Self(u32::from_be_bytes(*b"aqmc"));

    /// A read-only u32 that indicates the size in bytes of the largest single packet of
    /// data in the output format. This is mostly useful for recording/encoding variable bit rate
    /// compressed data.
    #[doc(alias = "kAudioQueueProperty_MaximumOutputPacketSize")]
    pub const MAX_OUTPUT_PACKET_SIZE: Self = Self(u32::from_be_bytes(*b"xops"));

    /// A read-only AudioStreamBasicDescription that indicates the queue's recording format.
    /// This is useful when recording, where you may specify a sample rate of 0 during
    /// construction.
    #[doc(alias = "kAudioQueueProperty_StreamDescription")]
    pub const STREAM_DESCRIPTION: Self = Self(u32::from_be_bytes(*b"aqft"));

    /// A read/write property whose value is an audio channel layout structure that describes
    /// the audio queue's channel layout. The number of channels must match the format of the
    /// queue. If more than two channels (for instance, a five-channel surround sound) exist in
    /// the queue, there may be a need to specify a channel layout. This layout indicates the
    /// specific order in which the channels do appear, such as left, then center, then right.
    #[doc(alias = "kAudioQueueProperty_ChannelLayout")]
    pub const CHANNEL_LAYOUT: Self = Self(u32::from_be_bytes(*b"aqcl"));

    /// A read-write property whose value is a UInt32 that indicates whether metering of
    /// audio levels is enabled for the audio queue. (0=off, 1=on).
    #[doc(alias = "kAudioQueueProperty_EnableLevelMetering")]
    pub const ENABLE_LEVEL_METERING: Self = Self(u32::from_be_bytes(*b"aqme"));

    /// A read-only property whose value is an array of AudioQueueLevelMeter structures, one
    /// array element per audio channel. The values in the AudioQueueLevelMeters are in the
    /// range 0-1.
    #[doc(alias = "kAudioQueueProperty_CurrentLevelMeter")]
    pub const CURRENT_LEVEL_METER: Self = Self(u32::from_be_bytes(*b"aqmv"));

    /// A read-only property whose value is an array of AudioQueueLevelMeter structures, one
    /// array element per audio channel. The values in the AudioQueueLevelMeters are in
    /// decibels.
    #[doc(alias = "kAudioQueueProperty_CurrentLevelMeterDB")]
    pub const CURRENT_LEVEL_METER_DB: Self = Self(u32::from_be_bytes(*b"aqmd"));

    /// A read/write property whose value is a UInt32 that is the size of the buffer into which
    /// an output audio queue decodes buffers. A large buffer provides more reliability and
    /// better long-term performance at the expense of memory and decreased responsiveness
    /// in some situations.
    #[doc(alias = "kAudioQueueProperty_DecodeBufferSizeFrames")]
    pub const DECODE_BUFFER_SIZE_FRAMES: Self = Self(u32::from_be_bytes(*b"dcbf"));

    /// A read-only property whose value is a UInt32 indicating the most recent error (if any)
    /// encountered by the queue's internal encoding/decoding process.
    #[doc(alias = "kAudioQueueProperty_ConverterError")]
    pub const CONVERTER_ERROR: Self = Self(u32::from_be_bytes(*b"qcve"));

    /// A read/write property whose value is a UInt32 describing whether there is a time/pitch unit
    /// inserted into the queue's audio signal chain. This property may only be set while
    /// the queue is stopped.
    #[doc(alias = "kAudioQueueProperty_EnableTimePitch")]
    pub const ENABLE_TIME_PITCH: Self = Self(u32::from_be_bytes(*b"q_tp"));

    /// A read/write property whose value is a UInt32 describing the time/pitch algorithm in use.
    /// This property is only valid while a time/pitch has been inserted, and may only be changed
    /// when the queue is not running.
    #[doc(alias = "kAudioQueueProperty_TimePitchAlgorithm")]
    pub const TIME_PITCH_ALGORITHM: Self = Self(u32::from_be_bytes(*b"qtpa"));

    /// A read/write property whose value is a UInt32 describing whether the time/pitch unit
    /// has been bypassed (1=bypassed, 0=not bypassed).
    #[doc(alias = "kAudioQueueProperty_TimePitchBypass")]
    pub const TIME_PITCH_BYPASS: Self = Self(u32::from_be_bytes(*b"qtpb"));
}

#[repr(transparent)]
pub struct QueueTimePitchAlgorithm(pub u32);

impl QueueTimePitchAlgorithm {
    /// Highest quality, most computationally expensive. Suitable for music.
    /// Default algorithm on macOS.
    #[doc(alias = "kAudioQueueTimePitchAlgorithm_Spectral")]
    pub const SPECTRAL: Self = Self(u32::from_be_bytes(*b"spec"));

    /// Modest quality, less expensive. Suitable for voice.
    #[doc(alias = "kAudioQueueTimePitchAlgorithm_TimeDomain")]
    pub const TIME_DOMAIN: Self = Self(u32::from_be_bytes(*b"tido"));

    /// High quality, but pitch varies with rate.
    #[doc(alias = "kAudioQueueTimePitchAlgorithm_Varispeed")]
    pub const VARISPEED: Self = Self(u32::from_be_bytes(*b"vspd"));
}

impl QueueProp {
    #[doc(alias = "kAudioQueueProperty_HardwareCodecPolicy")]
    pub const HARDWARE_CODEC_POLICY: Self = Self(u32::from_be_bytes(*b"aqcp"));

    /// A write-only property whose value is an array of AudioQueueChannelAssignment. There must be
    /// one array element for each channel of the queue's format as specified in the
    /// AudioStreamBasicDescription passed to AudioQueueNewOutput or AudioQueueNewInput.
    /// (New in iOS 6.0).
    #[doc(alias = "kAudioQueueProperty_ChannelAssignments")]
    pub const CHANNEL_ASSIGNMENTS: Self = Self(u32::from_be_bytes(*b"aqca"));
}

impl QueueParam {
    /// A value from 0.0 to 1.0 indicating the linearly scaled gain for the queue. A value of
    /// 1.0 (the default) indicates unity gain. A value of 0.0 indicates zero gain, or silence.
    #[doc(alias = "kAudioQueueParam_Volume")]
    pub const VOLUME: Self = Self(1);

    /// A value from 0.5 to 2.0 indicating the rate at which the queue is to play. A value of
    /// 1.0 (the default) indicates that the queue should play at its normal rate. Only
    /// applicable when the time/pitch processor has been enabled and on macOS 10.6 and higher.
    #[doc(alias = "kAudioQueueParam_PlayRate")]
    pub const PLAY_RATE: Self = Self(2);

    /// A value from -2400 to 2400 indicating the number of cents to pitch-shift the queue's
    /// playback. (1200 cents is one octave.) Only applicable when the time/pitch processor has
    /// been enabled with the spectral algorithm, and on macOS 10.6 and higher.
    #[doc(alias = "kAudioQueueParam_Pitch")]
    pub const PITCH: Self = Self(3);

    /// A value indicating the number of seconds over which subsequent volume changes will be
    /// ramped. For example, to fade out from full unity gain to silence over the course of 1
    /// second, set kAudioQueueParam_VolumeRampTime to 1 then kAudioQueueParam_Volume to 0.
    #[doc(alias = "kAudioQueueParam_VolumeRampTime")]
    pub const VOLUME_RAMP_TIME: Self = Self(4);

    /// A value from -1 to 1 indicating the pan position of a mono source (-1 = hard left, 0 =
    /// center, 1 = hard right). For a stereo source this parameter affects left/right balance.
    /// For multi-channel sources, this parameter has no effect.
    #[doc(alias = "kAudioQueueParam_Pan")]
    pub const PAN: Self = Self(4);
}

define_opts!(
    pub QueueProcessingTapFlags(u32)
);

impl QueueProcessingTapFlags {
    #[doc(alias = "kAudioQueueProcessingTap_PreEffects")]
    pub const PRE_EFFECTS: Self = Self(1 << 0);

    #[doc(alias = "kAudioQueueProcessingTap_PostEffects")]
    pub const POST_EFFECTS: Self = Self(1 << 1);

    #[doc(alias = "kAudioQueueProcessingTap_StartOfStream")]
    pub const START_OF_STREAM: Self = Self(1 << 8);

    #[doc(alias = "kAudioQueueProcessingTap_EndOfStream")]
    pub const END_OF_STREAM: Self = Self(1 << 9);
}

#[doc(alias = "AudioQueueBuffer")]
#[repr(C)]
pub struct QueueBuf {
    pub audio_data_bytes_capacity: u32,
    pub audio_data: *const c_void,
    pub audio_data_byte_size: u32,
    pub user_data: *mut c_void,
    pub packet_description_capacity: u32,
    pub packet_descriptions: *const audio::StreamPacketDesc,
    pub packet_description_count: u32,
}

#[repr(C)]
pub struct QueueParamEvent {
    pub id: QueueParam,
    pub value: QueueParamValue,
}

#[repr(C)]
pub struct QueueLevelMeterState {
    /// The audio channel's average RMS power.
    pub average_power: f32,
    /// The audio channel's peak RMS power
    pub peak_power: f32,
}

/// Defines a pointer to a block that is called when a playback audio
/// queue has finished taking data from a buffer.
///
/// A playback buffer callback is invoked when the audio queue has finished with the data to
/// be played and the buffer is available to your application for reuse. Your application
/// might want to immediately refill and re-enqueue the completed buffer at this time.
#[doc(alias = "AudioQueueOutputCallbackBlock")]
#[cfg(feature = "blocks")]
pub type QueueOutputCbBlock = blocks::EscBlock<fn(&mut Queue, &mut QueueBuf)>;

/// Defines a pointer to a block that is called when a recording audio
/// queue has finished filling a buffer.
///
/// You specify a recording buffer callback when calling AudioQueueNewInput. Your callback
/// is invoked each time the recording audio queue has filled a buffer with input data.
/// Typically, your callback should write the audio queue buffer's data to a file or other
/// buffer, and then re-queue the audio queue buffer to receive more data.
#[doc(alias = "AudioQueueInputCallbackBlock")]
#[cfg(feature = "blocks")]
pub type QueueInputCbBlock = blocks::EscBlock<
    fn(&mut Queue, &mut QueueBuf, *const audio::TimeStamp, u32, *const audio::StreamPacketDesc),
>;

pub type QueueOutputCb = extern "C" fn(*mut c_void, &mut Queue, &mut QueueBuf);

pub type QueueInputCb = extern "C" fn(
    *mut c_void,
    &mut Queue,
    &mut QueueBuf,
    *const audio::TimeStamp,
    u32,
    *const audio::StreamPacketDesc,
);

pub type QueuePropListenerProc = extern "C" fn(*mut c_void, &mut Queue, QueueProp);
