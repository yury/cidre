use std::ffi::c_void;

use crate::{arc, blocks, define_cls, define_obj_type, ns, objc};

#[cfg(feature = "at")]
use crate::at::{audio::StreamPacketDesc, AudioBufList};

use super::{Format, FrameCount, PacketCount};

define_obj_type!(
    #[doc(alias = "AVAudioBuffer")]
    pub Buf(ns::Id)
);

/// A buffer of audio data, with a format.
impl Buf {
    #[objc::msg_send(format)]
    pub fn format(&self) -> &Format;

    #[cfg(feature = "at")]
    #[objc::msg_send(audioBufferList)]
    pub fn audio_buffer_list(&self) -> &AudioBufList;

    #[cfg(feature = "at")]
    #[objc::msg_send(mutableAudioBufferList)]
    pub fn audio_buffer_list_mut(&mut self) -> &mut AudioBufList;
}

define_obj_type!(
    #[doc(alias = "AVAudioPCMBuffer")]
    pub PcmBuf(Buf)
);

unsafe impl Send for PcmBuf {}
unsafe impl Sync for PcmBuf {}

impl arc::A<PcmBuf> {
    #[objc::msg_send(initWithPCMFormat:frameCapacity:)]
    pub fn init_with_pcm_format_frame_capacity(
        self,
        format: &Format,
        frame_capacity: FrameCount,
    ) -> Option<arc::R<PcmBuf>>;

    #[objc::msg_send(initWithPCMFormat:bufferListNoCopy:deallocator:)]
    pub fn init_with_pcm_format_buf_list_no_copy<const N: usize>(
        self,
        format: &Format,
        buf_list: &AudioBufList<N>,
        deallocator: Option<&mut blocks::EscBlock<fn(buf_list: *const AudioBufList<N>)>>,
    ) -> Option<arc::R<PcmBuf>>;
}

/// Provides a number of methods useful for manipulating buffers of
/// audio in PCM format.
impl PcmBuf {
    define_cls!(AV_AUDIO_PCM_BUFFER);

    pub fn with_format(format: &Format, frame_capacity: FrameCount) -> Option<arc::R<Self>> {
        Self::alloc().init_with_pcm_format_frame_capacity(format, frame_capacity)
    }

    pub fn with_buf_list_no_copy<const N: usize>(
        format: &Format,
        buf_list: &AudioBufList<N>,
        deallocator: Option<&mut blocks::EscBlock<fn(buf_list: *const AudioBufList<N>)>>,
    ) -> Option<arc::R<Self>> {
        Self::alloc().init_with_pcm_format_buf_list_no_copy(format, buf_list, deallocator)
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
    pub fn set_frame_length_throws<'ear>(&mut self, value: FrameCount);

    pub fn set_frame_length<'ear>(&mut self, value: FrameCount) -> ns::ExResult<'ear> {
        ns::try_catch(|| self.set_frame_length_throws(value))
    }

    /// The buffer's number of interleaved channels.
    ///
    /// Useful in conjunction with floatChannelData etc.
    #[objc::msg_send(stride)]
    pub fn stride(&self) -> usize;

    /// The buffer's capacity, in audio sample frames.
    #[objc::msg_send(frameCapacity)]
    pub fn frame_capacity(&self) -> FrameCount;

    #[objc::msg_send(floatChannelData)]
    pub fn data_f32(&self) -> *const *const f32;

    #[objc::msg_send(floatChannelData)]
    pub unsafe fn data_f32_mut(&mut self) -> *const *mut f32;

    #[objc::msg_send(int16ChannelData)]
    pub fn data_i16(&self) -> *const *const i16;

    #[objc::msg_send(int16ChannelData)]
    pub unsafe fn data_i16_mut(&mut self) -> *const *mut i16;

    #[objc::msg_send(int32ChannelData)]
    pub fn data_i32(&self) -> *const *const i32;

    #[objc::msg_send(int32ChannelData)]
    pub unsafe fn data_i32_mut(&mut self) -> *const *mut i32;

    pub fn data_i16_at(&self, index: usize) -> Option<&[i16]> {
        pcm_slice_at(self, self.data_i16(), index)
    }

    pub fn data_i16_mut_at(&mut self, index: usize) -> Option<&mut [i16]> {
        let ptr = unsafe { self.data_i16_mut() };
        pcm_slice_at_mut(self, ptr, index)
    }

    pub fn data_f32_at(&self, index: usize) -> Option<&[f32]> {
        pcm_slice_at(self, self.data_f32(), index)
    }

    pub fn data_f32_mut_at(&mut self, index: usize) -> Option<&mut [f32]> {
        let ptr = unsafe { self.data_f32_mut() };
        pcm_slice_at_mut(self, ptr, index)
    }

    pub fn data_i32_at(&self, index: usize) -> Option<&[i32]> {
        pcm_slice_at(self, self.data_i32(), index)
    }

    pub fn data_i32_mut_at(&mut self, index: usize) -> Option<&mut [i32]> {
        let ptr = unsafe { self.data_i32_mut() };
        pcm_slice_at_mut(self, ptr, index)
    }
}

fn pcm_slice_at<T>(buf: &PcmBuf, ptr: *const *const T, index: usize) -> Option<&[T]> {
    if ptr.is_null() {
        return None;
    }

    let len = buf.frame_length() as usize;
    let stride = buf.stride() as usize;
    if stride > 1 {
        // if stride is not 1 it is interleaved, single buf
        // save some calls to format
        let slice = unsafe { std::slice::from_raw_parts(ptr, 1) };
        let data = unsafe { std::slice::from_raw_parts(slice[0], stride * len) };
        return Some(data);
    }
    let channels_n = buf.format().channel_count();
    let slice = unsafe { std::slice::from_raw_parts(ptr, channels_n as _) };
    let data = unsafe { std::slice::from_raw_parts(slice[index], len) };
    Some(data)
}

fn pcm_slice_at_mut<T>(buf: &mut PcmBuf, ptr: *const *mut T, index: usize) -> Option<&mut [T]> {
    if ptr.is_null() {
        return None;
    }

    let len = buf.frame_length() as usize;
    let stride = buf.stride() as usize;
    if stride > 1 {
        // interleaved, single buf
        let slice = unsafe { std::slice::from_raw_parts(ptr, 1) };
        let data = unsafe { std::slice::from_raw_parts_mut(slice[0], stride * len) };
        return Some(data);
    }
    let channels_n = buf.format().channel_count();
    let slice = unsafe { std::slice::from_raw_parts(ptr, channels_n as _) };
    let data = unsafe { std::slice::from_raw_parts_mut(slice[index], len) };
    Some(data)
}

define_obj_type!(
    #[doc(alias = "AVAudioCompressedBuffer")]
    pub CompressedBuf(ns::Id)
);

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
    #[cfg(feature = "at")]
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

#[cfg(test)]
mod tests {
    use crate::av;

    #[test]
    fn basics() {
        let channel_count = 2;
        let format =
            av::AudioFormat::standard_with_sample_rate_and_channels(44800.0, channel_count)
                .unwrap();
        let cap = 1024;
        let mut buf = av::AudioPcmBuf::with_format(&format, cap).unwrap();
        buf.set_frame_length(cap).unwrap();
        let data = buf.data_f32();
        let (n, cap) = if format.is_interleaved() {
            (1, channel_count * cap)
        } else {
            (channel_count, cap)
        };
        let buf = unsafe { std::slice::from_raw_parts(data, n as _) };
        let channel = buf[0];
        let left = unsafe { std::slice::from_raw_parts(channel, cap as _) };
        let sum = left.iter().sum();
        assert_eq!(0.0f32, sum);
    }

    #[test]
    fn safe() {
        let channel_count = 2;
        let format =
            av::AudioFormat::standard_with_sample_rate_and_channels(44800.0, channel_count)
                .unwrap();
        let cap = 1024;
        let mut buf = av::AudioPcmBuf::with_format(&format, cap).unwrap();
        let _err = buf.set_frame_length(1025).expect_err("Should fail");
        buf.set_frame_length(1024)
            .expect("Failed to set max cap frame len");
    }
}
