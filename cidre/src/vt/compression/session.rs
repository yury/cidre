use std::{ffi::c_void, intrinsics::transmute, ptr::NonNull};

use crate::{arc, cf, cm, cv, define_cf_type, os, vt};

#[cfg(feature = "blocks")]
use crate::blocks;

pub type OutputHandler =
    blocks::SendBlock<fn(os::Status, vt::EncodeInfoFlags, Option<&cm::SampleBuf>)>;

define_cf_type!(
    #[doc(alias = "VTCompressionSession")]
    Session(vt::Session)
);

pub type OutputCallback<T> = extern "C" fn(
    output_callback_ref_con: *mut T,
    source_frame_ref_con: *mut c_void,
    status: os::Status,
    info_flags: vt::EncodeInfoFlags,
    sample_buffer: Option<&cm::SampleBuf>,
);

impl Session {
    pub fn new<T>(
        width: u32,
        height: u32,
        codec: cm::VideoCodec,
        encoder_specification: Option<&cf::Dictionary>,
        source_image_buffer_attributes: Option<&cf::Dictionary>,
        compressed_data_allocator: Option<&cf::Allocator>,
        output_callback: Option<OutputCallback<T>>,
        output_callback_ref_con: *mut T,
    ) -> Result<arc::R<Self>, os::Status> {
        let mut session = None;
        unsafe {
            Self::create_in(
                width as _,
                height as _,
                codec,
                encoder_specification,
                source_image_buffer_attributes,
                compressed_data_allocator,
                transmute(output_callback),
                transmute(output_callback_ref_con),
                &mut session,
                None,
            )
            .to_result_unchecked(session)
        }
    }

    /// # Safety
    /// use ::new
    pub unsafe fn create_in(
        width: i32,
        height: i32,
        codec_type: cm::VideoCodec,
        encoder_specification: Option<&cf::Dictionary>,
        source_image_buffer_attributes: Option<&cf::Dictionary>,
        compressed_data_allocator: Option<&cf::Allocator>,
        output_callback: Option<OutputCallback<c_void>>,
        output_callback_ref_con: *mut c_void,
        compression_session_out: *mut Option<arc::R<Session>>,
        allocator: Option<&cf::Allocator>,
    ) -> os::Status {
        VTCompressionSessionCreate(
            allocator,
            width,
            height,
            codec_type,
            encoder_specification,
            source_image_buffer_attributes,
            compressed_data_allocator,
            output_callback,
            output_callback_ref_con,
            compression_session_out,
        )
    }

    #[doc(alias = "VTCompressionSessionInvalidate")]
    #[inline]
    pub fn invalidate(&mut self) {
        unsafe { VTCompressionSessionInvalidate(self) }
    }

    #[doc(alias = "VTCompressionSessionPrepareToEncodeFrames")]
    #[inline]
    pub fn prepare(&mut self) -> Result<(), os::Status> {
        unsafe { self.prepare_to_encode_frames().result() }
    }

    /// #Safety
    /// use `prepare`
    #[doc(alias = "VTCompressionSessionPrepareToEncodeFrames")]
    #[inline]
    pub unsafe fn prepare_to_encode_frames(&mut self) -> os::Status {
        VTCompressionSessionPrepareToEncodeFrames(self)
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
        image_buffer: &cv::ImageBuf,
        pts: cm::Time,
        duration: cm::Time,
        frame_properties: Option<&cf::DictionaryOf<cf::String, cf::Type>>,
        source_frame_ref_con: *mut c_void,
        info_flags_out: *mut Option<NonNull<vt::EncodeInfoFlags>>,
    ) -> os::Status {
        unsafe {
            VTCompressionSessionEncodeFrame(
                self,
                image_buffer,
                pts,
                duration,
                frame_properties,
                source_frame_ref_con,
                info_flags_out,
            )
        }
    }

    #[doc(alias = "VTCompressionSessionEncodeFrame")]
    #[inline]
    pub fn enc_frame(
        &self,
        image_buffer: &cv::ImageBuf,
        pts: cm::Time,
        duration: cm::Time,
        info_flags_out: *mut Option<NonNull<vt::EncodeInfoFlags>>,
    ) -> Result<(), os::Status> {
        unsafe {
            self.encode_frame(
                image_buffer,
                pts,
                duration,
                None,
                std::ptr::null_mut(),
                info_flags_out,
            )
            .to_result_unchecked(Some(()))
        }
    }

    #[doc(alias = "VTCompressionSessionEncodeFrameWithOutputHandler")]
    #[cfg(feature = "blocks")]
    #[inline]
    pub fn encode_frame_with_output_handler(
        &mut self,
        image_buffer: &cv::ImageBuf,
        pts: cm::Time,
        duration: cm::Time,
        frame_properties: Option<&cf::Dictionary>,
        info_flags_out: *mut Option<NonNull<vt::EncodeInfoFlags>>,
        block: &'static mut OutputHandler,
    ) -> os::Status {
        unsafe {
            VTCompressionSessionEncodeFrameWithOutputHandler(
                self,
                image_buffer,
                pts,
                duration,
                frame_properties,
                info_flags_out,
                block,
            )
        }
    }

    #[doc(alias = "VTCompressionSessionEncodeFrameWithOutputHandler")]
    #[cfg(feature = "blocks")]
    #[inline]
    pub fn enc_frame_with_output_handler_block(
        &mut self,
        image_buffer: &cv::ImageBuf,
        pts: cm::Time,
        duration: cm::Time,
        frame_properties: Option<&cf::Dictionary>,
        info_flags_out: *mut Option<NonNull<vt::EncodeInfoFlags>>,
        block: &mut OutputHandler,
    ) -> Result<(), os::Status> {
        unsafe {
            VTCompressionSessionEncodeFrameWithOutputHandler(
                self,
                image_buffer,
                pts,
                duration,
                frame_properties,
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
        image_buffer: &cv::ImageBuf,
        pts: cm::Time,
        duration: cm::Time,
        frame_properties: Option<&cf::Dictionary>,
        info_flags_out: *mut Option<NonNull<vt::EncodeInfoFlags>>,
        block: impl FnMut(os::Status, vt::EncodeInfoFlags, Option<&cm::SampleBuf>) + Send + 'static,
    ) -> Result<(), os::Status> {
        let mut block = OutputHandler::new3(block);
        self.enc_frame_with_output_handler_block(
            image_buffer,
            pts,
            duration,
            frame_properties,
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
    pub fn complete_frames(&self, complete_until_pts: cm::Time) -> os::Status {
        unsafe { VTCompressionSessionCompleteFrames(self, complete_until_pts) }
    }

    #[doc(alias = "VTCompressionSessionCompleteFrames")]
    #[inline]
    pub fn complete_until_pts(&self, complete_until_pts: cm::Time) -> Result<(), os::Status> {
        self.complete_frames(complete_until_pts).result()
    }

    #[doc(alias = "VTCompressionSessionCompleteFrames")]
    #[inline]
    pub fn complete_all(&self) -> Result<(), os::Status> {
        self.complete_frames(cm::Time::invalid()).result()
    }

    // TODO: multipass
}

#[link(name = "VideoToolbox", kind = "framework")]
extern "C" {
    fn VTCompressionSessionCreate(
        allocator: Option<&cf::Allocator>,
        width: i32,
        height: i32,
        codec_type: cm::VideoCodec,
        encoder_specification: Option<&cf::Dictionary>,
        source_image_buffer_attributes: Option<&cf::Dictionary>,
        compressed_data_allocator: Option<&cf::Allocator>,
        output_callback: Option<OutputCallback<c_void>>,
        output_callback_ref_con: *mut c_void,
        compression_session_out: *mut Option<arc::Retained<Session>>,
    ) -> os::Status;

    fn VTCompressionSessionInvalidate(session: &mut Session);
    fn VTCompressionSessionPrepareToEncodeFrames(session: &mut Session) -> os::Status;
    fn VTCompressionSessionEncodeFrame(
        session: &Session,
        image_buffer: &cv::ImageBuf,
        pts: cm::Time,
        duration: cm::Time,
        frame_properties: Option<&cf::DictionaryOf<cf::String, cf::Type>>,
        source_frame_ref_con: *mut c_void,
        info_flags_out: *mut Option<NonNull<vt::EncodeInfoFlags>>,
    ) -> os::Status;

    #[cfg(feature = "blocks")]
    fn VTCompressionSessionEncodeFrameWithOutputHandler(
        session: &Session,
        image_buffer: &cv::ImageBuf,
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

}
