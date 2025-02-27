use std::ffi::c_void;

use crate::{api, arc, cf, cm, cv, define_cf_type, os, vt};

#[cfg(feature = "blocks")]
use crate::blocks;

pub type OutputCb<O = c_void, F = c_void> = extern "C" fn(
    // passed during session creation
    output_ptr: *mut O,
    // passed during decode_frame call
    src_frame_ptr: *mut F,
    status: os::Status,
    info_flags: vt::DecodeInfoFlags,
    image_buf: Option<&cv::ImageBuf>,
    pts: cm::Time,
    duration: cm::Time,
);

#[doc(alias = "VTDecompressionOutputCallbackRecord")]
#[repr(C)]
pub struct OutputCbRecord<O, F> {
    pub cb: Option<OutputCb<O, F>>,
    pub ptr: *mut O,
}

impl<O, F> OutputCbRecord<O, F> {
    pub fn new(ptr: *mut O, cb: Option<OutputCb<O, F>>) -> Self {
        Self { cb, ptr }
    }
}

#[doc(alias = "VTDecompressionOutputMultiImageCallback")]
pub type OutputMultiImageCb<O = c_void, F = c_void> = extern "C" fn(
    // passed during session creation
    output_ptr: *mut O,
    // passed during decode_frame call
    src_frame_ptr: *mut F,
    status: os::Status,
    info_flags: vt::DecodeInfoFlags,
    tagged_buf_group: Option<&cm::TaggedBufGroup>,
    pts: cm::Time,
    duration: cm::Time,
);

#[doc(alias = "VTDecompressionMultiImageCapableOutputHandler")]
#[cfg(feature = "blocks")]
pub type MultiImageCapableOutputHandler = blocks::EscBlock<
    fn(
        status: os::Status,
        info_flags: vt::DecodeInfoFlags,
        image_buf: Option<&cv::ImageBuf>,
        tagged_buf_group: Option<&cm::TaggedBufGroup>,
        pts: cm::Time,
        duration: cm::Time,
    ),
>;

define_cf_type!(
    #[doc(alias = "VTDecompressionSessionRef")]
    Session(vt::Session)
);

impl Session {
    pub fn new<O, F>(
        video_format_desc: &cm::VideoFormatDesc,
        video_decoder_spec: Option<&cf::Dictionary>,
        dst_image_buf_attrs: Option<&cf::Dictionary>,
        record: Option<&OutputCbRecord<O, F>>,
    ) -> os::Result<arc::R<Self>> {
        unsafe {
            os::result_unchecked(|res| {
                Self::create_in(
                    None,
                    video_format_desc,
                    video_decoder_spec,
                    dst_image_buf_attrs,
                    std::mem::transmute(record),
                    res,
                )
            })
        }
    }

    /// # Safety
    /// Use safe new
    pub unsafe fn create_in(
        allocator: Option<&cf::Allocator>,
        video_format_desc: &cm::VideoFormatDesc,
        video_decoder_spec: Option<&cf::Dictionary>,
        destination_image_buf_attrs: Option<&cf::Dictionary>,
        output_callback: *const OutputCbRecord<c_void, c_void>,
        decompression_session_out: *mut Option<arc::R<Session>>,
    ) -> os::Status {
        unsafe {
            VTDecompressionSessionCreate(
                allocator,
                video_format_desc,
                video_decoder_spec,
                destination_image_buf_attrs,
                output_callback,
                decompression_session_out,
            )
        }
    }

    /// Tears down a decompression session
    /// When you are done with a decompression session you created, call VTDecompressionSessionInvalidate
    /// to tear it down and then CFRelease to release your object reference.
    /// When a decompression session's retain count reaches zero, it is automatically invalidated, but
    /// since sessions may be retained by multiple parties, it can be hard to predict when this will happen.
    /// Calling VTDecompressionSessionInvalidate ensures a deterministic, orderly teardown.
    #[inline]
    pub fn invalidate(&mut self) {
        unsafe { VTDecompressionSessionInvalidate(self) }
    }

    /// Decompresses a video frame.
    #[doc(alias = "VTDecompressionSessionDecodeFrame")]
    #[inline]
    pub fn decode(
        &self,
        sample_buf: &cm::SampleBuf,
        decode_flags: vt::DecodeFrameFlags,
    ) -> os::Result {
        unsafe {
            VTDecompressionSessionDecodeFrame(
                self,
                sample_buf,
                decode_flags,
                std::ptr::null_mut(),
                std::ptr::null_mut(),
            )
            .result()
        }
    }

    /// Decompresses a video frame.
    #[doc(alias = "VTDecompressionSessionDecodeFrame")]
    #[inline]
    pub unsafe fn decode_frame<F>(
        &self,
        sample_buf: &cm::SampleBuf,
        decode_flags: vt::DecodeFrameFlags,
        source_frame_ref_con: *mut F,
        info_flags_out: *mut vt::DecodeInfoFlags,
    ) -> os::Result {
        unsafe {
            VTDecompressionSessionDecodeFrame(
                self,
                sample_buf,
                decode_flags,
                std::mem::transmute(source_frame_ref_con),
                info_flags_out,
            )
            .result()
        }
    }

    pub fn finish_delayed_frames(&mut self) -> os::Result {
        unsafe { VTDecompressionSessionFinishDelayedFrames(self).result() }
    }

    pub fn wait_for_async_frames(&mut self) -> os::Result {
        unsafe { VTDecompressionSessionWaitForAsynchronousFrames(self).result() }
    }

    #[inline]
    pub fn can_accept_format_desc(&self, format_description: &cm::FormatDesc) -> bool {
        unsafe { VTDecompressionSessionCanAcceptFormatDescription(self, format_description) }
    }

    /// Copies a black pixel buffer from the decompression session.
    ///
    /// The pixel buffer is in the same format that the session is decompressing to.
    #[inline]
    pub fn black_pixel_buf(&self) -> os::Result<arc::R<cv::PixelBuf>> {
        unsafe { os::result_unchecked(|res| VTDecompressionSessionCopyBlackPixelBuffer(self, res)) }
    }
}

/// Multi-image decompression
impl Session {
    #[api::available(macos = 14.0, ios = 17.0, visionos = 1.0)]
    unsafe fn _set_multi_image_cb(
        &mut self,
        cb: OutputMultiImageCb,
        output_ptr: *mut c_void,
    ) -> os::Status {
        unsafe { VTDecompressionSessionSetMultiImageCallback(self, cb, output_ptr) }
    }

    #[api::available(macos = 14.0, ios = 17.0, visionos = 1.0)]
    pub fn set_multi_image_cb<T>(
        &mut self,
        cb: OutputMultiImageCb<T>,
        output_ptr: *mut c_void,
    ) -> os::Result {
        unsafe {
            self._set_multi_image_cb(std::mem::transmute(cb), output_ptr)
                .result()
        }
    }
}

/// Indicates whether the current system supports hardware decode for a given codec
///
/// This routine reports whether the current system supports hardware decode.  Using
/// this information, clients can make informed decisions regarding remote assets to load,
/// favoring alternate encodings when hardware decode is not supported.
/// This call returning true does not guarantee that hardware decode resources will be
/// available at all times.
#[doc(alias = "VTIsHardwareDecodeSupported")]
#[inline]
pub fn is_hardware_decode_supported(codec_type: cm::VideoCodec) -> bool {
    unsafe { VTIsHardwareDecodeSupported(codec_type) }
}

/// Indicates whether the current system supports stereo MV-HEVC decode.
///
/// This call returning true does not guarantee that decode resources will be available at all times.
#[doc(alias = "VTIsStereoMVHEVCDecodeSupported")]
#[inline]
pub fn is_stereo_mv_hevc_decode_supported() -> bool {
    unsafe { VTIsStereoMVHEVCDecodeSupported() }
}

#[link(name = "VideoToolbox", kind = "framework")]
#[api::weak]
unsafe extern "C-unwind" {

    fn VTDecompressionSessionCreate(
        allocator: Option<&cf::Allocator>,
        video_format_description: &cm::VideoFormatDesc,
        video_decoder_specification: Option<&cf::Dictionary>,
        destination_image_buffer_attirbutes: Option<&cf::Dictionary>,
        output_callback: *const OutputCbRecord<c_void, c_void>,
        decompression_session_out: *mut Option<arc::R<Session>>,
    ) -> os::Status;

    fn VTDecompressionSessionInvalidate(session: &Session);

    fn VTDecompressionSessionDecodeFrame(
        session: &Session,
        sample_buffer: &cm::SampleBuf,
        decode_flags: vt::DecodeFrameFlags,
        source_frame_ref_con: *mut c_void,
        info_flags_out: *mut vt::DecodeInfoFlags,
    ) -> os::Status;

    fn VTDecompressionSessionFinishDelayedFrames(session: &Session) -> os::Status;
    fn VTDecompressionSessionWaitForAsynchronousFrames(session: &Session) -> os::Status;

    fn VTDecompressionSessionCanAcceptFormatDescription(
        session: &Session,
        new_fromat: &cm::FormatDesc,
    ) -> bool;

    fn VTDecompressionSessionCopyBlackPixelBuffer(
        session: &Session,
        pixel_buffer_out: *mut Option<arc::R<cv::PixelBuf>>,
    ) -> os::Status;

    fn VTIsHardwareDecodeSupported(codec_type: cm::VideoCodec) -> bool;
    fn VTIsStereoMVHEVCDecodeSupported() -> bool;

    #[api::available(macos = 14.0, ios = 17.0, visionos = 1.0)]
    fn VTDecompressionSessionSetMultiImageCallback(
        session: &mut Session,
        output_multi_image_cb: OutputMultiImageCb,
        output_multi_ref_con: *mut c_void,
    ) -> os::Status;
}

#[cfg(test)]
mod tests {
    use std::ffi::c_void;

    use crate::{
        cm::{self, VideoCodec},
        cv, os, vt,
    };

    #[test]
    fn create_decompression_session() {
        let _desc = cm::VideoFormatDesc::video(VideoCodec::HEVC, 1920, 1080, None).unwrap();

        struct Context {}

        extern "C" fn callback(
            _output_ref_con: *mut Context,
            _source_frame_ref_con: *mut c_void,
            _status: os::Status,
            _info_flags: vt::DecodeInfoFlags,
            _image_buffer: Option<&cv::ImageBuf>,
            _pts: cm::Time,
            _duration: cm::Time,
        ) {
        }

        let mut ctx = Context {};

        let _record = vt::DecompressionOutputCbRecord::new(&mut ctx, Some(callback));

        //vt::DecompressionSession::new(&desc, None, None, None).unwrap();
    }
}
