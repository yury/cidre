use crate::ns;

impl ns::ErrorDomain {
    /// The domain Swift synthesizes when bridging
    /// `MusicUnderstanding.MusicUnderstandingError`.
    #[doc(alias = "MusicUnderstandingError")]
    pub fn music_understanding() -> &'static Self {
        let domain = crate::nsstr!(c"MusicUnderstanding.MusicUnderstandingError");
        unsafe { std::mem::transmute(domain) }
    }
}

/// `MusicUnderstandingError` codes.
///
/// The framework's error is a native Swift enum rather than an `NSError`
/// subclass, so bridging gives a code that is the case's declaration index and
/// a localized description that is only the runtime's generic fallback text.
/// Matching on these codes is the way to tell the failures apart.
pub mod code {
    use crate::ns;

    #[doc(alias = "MusicUnderstandingError.sessionInProgress")]
    pub const SESSION_IN_PROGRESS: ns::Integer = 0;

    #[doc(alias = "MusicUnderstandingError.emptyAnalysisSet")]
    pub const EMPTY_ANALYSIS_SET: ns::Integer = 1;

    #[doc(alias = "MusicUnderstandingError.invalidAsset")]
    pub const INVALID_ASSET: ns::Integer = 2;

    #[doc(alias = "MusicUnderstandingError.hasProtectedContent")]
    pub const HAS_PROTECTED_CONTENT: ns::Integer = 3;

    #[doc(alias = "MusicUnderstandingError.internalError")]
    pub const INTERNAL_ERROR: ns::Integer = 4;
}

#[doc(alias = "MusicUnderstandingError")]
pub fn domain() -> &'static ns::ErrorDomain {
    ns::ErrorDomain::music_understanding()
}
