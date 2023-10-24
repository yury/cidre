use std::ffi::c_void;

use crate::{
    arc,
    at::{audio::StreamPacketDesc, AudioBufList},
    define_cls, define_obj_type, ns, objc,
};

use super::{Format, FrameCount, PacketCount};

define_obj_type!(Buf(ns::Id));

/// A buffer of audio data, with a format.
impl Buf {
    #[objc::msg_send(format)]
    pub fn format(&self) -> &Format;

    #[objc::msg_send(audioBufferList)]
    pub fn audio_buffer_list(&self) -> &AudioBufList;

    #[objc::msg_send(mutableAudioBufferList)]
    pub fn audio_buffer_list_mut(&mut self) -> &mut AudioBufList;
}

define_obj_type!(PcmBuf(Buf));

impl arc::A<PcmBuf> {
    #[objc::msg_send(initWithPCMFormat:frameCapacity:)]
    pub fn init_with_pcm_format_frame_capacity(
        self,
        format: &Format,
        frame_capacity: FrameCount,
    ) -> Option<arc::R<PcmBuf>>;
}

/// Provides a number of methods useful for manipulating buffers of
/// audio in PCM format.
impl PcmBuf {
    define_cls!(AV_AUDIO_PCM_BUFFER);

    pub fn with_format_and_frame_capacity(
        format: &Format,
        frame_capacity: FrameCount,
    ) -> Option<arc::R<Self>> {
        Self::alloc().init_with_pcm_format_frame_capacity(format, frame_capacity)
    }
    /// The current number of valid sample frames in the buffer.
    ///
    /// You may modify the length of the buffer as part of an operation that modifies its contents.
    /// The length must be less than or equal to the frameCapacity. Modifying frameLength will update
    /// the mDataByteSize in each of the underlying AudioBufferList's AudioBuffer's correspondingly,
    /// and vice versa. Note that in the case of deinterleaved formats, mDataByteSize will refers
    /// the size of one channel's worth of audio samples.
    #[objc::msg_send(frameLength)]
    pub fn frame_length(&self) -> FrameCount;

    #[objc::msg_send(setFrameLength:)]
    pub fn set_frame_length(&mut self, value: FrameCount);

    /// The buffer's number of interleaved channels.
    ///
    /// Useful in conjunction with floatChannelData etc.
    #[objc::msg_send(stride)]
    pub fn stride(&self) -> usize;

    /// The buffer's capacity, in audio sample frames.
    #[objc::msg_send(frameCapacity)]
    pub fn frame_capacity(&self) -> FrameCount;
}

define_obj_type!(CompressedBuf(ns::Id));

impl arc::A<CompressedBuf> {
    #[objc::msg_send(initWithFormat:packetCapacity:)]
    pub fn init_with_format_and_packet_capacity(
        self,
        format: &Format,
        packet_capacity: PacketCount,
    ) -> arc::R<CompressedBuf>;

    #[objc::msg_send(initWithFormat:packetCapacity:maximumPacketSize:)]
    pub fn init_with_format_packet_capacity_and_maximum_packet_size(
        self,
        format: &Format,
        packet_capacity: PacketCount,
        maximum_packet_size: isize,
    ) -> arc::R<CompressedBuf>;
}

/// Use with compressed audio formats.
impl CompressedBuf {
    define_cls!(AV_AUDIO_COMPRESSED_BUFFER);

    /// Creates a buffer that contains constant bytes per packet of audio data in a compressed state.
    ///
    /// This fails if the format is PCM or if the format has variable bytes per packet (for example, format.streamDescription->mBytesPerPacket == 0).
    #[inline]
    pub fn with_format_and_packet_capacity(
        format: &Format,
        packet_capacity: PacketCount,
    ) -> arc::R<Self> {
        Self::alloc().init_with_format_and_packet_capacity(format, packet_capacity)
    }

    #[inline]
    pub fn with_format_packet_capacity_and_maximum_packet_size(
        format: &Format,
        packet_capacity: PacketCount,
        maximum_packet_size: isize,
    ) -> arc::R<Self> {
        Self::alloc().init_with_format_packet_capacity_and_maximum_packet_size(
            format,
            packet_capacity,
            maximum_packet_size,
        )
    }

    /// The number of compressed packets the buffer can contain.
    #[objc::msg_send(packetCapacity)]
    pub fn packet_capacity(&self) -> PacketCount;

    /// The current number of compressed packets in the buffer.
    ///
    /// You may modify the packetCount as part of an operation that modifies
    /// its contents. The packetCount must be less than or equal to the packet_capacity.
    #[objc::msg_send(packetCount)]
    pub fn packet_count(&self) -> PacketCount;

    #[objc::msg_send(setPacketCount:)]
    pub fn set_packet_count(&self, value: PacketCount);

    /// The maximum size of a compressed packet in bytes.
    #[objc::msg_send(maximumPacketSize)]
    pub fn maximum_packet_size(&self) -> isize;

    /// The buffer's capacity in bytes
    #[objc::msg_send(byteCapacity)]
    pub fn byte_capacity(&self) -> u32;

    #[objc::msg_send(byteLength)]
    pub fn byte_length(&self) -> u32;

    #[objc::msg_send(setByteLength:)]
    pub fn set_byte_length(&self, value: u32);

    /// Access the buffer's array of packet descriptions, if any.
    ///
    /// If the format has constant bytes per packet (format.streamDescription->mBytesPerPacket != 0), then this will return nil.
    #[objc::msg_send(packetDescriptions)]
    pub fn packet_descs(&self) -> Option<&StreamPacketDesc>;

    #[objc::msg_send(data)]
    pub fn data(&self) -> *const c_void;
}

#[link(name = "av", kind = "static")]
extern "C" {
    static AV_AUDIO_PCM_BUFFER: &'static objc::Class<PcmBuf>;
    static AV_AUDIO_COMPRESSED_BUFFER: &'static objc::Class<CompressedBuf>;
}
