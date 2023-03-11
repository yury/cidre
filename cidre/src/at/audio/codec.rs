use crate::{at::audio, os};

pub type Codec = audio::ComponentInstance;
pub struct CodecRef(audio::ComponentInstanceRef);
pub struct PropertyID(u32);

/// Structure holding the magic cookie information.
#[repr(C)]
pub struct MagicCookieInfo {
    /// The size of the magic cookie
    pub size: u32,
    /// Generic const pointer to magic cookie
    /// usually it is esds
    pub value: *const u8,
}

impl audio::ComponentInstanceRef {
    pub fn into_codec(
        mut self,
        input_format: *const audio::StreamBasicDescription,
        output_format: *const audio::StreamBasicDescription,
        magic_cookie: Option<&[u8]>,
    ) -> Result<CodecRef, os::Status> {
        let res = unsafe { self.init_codec(input_format, output_format, magic_cookie) };
        if res.is_ok() {
            Ok(CodecRef(self))
        } else {
            Err(res.err().unwrap())
        }
    }
}

impl Drop for CodecRef {
    fn drop(&mut self) {
        let res = unsafe { self.0.uninitialize() };
        debug_assert!(res.is_ok());
    }
}

impl CodecRef {
    /// Append as much of the given data to the codec's input buffer as possible
    /// and return in (data_len, packets_len) the amount of data and packets used.
    #[inline]
    pub fn append_input_data(
        &mut self,
        data: &[u8],
        packets: &[audio::StreamPacketDescription],
    ) -> Result<(u32, u32), os::Status> {
        let mut data_len: u32 = data.len() as _;
        let mut packets_len: u32 = packets.len() as _;
        let res = unsafe {
            AudioCodecAppendInputData(
                &mut self.0,
                data.as_ptr(),
                &mut data_len,
                &mut packets_len,
                packets.as_ptr(),
            )
        };

        if res.is_ok() {
            Ok((data_len, packets_len))
        } else {
            Err(res)
        }
    }

    #[inline]
    pub fn produce_output_packets(
        &mut self,
        data: &mut [u8],
    ) -> Result<(u32, os::Status), os::Status> {
        let mut data_len: u32 = data.len() as _;
        let mut packets_len: u32 = 0;
        let mut status = os::Status::NO_ERR;

        unsafe {
            let res = AudioCodecProduceOutputPackets(
                &mut self.0,
                data.as_mut_ptr(),
                &mut data_len,
                &mut packets_len,
                std::ptr::null_mut(),
                &mut status,
            );
            if res.is_ok() {
                Ok((data_len, status))
            } else {
                Err(res)
            }
        }
    }

    #[inline]
    pub fn produce_output_packets_with_descriptions(
        &mut self,
        data: &mut [u8],
        out_packet_descriptions: &mut [audio::StreamPacketDescription],
    ) -> Result<(u32, u32, os::Status), os::Status> {
        let mut data_len: u32 = data.len() as _;
        let mut packets_len: u32 = out_packet_descriptions.len() as _;
        let mut status = os::Status::NO_ERR;

        unsafe {
            let res = AudioCodecProduceOutputPackets(
                &mut self.0,
                data.as_mut_ptr(),
                &mut data_len,
                &mut packets_len,
                out_packet_descriptions.as_mut_ptr(),
                &mut status,
            );
            if res.is_ok() {
                Ok((data_len, packets_len, status))
            } else {
                Err(res)
            }
        }
    }

    pub fn append_input_buffer_list(
        &mut self,
        in_buffer_list: &audio::BufferList,
    ) -> Result<u32, os::Status> {
        let mut bytes_consumed: u32 = 0;
        let mut packets_len: u32 = 0;
        unsafe {
            let res = AudioCodecAppendInputBufferList(
                &mut self.0,
                in_buffer_list,
                &mut packets_len,
                std::ptr::null(),
                &mut bytes_consumed,
            );
            if res.is_ok() {
                Ok(bytes_consumed)
            } else {
                Err(res)
            }
        }
    }

    pub fn append_input_buffer_list_with_descriptions(
        &mut self,
        in_buffer_list: &audio::BufferList,
        packet_descriptions: &mut [audio::StreamPacketDescription],
    ) -> Result<(u32, u32), os::Status> {
        let mut bytes_consumed: u32 = 0;
        let mut packets_len: u32 = packet_descriptions.len() as _;
        unsafe {
            let res = AudioCodecAppendInputBufferList(
                &mut self.0,
                in_buffer_list,
                &mut packets_len,
                packet_descriptions.as_ptr(),
                &mut bytes_consumed,
            );
            if res.is_ok() {
                Ok((bytes_consumed, packets_len))
            } else {
                Err(res)
            }
        }
    }

    pub fn produce_output_buffer_list(
        &mut self,
        buffer_list: &mut audio::BufferList,
    ) -> Result<os::Status, os::Status> {
        let mut number_packets: u32 = 0;
        let mut status = os::Status::NO_ERR;
        unsafe {
            let res = AudioCodecProduceOutputBufferList(
                &mut self.0,
                buffer_list,
                &mut number_packets,
                std::ptr::null_mut(),
                &mut status,
            );

            if res.is_ok() {
                Ok(status)
            } else {
                Err(res)
            }
        }
    }

    pub fn produce_output_buffer_list_with_descriptions(
        &mut self,
        buffer_list: &mut audio::BufferList,
        packet_descriptions: &mut [audio::StreamPacketDescription],
    ) -> Result<(u32, os::Status), os::Status> {
        let mut number_packets: u32 = packet_descriptions.len() as _;
        let mut status = os::Status::NO_ERR;
        unsafe {
            let res = AudioCodecProduceOutputBufferList(
                &mut self.0,
                buffer_list,
                &mut number_packets,
                packet_descriptions.as_mut_ptr(),
                &mut status,
            );

            if res.is_ok() {
                Ok((number_packets, status))
            } else {
                Err(res)
            }
        }
    }
}

impl Codec {
    pub unsafe fn init_codec(
        &mut self,
        input_format: *const audio::StreamBasicDescription,
        output_format: *const audio::StreamBasicDescription,
        magic_cookie: Option<&[u8]>,
    ) -> Result<(), os::Status> {
        unsafe {
            let res = match magic_cookie {
                Some(cookie) => AudioCodecInitialize(
                    self,
                    input_format,
                    output_format,
                    cookie.as_ptr(),
                    cookie.len() as _,
                ),
                None => {
                    AudioCodecInitialize(self, input_format, output_format, std::ptr::null(), 0)
                }
            };
            res.result()
        }
    }

    /// Flushes all the data in the codec and clears the input buffer. Note that
    /// the formats, and magic cookie will be retained so they won't need to be
    /// set up again to decode the same data.
    #[doc(alias = "AudioCodecReset")]
    pub fn reset(&mut self) -> Result<(), os::Status> {
        unsafe { AudioCodecReset(self).result() }
    }

    /// This call will move the codec from the initialized state back to the
    /// uninitialized state. The codec will release any resources it allocated
    /// or claimed in AudioCodecInitialize.
    pub unsafe fn uninitialize(&mut self) -> Result<(), os::Status> {
        AudioCodecUninitialize(self).result()
    }
}

impl Drop for Codec {
    fn drop(&mut self) {
        let res = unsafe { self.uninitialize() };
        debug_assert!(res.is_ok());
    }
}

extern "C" {
    fn AudioCodecReset(in_codec: &mut Codec) -> os::Status;
    fn AudioCodecInitialize(
        in_codec: &mut Codec,
        in_input_format: *const audio::StreamBasicDescription,
        in_output_format: *const audio::StreamBasicDescription,
        in_magic_cookie: *const u8,
        in_magic_cookie_size: u32,
    ) -> os::Status;

    fn AudioCodecUninitialize(in_codec: &mut Codec) -> os::Status;

    fn AudioCodecAppendInputData(
        in_codec: &mut Codec,
        in_input_data: *const u8,
        io_input_data_byte_size: &mut u32,
        io_number_packets: &mut u32,
        in_packet_description: *const audio::StreamPacketDescription,
    ) -> os::Status;

    fn AudioCodecProduceOutputPackets(
        in_codec: &mut Codec,
        out_output_data: *mut u8,
        io_output_data_byte_size: &mut u32,
        io_number_packets: &mut u32,
        out_packet_description: *mut audio::StreamPacketDescription,
        out_status: &mut os::Status,
    ) -> os::Status;

    fn AudioCodecAppendInputBufferList(
        in_codec: &mut Codec,
        in_buffer_list: *const audio::BufferList,
        io_number_packets: &mut u32,
        in_packet_descriptions: *const audio::StreamPacketDescription,
        out_bytes_consumed: &mut u32,
    ) -> os::Status;

    fn AudioCodecProduceOutputBufferList(
        in_codec: &mut Codec,
        io_buffer_list: &mut audio::BufferList,
        io_number_packets: &mut u32,
        out_packet_description: *mut audio::StreamPacketDescription,
        out_status: &mut os::Status,
    ) -> os::Status;

}

#[cfg(test)]
mod tests {
    use crate::at::audio;

    #[test]
    fn basics() {
        let channels_per_frame = 2;
        let sample_rate = 44100.0;
        let src_asbd = audio::StreamBasicDescription {
            sample_rate,
            channels_per_frame,
            format_id: audio::FormatID::LINEAR_PCM,
            format_flags: audio::FormatFlags::IS_FLOAT | audio::FormatFlags::IS_PACKED,
            bytes_per_packet: 4 * channels_per_frame,
            frames_per_packet: 1,
            bytes_per_frame: 4 * channels_per_frame,
            bits_per_channel: 32,
            ..Default::default()
        };

        let dst_asbd = audio::StreamBasicDescription {
            sample_rate,
            channels_per_frame,
            format_id: audio::FormatID::MPEG4_AAC,
            format_flags: audio::FormatFlags::ALL_CLEAR,
            frames_per_packet: 1024,
            ..Default::default()
        };

        let desc = audio::ComponentDescription {
            type_: u32::from_be_bytes(*b"aenc"),
            sub_type: u32::from_be_bytes(*b"aac "),
            ..Default::default()
        };

        let inst = desc.into_iter().last().unwrap().new_instance().unwrap();
        let codec = inst.into_codec(&src_asbd, &dst_asbd, None).unwrap();

        // println!("nice");
        // let codec = audio::Codec::new(&src_asbd, &dst_asbd, None).unwrap();
        // println!("codec {codec:?}");
    }
}
