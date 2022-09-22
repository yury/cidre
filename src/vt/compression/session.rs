use std::{ffi::c_void, intrinsics::transmute, ptr::NonNull};

use crate::{
    cf::{self, Retained},
    cm::{self, SampleBuffer, VideoCodecType},
    cv, define_cf_type, os, vt,
};

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
        output_callback: Option<OutputCallback<T>>,
        output_callback_ref_con: *mut T,
    ) -> Result<Retained<Self>, os::Status> {
        unsafe {
            let mut session = None;
            Self::create(
                None,
                width as _,
                height as _,
                codec,
                encoder_specification,
                source_image_buffer_attributes,
                None,
                transmute(output_callback),
                transmute(output_callback_ref_con),
                &mut session,
            )
            .to_result_unchecked(session)
        }
    }

    /// use ::new
    pub unsafe fn create(
        allocator: Option<&cf::Allocator>,
        width: i32,
        height: i32,
        codec_type: VideoCodecType,
        encoder_specification: Option<&cf::Dictionary>,
        source_image_buffer_attributes: Option<&cf::Dictionary>,
        compressed_data_allocator: Option<&cf::Allocator>,
        output_callback: Option<OutputCallback<c_void>>,
        output_callback_ref_con: *mut c_void,
        compression_session_out: &mut Option<cf::Retained<Session>>,
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

    #[inline]
    pub fn invalidate(&mut self) {
        unsafe { VTCompressionSessionInvalidate(self) }
    }

    #[inline]
    pub fn prepare(&mut self) -> Result<(), os::Status> {
        self.prepare_to_encode_frames().result()
    }

    #[inline]
    pub fn prepare_to_encode_frames(&mut self) -> os::Status {
        unsafe { VTCompressionSessionPrepareToEncodeFrames(self) }
    }

    #[inline]
    pub fn encode_frame(
        &self,
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
    pub fn pixel_buffer_pool(&self) -> Option<&cv::PixelBufferPool> {
        unsafe { VTCompressionSessionGetPixelBufferPool(self) }
    }

    #[inline]
    pub fn complete_frames(&self) -> os::Status {
        unsafe { VTCompressionSessionCompleteFrames(self) }
    }

    #[inline]
    pub fn complete(&self) -> Result<(), os::Status> {
        self.complete_frames().result()
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
        compression_session_out: &mut Option<cf::Retained<Session>>,
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

    fn VTCompressionSessionGetPixelBufferPool(session: &Session) -> Option<&cv::PixelBufferPool>;

    fn VTCompressionSessionCompleteFrames(session: &Session) -> os::Status;
}
