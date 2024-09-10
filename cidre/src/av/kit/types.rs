use crate::define_opts;

define_opts!(
    #[doc(alias = "AVVideoFrameAnalysisType")]
    pub VideoFrameAnalysisType(usize)
);

/// The types of items that AVPlayerViewController analyzes in a paused video frame.
impl VideoFrameAnalysisType {
    /// Defines no items to be analyzed by the analyzer.
    #[doc(alias = "AVVideoFrameAnalysisTypeNone")]
    pub const NONE: Self = Self(0);

    /// Defines the default set of analysis types.
    #[doc(alias = "AVVideoFrameAnalysisTypeDefault")]
    pub const DEFAULT: Self = Self(1 << 0);

    /// Text that appears in a paused video frame.
    #[doc(alias = "AVVideoFrameAnalysisTypeText")]
    pub const TEXT: Self = Self(1 << 1);

    /// Subject that the user can copy out of frame.
    #[doc(alias = "AVVideoFrameAnalysisTypeSubject")]
    pub const SUBJECT: Self = Self(1 << 2);

    /// Objects, landmarks, art, etc. recognized visually in a paused video frame.
    #[doc(alias = "AVVideoFrameAnalysisTypeVisualSearch")]
    pub const VISUAL_SEARCH: Self = Self(1 << 3);

    /// Machine-readable codes, such as QR codes, that appear in a paused video frame.
    #[doc(alias = "AVVideoFrameAnalysisTypeMachineReadableCode")]
    pub const READABLE_CODE: Self = Self(1 << 4);
}
