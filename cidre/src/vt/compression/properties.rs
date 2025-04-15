/// Recommendations for configuring VTCompressionSessions
///
/// These are recommendations for configuring VTCompressionSessions for some common scenarios.  
/// These are starting points; you may find that the details of your application necessitate further adjustments.
///
/// Video-conferencing, live capture, and live broadcast scenarios:
/// • kVTCompressionPropertyKey_RealTime: kCFBooleanTrue
/// • kVTCompressionPropertyKey_ExpectedFrameRate: set to real-time frame rate if possible
///
/// Offline transcode initiated by a user, who is waiting for the results:
/// • kVTCompressionPropertyKey_RealTime: kCFBooleanFalse
/// • kVTCompressionPropertyKey_MaximizePowerEfficiency: kCFBooleanFalse
///
/// Offline transcode in the background (when the user is not aware):
/// • kVTCompressionPropertyKey_RealTime: kCFBooleanFalse
/// • kVTCompressionPropertyKey_MaximizePowerEfficiency: kCFBooleanTrue
///
/// Ultra-low-latency capture / conferencing / cloud gaming (cases where every millisecond counts):
/// • kVTCompressionPropertyKey_RealTime: kCFBooleanTrue
/// • kVTCompressionPropertyKey_PrioritizeEncodingSpeedOverQuality: kCFBooleanTrue
pub mod keys {
    use crate::{api, cf};

    /// The number of pending frames in the compression session.
    ///
    /// This number may decrease asynchronously.
    #[doc(alias = "kVTCompressionPropertyKey_NumberOfPendingFrames")]
    #[inline]
    pub fn number_of_pending_frames() -> &'static cf::String {
        unsafe { kVTCompressionPropertyKey_NumberOfPendingFrames }
    }

    /// Indicates whether the a common pixel buffer pool is shared between
    /// the video encoder and the session client.
    ///
    /// This is false if separate pools are used because the video encoder's
    /// and the client's pixel buffer attributes were incompatible.
    #[doc(alias = "kVTCompressionPropertyKey_PixelBufferPoolIsShared")]
    #[inline]
    pub fn pixel_buf_pool_is_shared() -> &'static cf::String {
        unsafe { kVTCompressionPropertyKey_PixelBufferPoolIsShared }
    }

    /// The video encoder's pixel buffer attributes for the compression session.
    ///
    /// You can use these to create a pixel buffer pool for source pixel buffers.
    #[doc(alias = "kVTCompressionPropertyKey_VideoEncoderPixelBufferAttributes")]
    #[inline]
    pub fn video_encoder_pixel_buf_attrs() -> &'static cf::String {
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
    #[doc(alias = "kVTCompressionPropertyKey_MaxKeyFrameInterval")]
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
    #[doc(alias = "kVTCompressionPropertyKey_MaxKeyFrameIntervalDuration")]
    #[inline]
    pub fn max_key_frame_interval_duration() -> &'static cf::String {
        unsafe { kVTCompressionPropertyKey_MaxKeyFrameIntervalDuration }
    }

    /// Enables temporal compression.
    ///
    /// True by default.  Set this to false to require key-frame-only compression.
    #[doc(alias = "kVTCompressionPropertyKey_AllowTemporalCompression")]
    #[inline]
    pub fn allow_temporal_compression() -> &'static cf::String {
        unsafe { kVTCompressionPropertyKey_AllowTemporalCompression }
    }

    /// Enables frame reordering.
    ///
    /// In order to encode B frames, a video encoder must reorder frames,
    /// which means that the order in which they will be emitted and
    /// stored (the decode order) is different from the order in which
    /// they were presented to the video encoder (the display order).
    /// True by default. Set this to false to prevent frame reordering.
    #[doc(alias = "kVTCompressionPropertyKey_AllowFrameReordering")]
    #[inline]
    pub fn allow_frame_reordering() -> &'static cf::String {
        unsafe { kVTCompressionPropertyKey_AllowFrameReordering }
    }

    /// Enables Open GOP (Group Of Pictures) encoding.
    ///
    /// Only applicable to certain video encoders. For HEVC encoding the property
    /// value is kCFBooleanTrue by default - which means that the encoder may use
    /// Open GOP structures, but it is not guaranteed to. Set this to kCFBooleanFalse
    /// to prevent the encoder from using any form of Open GOP during encoding.
    /// If set to kCFBooleanFalse, this will mean that the encoder will generate groups
    /// of pictures that are independently decodable; there will be no frame dependencies
    /// across sync samples (frames where kCMSampleAttachmentKey_NotSync is absent or kCFBooleanFalse)
    /// When the value is kCFBooleanTrue, the encoder may use Open GOP frame dependency
    /// structures to increase compression efficiency or quality.
    #[doc(alias = "kVTCompressionPropertyKey_AllowOpenGOP")]
    #[inline]
    pub fn allow_open_gop() -> &'static cf::String {
        unsafe { kVTCompressionPropertyKey_AllowOpenGOP }
    }

    /// The long-term desired average bit rate in bits per second.
    ///
    /// This is not a hard limit; the bit rate may peak above this.
    /// The default bit rate is zero, which indicates that the video encoder
    /// should determine the size of compressed data.
    /// Note that bit rate settings only have an effect when timing
    /// information is provided for source frames, and that some codecs do
    /// not support limiting to specified bit rates.
    #[doc(alias = "kVTCompressionPropertyKey_AverageBitRate")]
    #[inline]
    pub fn avarage_bit_rate() -> &'static cf::String {
        unsafe { kVTCompressionPropertyKey_AverageBitRate }
    }

    /// Zero, one or two hard limits on data rate.
    ///
    /// Each hard limit is described by a data size in bytes and a
    /// duration in seconds, and requires that the total size of
    /// compressed data for any contiguous segment of that duration (in
    /// decode time) must not exceed the data size.
    /// By default, no data rate limits are set.
    /// The property is a CFArray of an even number of CFNumbers,
    /// alternating between bytes and seconds.
    /// Note that data rate settings only have an effect when timing
    /// information is provided for source frames, and that some codecs do
    /// not support limiting to specified data rates.
    #[doc(alias = "kVTCompressionPropertyKey_DataRateLimits")]
    #[inline]
    pub fn data_rate_limits() -> &'static cf::String {
        unsafe { kVTCompressionPropertyKey_DataRateLimits }
    }

    /// The desired compression quality.
    ///
    /// Some encoders, such as JPEG, describe the compression level of each
    /// image with a quality value.  This value should be specified as a
    /// number in the range of 0.0 to 1.0, where low = 0.25, normal = 0.50,
    /// high = 0.75, and 1.0 implies lossless compression for encoders that
    /// support it.
    #[doc(alias = "kVTCompressionPropertyKey_Quality")]
    #[inline]
    pub fn quality() -> &'static cf::String {
        unsafe { kVTCompressionPropertyKey_Quality }
    }

    #[doc(alias = "kVTCompressionPropertyKey_TargetQualityForAlpha")]
    #[inline]
    pub fn target_quality_for_alpha() -> &'static cf::String {
        unsafe { kVTCompressionPropertyKey_TargetQualityForAlpha }
    }

    #[doc(alias = "kVTCompressionPropertyKey_MoreFramesBeforeStart")]
    #[inline]
    pub fn more_frames_before_start() -> &'static cf::String {
        unsafe { kVTCompressionPropertyKey_MoreFramesBeforeStart }
    }

    #[doc(alias = "kVTCompressionPropertyKey_MoreFramesAfterEnd")]
    #[inline]
    pub fn more_frames_after_end() -> &'static cf::String {
        unsafe { kVTCompressionPropertyKey_MoreFramesAfterEnd }
    }

    /// Hint for the video encoder that it should maximize its speed during encode, sacrificing quality
    /// if needed
    ///
    /// Video encoders sometimes have a tradeoff available between encoding speed and quality at a given bitrate.
    /// For example, by spending more time refining encoding decisions, it may be possible to make marginal improvements on quality.
    /// This property lets a client indicate its preference for any such tradeoff.
    /// Clients may set this property to kCFBooleanTrue to indicate that
    /// the encoder can take steps to maximize its speed by reducing quality,
    /// or to kCFBooleanFalse to indicate that the priority should be maximizing quality (at a given bitrate).
    /// When the property value is NULL, the video encoder will choose its default behavior.
    /// H.264 and HEVC hardware video encoders prioritize quality over speed by default.
    /// ProRes hardware encoders currently prioritize speed over quality by default.
    /// Not all video encoders support this property.
    /// By default, this property is NULL.
    #[doc(alias = "kVTCompressionPropertyKey_PrioritizeEncodingSpeedOverQuality")]
    #[inline]
    pub fn prioritize_encoding_speed_over_quality() -> &'static cf::String {
        unsafe { kVTCompressionPropertyKey_PrioritizeEncodingSpeedOverQuality }
    }

    #[doc(alias = "kVTCompressionPropertyKey_ProfileLevel")]
    #[inline]
    pub fn profile_lvl() -> &'static cf::String {
        unsafe { kVTCompressionPropertyKey_ProfileLevel }
    }

    #[doc(alias = "kVTCompressionPropertyKey_OutputBitDepth")]
    #[inline]
    pub fn output_bit_depth() -> &'static cf::String {
        unsafe { kVTCompressionPropertyKey_OutputBitDepth }
    }

    #[doc(alias = "kVTCompressionPropertyKey_HDRMetadataInsertionMode")]
    #[inline]
    pub fn hdr_metadata_insertion_mode() -> &'static cf::String {
        unsafe { kVTCompressionPropertyKey_HDRMetadataInsertionMode }
    }

    #[doc(alias = "kVTCompressionPropertyKey_H264EntropyMode")]
    #[inline]
    pub fn h264_entropy_mode() -> &'static cf::String {
        unsafe { kVTCompressionPropertyKey_H264EntropyMode }
    }

    #[doc(alias = "kVTCompressionPropertyKey_Depth")]
    #[inline]
    pub fn depth() -> &'static cf::String {
        unsafe { kVTCompressionPropertyKey_Depth }
    }

    /// The maximum frame delay count is the maximum number of frames that
    /// a compressor is allowed to hold before it must output a compressed
    /// frame.
    ///
    /// It limits the number of frames that may be held in the
    /// "compression window". If the maximum frame delay count is M, then
    /// before the call to encode frame N returns, frame N-M must have
    /// been emitted.
    ///
    /// The default is UNLIMITED_FRAME_DELAY_COUNT, which sets no limit
    /// on the compression window.
    #[doc(alias = "kVTCompressionPropertyKey_MaxFrameDelayCount")]
    #[inline]
    pub fn max_frame_delay_count() -> &'static cf::String {
        unsafe { kVTCompressionPropertyKey_MaxFrameDelayCount }
    }

    #[doc(alias = "kVTCompressionPropertyKey_MaxH264SliceBytes")]
    #[inline]
    pub fn max_h264_slice_bytes() -> &'static cf::String {
        unsafe { kVTCompressionPropertyKey_MaxH264SliceBytes }
    }

    /// Hints the video encoder that compression is, or is not, being performed in real time.
    ///
    /// For offline compression, clients are advised to set this property to kCFBooleanFalse.
    /// This indicates that the timeline of the frames is not connected to wall-clock time at all.
    /// This allows the video encoder to run either slower than real-time or faster than real-time,
    /// depending upon the situation and device capabilities.
    /// For real-time compression, clients are advised to set this property to kCFBooleanTrue.
    /// This indicates that the timeline of frames is connected to real time (so, for example,
    /// the client expects to encode one second of frames each second, on average).
    /// In such a case, it is important that encoding keep up, but it is also not valuable to encode
    /// substantially faster than real time.  
    /// When this property is set to kCFBooleanTrue, the video encoder may use the value of the
    /// kVTCompressionPropertyKey_ExpectedFrameRate property to optimize energy usage,
    /// by anticipating that encoding requests will happen at this rate.
    /// By default, this property is NULL, indicating unknown.  
    #[doc(alias = "kVTCompressionPropertyKey_RealTime")]
    #[inline]
    pub fn real_time() -> &'static cf::String {
        unsafe { kVTCompressionPropertyKey_RealTime }
    }

    /// Hints to the video encoder that it should maximize power efficiency during encode.
    ///
    /// For compression where the client is operating in the background, clients may set this property to kCFBooleanTrue, which indicates that
    /// the encoder can take steps to minimize impact on power usage and other system activity.
    /// Setting the property to NULL is equivalent to setting it to kCFBooleanFalse.
    /// Not all video encoders support this property.
    /// By default, this property is NULL.
    /// If the kVTCompressionPropertyKey_RealTime property is set to kCFBooleanTrue,
    /// the video encoder may act as though this property were set to kCFBooleanFalse.
    #[doc(alias = "kVTCompressionPropertyKey_MaximizePowerEfficiency")]
    #[inline]
    pub fn maximize_power_efficiecy() -> &'static cf::String {
        unsafe { kVTCompressionPropertyKey_MaximizePowerEfficiency }
    }

    #[doc(alias = "kVTCompressionPropertyKey_SourceFrameCount")]
    #[inline]
    pub fn source_frame_count() -> &'static cf::String {
        unsafe { kVTCompressionPropertyKey_SourceFrameCount }
    }

    #[doc(alias = "kVTCompressionPropertyKey_ExpectedFrameRate")]
    #[inline]
    pub fn expected_frame_rate() -> &'static cf::String {
        unsafe { kVTCompressionPropertyKey_ExpectedFrameRate }
    }

    /// Indicates the maximum real time rate at which frames could be submitted to the VTCompressionSession
    /// CFNumber/Optional
    ///
    /// The frame rate is measured in frames per second.
    /// This property can be used to inform the encoder of the maximum rate that frames could be submitted to the encoder during realtime
    /// encoding.  This allows the encoder to configure itself to ensure this capability.
    /// This property can only be used when kVTCompressionPropertyKey_RealTime has been set to true.
    /// Unlike kVTCompressionPropertyKey_ExpectedFrameRate, this property informs the maximum possible rate that the
    /// VTCompressionSession could see, not the average frame rate that is expected in normal operation.
    /// By default, the property has a value of zero indicating "unknown".
    #[doc(alias = "kVTCompressionPropertyKey_MaximumRealTimeFrameRate")]
    #[api::available(macos = 15.0, ios = 18.0, tvos = 18.0, visionos = 2.0)]
    #[inline]
    pub fn max_real_time_frame_rate() -> &'static cf::String {
        unsafe { kVTCompressionPropertyKey_MaximumRealTimeFrameRate }
    }

    /// An estimate of the expected size in bytes of a single encoded frame based
    /// on the current configuration.
    #[doc(alias = "kVTCompressionPropertyKey_EstimatedAverageBytesPerFrame")]
    #[inline]
    pub fn estimated_average_bytes_per_frame() -> &'static cf::String {
        unsafe { kVTCompressionPropertyKey_EstimatedAverageBytesPerFrame }
    }

    #[doc(alias = "kVTCompressionPropertyKey_BaseLayerFrameRateFraction")]
    #[inline]
    pub fn base_layer_frame_rate_fraction() -> &'static cf::String {
        unsafe { kVTCompressionPropertyKey_BaseLayerFrameRateFraction }
    }

    #[doc(alias = "kVTCompressionPropertyKey_BaseLayerBitRateFraction")]
    #[inline]
    pub fn base_layer_bit_rate_fraction() -> &'static cf::String {
        unsafe { kVTCompressionPropertyKey_BaseLayerBitRateFraction }
    }

    #[doc(alias = "kVTCompressionPropertyKey_ExpectedDuration")]
    #[inline]
    pub fn expected_duration() -> &'static cf::String {
        unsafe { kVTCompressionPropertyKey_ExpectedDuration }
    }

    #[doc(alias = "kVTCompressionPropertyKey_BaseLayerFrameRate")]
    #[inline]
    pub fn base_layer_frame_rate() -> &'static cf::String {
        unsafe { kVTCompressionPropertyKey_BaseLayerFrameRate }
    }

    #[doc(alias = "kVTCompressionPropertyKey_SupportsBaseFrameQP")]
    #[inline]
    pub fn support_base_frame_qp() -> &'static cf::String {
        unsafe { kVTCompressionPropertyKey_SupportsBaseFrameQP }
    }

    #[doc(alias = "kVTCompressionPropertyKey_CleanAperture")]
    #[inline]
    pub fn clean_aperture() -> &'static cf::String {
        unsafe { kVTCompressionPropertyKey_CleanAperture }
    }

    /// Describes the pixel aspect ratio for encoded frames.
    ///
    /// Some video encoders may enforce specific pixel aspect ratios;
    /// in those cases this property will be read-only (SetProperty will return kVTPropertyReadOnlyErr).
    /// The pixel aspect ratio will be set on the format description for output samples,
    /// and may affect source frame scaling.
    /// None is a valid value for this property, meaning square pixels (1:1).
    #[doc(alias = "kVTCompressionPropertyKey_PixelAspectRatio")]
    #[inline]
    pub fn pixel_aspect_ratio() -> &'static cf::String {
        unsafe { kVTCompressionPropertyKey_PixelAspectRatio }
    }

    #[doc(alias = "kVTCompressionPropertyKey_FieldCount")]
    #[inline]
    pub fn field_count() -> &'static cf::String {
        unsafe { kVTCompressionPropertyKey_FieldCount }
    }

    #[doc(alias = "kVTCompressionPropertyKey_FieldDetail")]
    #[inline]
    pub fn field_detail() -> &'static cf::String {
        unsafe { kVTCompressionPropertyKey_FieldDetail }
    }

    #[doc(alias = "kVTCompressionPropertyKey_AspectRatio16x9")]
    #[inline]
    pub fn aspect_ratio_16x9() -> &'static cf::String {
        unsafe { kVTCompressionPropertyKey_AspectRatio16x9 }
    }

    #[doc(alias = "kVTCompressionPropertyKey_ProgressiveScan")]
    #[inline]
    pub fn progressive_scan() -> &'static cf::String {
        unsafe { kVTCompressionPropertyKey_ProgressiveScan }
    }

    #[doc(alias = "kVTCompressionPropertyKey_ColorPrimaries")]
    #[inline]
    pub fn color_primaries() -> &'static cf::String {
        unsafe { kVTCompressionPropertyKey_ColorPrimaries }
    }

    #[doc(alias = "kVTCompressionPropertyKey_TransferFunction")]
    #[inline]
    pub fn transfer_fn() -> &'static cf::String {
        unsafe { kVTCompressionPropertyKey_TransferFunction }
    }

    #[doc(alias = "kVTCompressionPropertyKey_YCbCrMatrix")]
    #[inline]
    pub fn ycbcr_matrix() -> &'static cf::String {
        unsafe { kVTCompressionPropertyKey_YCbCrMatrix }
    }

    #[doc(alias = "kVTCompressionPropertyKey_ICCProfile")]
    #[inline]
    pub fn icc_profile() -> &'static cf::String {
        unsafe { kVTCompressionPropertyKey_ICCProfile }
    }

    #[doc(alias = "kVTCompressionPropertyKey_MasteringDisplayColorVolume")]
    #[inline]
    pub fn master_display_color_volume() -> &'static cf::String {
        unsafe { kVTCompressionPropertyKey_MasteringDisplayColorVolume }
    }

    #[doc(alias = "kVTCompressionPropertyKey_ContentLightLevelInfo")]
    #[inline]
    pub fn content_light_lvl_info() -> &'static cf::String {
        unsafe { kVTCompressionPropertyKey_ContentLightLevelInfo }
    }

    #[doc(alias = "kVTCompressionPropertyKey_GammaLevel")]
    #[inline]
    pub fn gamma_lvl() -> &'static cf::String {
        unsafe { kVTCompressionPropertyKey_GammaLevel }
    }

    #[doc(alias = "kVTCompressionPropertyKey_AlphaChannelMode")]
    #[inline]
    pub fn alpha_channel_mode() -> &'static cf::String {
        unsafe { kVTCompressionPropertyKey_AlphaChannelMode }
    }

    #[doc(alias = "kVTCompressionPropertyKey_PixelTransferProperties")]
    #[inline]
    pub fn pixel_transfer_properties() -> &'static cf::String {
        unsafe { kVTCompressionPropertyKey_PixelTransferProperties }
    }

    #[doc(alias = "kVTCompressionPropertyKey_MultiPassStorage")]
    #[inline]
    pub fn multi_pass_storage() -> &'static cf::String {
        unsafe { kVTCompressionPropertyKey_MultiPassStorage }
    }

    #[doc(alias = "kVTCompressionPropertyKey_EncoderID")]
    #[inline]
    pub fn encoder_id() -> &'static cf::String {
        unsafe { kVTCompressionPropertyKey_EncoderID }
    }

    #[doc(alias = "kVTCompressionPropertyKey_PreserveDynamicHDRMetadata")]
    #[inline]
    pub fn preserve_dynamic_hdr_metadata() -> &'static cf::String {
        unsafe { kVTCompressionPropertyKey_PreserveDynamicHDRMetadata }
    }

    /// Specifies the minimum allowed encoded frame QP (Quantization Parameter).
    ///
    /// This is an optional parameter. Use it only when you have a specific requirement for the video quality and you are
    /// familiar with frame QP.
    /// This is not supported in all encoders or in all encoder operating modes. kVTPropertyNotSupportedErr will be
    /// returned when this option is not supported.
    #[doc(alias = "kVTCompressionPropertyKey_MaxAllowedFrameQP")]
    #[inline]
    pub fn max_allowed_frame_qp() -> &'static cf::String {
        unsafe { kVTCompressionPropertyKey_MaxAllowedFrameQP }
    }

    /// Enable Long Term Reference (LTR) frames during encoding
    ///
    /// When an LTR frame is encoded, encoder will signal a unique token of the LTR frame in the encoder callback through:
    /// kVTSampleAttachmentKey_RequireLTRAcknowledgementToken
    /// Clients are responsible for reporting acknowledged LTR frames to the encoder through:
    /// - kVTEncodeFrameOptionKey_AcknowledgedLTRTokens
    ///    Client can request a refresh frame at any time through:
    /// - kVTEncodeFrameOptionKey_ForceLTRRefresh
    ///   Encoder will encode a P frame by using one of acknowledged LTR frames
    /// as the reference. Encoder will encode a new reference frame using an acknowledged LTR,
    /// or an IDR if no LTR frames have been acknowledged.
    #[doc(alias = "kVTCompressionPropertyKey_EnableLTR")]
    #[inline]
    pub fn enable_ltr() -> &'static cf::String {
        unsafe { kVTCompressionPropertyKey_EnableLTR }
    }

    /// Requires that the encoder use a Constant Bit Rate algorithm.
    ///
    /// The property kVTCompressionPropertyKey_ExpectedFrameRate should be set
    /// along with kVTCompressionPropertyKey_ConstantBitRate
    /// to ensure effective CBR rate control.
    ///
    /// This property is not compatible with kVTCompressionPropertyKey_DataRateLimits and
    /// kVTCompressionPropertyKey_AverageBitRate.
    ///
    /// The encoder will pad the frame if they are smaller than they need to be based on the Constant BitRate. This
    /// property is not recommended for general streaming or export scenarios. It is intended for interoperability with
    /// stremaing CDNs which specifically require that data rates not drop even during low motion and activity scenes.
    /// This is not supported in all encoders or in all encoder operating modes. kVTPropertyNotSupportedErr will be
    /// returned when this option is not supported.
    #[doc(alias = "kVTCompressionPropertyKey_ConstantBitRate")]
    #[inline]
    pub fn constant_bit_rate() -> &'static cf::String {
        unsafe { kVTCompressionPropertyKey_ConstantBitRate }
    }

    /// Read/write, cf::Number, Optional
    ///
    /// Requests that the encoder retain the specified number of frames during encoding. These frames will be used for additional analysis and statistics
    /// gathering before the frame is finally encoded at the end of the window. When this property is not set, video encoder will automatically determine
    /// the number of lookahead frames.
    ///
    /// Encoder will choose number of lookahead frames closer to the suggested value based on internal configuration. This property directly affects latency
    /// of the video encoder. The following properties also affect look ahead frames:
    ///
    /// 1. Value of this property must be less than or equal to `kVTCompressionPropertyKey_MaxFrameDelayCount`.
    /// 2. This property is ignored when `VTVideoEncoderSpecification_EnableLowLatencyRateControl` is set to true
    /// 3. This property is ignored when `kVTCompressionPropertyKey_Quality` is set to 1.0
    /// 4. This property can not be used in conjunction with multi-pass feature (`kVTCompressionPropertyKey_MultiPassStorage`)
    #[doc(alias = "kVTCompressionPropertyKey_SuggestedLookAheadFrameCount")]
    #[inline]
    #[api::available(macos = 15.0)]
    pub fn suggested_look_ahead_frame_count() -> &'static cf::String {
        unsafe { kVTCompressionPropertyKey_SuggestedLookAheadFrameCount }
    }

    /// Read/write, cf::Number, Optional
    /// Control spatial adaptation of the quantization parameter (QP) based on per-frame statistics.
    ///
    /// If set to `qp_modulation_level::disable()`, spatial QP adaptation is not applied based on per-frame statistics.
    /// If set to `qp_modulation_level::default()`, video encoder is allowed to apply spatial QP adaptation for each macro block (or coding unit) within a video frame.
    /// QP adaptation is based on spatial characteristics of a frame and the level of spatial QP adaptation is decided internally by the rate controller.
    #[doc(alias = "kVTCompressionPropertyKey_SpatialAdaptiveQPLevel")]
    #[inline]
    #[api::available(macos = 15.0)]
    pub fn spatial_adaptive_qp_level() -> &'static cf::String {
        unsafe { kVTCompressionPropertyKey_SpatialAdaptiveQPLevel }
    }

    #[link(name = "VideoToolbox", kind = "framework")]
    #[api::weak]
    unsafe extern "C" {
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

        #[api::available(macos = 15.0, ios = 18.0, tvos = 18.0, visionos = 2.0)]
        static kVTCompressionPropertyKey_MaximumRealTimeFrameRate: &'static cf::String;
        static kVTCompressionPropertyKey_EstimatedAverageBytesPerFrame: &'static cf::String;
        static kVTCompressionPropertyKey_ConstantBitRate: &'static cf::String;
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

        #[api::available(macos = 15.0)]
        static kVTCompressionPropertyKey_SuggestedLookAheadFrameCount: &'static cf::String;

        #[api::available(macos = 15.0)]
        static kVTCompressionPropertyKey_SpatialAdaptiveQPLevel: &'static cf::String;
    }
}

pub mod qp_modulation_level {
    use crate::cf;

    #[doc(alias = "kVTQPModulationLevel_Default")]
    pub fn default() -> &'static cf::Number {
        cf::Number::tagged_i32(-1)
    }

    #[doc(alias = "kVTQPModulationLevel_Disable")]
    pub fn disabled() -> &'static cf::Number {
        cf::Number::tagged_i32(0)
    }
}

#[doc(alias = "kVTUnlimitedFrameDelayCount")]
pub const UNLIMITED_FRAME_DELAY_COUNT: i32 = -1;

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

    #[link(name = "VideoToolbox", kind = "framework")]
    unsafe extern "C" {
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

    #[link(name = "VideoToolbox", kind = "framework")]
    unsafe extern "C" {
        static kVTHDRMetadataInsertionMode_None: &'static cf::String;
        static kVTHDRMetadataInsertionMode_Auto: &'static cf::String;
    }
}

pub mod profile_level {

    pub mod hevc {
        use crate::cf;
        pub fn main_auto_lvl() -> &'static cf::String {
            unsafe { kVTProfileLevel_HEVC_Main_AutoLevel }
        }

        pub fn main10_auto_lvl() -> &'static cf::String {
            unsafe { kVTProfileLevel_HEVC_Main10_AutoLevel }
        }

        pub fn main42210_auto_lvl() -> &'static cf::String {
            unsafe { kVTProfileLevel_HEVC_Main42210_AutoLevel }
        }

        #[link(name = "VideoToolbox", kind = "framework")]
        unsafe extern "C" {
            static kVTProfileLevel_HEVC_Main_AutoLevel: &'static cf::String;
            static kVTProfileLevel_HEVC_Main10_AutoLevel: &'static cf::String;
            static kVTProfileLevel_HEVC_Main42210_AutoLevel: &'static cf::String;
        }
    }

    pub mod h264 {
        use crate::cf;

        #[doc(alias = "kVTProfileLevel_H264_Baseline_1_3")]
        pub fn baseline_1_3() -> &'static cf::String {
            unsafe { kVTProfileLevel_H264_Baseline_1_3 }
        }

        #[doc(alias = "kVTProfileLevel_H264_Baseline_3_0")]
        pub fn baseline_3_0() -> &'static cf::String {
            unsafe { kVTProfileLevel_H264_Baseline_3_0 }
        }

        #[doc(alias = "kVTProfileLevel_H264_Baseline_3_1")]
        pub fn baseline_3_1() -> &'static cf::String {
            unsafe { kVTProfileLevel_H264_Baseline_3_1 }
        }

        #[doc(alias = "kVTProfileLevel_H264_Baseline_3_2")]
        pub fn baseline_3_2() -> &'static cf::String {
            unsafe { kVTProfileLevel_H264_Baseline_3_2 }
        }

        #[doc(alias = "kVTProfileLevel_H264_Baseline_4_0")]
        pub fn baseline_4_0() -> &'static cf::String {
            unsafe { kVTProfileLevel_H264_Baseline_4_0 }
        }

        #[doc(alias = "kVTProfileLevel_H264_Baseline_4_1")]
        pub fn baseline_4_1() -> &'static cf::String {
            unsafe { kVTProfileLevel_H264_Baseline_4_1 }
        }

        #[doc(alias = "kVTProfileLevel_H264_Baseline_4_2")]
        pub fn baseline_4_2() -> &'static cf::String {
            unsafe { kVTProfileLevel_H264_Baseline_4_2 }
        }

        #[doc(alias = "kVTProfileLevel_H264_Baseline_5_0")]
        pub fn baseline_5_0() -> &'static cf::String {
            unsafe { kVTProfileLevel_H264_Baseline_5_0 }
        }

        #[doc(alias = "kVTProfileLevel_H264_Baseline_5_1")]
        pub fn baseline_5_1() -> &'static cf::String {
            unsafe { kVTProfileLevel_H264_Baseline_5_1 }
        }

        #[doc(alias = "kVTProfileLevel_H264_Baseline_5_2")]
        pub fn baseline_5_2() -> &'static cf::String {
            unsafe { kVTProfileLevel_H264_Baseline_5_2 }
        }

        #[doc(alias = "kVTProfileLevel_H264_Baseline_AutoLevel")]
        pub fn baseline_auto_lvl() -> &'static cf::String {
            unsafe { kVTProfileLevel_H264_Baseline_AutoLevel }
        }

        #[doc(alias = "kVTProfileLevel_H264_ConstrainedBaseline_AutoLevel")]
        pub fn constrained_baseline_auto_lvl() -> &'static cf::String {
            unsafe { kVTProfileLevel_H264_ConstrainedBaseline_AutoLevel }
        }

        #[doc(alias = "kVTProfileLevel_H264_Main_3_0")]
        pub fn main_3_0() -> &'static cf::String {
            unsafe { kVTProfileLevel_H264_Main_3_0 }
        }

        #[doc(alias = "kVTProfileLevel_H264_Main_3_1")]
        pub fn main_3_1() -> &'static cf::String {
            unsafe { kVTProfileLevel_H264_Main_3_1 }
        }

        #[doc(alias = "kVTProfileLevel_H264_Main_3_2")]
        pub fn main_3_2() -> &'static cf::String {
            unsafe { kVTProfileLevel_H264_Main_3_2 }
        }

        #[doc(alias = "kVTProfileLevel_H264_Main_4_0")]
        pub fn main_4_0() -> &'static cf::String {
            unsafe { kVTProfileLevel_H264_Main_4_0 }
        }

        #[doc(alias = "kVTProfileLevel_H264_Main_4_1")]
        pub fn main_4_1() -> &'static cf::String {
            unsafe { kVTProfileLevel_H264_Main_4_1 }
        }

        #[doc(alias = "kVTProfileLevel_H264_Main_4_2")]
        pub fn main_4_2() -> &'static cf::String {
            unsafe { kVTProfileLevel_H264_Main_4_2 }
        }

        #[doc(alias = "kVTProfileLevel_H264_Main_5_0")]
        pub fn main_5_0() -> &'static cf::String {
            unsafe { kVTProfileLevel_H264_Main_5_0 }
        }

        #[doc(alias = "kVTProfileLevel_H264_Main_5_1")]
        pub fn main_5_1() -> &'static cf::String {
            unsafe { kVTProfileLevel_H264_Main_5_1 }
        }

        #[doc(alias = "kVTProfileLevel_H264_Main_5_2")]
        pub fn main_5_2() -> &'static cf::String {
            unsafe { kVTProfileLevel_H264_Main_5_2 }
        }

        #[doc(alias = "kVTProfileLevel_H264_Main_AutoLevel")]
        pub fn main_auto_lvl() -> &'static cf::String {
            unsafe { kVTProfileLevel_H264_Main_AutoLevel }
        }

        #[doc(alias = "kVTProfileLevel_H264_Extended_5_0")]
        pub fn extended_5_0() -> &'static cf::String {
            unsafe { kVTProfileLevel_H264_Extended_5_0 }
        }

        #[doc(alias = "kVTProfileLevel_H264_Extended_AutoLevel")]
        pub fn extended_auto_lvl() -> &'static cf::String {
            unsafe { kVTProfileLevel_H264_Extended_AutoLevel }
        }

        #[doc(alias = "kVTProfileLevel_H264_High_3_0")]
        pub fn high_3_0() -> &'static cf::String {
            unsafe { kVTProfileLevel_H264_High_3_0 }
        }

        #[doc(alias = "kVTProfileLevel_H264_High_3_1")]
        pub fn high_3_1() -> &'static cf::String {
            unsafe { kVTProfileLevel_H264_High_3_1 }
        }

        #[doc(alias = "kVTProfileLevel_H264_High_3_2")]
        pub fn high_3_2() -> &'static cf::String {
            unsafe { kVTProfileLevel_H264_High_3_2 }
        }

        #[doc(alias = "kVTProfileLevel_H264_High_4_0")]
        pub fn high_4_0() -> &'static cf::String {
            unsafe { kVTProfileLevel_H264_High_4_0 }
        }

        #[doc(alias = "kVTProfileLevel_H264_High_4_1")]
        pub fn high_4_1() -> &'static cf::String {
            unsafe { kVTProfileLevel_H264_High_4_1 }
        }

        #[doc(alias = "kVTProfileLevel_H264_High_4_2")]
        pub fn high_4_2() -> &'static cf::String {
            unsafe { kVTProfileLevel_H264_High_4_2 }
        }

        #[doc(alias = "kVTProfileLevel_H264_High_5_0")]
        pub fn high_5_0() -> &'static cf::String {
            unsafe { kVTProfileLevel_H264_High_5_0 }
        }

        #[doc(alias = "kVTProfileLevel_H264_High_5_1")]
        pub fn high_5_1() -> &'static cf::String {
            unsafe { kVTProfileLevel_H264_High_5_1 }
        }

        #[doc(alias = "kVTProfileLevel_H264_High_5_2")]
        pub fn high_5_2() -> &'static cf::String {
            unsafe { kVTProfileLevel_H264_High_5_2 }
        }

        #[doc(alias = "kVTProfileLevel_H264_High_AutoLevel")]
        pub fn high_auto_lvl() -> &'static cf::String {
            unsafe { kVTProfileLevel_H264_High_AutoLevel }
        }

        #[doc(alias = "kVTProfileLevel_H264_ConstrainedHigh_AutoLevel")]
        pub fn constrained_high_auto_lvl() -> &'static cf::String {
            unsafe { kVTProfileLevel_H264_ConstrainedHigh_AutoLevel }
        }

        #[link(name = "VideoToolbox", kind = "framework")]
        unsafe extern "C" {
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

        #[link(name = "VideoToolbox", kind = "framework")]
        unsafe extern "C" {
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

        pub fn profile0_lvl_10() -> &'static cf::String {
            unsafe { kVTProfileLevel_H263_Profile0_Level10 }
        }

        pub fn profile0_lvl_45() -> &'static cf::String {
            unsafe { kVTProfileLevel_H263_Profile0_Level45 }
        }

        pub fn profile3_lvl_45() -> &'static cf::String {
            unsafe { kVTProfileLevel_H263_Profile3_Level45 }
        }

        #[link(name = "VideoToolbox", kind = "framework")]
        unsafe extern "C" {
            static kVTProfileLevel_H263_Profile0_Level10: &'static cf::String;
            static kVTProfileLevel_H263_Profile0_Level45: &'static cf::String;
            static kVTProfileLevel_H263_Profile3_Level45: &'static cf::String;
        }
    }
}

/// Per-Frame Configuration
pub mod frame_keys {
    use crate::cf;

    /// [`cf::Boolean`] value indicating whether the current frame is forced to be a key frame.
    pub fn force_key_frame() -> &'static cf::String {
        unsafe { kVTEncodeFrameOptionKey_ForceKeyFrame }
    }

    pub fn base_frame_qp() -> &'static cf::String {
        unsafe { kVTEncodeFrameOptionKey_BaseFrameQP }
    }

    #[link(name = "VideoToolbox", kind = "framework")]
    unsafe extern "C" {
        static kVTEncodeFrameOptionKey_ForceKeyFrame: &'static cf::String;
        static kVTEncodeFrameOptionKey_BaseFrameQP: &'static cf::String;
    }
}
