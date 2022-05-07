use std::{ffi::c_void, ptr::NonNull};

use crate::{
    cf::{self, Retained},
    cm::{self, SampleBuffer, VideoCodecType},
    cv, define_cf_type,
    os::{self, NO_ERR},
    vt,
};

define_cf_type!(Session(crate::vt::Session));

pub type OutputCallback = extern "C" fn(
    output_callback_ref_con: *mut c_void,
    source_frame_ref_con: *mut c_void,
    status: os::Status,
    info_flags: vt::EncodeInfoFlags,
    sample_buffer: Option<&SampleBuffer>,
);

impl Session {
    pub fn new<'a>(
        width: u32,
        height: u32,
        codec: VideoCodecType,
        encoder_specification: Option<&cf::Dictionary>,
        source_image_buffer_attributes: Option<&cf::Dictionary>,
        output_callback: Option<&OutputCallback>,
        output_callback_ref_con: *mut c_void,
    ) -> Result<Retained<'a, Self>, os::Status> {
        let mut session = None;
        let res = Self::create(
            None,
            width as _,
            height as _,
            codec,
            encoder_specification,
            source_image_buffer_attributes,
            None,
            output_callback,
            output_callback_ref_con,
            &mut session,
        );
        if res == NO_ERR {
            Ok(session.unwrap())
        } else {
            Err(res)
        }
    }

    pub fn create(
        allocator: Option<&cf::Allocator>,
        width: i32,
        height: i32,
        codec_type: VideoCodecType,
        encoder_specification: Option<&cf::Dictionary>,
        source_image_buffer_attributes: Option<&cf::Dictionary>,
        compressed_data_allocator: Option<&cf::Allocator>,
        output_callback: Option<&OutputCallback>,
        output_callback_ref_con: *mut c_void,
        compression_session_out: &mut Option<cf::Retained<Session>>,
    ) -> os::Status {
        unsafe {
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
    }

    #[inline]
    pub fn invalidate(&mut self) {
        unsafe { VTCompressionSessionInvalidate(self) }
    }

    #[inline]
    pub fn prepare_to_encode_frames(&mut self) {
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
        output_callback: Option<&OutputCallback>,
        output_callback_ref_con: *mut c_void,
        compression_session_out: &mut Option<cf::Retained<Session>>,
    ) -> os::Status;

    fn VTCompressionSessionInvalidate(session: &mut Session);
    fn VTCompressionSessionPrepareToEncodeFrames(session: &mut Session);

    fn VTCompressionSessionEncodeFrame(
        session: &Session,
        image_buffer: &cv::ImageBuffer,
        pts: cm::Time,
        duration: cm::Time,
        frame_properties: Option<&cf::Dictionary>,
        source_frame_ref_con: *mut c_void,
        info_flags_out: &mut Option<NonNull<vt::EncodeInfoFlags>>,
    ) -> os::Status;
}
