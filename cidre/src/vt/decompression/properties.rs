pub mod keys {

    use crate::cf;

    pub fn pixel_buffer_pool() -> &'static cf::String {
        unsafe { kVTDecompressionPropertyKey_PixelBufferPool }
    }

    pub fn pixel_buffer_is_shared() -> &'static cf::String {
        unsafe { kVTDecompressionPropertyKey_PixelBufferPoolIsShared }
    }

    pub fn output_pool_requested_minimum_buffer_count() -> &'static cf::String {
        unsafe { kVTDecompressionPropertyKey_OutputPoolRequestedMinimumBufferCount }
    }

    pub fn number_of_frames_being_decoded() -> &'static cf::String {
        unsafe { kVTDecompressionPropertyKey_NumberOfFramesBeingDecoded }
    }

    pub fn min_output_pts_of_frames_being_decoded() -> &'static cf::String {
        unsafe { kVTDecompressionPropertyKey_MinOutputPresentationTimeStampOfFramesBeingDecoded }
    }

    pub fn max_output_pts_of_frames_being_decoded() -> &'static cf::String {
        unsafe { kVTDecompressionPropertyKey_MaxOutputPresentationTimeStampOfFramesBeingDecoded }
    }

    pub fn content_has_interframe_dependencies() -> &'static cf::String {
        unsafe { kVTDecompressionPropertyKey_ContentHasInterframeDependencies }
    }

    pub fn real_time() -> &'static cf::String {
        unsafe { kVTDecompressionPropertyKey_RealTime }
    }

    pub fn maximize_power_efficiency() -> &'static cf::String {
        unsafe { kVTDecompressionPropertyKey_MaximizePowerEfficiency }
    }

    pub fn thread_count() -> &'static cf::String {
        unsafe { kVTDecompressionPropertyKey_ThreadCount }
    }

    pub fn field_mode() -> &'static cf::String {
        unsafe { kVTDecompressionPropertyKey_FieldMode }
    }

    pub fn deinterlace_mode() -> &'static cf::String {
        unsafe { kVTDecompressionPropertyKey_DeinterlaceMode }
    }

    pub fn reduced_resolution_decode() -> &'static cf::String {
        unsafe { kVTDecompressionPropertyKey_ReducedResolutionDecode }
    }

    pub fn reduced_coefficent_decode() -> &'static cf::String {
        unsafe { kVTDecompressionPropertyKey_ReducedCoefficientDecode }
    }

    pub fn reduced_frame_delivery() -> &'static cf::String {
        unsafe { kVTDecompressionPropertyKey_ReducedFrameDelivery }
    }

    pub fn only_these_frames() -> &'static cf::String {
        unsafe { kVTDecompressionPropertyKey_OnlyTheseFrames }
    }

    pub fn supported_pixel_formats_ordered_by_performance() -> &'static cf::String {
        unsafe { kVTDecompressionPropertyKey_SupportedPixelFormatsOrderedByPerformance }
    }

    pub fn pixel_formats_with_reduced_resolution_support() -> &'static cf::String {
        unsafe { kVTDecompressionPropertyKey_PixelFormatsWithReducedResolutionSupport }
    }

    pub fn propogate_per_frame_hdr_display_metadata() -> &'static cf::String {
        unsafe { kVTDecompressionPropertyKey_PropagatePerFrameHDRDisplayMetadata }
    }

    pub fn pixel_transfer_properties() -> &'static cf::String {
        unsafe { kVTDecompressionPropertyKey_PixelTransferProperties }
    }

    #[link(name = "VideoToolbox", kind = "framework")]
    extern "C" {
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

        static kVTDecompressionPropertyKey_PropagatePerFrameHDRDisplayMetadata: &'static cf::String;
        static kVTDecompressionPropertyKey_PixelTransferProperties: &'static cf::String;

    }
}
