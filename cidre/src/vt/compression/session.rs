use std::{ffi::c_void, intrinsics::transmute, ptr::NonNull};

use crate::{
    arc, cf,
    cm::{self, SampleBuffer, VideoCodecType},
    cv, define_cf_type, os, vt,
};

#[cfg(feature = "blocks")]
use crate::blocks;

define_cf_type!(Session(vt::Session));

pub type OutputCallback<T> = extern "C" fn(
    output_callback_ref_con: *mut T,
    source_frame_ref_con: *mut c_void,
    status: os::Status,
    info_flags: vt::EncodeInfoFlags,
    sample_buffer: Option<&SampleBuffer>,
);

impl Session {
    pub fn new<T>(
        width: u32,
        height: u32,
        codec: VideoCodecType,
        encoder_specification: Option<&cf::Dictionary>,
        source_image_buffer_attributes: Option<&cf::Dictionary>,
        compressed_data_allocator: Option<&cf::Allocator>,
        output_callback: Option<OutputCallback<T>>,
        output_callback_ref_con: *mut T,
    ) -> Result<arc::R<Self>, os::Status> {
        unsafe {
            let mut session = None;
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
        codec_type: VideoCodecType,
        encoder_specification: Option<&cf::Dictionary>,
        source_image_buffer_attributes: Option<&cf::Dictionary>,
        compressed_data_allocator: Option<&cf::Allocator>,
        output_callback: Option<OutputCallback<c_void>>,
        output_callback_ref_con: *mut c_void,
        compression_session_out: &mut Option<arc::R<Session>>,
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
        &mut self,
        image_buffer: &cv::ImageBuffer,
        pts: cm::Time,
        duration: cm::Time,
        frame_properties: Option<&cf::Dictionary>,
        source_frame_ref_con: *mut c_void,
        info_flags_out: &mut Option<NonNull<vt::EncodeInfoFlags>>,
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

    #[inline]
    pub fn enc_frame(
        &mut self,
        image_buffer: &cv::ImageBuffer,
        pts: cm::Time,
        duration: cm::Time,
        info_flags_out: &mut Option<NonNull<vt::EncodeInfoFlags>>,
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
        image_buffer: &cv::ImageBuffer,
        pts: cm::Time,
        duration: cm::Time,
        frame_properties: Option<&cf::Dictionary>,
        info_flags_out: &mut Option<NonNull<vt::EncodeInfoFlags>>,
        block: &'static mut blocks::BlMut<
            impl FnMut(os::Status, vt::EncodeInfoFlags, Option<&cm::SampleBuffer>),
        >,
    ) -> os::Status {
        unsafe {
            VTCompressionSessionEncodeFrameWithOutputHandler(
                self,
                image_buffer,
                pts,
                duration,
                frame_properties,
                info_flags_out,
                block.as_ptr(),
            )
        }
    }

    #[doc(alias = "VTCompressionSessionEncodeFrameWithOutputHandler")]
    #[cfg(feature = "blocks")]
    #[inline]
    pub fn enc_frame_with_output_handler<'a>(
        &mut self,
        image_buffer: &cv::ImageBuffer,
        pts: cm::Time,
        duration: cm::Time,
        frame_properties: Option<&cf::Dictionary>,
        info_flags_out: &mut Option<NonNull<vt::EncodeInfoFlags>>,
        block: &'static mut blocks::Block<
            impl FnMut(os::Status, vt::EncodeInfoFlags, Option<&'a cm::SampleBuffer>),
        >,
    ) -> Result<(), os::Status> {
        unsafe {
            VTCompressionSessionEncodeFrameWithOutputHandler(
                self,
                image_buffer,
                pts,
                duration,
                frame_properties,
                info_flags_out,
                block.as_ptr(),
            )
            .result()
        }
    }

    #[doc(alias = "VTCompressionSessionGetPixelBufferPool")]
    #[inline]
    pub fn pixel_buffer_pool(&self) -> Option<&cv::PixelBufferPool> {
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

extern "C" {
    fn VTCompressionSessionCreate(
        allocator: Option<&cf::Allocator>,
        width: i32,
        height: i32,
        codec_type: VideoCodecType,
        encoder_specification: Option<&cf::Dictionary>,
        source_image_buffer_attributes: Option<&cf::Dictionary>,
        compressed_data_allocator: Option<&cf::Allocator>,
        output_callback: Option<OutputCallback<c_void>>,
        output_callback_ref_con: *mut c_void,
        compression_session_out: &mut Option<arc::R<Session>>,
    ) -> os::Status;

    fn VTCompressionSessionInvalidate(session: &mut Session);
    fn VTCompressionSessionPrepareToEncodeFrames(session: &mut Session) -> os::Status;
    fn VTCompressionSessionEncodeFrame(
        session: &Session,
        image_buffer: &cv::ImageBuffer,
        pts: cm::Time,
        duration: cm::Time,
        frame_properties: Option<&cf::Dictionary>,
        source_frame_ref_con: *mut c_void,
        info_flags_out: &mut Option<NonNull<vt::EncodeInfoFlags>>,
    ) -> os::Status;
    fn VTCompressionSessionEncodeFrameWithOutputHandler(
        session: &Session,
        image_buffer: &cv::ImageBuffer,
        pts: cm::Time,
        duration: cm::Time,
        frame_properties: Option<&cf::Dictionary>,
        info_flags_out: &mut Option<NonNull<vt::EncodeInfoFlags>>,
        output_handler: *mut c_void,
    ) -> os::Status;

    fn VTCompressionSessionGetPixelBufferPool(session: &Session) -> Option<&cv::PixelBufferPool>;
    fn VTCompressionSessionCompleteFrames(
        session: &Session,
        complete_until_pts: cm::Time,
    ) -> os::Status;

}
