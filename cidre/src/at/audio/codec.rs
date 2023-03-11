use std::{
    ops::{Deref, DerefMut},
    ptr::NonNull,
};

use crate::{at::audio, os};

#[derive(Debug)]
#[repr(transparent)]
pub struct Codec(audio::ComponentInstance);
pub struct PropertyID(u32);

#[derive(Debug)]
#[repr(transparent)]
pub struct CodecRef(NonNull<Codec>);

impl Deref for CodecRef {
    type Target = Codec;

    #[inline]
    fn deref(&self) -> &Self::Target {
        unsafe { self.0.as_ref() }
    }
}

impl DerefMut for CodecRef {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { self.0.as_mut() }
    }
}

/// Structure holding the magic cookie information.
#[repr(C)]
pub struct MagicCookieInfo {
    /// The size of the magic cookie
    pub size: u32,
    /// Generic const pointer to magic cookie
    /// usually it is esds
    pub value: *const u8,
}

impl Codec {
    pub fn initialize(
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

        // println!("nice");
        // let codec = audio::Codec::new(&src_asbd, &dst_asbd, None).unwrap();
        // println!("codec {codec:?}");
    }
}
