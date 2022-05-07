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

pub mod h264_entropy_mode {
    use crate::cf;

    #[inline]
    pub fn cavlc() -> &'static cf::String {
        unsafe { kVTH264EntropyMode_CAVLC }
    }

    #[inline]
    pub fn cabac() -> &'static cf::String {
        unsafe { kVTH264EntropyMode_CABAC }
    }

    extern "C" {
        static kVTH264EntropyMode_CAVLC: &'static cf::String;
        static kVTH264EntropyMode_CABAC: &'static cf::String;
    }
}

pub mod hdr_metadata_insertion_mode {
    use crate::cf;

    #[inline]
    pub fn none() -> &'static cf::String {
        unsafe { kVTHDRMetadataInsertionMode_None }
    }

    #[inline]
    pub fn auto() -> &'static cf::String {
        unsafe { kVTHDRMetadataInsertionMode_Auto }
    }

    extern "C" {
        static kVTHDRMetadataInsertionMode_None: &'static cf::String;
        static kVTHDRMetadataInsertionMode_Auto: &'static cf::String;
    }
}

pub mod profile_level {

    pub mod hevc {
        use crate::cf;
        pub fn main_auto_level() -> &'static cf::String {
            unsafe { kVTProfileLevel_HEVC_Main_AutoLevel }
        }

        pub fn main10_auto_level() -> &'static cf::String {
            unsafe { kVTProfileLevel_HEVC_Main10_AutoLevel }
        }

        pub fn main42210_auto_level() -> &'static cf::String {
            unsafe { kVTProfileLevel_HEVC_Main42210_AutoLevel }
        }
        extern "C" {
            static kVTProfileLevel_HEVC_Main_AutoLevel: &'static cf::String;
            static kVTProfileLevel_HEVC_Main10_AutoLevel: &'static cf::String;
            static kVTProfileLevel_HEVC_Main42210_AutoLevel: &'static cf::String;
        }
    }

    pub mod h264 {
        use crate::cf;

        pub fn baseline_1_3() -> &'static cf::String {
            unsafe { kVTProfileLevel_H264_Baseline_1_3 }
        }

        pub fn baseline_3_0() -> &'static cf::String {
            unsafe { kVTProfileLevel_H264_Baseline_3_0 }
        }

        pub fn baseline_3_1() -> &'static cf::String {
            unsafe { kVTProfileLevel_H264_Baseline_3_1 }
        }

        pub fn baseline_3_2() -> &'static cf::String {
            unsafe { kVTProfileLevel_H264_Baseline_3_2 }
        }

        pub fn baseline_4_0() -> &'static cf::String {
            unsafe { kVTProfileLevel_H264_Baseline_4_0 }
        }

        pub fn baseline_4_1() -> &'static cf::String {
            unsafe { kVTProfileLevel_H264_Baseline_4_1 }
        }

        pub fn baseline_4_2() -> &'static cf::String {
            unsafe { kVTProfileLevel_H264_Baseline_4_2 }
        }

        pub fn baseline_5_0() -> &'static cf::String {
            unsafe { kVTProfileLevel_H264_Baseline_5_0 }
        }

        pub fn baseline_5_1() -> &'static cf::String {
            unsafe { kVTProfileLevel_H264_Baseline_5_1 }
        }

        pub fn baseline_5_2() -> &'static cf::String {
            unsafe { kVTProfileLevel_H264_Baseline_5_2 }
        }

        pub fn baseline_auto_level() -> &'static cf::String {
            unsafe { kVTProfileLevel_H264_Baseline_AutoLevel }
        }

        pub fn consstrained_baseline_auto_level() -> &'static cf::String {
            unsafe { kVTProfileLevel_H264_ConstrainedBaseline_AutoLevel }
        }

        pub fn main_3_0() -> &'static cf::String {
            unsafe { kVTProfileLevel_H264_Main_3_0 }
        }

        pub fn main_3_1() -> &'static cf::String {
            unsafe { kVTProfileLevel_H264_Main_3_1 }
        }

        pub fn main_3_2() -> &'static cf::String {
            unsafe { kVTProfileLevel_H264_Main_3_2 }
        }

        pub fn main_4_0() -> &'static cf::String {
            unsafe { kVTProfileLevel_H264_Main_4_0 }
        }

        pub fn main_4_1() -> &'static cf::String {
            unsafe { kVTProfileLevel_H264_Main_4_1 }
        }

        pub fn main_4_2() -> &'static cf::String {
            unsafe { kVTProfileLevel_H264_Main_4_2 }
        }

        pub fn main_5_0() -> &'static cf::String {
            unsafe { kVTProfileLevel_H264_Main_5_0 }
        }

        pub fn main_5_1() -> &'static cf::String {
            unsafe { kVTProfileLevel_H264_Main_5_1 }
        }

        pub fn main_5_2() -> &'static cf::String {
            unsafe { kVTProfileLevel_H264_Main_5_2 }
        }

        pub fn main_auto_level() -> &'static cf::String {
            unsafe { kVTProfileLevel_H264_Main_AutoLevel }
        }

        pub fn extended_5_0() -> &'static cf::String {
            unsafe { kVTProfileLevel_H264_Extended_5_0 }
        }

        pub fn extended_auto_level() -> &'static cf::String {
            unsafe { kVTProfileLevel_H264_Extended_AutoLevel }
        }

        pub fn high_3_0() -> &'static cf::String {
            unsafe { kVTProfileLevel_H264_High_3_0 }
        }

        pub fn high_3_1() -> &'static cf::String {
            unsafe { kVTProfileLevel_H264_High_3_1 }
        }

        pub fn high_3_2() -> &'static cf::String {
            unsafe { kVTProfileLevel_H264_High_3_2 }
        }

        pub fn high_4_0() -> &'static cf::String {
            unsafe { kVTProfileLevel_H264_High_4_0 }
        }

        pub fn high_4_1() -> &'static cf::String {
            unsafe { kVTProfileLevel_H264_High_4_1 }
        }

        pub fn high_4_2() -> &'static cf::String {
            unsafe { kVTProfileLevel_H264_High_4_2 }
        }

        pub fn high_5_0() -> &'static cf::String {
            unsafe { kVTProfileLevel_H264_High_5_0 }
        }

        pub fn high_5_1() -> &'static cf::String {
            unsafe { kVTProfileLevel_H264_High_5_1 }
        }

        pub fn high_5_2() -> &'static cf::String {
            unsafe { kVTProfileLevel_H264_High_5_2 }
        }

        pub fn high_auto_level() -> &'static cf::String {
            unsafe { kVTProfileLevel_H264_High_AutoLevel }
        }

        pub fn constrained_high_auto_level() -> &'static cf::String {
            unsafe { kVTProfileLevel_H264_ConstrainedHigh_AutoLevel }
        }

        extern "C" {
            static kVTProfileLevel_H264_Baseline_1_3: &'static cf::String;
            static kVTProfileLevel_H264_Baseline_3_0: &'static cf::String;
            static kVTProfileLevel_H264_Baseline_3_1: &'static cf::String;
            static kVTProfileLevel_H264_Baseline_3_2: &'static cf::String;
            static kVTProfileLevel_H264_Baseline_4_0: &'static cf::String;
            static kVTProfileLevel_H264_Baseline_4_1: &'static cf::String;
            static kVTProfileLevel_H264_Baseline_4_2: &'static cf::String;
            static kVTProfileLevel_H264_Baseline_5_0: &'static cf::String;
            static kVTProfileLevel_H264_Baseline_5_1: &'static cf::String;
            static kVTProfileLevel_H264_Baseline_5_2: &'static cf::String;
            static kVTProfileLevel_H264_Baseline_AutoLevel: &'static cf::String;
            static kVTProfileLevel_H264_ConstrainedBaseline_AutoLevel: &'static cf::String;

            static kVTProfileLevel_H264_Main_3_0: &'static cf::String;
            static kVTProfileLevel_H264_Main_3_1: &'static cf::String;
            static kVTProfileLevel_H264_Main_3_2: &'static cf::String;
            static kVTProfileLevel_H264_Main_4_0: &'static cf::String;
            static kVTProfileLevel_H264_Main_4_1: &'static cf::String;
            static kVTProfileLevel_H264_Main_4_2: &'static cf::String;
            static kVTProfileLevel_H264_Main_5_0: &'static cf::String;
            static kVTProfileLevel_H264_Main_5_1: &'static cf::String;
            static kVTProfileLevel_H264_Main_5_2: &'static cf::String;
            static kVTProfileLevel_H264_Main_AutoLevel: &'static cf::String;
            static kVTProfileLevel_H264_Extended_5_0: &'static cf::String;
            static kVTProfileLevel_H264_Extended_AutoLevel: &'static cf::String;

            static kVTProfileLevel_H264_High_3_0: &'static cf::String;
            static kVTProfileLevel_H264_High_3_1: &'static cf::String;
            static kVTProfileLevel_H264_High_3_2: &'static cf::String;

            static kVTProfileLevel_H264_High_4_0: &'static cf::String;
            static kVTProfileLevel_H264_High_4_1: &'static cf::String;
            static kVTProfileLevel_H264_High_4_2: &'static cf::String;

            static kVTProfileLevel_H264_High_5_0: &'static cf::String;
            static kVTProfileLevel_H264_High_5_1: &'static cf::String;
            static kVTProfileLevel_H264_High_5_2: &'static cf::String;
            static kVTProfileLevel_H264_High_AutoLevel: &'static cf::String;
            static kVTProfileLevel_H264_ConstrainedHigh_AutoLevel: &'static cf::String;
        }
    }

    pub mod mp4v {
        use crate::cf;
        pub fn simple_l0() -> &'static cf::String {
            unsafe { kVTProfileLevel_MP4V_Simple_L0 }
        }

        pub fn simple_l1() -> &'static cf::String {
            unsafe { kVTProfileLevel_MP4V_Simple_L1 }
        }

        pub fn simple_l2() -> &'static cf::String {
            unsafe { kVTProfileLevel_MP4V_Simple_L2 }
        }

        pub fn simple_l3() -> &'static cf::String {
            unsafe { kVTProfileLevel_MP4V_Simple_L3 }
        }

        pub fn main_l2() -> &'static cf::String {
            unsafe { kVTProfileLevel_MP4V_Main_L2 }
        }

        pub fn main_l3() -> &'static cf::String {
            unsafe { kVTProfileLevel_MP4V_Main_L3 }
        }

        pub fn main_l4() -> &'static cf::String {
            unsafe { kVTProfileLevel_MP4V_Main_L4 }
        }

        pub fn advanced_simple_l0() -> &'static cf::String {
            unsafe { kVTProfileLevel_MP4V_AdvancedSimple_L0 }
        }

        pub fn advanced_simple_l1() -> &'static cf::String {
            unsafe { kVTProfileLevel_MP4V_AdvancedSimple_L1 }
        }

        pub fn advanced_simple_l2() -> &'static cf::String {
            unsafe { kVTProfileLevel_MP4V_AdvancedSimple_L2 }
        }
        pub fn advanced_simple_l3() -> &'static cf::String {
            unsafe { kVTProfileLevel_MP4V_AdvancedSimple_L3 }
        }

        pub fn advanced_simple_l4() -> &'static cf::String {
            unsafe { kVTProfileLevel_MP4V_AdvancedSimple_L4 }
        }

        extern "C" {
            static kVTProfileLevel_MP4V_Simple_L0: &'static cf::String;
            static kVTProfileLevel_MP4V_Simple_L1: &'static cf::String;
            static kVTProfileLevel_MP4V_Simple_L2: &'static cf::String;
            static kVTProfileLevel_MP4V_Simple_L3: &'static cf::String;

            static kVTProfileLevel_MP4V_Main_L2: &'static cf::String;
            static kVTProfileLevel_MP4V_Main_L3: &'static cf::String;
            static kVTProfileLevel_MP4V_Main_L4: &'static cf::String;

            static kVTProfileLevel_MP4V_AdvancedSimple_L0: &'static cf::String;
            static kVTProfileLevel_MP4V_AdvancedSimple_L1: &'static cf::String;
            static kVTProfileLevel_MP4V_AdvancedSimple_L2: &'static cf::String;
            static kVTProfileLevel_MP4V_AdvancedSimple_L3: &'static cf::String;
            static kVTProfileLevel_MP4V_AdvancedSimple_L4: &'static cf::String;
        }
    }

    pub mod h263 {
        use crate::cf;
        pub fn profile0_level_10() -> &'static cf::String {
            unsafe { kVTProfileLevel_H263_Profile0_Level10 }
        }

        pub fn profile0_level_45() -> &'static cf::String {
            unsafe { kVTProfileLevel_H263_Profile0_Level45 }
        }

        pub fn profile3_level_45() -> &'static cf::String {
            unsafe { kVTProfileLevel_H263_Profile3_Level45 }
        }
        extern "C" {
            static kVTProfileLevel_H263_Profile0_Level10: &'static cf::String;
            static kVTProfileLevel_H263_Profile0_Level45: &'static cf::String;
            static kVTProfileLevel_H263_Profile3_Level45: &'static cf::String;
        }
    }
}
