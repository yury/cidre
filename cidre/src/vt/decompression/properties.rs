pub mod keys {

    use crate::{api, cf};

    #[doc(alias = "kVTDecompressionPropertyKey_PixelBufferPool")]
    pub fn pixel_buffer_pool() -> &'static cf::String {
        unsafe { kVTDecompressionPropertyKey_PixelBufferPool }
    }

    #[doc(alias = "kVTDecompressionPropertyKey_PixelBufferPoolIsShared")]
    pub fn pixel_buffer_is_shared() -> &'static cf::String {
        unsafe { kVTDecompressionPropertyKey_PixelBufferPoolIsShared }
    }

    #[doc(alias = "kVTDecompressionPropertyKey_OutputPoolRequestedMinimumBufferCount")]
    pub fn output_pool_requested_minimum_buffer_count() -> &'static cf::String {
        unsafe { kVTDecompressionPropertyKey_OutputPoolRequestedMinimumBufferCount }
    }

    #[doc(alias = "kVTDecompressionPropertyKey_NumberOfFramesBeingDecoded")]
    pub fn number_of_frames_being_decoded() -> &'static cf::String {
        unsafe { kVTDecompressionPropertyKey_NumberOfFramesBeingDecoded }
    }

    #[doc(alias = "kVTDecompressionPropertyKey_MinOutputPresentationTimeStampOfFramesBeingDecoded")]
    pub fn min_output_pts_of_frames_being_decoded() -> &'static cf::String {
        unsafe { kVTDecompressionPropertyKey_MinOutputPresentationTimeStampOfFramesBeingDecoded }
    }

    #[doc(alias = "kVTDecompressionPropertyKey_MaxOutputPresentationTimeStampOfFramesBeingDecoded")]
    pub fn max_output_pts_of_frames_being_decoded() -> &'static cf::String {
        unsafe { kVTDecompressionPropertyKey_MaxOutputPresentationTimeStampOfFramesBeingDecoded }
    }

    #[doc(alias = "kVTDecompressionPropertyKey_ContentHasInterframeDependencies")]
    pub fn content_has_interframe_dependencies() -> &'static cf::String {
        unsafe { kVTDecompressionPropertyKey_ContentHasInterframeDependencies }
    }

    /// Hints the video decoder that decompression is, or is not, being performed in real time.
    pub fn real_time() -> &'static cf::String {
        unsafe { kVTDecompressionPropertyKey_RealTime }
    }

    /// `cf::Boolean`, Read; assumed false by default
    #[doc(alias = "kVTDecompressionPropertyKey_UsingHardwareAcceleratedVideoDecoder")]
    pub fn using_hardware_accelerated_video_decoder() -> &'static cf::String {
        unsafe { kVTDecompressionPropertyKey_UsingHardwareAcceleratedVideoDecoder }
    }

    #[doc(alias = "kVTDecompressionPropertyKey_MaximizePowerEfficiency")]
    pub fn maximize_power_efficiency() -> &'static cf::String {
        unsafe { kVTDecompressionPropertyKey_MaximizePowerEfficiency }
    }

    #[doc(alias = "kVTDecompressionPropertyKey_ThreadCount")]
    pub fn thread_count() -> &'static cf::String {
        unsafe { kVTDecompressionPropertyKey_ThreadCount }
    }

    #[doc(alias = "kVTDecompressionPropertyKey_FieldMode")]
    pub fn field_mode() -> &'static cf::String {
        unsafe { kVTDecompressionPropertyKey_FieldMode }
    }

    #[doc(alias = "kVTDecompressionPropertyKey_DeinterlaceMode")]
    pub fn deinterlace_mode() -> &'static cf::String {
        unsafe { kVTDecompressionPropertyKey_DeinterlaceMode }
    }

    #[doc(alias = "kVTDecompressionPropertyKey_ReducedResolutionDecode")]
    pub fn reduced_resolution_decode() -> &'static cf::String {
        unsafe { kVTDecompressionPropertyKey_ReducedResolutionDecode }
    }

    #[doc(alias = "kVTDecompressionPropertyKey_ReducedCoefficientDecode")]
    pub fn reduced_coefficent_decode() -> &'static cf::String {
        unsafe { kVTDecompressionPropertyKey_ReducedCoefficientDecode }
    }

    #[doc(alias = "kVTDecompressionPropertyKey_ReducedFrameDelivery")]
    pub fn reduced_frame_delivery() -> &'static cf::String {
        unsafe { kVTDecompressionPropertyKey_ReducedFrameDelivery }
    }

    #[doc(alias = "kVTDecompressionPropertyKey_OnlyTheseFrames")]
    pub fn only_these_frames() -> &'static cf::String {
        unsafe { kVTDecompressionPropertyKey_OnlyTheseFrames }
    }

    #[doc(alias = "kVTDecompressionPropertyKey_SupportedPixelFormatsOrderedByPerformance")]
    pub fn supported_pixel_formats_ordered_by_performance() -> &'static cf::String {
        unsafe { kVTDecompressionPropertyKey_SupportedPixelFormatsOrderedByPerformance }
    }

    #[doc(alias = "kVTDecompressionPropertyKey_PixelFormatsWithReducedResolutionSupport")]
    pub fn pixel_formats_with_reduced_resolution_support() -> &'static cf::String {
        unsafe { kVTDecompressionPropertyKey_PixelFormatsWithReducedResolutionSupport }
    }

    #[doc(alias = "kVTDecompressionPropertyKey_PropagatePerFrameHDRDisplayMetadata")]
    #[api::available(macos = 11.0, ios = 14.0, tvos = 14.0, visionos = 1.0)]
    pub fn propogate_per_frame_hdr_display_metadata() -> &'static cf::String {
        unsafe { kVTDecompressionPropertyKey_PropagatePerFrameHDRDisplayMetadata }
    }

    #[doc(alias = "kVTDecompressionPropertyKey_PixelTransferProperties")]
    pub fn pixel_transfer_props() -> &'static cf::String {
        unsafe { kVTDecompressionPropertyKey_PixelTransferProperties }
    }

    /// cf::Number indicating the gpu registryID of the decoder that was selected.
    /// NULL indicates a built-in decoder or software decode was used.
    /// read-only
    #[doc(alias = "kVTDecompressionPropertyKey_UsingGPURegistryID")]
    pub fn using_gpu_registry_id() -> &'static cf::String {
        unsafe { kVTDecompressionPropertyKey_UsingGPURegistryID }
    }

    /// Generates per frame HDR Metadata and attaches it to the resulting decoded cv::PixelBufs.
    ///
    /// If the color space and YCbCrMatrix matches a supported HDR format such as HLG (kCMFormatDescriptionTransferFunction_ITU_R_2100_HLG)
    /// the decoded frame will be analyzed and metadata will be added as an attachment to the CVPixelBuffer.
    /// CFBoolean, Read/Write, Optional, kCFBooleanFalse by default
    #[doc(alias = "kVTDecompressionPropertyKey_GeneratePerFrameHDRDisplayMetadata")]
    #[api::available(macos = 14.0, ios = 17.0, tvos = 17.0, visionos = 1.0)]
    pub fn generate_per_frame_hdr_display_metadata() -> &'static cf::String {
        unsafe { kVTDecompressionPropertyKey_GeneratePerFrameHDRDisplayMetadata }
    }

    /// cf::Boolean, Read, Optional, kCFBooleanFalse by default
    #[doc(alias = "kVTDecompressionPropertyKey_DecoderProducesRAWOutput")]
    #[api::available(macos = 15.0)]
    pub fn decoder_produces_raw_output() -> &'static cf::String {
        unsafe { kVTDecompressionPropertyKey_DecoderProducesRAWOutput }
    }

    /// cf::Boolean, Read/Write, Optional, kCFBooleanFalse by default
    #[doc(alias = "kVTDecompressionPropertyKey_RequestRAWOutput")]
    #[api::available(macos = 15.0)]
    pub fn request_raw_output() -> &'static cf::String {
        unsafe { kVTDecompressionPropertyKey_RequestRAWOutput }
    }

    /// Requests multi-image decoding of specific MV-HEVC VideoLayerIDs.
    ///
    /// cf::Array of CFNumbers, Optional
    #[doc(alias = "kVTDecompressionPropertyKey_RequestedMVHEVCVideoLayerIDs")]
    #[api::available(macos = 14.0, ios = 17.0, visionos = 1.0)]
    pub fn requested_mv_hevc_video_layer_ids() -> &'static cf::String {
        unsafe { kVTDecompressionPropertyKey_RequestedMVHEVCVideoLayerIDs }
    }

    #[link(name = "VideoToolbox", kind = "framework")]
    #[api::weak]
    unsafe extern "C" {
        static kVTDecompressionPropertyKey_PixelBufferPool: &'static cf::String;
        static kVTDecompressionPropertyKey_PixelBufferPoolIsShared: &'static cf::String;
        static kVTDecompressionPropertyKey_OutputPoolRequestedMinimumBufferCount:
            &'static cf::String;
        static kVTDecompressionPropertyKey_NumberOfFramesBeingDecoded: &'static cf::String;
        static kVTDecompressionPropertyKey_MinOutputPresentationTimeStampOfFramesBeingDecoded:
            &'static cf::String;
        static kVTDecompressionPropertyKey_MaxOutputPresentationTimeStampOfFramesBeingDecoded:
            &'static cf::String;

        static kVTDecompressionPropertyKey_ContentHasInterframeDependencies: &'static cf::String;

        static kVTDecompressionPropertyKey_RealTime: &'static cf::String;
        static kVTDecompressionPropertyKey_UsingHardwareAcceleratedVideoDecoder:
            &'static cf::String;

        static kVTDecompressionPropertyKey_MaximizePowerEfficiency: &'static cf::String;
        static kVTDecompressionPropertyKey_ThreadCount: &'static cf::String;
        static kVTDecompressionPropertyKey_FieldMode: &'static cf::String;

        static kVTDecompressionPropertyKey_DeinterlaceMode: &'static cf::String;
        static kVTDecompressionPropertyKey_ReducedResolutionDecode: &'static cf::String;
        static kVTDecompressionPropertyKey_ReducedCoefficientDecode: &'static cf::String;
        static kVTDecompressionPropertyKey_ReducedFrameDelivery: &'static cf::String;
        static kVTDecompressionPropertyKey_OnlyTheseFrames: &'static cf::String;
        static kVTDecompressionPropertyKey_SupportedPixelFormatsOrderedByPerformance:
            &'static cf::String;

        static kVTDecompressionPropertyKey_PixelFormatsWithReducedResolutionSupport:
            &'static cf::String;

        #[api::available(macos = 11.0, ios = 14.0, tvos = 14.0, visionos = 1.0)]
        static kVTDecompressionPropertyKey_PropagatePerFrameHDRDisplayMetadata: &'static cf::String;
        static kVTDecompressionPropertyKey_PixelTransferProperties: &'static cf::String;

        static kVTDecompressionPropertyKey_UsingGPURegistryID: &'static cf::String;

        #[api::available(macos = 14.0, ios = 17.0, tvos = 17.0, visionos = 1.0)]
        static kVTDecompressionPropertyKey_GeneratePerFrameHDRDisplayMetadata: &'static cf::String;

        #[api::available(macos = 15.0)]
        static kVTDecompressionPropertyKey_DecoderProducesRAWOutput: &'static cf::String;

        #[api::available(macos = 15.0)]
        static kVTDecompressionPropertyKey_RequestRAWOutput: &'static cf::String;

        #[api::available(macos = 14.0, ios = 17.0, visionos = 1.0)]
        static kVTDecompressionPropertyKey_RequestedMVHEVCVideoLayerIDs: &'static cf::String;

    }
}

pub mod video_decoder_specification {
    use crate::cf;

    /// If set to `cf::Boolean::value_true()`, the VideoToolbox will use a hardware accelerated video decoder if available.  If set to
    /// `cf::Boolean::value_false()`, hardware decode will never be used.
    ///
    /// This key is set in the `decoderSpecification` passed in to `vt::DecompressionSession::new()`.  Set it
    /// to `cf::Boolean::value_true()` to allow hardware accelerated decode.  To  prevent hardware decode,
    /// this property can be set to `cf::Boolean::value_false()`.
    /// In MacOS 10.15 and later, hardware decode is enabled in vt::DecompressionSessions by default.
    ///
    /// `cf::Boolean`, Optional, true by default
    #[doc(alias = "kVTVideoDecoderSpecification_EnableHardwareAcceleratedVideoDecoder")]
    pub fn enable_hardware_accelerated_video_decoder() -> &'static cf::String {
        unsafe { kVTVideoDecoderSpecification_EnableHardwareAcceleratedVideoDecoder }
    }

    /// if set to `cf::Boolean::value_true()`, the VideoToolbox will try to allocate a hardware accelerated
    /// decoder and return an error if that isn't possible.
    /// Setting this key automatically implies kVTVideoDecoderSpecification_EnableHardwareAcceleratedVideoDecoder --
    /// there is no need to set both and the Enable key does nothing if the Require key is set.
    #[doc(alias = "kVTVideoDecoderSpecification_RequireHardwareAcceleratedVideoDecoder")]
    pub fn require_hardware_accelerated_video_decoder() -> &'static cf::String {
        unsafe { kVTVideoDecoderSpecification_RequireHardwareAcceleratedVideoDecoder }
    }

    /// cf::Number Optional
    #[doc(alias = "kVTVideoDecoderSpecification_RequiredDecoderGPURegistryID")]
    pub fn required_decoder_gpu_registry_id() -> &'static cf::String {
        unsafe { kVTVideoDecoderSpecification_RequiredDecoderGPURegistryID }
    }

    /// cf::Number Optional
    #[doc(alias = "kVTVideoDecoderSpecification_PreferredDecoderGPURegistryID")]
    pub fn preferred_decoder_gpu_registry_id() -> &'static cf::String {
        unsafe { kVTVideoDecoderSpecification_PreferredDecoderGPURegistryID }
    }

    #[link(name = "VideoToolbox", kind = "framework")]
    unsafe extern "C" {
        static kVTVideoDecoderSpecification_EnableHardwareAcceleratedVideoDecoder:
            &'static cf::String;

        static kVTVideoDecoderSpecification_RequireHardwareAcceleratedVideoDecoder:
            &'static cf::String;

        static kVTVideoDecoderSpecification_RequiredDecoderGPURegistryID: &'static cf::String;
        static kVTVideoDecoderSpecification_PreferredDecoderGPURegistryID: &'static cf::String;
    }
}
