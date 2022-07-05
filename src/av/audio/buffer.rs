use std::ffi::c_void;

use crate::{at::{AudioBufferList, audio::StreamPacketDescription}, cf::{self, Retained}, define_obj_type, ns};

use super::{Format, FrameCount, PacketCount};

define_obj_type!(Buffer(ns::Id));

/// A buffer of audio data, with a format.
impl Buffer {
    pub fn format(&self) -> cf::Retained<Format> {
        unsafe { rsel_format(self) }
    }

    pub fn audio_buffer_list(&self) -> &AudioBufferList<1, 1> {
        unsafe { rsel_audioBufferList(self) }
    }

    pub fn audio_buffer_list_mut(&self) -> &mut AudioBufferList<1, 1> {
        unsafe { rsel_mutableAudioBufferList(self) }
    }
}

define_obj_type!(PCMBuffer(Buffer));

/// Provides a number of methods useful for manipulating buffers of
/// audio in PCM format.
impl PCMBuffer {

    /// The current number of valid sample frames in the buffer.
    /// 
    /// You may modify the length of the buffer as part of an operation that modifies its contents.
	/// The length must be less than or equal to the frameCapacity. Modifying frameLength will update
    /// the mDataByteSize in each of the underlying AudioBufferList's AudioBuffer's correspondingly,
    /// and vice versa. Note that in the case of deinterleaved formats, mDataByteSize will refers
    /// the size of one channel's worth of audio samples.
    pub fn frame_length(&self) -> FrameCount {
        unsafe { rsel_frameLength(self) }
    }

    pub fn set_frame_length(&self, value: FrameCount) {
        unsafe { wsel_setFrameLength(self, value) }
    } 

    /// The buffer's number of interleaved channels.
    /// 
    /// Useful in conjunction with floatChannelData etc.
    pub fn stride(&self) -> usize {
        unsafe { rsel_stride(self) }
    }

    /// The buffer's capacity, in audio sample frames.
    pub fn frame_capacity(&self) -> FrameCount {
        unsafe { rsel_frameCapacity(self) }
    }
}

define_obj_type!(CompressedBuffer(ns::Id));

/// Use with compressed audio formats.
impl CompressedBuffer {

    /// The number of compressed packets the buffer can contain.
    pub fn packet_capacity(&self) -> PacketCount {
        unsafe {  rsel_packetCapacity(self) }
    }

    /// The current number of compressed packets in the buffer.
    /// 
    /// You may modify the packetCount as part of an operation that modifies
    /// its contents. The packetCount must be less than or equal to the packet_capacity.
    pub fn packet_count(&self) -> PacketCount {
        unsafe { rsel_packetCount(self) }
    }

    pub fn set_packet_count(&self, value: PacketCount) {
        unsafe { wsel_setPacketCount(self, value) }
    }

    /// The maximum size of a compressed packet in bytes.
    pub fn maximum_packet_size(&self) -> isize {
        unsafe { rsel_maximumPacketSize(self) }
    }

    /// The buffer's capacity in bytes
    pub fn byte_capacity(&self) -> u32 {
        unsafe { rsel_byteCapacity(self) }
    }

    pub fn byte_length(&self) -> u32 {
        unsafe { rsel_byteLength(self) }
    }

    pub fn set_byte_length(&self, value: u32) {
        unsafe { wsel_setByteLength(self, value) }
    }

    /// Access the buffer's array of packet descriptions, if any.
    /// 
    /// If the format has constant bytes per packet (format.streamDescription->mBytesPerPacket != 0), then this will return nil.
    pub fn packet_descriptions(&self) -> Option<&StreamPacketDescription> {
        unsafe { rsel_packetDescriptions(self) }
    }

    pub fn data(&self) -> *const c_void {
        unsafe { rsel_data(self) }
    }

    /// Creates a buffer that contains constant bytes per packet of audio data in a compressed state.
    /// 
    /// This fails if the format is PCM or if the format has variable bytes per packet (for example, format.streamDescription->mBytesPerPacket == 0).
    pub fn with_format_and_packet_capacity<'a>(format: &Format, packet_capacity: PacketCount) -> Retained<'a, Self> {
        unsafe {
            AVAudioCompressedBuffer_initWithFormat_packetCapacity(format, packet_capacity)
        }
    }

    pub fn with_format_packet_capacity_and_maximum_packet_size<'a>(format: &Format, packet_capacity: PacketCount, maximum_packet_size: isize) -> Retained<'a, Self> {
        unsafe {
            AVAudioCompressedBuffer_initWithFormat_packetCapacity_maximumPacketSize(format, packet_capacity, maximum_packet_size)
        }
    }
}

#[link(name = "av", kind = "static")]
extern "C" {
    fn rsel_format(id: &ns::Id) -> cf::Retained<Format>;
    fn rsel_audioBufferList(id: &ns::Id) -> &AudioBufferList<1, 1>;
    fn rsel_mutableAudioBufferList(id: &ns::Id) -> &mut AudioBufferList<1, 1>;
    fn rsel_frameCapacity(id: &ns::Id) -> FrameCount;
    fn rsel_frameLength(id: &ns::Id) -> FrameCount;
    fn wsel_setFrameLength(id: &ns::Id, value: FrameCount);

    fn rsel_stride(id: &ns::Id) -> usize;

    fn rsel_packetCapacity(id: &ns::Id) ->  PacketCount; 

    fn rsel_packetCount(id: &ns::Id) -> PacketCount;
    fn wsel_setPacketCount(id: &ns::Id, value: PacketCount);

    fn rsel_maximumPacketSize(id: &ns::Id) -> isize;
    fn rsel_byteCapacity(id: &ns::Id) -> u32;

    fn rsel_byteLength(id: &ns::Id) -> u32;
    fn wsel_setByteLength(id: &ns::Id, value: u32);

    fn rsel_packetDescriptions(id: &ns::Id) -> Option<&StreamPacketDescription>;

    fn rsel_data(id: &ns::Id) -> *const c_void;

    fn AVAudioCompressedBuffer_initWithFormat_packetCapacity<'a>(format: &Format, packet_capacity: PacketCount) -> Retained<'a, CompressedBuffer>;
    fn AVAudioCompressedBuffer_initWithFormat_packetCapacity_maximumPacketSize<'a>(format: &Format, packet_capacity: PacketCount, maximum_packet_size: isize) -> Retained<'a, CompressedBuffer>;
}
