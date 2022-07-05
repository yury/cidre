use std::ffi::c_void;

use crate::{
    cf::{self, Allocator, Retained, Type},
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
    #[inline]
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
            Self::create(
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
            )
            .to_result(sample_buffer_out)
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

    #[inline]
    pub fn image_buffer(&self) -> Option<&cv::ImageBuffer> {
        unsafe { CMSampleBufferGetImageBuffer(self) }
    }

    #[inline]
    pub fn data_buffer(&self) -> Option<&cm::BlockBuffer> {
        unsafe { CMSampleBufferGetDataBuffer(self) }
    }

    #[inline]
    pub fn duration(&self) -> cm::Time {
        unsafe { CMSampleBufferGetDuration(self) }
    }

    #[inline]
    pub fn presentation_time_stamp(&self) -> cm::Time {
        unsafe { CMSampleBufferGetPresentationTimeStamp(self) }
    }

    #[inline]
    pub fn decode_time_stamp(&self) -> cm::Time {
        unsafe { CMSampleBufferGetDecodeTimeStamp(self) }
    }

    /// Returns the output presentation timestamp of the CMSampleBuffer.
    #[inline]
    pub fn output_presentation_time_stamp(&self) -> cm::Time {
        unsafe { CMSampleBufferGetOutputPresentationTimeStamp(self) }
    }

    #[inline]
    pub fn set_output_presentation_time_stamp(&self, value: cm::Time) {
        unsafe { CMSampleBufferSetOutputPresentationTimeStamp(self, value) }
    }

    /// Returns the size in bytes of a specified sample in a CMSampleBuffer.
    ///
    /// Size in bytes of the specified sample in the CMSampleBuffer.
    #[inline]
    pub fn sample_size(&self, sample_index: cm::ItemIndex) -> usize {
        unsafe { CMSampleBufferGetSampleSize(self, sample_index) }
    }

    /// Returns the total size in bytes of sample data in a CMSampleBuffer.
    ///
    /// Total size in bytes of sample data in the CMSampleBuffer.
    /// If there are no sample sizes in this CMSampleBuffer, a size of 0 will be returned.  
    #[inline]
    pub fn total_sample_size(&self) -> usize {
        unsafe { CMSampleBufferGetTotalSampleSize(self) }
    }

    #[inline]
    pub fn format_description(&self) -> Option<&cm::FormatDescription> {
        unsafe { CMSampleBufferGetFormatDescription(self) }
    }

    /// Returns a reference to a CMSampleBuffer's immutable array of mutable sample attachments dictionaries (one dictionary
    /// per sample in the CMSampleBuffer)
    #[inline]
    pub fn attachments(
        &self,
        create_if_necessary: bool,
    ) -> Option<&cf::ArrayOf<cf::MutableDictionary>> {
        unsafe { CMSampleBufferGetSampleAttachmentsArray(self, create_if_necessary) }
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

    fn CMSampleBufferGetImageBuffer(sbuf: &SampleBuffer) -> Option<&cv::ImageBuffer>;
    fn CMSampleBufferGetDataBuffer(sbuf: &SampleBuffer) -> Option<&cm::BlockBuffer>;
    fn CMSampleBufferGetDuration(sbuf: &SampleBuffer) -> cm::Time;
    fn CMSampleBufferGetPresentationTimeStamp(sbuf: &SampleBuffer) -> cm::Time;
    fn CMSampleBufferGetDecodeTimeStamp(sbuf: &SampleBuffer) -> cm::Time;
    fn CMSampleBufferGetOutputPresentationTimeStamp(sbuf: &SampleBuffer) -> cm::Time;
    fn CMSampleBufferSetOutputPresentationTimeStamp(sbuf: &SampleBuffer, value: cm::Time);
    fn CMSampleBufferGetSampleSize(sbuf: &SampleBuffer, sample_index: cm::ItemIndex) -> usize;
    fn CMSampleBufferGetTotalSampleSize(sbuf: &SampleBuffer) -> usize;
    fn CMSampleBufferGetFormatDescription(sbuf: &SampleBuffer) -> Option<&cm::FormatDescription>;
    fn CMSampleBufferGetSampleAttachmentsArray(
        sbuf: &SampleBuffer,
        create_if_necessary: bool,
    ) -> Option<&cf::ArrayOf<cf::MutableDictionary>>;
}

pub mod attachment_keys {
    use crate::cf;

    /// Indicates the reason the current video frame was dropped.
    /// 
    /// Sample buffers with this attachment contain no image or data buffer.  They mark a dropped video
	/// frame.  This attachment identifies the reason for the droppage.
    #[inline]
    pub fn dropped_frame_reason() -> &'static cf::String {
        unsafe { kCMSampleBufferAttachmentKey_DroppedFrameReason }
    }
    /// Indicates additional information regarding the dropped video frame.
    #[inline]
    pub fn dropped_frame_reason_info() -> &'static cf::String {
        unsafe { kCMSampleBufferAttachmentKey_DroppedFrameReasonInfo }
    }
    /// Indicates information about the lens stabilization applied to the current still image buffer.
    /// 
    /// Sample buffers that have been captured with a lens stabilization module may have an attachment of
	/// kCMSampleBufferAttachmentKey_StillImageLensStabilizationInfo which has information about the stabilization status
    /// during the capture.  This key will not be present in CMSampleBuffers coming from cameras without a lens stabilization module.
    #[inline]
    pub fn still_image_lens_stabilization_info() -> &'static cf::String {
        unsafe { kCMSampleBufferAttachmentKey_StillImageLensStabilizationInfo }
    }

    /// Indicates the 3x3 camera intrinsic matrix applied to the current sample buffer.
    ///
    /// Camera intrinsic matrix is a CFData containing a matrix_float3x3, which is column-major. It has the following contents:
    /// fx	0	ox
    /// 0	fy	oy
    /// 0	0	1
    /// fx and fy are the focal length in pixels. For square pixels, they will have the same value.
    /// ox and oy are the coordinates of the principal point. The origin is the upper left of the frame.
    pub fn camera_intrinsic_matrix() -> &'static cf::String {
        unsafe { kCMSampleBufferAttachmentKey_CameraIntrinsicMatrix }
    }

    /// Indicates that the current or next video sample buffer should be forced to be encoded as a key frame.
    ///
    /// A value of kCFBooleanTrue for kCMSampleBufferAttachmentKey_ForceKeyFrame indicates
    /// that the current or next video sample buffer processed in the stream should be forced
    /// to be encoded as a key frame.
    /// If this attachment is present and kCFBooleanTrue on a sample buffer with a video
    /// frame, that video frame will be forced to become a key frame.  If the sample
    /// buffer for which this is present and kCFBooleanTrue does not have a valid video
    /// frame, the next sample buffer processed that contains a valid video frame will be
    /// encoded as a key frame.
    #[inline]
    pub fn force_key_frame() -> &'static cf::String {
        unsafe { kCMSampleBufferAttachmentKey_ForceKeyFrame }
    }

    #[inline]
    pub fn cryptor_subsample_auxiliary_data() -> &'static cf::String {
        unsafe { kCMSampleAttachmentKey_CryptorSubsampleAuxiliaryData }
    }

    extern "C" {
        static kCMSampleBufferAttachmentKey_DroppedFrameReason: &'static cf::String;
        static kCMSampleBufferAttachmentKey_DroppedFrameReasonInfo: &'static cf::String;
        static kCMSampleBufferAttachmentKey_StillImageLensStabilizationInfo: &'static cf::String;
        static kCMSampleBufferAttachmentKey_CameraIntrinsicMatrix: &'static cf::String;
        static kCMSampleBufferAttachmentKey_ForceKeyFrame: &'static cf::String;
        static kCMSampleAttachmentKey_CryptorSubsampleAuxiliaryData: &'static cf::String;
    }
}
