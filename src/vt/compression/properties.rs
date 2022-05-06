pub mod keys {
    use crate::cf;

    pub fn number_of_pending_frames() -> &'static cf::String {
        unsafe { kVTCompressionPropertyKey_NumberOfPendingFrames }
    }

    pub fn pixel_buffer_is_shared() -> &'static cf::String {
        unsafe { kVTCompressionPropertyKey_PixelBufferPoolIsShared }
    }

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
    pub fn max_key_frame_interval_duration() -> &'static cf::String {
        unsafe { kVTCompressionPropertyKey_MaxKeyFrameIntervalDuration }
    }

    /// Enables temporal compression.
    /// 
    /// True by default.  Set this to false to require key-frame-only compression.
    pub fn allow_temporal_compression() -> &'static cf::String {
        unsafe { kVTCompressionPropertyKey_AllowTemporalCompression }
    }

    pub fn allow_frame_reordering() -> &'static cf::String {
        unsafe { kVTCompressionPropertyKey_AllowFrameReordering }
    }

    pub fn allow_open_gop() -> &'static cf::String {
        unsafe { kVTCompressionPropertyKey_AllowOpenGOP }
    }

    pub fn avarage_bit_rate() -> &'static cf::String {
        unsafe { kVTCompressionPropertyKey_AverageBitRate }
    }

    pub fn data_rate_limits() -> &'static cf::String {
        unsafe { kVTCompressionPropertyKey_DataRateLimits }
    }

    pub fn quality() -> &'static cf::String {
        unsafe { kVTCompressionPropertyKey_Quality }
    }

    pub fn target_quality_for_alpha() -> &'static cf::String {
        unsafe { kVTCompressionPropertyKey_TargetQualityForAlpha }
    }

    pub fn more_frames_before_start() -> &'static cf::String {
        unsafe { kVTCompressionPropertyKey_MoreFramesBeforeStart }
    }

    pub fn more_frames_after_end() -> &'static cf::String {
        unsafe { kVTCompressionPropertyKey_MoreFramesAfterEnd }
    }

    pub fn prioritize_encoding_speed_over_quality() -> &'static cf::String {
        unsafe { kVTCompressionPropertyKey_PrioritizeEncodingSpeedOverQuality }
    }

    pub fn profile_level() -> &'static cf::String {
        unsafe { kVTCompressionPropertyKey_ProfileLevel }
    }

    pub fn output_bit_depth() -> &'static cf::String {
        unsafe { kVTCompressionPropertyKey_OutputBitDepth }
    }

    pub fn hdr_metadata_insertion_mode() -> &'static cf::String {
        unsafe { kVTCompressionPropertyKey_HDRMetadataInsertionMode }
    }

    pub fn h264_entropy_mode() -> &'static cf::String {
        unsafe { kVTCompressionPropertyKey_H264EntropyMode }
    }

    pub fn depth() -> &'static cf::String {
        unsafe { kVTCompressionPropertyKey_Depth }
    }

    pub fn max_frame_delay_count() -> &'static cf::String {
        unsafe { kVTCompressionPropertyKey_MaxFrameDelayCount }
    }

    pub fn max_h264_slice_bytes() -> &'static cf::String {
        unsafe { kVTCompressionPropertyKey_MaxH264SliceBytes }
    }

    pub fn real_time() -> &'static cf::String {
        unsafe { kVTCompressionPropertyKey_RealTime }
    }

    pub fn maximize_power_efficiecy() -> &'static cf::String {
        unsafe { kVTCompressionPropertyKey_MaximizePowerEfficiency }
    }

    /// Hint
    pub fn source_frame_count() -> &'static cf::String {
        unsafe { kVTCompressionPropertyKey_SourceFrameCount }
    }

    /// Hint
    pub fn expected_frame_rate() -> &'static cf::String {
        unsafe { kVTCompressionPropertyKey_ExpectedFrameRate }
    }

    pub fn base_layer_frame_rate_fraction() -> &'static cf::String {
        unsafe { kVTCompressionPropertyKey_BaseLayerFrameRateFraction }
    }

    pub fn base_layer_bit_rate_fraction() -> &'static cf::String {
        unsafe { kVTCompressionPropertyKey_BaseLayerBitRateFraction }
    }

    pub fn expected_duration() -> &'static cf::String {
        unsafe { kVTCompressionPropertyKey_ExpectedDuration }
    }

    pub fn base_layer_frame_rate() -> &'static cf::String {
        unsafe { kVTCompressionPropertyKey_BaseLayerFrameRate }
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
    }
}
