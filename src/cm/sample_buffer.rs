use std::ffi::c_void;

use crate::{
    cf::{Allocator, Retained, Type},
    cm, cv, define_cf_type, os,
};

// use super::{formaFormatDescription};

// pub type ArrayRetainCallBack = extern "C" fn(allocator: Option<&Allocator>, value: *const c_void);

pub type SampleBufferMakeDataReadyCallback =
    extern "C" fn(sbuf: &SampleBuffer, make_data_ready_refcon: *const c_void);

#[derive(Clone, Copy)]
#[repr(C)]
pub struct SampleTimingInfo {
    /// The duration of the sample. If a single struct applies to
    /// each of the samples, they all will have this duration
    pub duration: cm::Time,

    /// The time at which the sample will be presented. If a single
    /// struct applies to each of the samples, this is the presentationTime of the
    /// first sample. The presentationTime of subsequent samples will be derived by
    /// repeatedly adding the sample duration.
    pub pts: cm::Time,

    /// The time at which the sample will be decoded. If the samples
    /// are in presentation order (eg. audio samples, or video samples from a codec
    /// that doesn't support out-of-order samples), this can be set to kCMTimeInvalid.
    pub dts: cm::Time,
}

impl SampleTimingInfo {
    pub fn invalid() -> Self {
        unsafe { kCMTimingInfoInvalid }
    }
}

// typedef OSStatus (*CMSampleBufferMakeDataReadyCallback)
// 	(CMSampleBufferRef CM_NONNULL sbuf,	/*! @param sbuf
// 											The CMSampleBuffer to make ready. */
// 	void * CM_NULLABLE makeDataReadyRefcon)	/*! @param makeDataReadyRefcon
// 												Client refcon provided to CMSampleBufferCreate.
// 												For example, it could point at info about the
// 												scheduled read that needs to be forced to finish. */
// 	API_AVAILABLE(macos(10.7), ios(4.0), tvos(9.0), watchos(6.0));

define_cf_type!(SampleBuffer(Type));

impl SampleBuffer {
    /// Returns whether or not a cm::SampleBuffer's data is ready.
    ///
    /// Example:
    /// ```
    /// use cidre::{cf, cm};
    ///
    /// let res = cm::SampleBuffer::new(
    ///     None,
    ///     true,
    ///     None,
    /// );
    ///
    /// let buf = res.unwrap();
    /// assert!(buf.data_is_ready());
    /// ```
    pub fn data_is_ready(&self) -> bool {
        unsafe { CMSampleBufferDataIsReady(self) }
    }
    /// ```
    /// use cidre::{cf, cm};
    ///
    /// let res = cm::SampleBuffer::new(
    ///     None,
    ///     true,
    ///     None,
    /// );
    ///
    /// let buf = res.unwrap();
    /// buf.show();
    /// ```

    pub fn new<'a>(
        data_buffer: Option<&cm::BlockBuffer>,
        data_ready: bool,
        format_description: Option<&cm::FormatDescription>,
    ) -> Result<Retained<'a, SampleBuffer>, os::Status> {
        let mut sample_buffer_out = None;

        unsafe {
            let res = Self::create(
                None,
                data_buffer,
                data_ready,
                None,
                std::ptr::null(),
                format_description,
                0,
                0,
                std::ptr::null(),
                0,
                std::ptr::null(),
                &mut sample_buffer_out,
            );

            if res == os::NO_ERR {
                Ok(sample_buffer_out.unwrap_unchecked())
            } else {
                Err(res)
            }
        }
    }

    pub unsafe fn create<'a>(
        allocator: Option<&Allocator>,
        data_buffer: Option<&cm::BlockBuffer>,
        data_ready: bool,
        make_data_ready_callback: Option<&SampleBufferMakeDataReadyCallback>,
        make_data_ready_refcon: *const c_void,
        format_description: Option<&cm::FormatDescription>,
        num_samples: cm::ItemCount,
        num_samples_timing_entries: cm::ItemCount,
        sample_timing_array: *const SampleTimingInfo,
        num_sample_size_entries: cm::ItemCount,
        sample_size_array: *const usize,
        sample_buffer_out: &mut Option<Retained<SampleBuffer>>,
    ) -> os::Status {
        CMSampleBufferCreate(
            allocator,
            data_buffer,
            data_ready,
            make_data_ready_callback,
            make_data_ready_refcon,
            format_description,
            num_samples,
            num_samples_timing_entries,
            sample_timing_array,
            num_sample_size_entries,
            sample_size_array,
            sample_buffer_out,
        )
    }

    pub fn create_for_image_buffer(
        allocator: Option<&Allocator>,
        image_buffer: &cv::ImageBuffer,
        data_ready: bool,
        make_data_ready_callback: Option<&SampleBufferMakeDataReadyCallback>,
        make_data_ready_refcon: *const c_void,
        format_description: &cm::FormatDescription,
        sample_timing: &SampleTimingInfo,
        sample_buffer_out: &mut Option<Retained<SampleBuffer>>,
    ) -> os::Status {
        unsafe {
            CMSampleBufferCreateForImageBuffer(
                allocator,
                image_buffer,
                data_ready,
                make_data_ready_callback,
                make_data_ready_refcon,
                format_description,
                sample_timing,
                sample_buffer_out,
            )
        }
    }
}

extern "C" {
    static kCMTimingInfoInvalid: SampleTimingInfo;

    fn CMSampleBufferCreate(
        allocator: Option<&Allocator>,
        data_buffer: Option<&cm::BlockBuffer>,
        data_ready: bool,
        make_data_ready_callback: Option<&SampleBufferMakeDataReadyCallback>,
        make_data_ready_refcon: *const c_void,
        format_description: Option<&super::FormatDescription>,
        num_samples: cm::ItemCount,
        num_samples_timing_entries: cm::ItemCount,
        sample_timing_array: *const SampleTimingInfo,
        num_sample_size_entries: cm::ItemCount,
        sample_size_array: *const usize,
        sample_buffer_out: &mut Option<Retained<SampleBuffer>>,
    ) -> crate::os::Status;

    fn CMSampleBufferCreateForImageBuffer(
        allocator: Option<&Allocator>,
        image_buffer: &cv::ImageBuffer,
        data_ready: bool,
        make_data_ready_callback: Option<&SampleBufferMakeDataReadyCallback>,
        make_data_ready_refcon: *const c_void,
        format_description: &cm::VideoFormatDescription,
        sample_timing: &SampleTimingInfo,
        sample_buffer_out: &mut Option<Retained<SampleBuffer>>,
    ) -> crate::os::Status;

    fn CMSampleBufferDataIsReady(sbuf: &SampleBuffer) -> bool;
}
