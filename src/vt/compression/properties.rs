pub mod keys {
    use crate::cf;

    #[inline]
    pub fn number_of_pending_frames() -> &'static cf::String {
        unsafe { kVTCompressionPropertyKey_NumberOfPendingFrames }
    }

    #[inline]
    pub fn pixel_buffer_is_shared() -> &'static cf::String {
        unsafe { kVTCompressionPropertyKey_PixelBufferPoolIsShared }
    }

    #[inline]
    pub fn video_encoder_pixel_buffer_attributes() -> &'static cf::String {
        unsafe { kVTCompressionPropertyKey_VideoEncoderPixelBufferAttributes }
    }

    /// The maximum interval between key frames, also known as the key frame rate.
    ///
    /// Key frames, also known as sync frames, reset inter-frame
    /// dependencies; decoding a key frame is sufficient to prepare a
    /// decoder for correctly decoding the difference frames that
    /// follow.
    /// Video encoders are allowed to generate key frames more frequently if
    /// this would result in more efficient compression.
    /// The default key frame interval is 0, which indicates that the
    /// video encoder should choose where to place all key frames. A key
    /// frame interval of 1 indicates that every frame must be a key
    /// frame, 2 indicates that at least every other frame must be a key
    /// frame, etc.
    /// See also kVTCompressionPropertyKey_AllowTemporalCompression.
    ///
    /// This key can be set in conjunction with
    /// kVTCompressionPropertyKey_MaxKeyFrameIntervalDuration,
    /// and both limits will be enforced - requiring a keyframe every X
    /// frames or every Y seconds, whichever comes first.
    #[inline]
    pub fn max_key_frame_interval() -> &'static cf::String {
        unsafe { kVTCompressionPropertyKey_MaxKeyFrameInterval }
    }

    /// The maximum duration from one key frame to the next in seconds.
    ///
    /// Zero by default, which means no limit.  
    /// This property is particularly useful when the frame rate is variable.
    /// See kVTCompressionPropertyKey_MaxKeyFrameInterval for more discussion
    /// of key frames.
    ///
    /// This key can be set in conjunction with
    /// kVTCompressionPropertyKey_MaxKeyFrameInterval,
    /// and both limits will be enforced - requiring a keyframe every X
    /// frames or every Y seconds, whichever comes first.
    #[inline]
    pub fn max_key_frame_interval_duration() -> &'static cf::String {
        unsafe { kVTCompressionPropertyKey_MaxKeyFrameIntervalDuration }
    }

    /// Enables temporal compression.
    ///
    /// True by default.  Set this to false to require key-frame-only compression.
    #[inline]
    pub fn allow_temporal_compression() -> &'static cf::String {
        unsafe { kVTCompressionPropertyKey_AllowTemporalCompression }
    }

    #[inline]
    pub fn allow_frame_reordering() -> &'static cf::String {
        unsafe { kVTCompressionPropertyKey_AllowFrameReordering }
    }

    #[inline]
    pub fn allow_open_gop() -> &'static cf::String {
        unsafe { kVTCompressionPropertyKey_AllowOpenGOP }
    }

    #[inline]
    pub fn avarage_bit_rate() -> &'static cf::String {
        unsafe { kVTCompressionPropertyKey_AverageBitRate }
    }

    #[inline]
    pub fn data_rate_limits() -> &'static cf::String {
        unsafe { kVTCompressionPropertyKey_DataRateLimits }
    }

    #[inline]
    pub fn quality() -> &'static cf::String {
        unsafe { kVTCompressionPropertyKey_Quality }
    }

    #[inline]
    pub fn target_quality_for_alpha() -> &'static cf::String {
        unsafe { kVTCompressionPropertyKey_TargetQualityForAlpha }
    }

    #[inline]
    pub fn more_frames_before_start() -> &'static cf::String {
        unsafe { kVTCompressionPropertyKey_MoreFramesBeforeStart }
    }

    #[inline]
    pub fn more_frames_after_end() -> &'static cf::String {
        unsafe { kVTCompressionPropertyKey_MoreFramesAfterEnd }
    }

    #[inline]
    pub fn prioritize_encoding_speed_over_quality() -> &'static cf::String {
        unsafe { kVTCompressionPropertyKey_PrioritizeEncodingSpeedOverQuality }
    }

    #[inline]
    pub fn profile_level() -> &'static cf::String {
        unsafe { kVTCompressionPropertyKey_ProfileLevel }
    }

    #[inline]
    pub fn output_bit_depth() -> &'static cf::String {
        unsafe { kVTCompressionPropertyKey_OutputBitDepth }
    }

    #[inline]
    pub fn hdr_metadata_insertion_mode() -> &'static cf::String {
        unsafe { kVTCompressionPropertyKey_HDRMetadataInsertionMode }
    }

    #[inline]
    pub fn h264_entropy_mode() -> &'static cf::String {
        unsafe { kVTCompressionPropertyKey_H264EntropyMode }
    }

    #[inline]
    pub fn depth() -> &'static cf::String {
        unsafe { kVTCompressionPropertyKey_Depth }
    }

    #[inline]
    pub fn max_frame_delay_count() -> &'static cf::String {
        unsafe { kVTCompressionPropertyKey_MaxFrameDelayCount }
    }

    #[inline]
    pub fn max_h264_slice_bytes() -> &'static cf::String {
        unsafe { kVTCompressionPropertyKey_MaxH264SliceBytes }
    }

    #[inline]
    pub fn real_time() -> &'static cf::String {
        unsafe { kVTCompressionPropertyKey_RealTime }
    }

    #[inline]
    pub fn maximize_power_efficiecy() -> &'static cf::String {
        unsafe { kVTCompressionPropertyKey_MaximizePowerEfficiency }
    }

    /// Hint
    #[inline]
    pub fn source_frame_count() -> &'static cf::String {
        unsafe { kVTCompressionPropertyKey_SourceFrameCount }
    }

    /// Hint
    #[inline]
    pub fn expected_frame_rate() -> &'static cf::String {
        unsafe { kVTCompressionPropertyKey_ExpectedFrameRate }
    }

    #[inline]
    pub fn base_layer_frame_rate_fraction() -> &'static cf::String {
        unsafe { kVTCompressionPropertyKey_BaseLayerFrameRateFraction }
    }

    #[inline]
    pub fn base_layer_bit_rate_fraction() -> &'static cf::String {
        unsafe { kVTCompressionPropertyKey_BaseLayerBitRateFraction }
    }

    #[inline]
    pub fn expected_duration() -> &'static cf::String {
        unsafe { kVTCompressionPropertyKey_ExpectedDuration }
    }

    #[inline]
    pub fn base_layer_frame_rate() -> &'static cf::String {
        unsafe { kVTCompressionPropertyKey_BaseLayerFrameRate }
    }

    #[inline]
    pub fn support_base_frame_qp() -> &'static cf::String {
        unsafe { kVTCompressionPropertyKey_SupportsBaseFrameQP }
    }

    #[inline]
    pub fn clean_aperture() -> &'static cf::String {
        unsafe { kVTCompressionPropertyKey_CleanAperture }
    }

    #[inline]
    pub fn pixel_aspect_ratio() -> &'static cf::String {
        unsafe { kVTCompressionPropertyKey_PixelAspectRatio }
    }

    #[inline]
    pub fn field_count() -> &'static cf::String {
        unsafe { kVTCompressionPropertyKey_FieldCount }
    }

    #[inline]
    pub fn field_detail() -> &'static cf::String {
        unsafe { kVTCompressionPropertyKey_FieldDetail }
    }

    #[inline]
    pub fn aspect_ratio_16x9() -> &'static cf::String {
        unsafe { kVTCompressionPropertyKey_AspectRatio16x9 }
    }

    #[inline]
    pub fn progressive_scan() -> &'static cf::String {
        unsafe { kVTCompressionPropertyKey_ProgressiveScan }
    }

    #[inline]
    pub fn color_primaries() -> &'static cf::String {
        unsafe { kVTCompressionPropertyKey_ColorPrimaries }
    }

    #[inline]
    pub fn transfer_function() -> &'static cf::String {
        unsafe { kVTCompressionPropertyKey_TransferFunction }
    }

    #[inline]
    pub fn ycbcr_matrix() -> &'static cf::String {
        unsafe { kVTCompressionPropertyKey_YCbCrMatrix }
    }

    #[inline]
    pub fn icc_profile() -> &'static cf::String {
        unsafe { kVTCompressionPropertyKey_ICCProfile }
    }

    #[inline]
    pub fn master_display_color_volume() -> &'static cf::String {
        unsafe { kVTCompressionPropertyKey_MasteringDisplayColorVolume }
    }

    #[inline]
    pub fn content_light_level_info() -> &'static cf::String {
        unsafe { kVTCompressionPropertyKey_ContentLightLevelInfo }
    }

    #[inline]
    pub fn gamma_level() -> &'static cf::String {
        unsafe { kVTCompressionPropertyKey_GammaLevel }
    }

    #[inline]
    pub fn alpha_channel_mode() -> &'static cf::String {
        unsafe { kVTCompressionPropertyKey_AlphaChannelMode }
    }

    #[inline]
    pub fn pixel_transfer_properties() -> &'static cf::String {
        unsafe { kVTCompressionPropertyKey_PixelTransferProperties }
    }

    #[inline]
    pub fn multi_pass_storage() -> &'static cf::String {
        unsafe { kVTCompressionPropertyKey_MultiPassStorage }
    }

    #[inline]
    pub fn encoder_id() -> &'static cf::String {
        unsafe { kVTCompressionPropertyKey_EncoderID }
    }

    #[inline]
    pub fn preserve_dynamic_hdr_metadata() -> &'static cf::String {
        unsafe { kVTCompressionPropertyKey_PreserveDynamicHDRMetadata }
    }

    #[inline]
    pub fn max_allowed_frame_qp() -> &'static cf::String {
        unsafe { kVTCompressionPropertyKey_MaxAllowedFrameQP }
    }

    #[inline]
    pub fn enable_ltr() -> &'static cf::String {
        unsafe { kVTCompressionPropertyKey_EnableLTR }
    }

    extern "C" {
        static kVTCompressionPropertyKey_NumberOfPendingFrames: &'static cf::String;
        static kVTCompressionPropertyKey_PixelBufferPoolIsShared: &'static cf::String;
        static kVTCompressionPropertyKey_VideoEncoderPixelBufferAttributes: &'static cf::String;
        static kVTCompressionPropertyKey_MaxKeyFrameInterval: &'static cf::String;
        static kVTCompressionPropertyKey_MaxKeyFrameIntervalDuration: &'static cf::String;
        static kVTCompressionPropertyKey_AllowTemporalCompression: &'static cf::String;
        static kVTCompressionPropertyKey_AllowFrameReordering: &'static cf::String;
        static kVTCompressionPropertyKey_AllowOpenGOP: &'static cf::String;
        static kVTCompressionPropertyKey_AverageBitRate: &'static cf::String;
        static kVTCompressionPropertyKey_DataRateLimits: &'static cf::String;
        static kVTCompressionPropertyKey_Quality: &'static cf::String;
        static kVTCompressionPropertyKey_TargetQualityForAlpha: &'static cf::String;
        static kVTCompressionPropertyKey_MoreFramesBeforeStart: &'static cf::String;
        static kVTCompressionPropertyKey_MoreFramesAfterEnd: &'static cf::String;
        static kVTCompressionPropertyKey_PrioritizeEncodingSpeedOverQuality: &'static cf::String;
        static kVTCompressionPropertyKey_ProfileLevel: &'static cf::String;
        static kVTCompressionPropertyKey_OutputBitDepth: &'static cf::String;
        static kVTCompressionPropertyKey_HDRMetadataInsertionMode: &'static cf::String;
        static kVTCompressionPropertyKey_H264EntropyMode: &'static cf::String;
        static kVTCompressionPropertyKey_Depth: &'static cf::String;
        static kVTCompressionPropertyKey_MaxFrameDelayCount: &'static cf::String;
        static kVTCompressionPropertyKey_MaxH264SliceBytes: &'static cf::String;
        static kVTCompressionPropertyKey_RealTime: &'static cf::String;
        static kVTCompressionPropertyKey_MaximizePowerEfficiency: &'static cf::String;
        static kVTCompressionPropertyKey_SourceFrameCount: &'static cf::String;
        static kVTCompressionPropertyKey_ExpectedFrameRate: &'static cf::String;
        static kVTCompressionPropertyKey_BaseLayerFrameRateFraction: &'static cf::String;
        static kVTCompressionPropertyKey_BaseLayerBitRateFraction: &'static cf::String;
        static kVTCompressionPropertyKey_ExpectedDuration: &'static cf::String;
        static kVTCompressionPropertyKey_BaseLayerFrameRate: &'static cf::String;
        static kVTCompressionPropertyKey_SupportsBaseFrameQP: &'static cf::String;
        static kVTCompressionPropertyKey_CleanAperture: &'static cf::String;
        static kVTCompressionPropertyKey_PixelAspectRatio: &'static cf::String;
        static kVTCompressionPropertyKey_FieldCount: &'static cf::String;
        static kVTCompressionPropertyKey_FieldDetail: &'static cf::String;
        static kVTCompressionPropertyKey_AspectRatio16x9: &'static cf::String;
        static kVTCompressionPropertyKey_ProgressiveScan: &'static cf::String;
        static kVTCompressionPropertyKey_ColorPrimaries: &'static cf::String;
        static kVTCompressionPropertyKey_TransferFunction: &'static cf::String;
        static kVTCompressionPropertyKey_YCbCrMatrix: &'static cf::String;
        static kVTCompressionPropertyKey_ICCProfile: &'static cf::String;
        static kVTCompressionPropertyKey_MasteringDisplayColorVolume: &'static cf::String;
        static kVTCompressionPropertyKey_ContentLightLevelInfo: &'static cf::String;
        static kVTCompressionPropertyKey_GammaLevel: &'static cf::String;
        static kVTCompressionPropertyKey_AlphaChannelMode: &'static cf::String;
        static kVTCompressionPropertyKey_PixelTransferProperties: &'static cf::String;
        static kVTCompressionPropertyKey_MultiPassStorage: &'static cf::String;
        static kVTCompressionPropertyKey_EncoderID: &'static cf::String;
        static kVTCompressionPropertyKey_PreserveDynamicHDRMetadata: &'static cf::String;
        static kVTCompressionPropertyKey_MaxAllowedFrameQP: &'static cf::String;
        static kVTCompressionPropertyKey_EnableLTR: &'static cf::String;
    }
}
