use std::{ffi::c_void, ptr::NonNull};

use crate::{api, arc, cf, cm, cv, define_cf_type, os, vt};

#[cfg(feature = "blocks")]
use crate::blocks;

#[cfg(feature = "blocks")]
pub type OutputHandler =
    blocks::SendBlock<fn(os::Status, vt::EncodeInfoFlags, Option<&cm::SampleBuf>)>;

define_cf_type!(
    #[doc(alias = "VTCompressionSession")]
    Session(vt::Session)
);

pub type OutputCallback<T> = extern "C" fn(
    output_cb_ref_con: *mut T,
    src_frame_ref_con: *mut c_void,
    status: os::Status,
    info_flags: vt::EncodeInfoFlags,
    sample_buf: Option<&cm::SampleBuf>,
);

impl Session {
    pub fn new<T>(
        width: u32,
        height: u32,
        codec: cm::VideoCodec,
        encoder_spec: Option<&cf::Dictionary>,
        src_image_buf_attrs: Option<&cf::Dictionary>,
        // apple don't use this allocator any more
        compressed_data_allocator: Option<&cf::Allocator>,
        output_cb: Option<OutputCallback<T>>,
        output_cb_ref_con: *mut T,
    ) -> os::Result<arc::R<Self>> {
        unsafe {
            os::result_unchecked(|res| {
                Self::create_in(
                    width as _,
                    height as _,
                    codec,
                    encoder_spec,
                    src_image_buf_attrs,
                    compressed_data_allocator,
                    std::mem::transmute(output_cb),
                    std::mem::transmute(output_cb_ref_con),
                    res,
                    None,
                )
            })
        }
    }

    /// # Safety
    /// use ::new
    pub unsafe fn create_in(
        width: i32,
        height: i32,
        codec_type: cm::VideoCodec,
        encoder_spec: Option<&cf::Dictionary>,
        src_image_buf_attrs: Option<&cf::Dictionary>,
        compressed_data_allocator: Option<&cf::Allocator>,
        output_cb: Option<OutputCallback<c_void>>,
        output_cb_ref_con: *mut c_void,
        compression_session_out: *mut Option<arc::R<Session>>,
        allocator: Option<&cf::Allocator>,
    ) -> os::Result {
        unsafe {
            VTCompressionSessionCreate(
                allocator,
                width,
                height,
                codec_type,
                encoder_spec,
                src_image_buf_attrs,
                compressed_data_allocator,
                output_cb,
                output_cb_ref_con,
                compression_session_out,
            )
            .result()
        }
    }

    #[doc(alias = "VTCompressionSessionInvalidate")]
    #[inline]
    pub fn invalidate(&mut self) {
        unsafe { VTCompressionSessionInvalidate(self) }
    }

    #[doc(alias = "VTCompressionSessionPrepareToEncodeFrames")]
    #[inline]
    pub fn prepare(&mut self) -> os::Result {
        unsafe { VTCompressionSessionPrepareToEncodeFrames(self).result() }
    }

    /// Encoded frames may or may not be output before the function returns.
    /// The client should not modify the pixel data after making this call.
    /// The session and/or encoder retains the image buffer as long as necessary.
    ///
    /// If you don't know duration. Set it invalid.
    /// TODO: investigate. Right now we keep buf to calculate ts delta
    #[doc(alias = "VTCompressionSessionEncodeFrame")]
    #[inline]
    pub fn encode_frame(
        &self,
        image_buf: &cv::ImageBuf,
        pts: cm::Time,
        duration: cm::Time,
        frame_props: Option<&cf::DictionaryOf<cf::String, cf::Type>>,
        src_frame_ref_con: *mut c_void,
        info_flags_out: *mut Option<NonNull<vt::EncodeInfoFlags>>,
    ) -> os::Result {
        unsafe {
            VTCompressionSessionEncodeFrame(
                self,
                image_buf,
                pts,
                duration,
                frame_props,
                src_frame_ref_con,
                info_flags_out,
            )
            .result()
        }
    }

    #[doc(alias = "VTCompressionSessionEncodeFrame")]
    #[inline]
    pub fn enc_frame(
        &self,
        image_buf: &cv::ImageBuf,
        pts: cm::Time,
        duration: cm::Time,
        info_flags_out: *mut Option<NonNull<vt::EncodeInfoFlags>>,
    ) -> os::Result {
        self.encode_frame(
            image_buf,
            pts,
            duration,
            None,
            std::ptr::null_mut(),
            info_flags_out,
        )
    }

    #[doc(alias = "VTCompressionSessionEncodeFrameWithOutputHandler")]
    #[cfg(feature = "blocks")]
    #[inline]
    pub fn encode_frame_with_output_handler(
        &mut self,
        image_buf: &cv::ImageBuf,
        pts: cm::Time,
        duration: cm::Time,
        frame_props: Option<&cf::Dictionary>,
        info_flags_out: *mut Option<NonNull<vt::EncodeInfoFlags>>,
        block: &'static mut OutputHandler,
    ) -> os::Result {
        unsafe {
            VTCompressionSessionEncodeFrameWithOutputHandler(
                self,
                image_buf,
                pts,
                duration,
                frame_props,
                info_flags_out,
                block,
            )
            .result()
        }
    }

    #[doc(alias = "VTCompressionSessionEncodeFrameWithOutputHandler")]
    #[cfg(feature = "blocks")]
    #[inline]
    pub fn enc_frame_with_output_handler_block(
        &mut self,
        image_buf: &cv::ImageBuf,
        pts: cm::Time,
        duration: cm::Time,
        frame_props: Option<&cf::Dictionary>,
        info_flags_out: *mut Option<NonNull<vt::EncodeInfoFlags>>,
        block: &mut OutputHandler,
    ) -> os::Result {
        unsafe {
            VTCompressionSessionEncodeFrameWithOutputHandler(
                self,
                image_buf,
                pts,
                duration,
                frame_props,
                info_flags_out,
                block,
            )
            .result()
        }
    }
    #[doc(alias = "VTCompressionSessionEncodeFrameWithOutputHandler")]
    #[cfg(feature = "blocks")]
    #[inline]
    pub fn enc_frame_with_output_handler(
        &mut self,
        image_buf: &cv::ImageBuf,
        pts: cm::Time,
        duration: cm::Time,
        frame_props: Option<&cf::Dictionary>,
        info_flags_out: *mut Option<NonNull<vt::EncodeInfoFlags>>,
        block: impl FnMut(os::Status, vt::EncodeInfoFlags, Option<&cm::SampleBuf>) + Send + 'static,
    ) -> os::Result {
        let mut block = OutputHandler::new3(block);
        self.enc_frame_with_output_handler_block(
            image_buf,
            pts,
            duration,
            frame_props,
            info_flags_out,
            &mut block,
        )
    }

    #[doc(alias = "VTCompressionSessionGetPixelBufferPool")]
    #[inline]
    pub fn pixel_buf_pool(&self) -> Option<&cv::PixelBufPool> {
        unsafe { VTCompressionSessionGetPixelBufferPool(self) }
    }

    #[doc(alias = "VTCompressionSessionCompleteFrames")]
    #[inline]
    pub fn complete_frames(&self, until_pts: cm::Time) -> os::Result {
        unsafe { VTCompressionSessionCompleteFrames(self, until_pts).result() }
    }

    #[doc(alias = "VTCompressionSessionCompleteFrames")]
    #[inline]
    pub fn complete_all(&self) -> os::Result {
        self.complete_frames(cm::Time::invalid())
    }

    /// Indicates whether the current system supports stereo MV-HEVC encode.
    ///
    /// This call returning true does not guarantee that encode resources will be available at all times.
    #[doc(alias = "VTIsStereoMVHEVCEncodeSupported")]
    #[api::available(macos = 14.0, ios = 17.0, tvos = 17.0, visionos = 1.0)]
    pub fn is_stereo_mv_hevc_encode_supported() -> bool {
        unsafe { VTIsStereoMVHEVCEncodeSupported() }
    }

    // TODO: multipass
}

#[link(name = "VideoToolbox", kind = "framework")]
#[api::weak]
unsafe extern "C" {
    fn VTCompressionSessionCreate(
        allocator: Option<&cf::Allocator>,
        width: i32,
        height: i32,
        codec_type: cm::VideoCodec,
        encoder_specification: Option<&cf::Dictionary>,
        src_image_buf_attrs: Option<&cf::Dictionary>,
        compressed_data_allocator: Option<&cf::Allocator>,
        output_cb: Option<OutputCallback<c_void>>,
        output_cb_ref_con: *mut c_void,
        compression_session_out: *mut Option<arc::Retained<Session>>,
    ) -> os::Status;

    fn VTCompressionSessionInvalidate(session: &mut Session);
    fn VTCompressionSessionPrepareToEncodeFrames(session: &mut Session) -> os::Status;
    fn VTCompressionSessionEncodeFrame(
        session: &Session,
        image_buf: &cv::ImageBuf,
        pts: cm::Time,
        duration: cm::Time,
        frame_properties: Option<&cf::DictionaryOf<cf::String, cf::Type>>,
        src_frame_ref_con: *mut c_void,
        info_flags_out: *mut Option<NonNull<vt::EncodeInfoFlags>>,
    ) -> os::Status;

    #[cfg(feature = "blocks")]
    fn VTCompressionSessionEncodeFrameWithOutputHandler(
        session: &Session,
        image_buf: &cv::ImageBuf,
        pts: cm::Time,
        duration: cm::Time,
        frame_properties: Option<&cf::Dictionary>,
        info_flags_out: *mut Option<NonNull<vt::EncodeInfoFlags>>,
        output_handler: &mut OutputHandler,
    ) -> os::Status;

    fn VTCompressionSessionGetPixelBufferPool(session: &Session) -> Option<&cv::PixelBufPool>;
    fn VTCompressionSessionCompleteFrames(
        session: &Session,
        complete_until_pts: cm::Time,
    ) -> os::Status;

    #[api::available(macos = 14.0, ios = 17.0, tvos = 17.0, visionos = 1.0)]
    fn VTIsStereoMVHEVCEncodeSupported() -> bool;
}

#[cfg(not(feature = "macos_26_0"))]
#[cfg(test)]
mod tests {
    use crate::{api, cm, vt};
    #[test]
    fn basics() {
        let session = vt::CompressionSession::new::<()>(
            1920,
            1080,
            cm::VideoCodec::HEVC,
            None,
            None,
            None,
            None,
            std::ptr::null_mut(),
        )
        .unwrap();

        if api::macos_available("26.0") {
            let _val = session
                .prop(unsafe {
                    vt::compression_props_keys::supported_preset_dictionaries().unwrap()
                })
                .unwrap();
        }
    }
}
