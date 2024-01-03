use crate::{arc, define_cls, define_obj_type, define_options, ns, objc};

define_options!(pub Opts(usize));

impl Opts {
    /// Match letters in the pattern independent of case.
    #[doc(alias = "NSRegularExpressionCaseInsensitive")]
    pub const CASE_INSESITIVE: Self = Self(1 << 0);

    /// Ignore whitespace and #-prefixed comments in the pattern.
    #[doc(alias = "NSRegularExpressionAllowCommentsAndWhitespace")]
    pub const ALLOW_COMMENTS_AND_WHITESPACE: Self = Self(1 << 1);

    /// Treat the entire pattern as a literal string.
    #[doc(alias = "NSRegularExpressionIgnoreMetacharacters")]
    pub const IGNORE_METACHARACTERS: Self = Self(1 << 2);

    /// Allow . to match any character, including line separators.
    #[doc(alias = "NSRegularExpressionDotMatchesLineSeparators")]
    pub const DOT_MATCHES_LINE_SEPARATORS: Self = Self(1 << 3);

    /// Allow ^ and $ to match the start and end of lines.
    #[doc(alias = "NSRegularExpressionAnchorsMatchLines")]
    pub const ANCHORS_MATCH_LINES: Self = Self(1 << 4);

    /// Treat only \n as a line separator (otherwise, all standard line separators are used).
    #[doc(alias = "NSRegularExpressionUseUnixLineSeparators")]
    pub const USE_UNIX_LINE_SEPARATORS: Self = Self(1 << 5);

    /// Use Unicode TR#29 to specify word boundaries
    /// (otherwise, traditional regular expression word boundaries are used).
    #[doc(alias = "NSRegularExpressionUseUnicodeWordBoundaries")]
    pub const USE_UNICODE_WORD_BOUNDARIES: Self = Self(1 << 6);
}

define_options!(
    #[doc(alias = "NSMatchingOptions")]
    pub MatchOpts(usize)
);

impl MatchOpts {
    /// Call the block periodically during long-running match operations.
    #[doc(alias = "NSMatchingReportProgress")]
    pub const REPORT_PROGRESS: Self = Self(1 << 0);

    /// Call the block once after the completion of any matching.
    #[doc(alias = "NSMatchingReportCompletion")]
    pub const REPORT_COMPLETION: Self = Self(1 << 1);

    /// Limit matches to those at the start of the search range.
    #[doc(alias = "NSMatchingAnchored")]
    pub const ANCHORED: Self = Self(1 << 2);

    /// Allow matching to look beyond the bounds of the search range.
    #[doc(alias = "NSMatchingWithTransparentBounds")]
    pub const WITH_TRANSPARENT_BOUNDS: Self = Self(1 << 3);

    /// Prevent ^ and $ from automatically matching the beginning and end of the search range.
    #[doc(alias = "NSMatchingWithoutAnchoringBounds")]
    pub const ANCHORING_BOUNDS: Self = Self(1 << 4);
}

define_options!(
    #[doc(alias = "NSMatchingFlags")]
    pub MatchFlags(usize)
);

impl MatchFlags {
    /// Set when the block is called to report progress during a long-running match operation.
    #[doc(alias = "NSMatchingProgress")]
    pub const PROGRESS: Self = Self(1 << 0);

    /// Set when the block is called after completion of any matching.
    #[doc(alias = "NSMatchingCompleted")]
    pub const COMPLETED: Self = Self(1 << 1);

    /// Set when the current match operation reached the end of the search range.
    #[doc(alias = "NSMatchingHitEnd")]
    pub const HIT_END: Self = Self(1 << 2);

    /// Set when the current match depended on the location of the end of the search range.
    #[doc(alias = "NSMatchingRequiredEnd")]
    pub const REQUIRED_END: Self = Self(1 << 3);

    /// Set when matching failed due to an internal error.
    #[doc(alias = "NSMatchingInternalError")]
    pub const INTERNAL_ERROR: Self = Self(1 << 4);
}

define_obj_type!(
    #[doc(alias = "NSRegularExpression")]
    pub Regex(ns::Id)
);

impl arc::A<Regex> {
    #[objc::msg_send(initWithPattern:options:error:)]
    pub fn init_with_pattern_opts_err<'ear>(
        self,
        pattern: &ns::String,
        options: Opts,
        error: *mut Option<&'ear ns::Error>,
    ) -> Option<arc::R<Regex>>;
}

impl Regex {
    define_cls!(NS_REGULAR_EXPRESSION);

    #[inline]
    pub fn with_pattern<'ear>(
        pattern: &ns::String,
        options: Opts,
    ) -> Result<arc::R<Self>, &'ear ns::Error> {
        ns::if_none(|err| Self::alloc().init_with_pattern_opts_err(pattern, options, err))
    }
}

#[link(name = "ns", kind = "static")]
extern "C" {
    static NS_REGULAR_EXPRESSION: &'static objc::Class<Regex>;
}

#[cfg(test)]
mod tests {
    use crate::{ns, objc::ar_pool};

    #[test]
    fn basics() {
        let pat = ns::Regex::with_pattern(&ns::String::with_str(".*"), Default::default()).unwrap();
        println!("pat {:?}", pat);
    }
    #[test]
    fn error_autorelease() {
        ar_pool(|| {
            let err = ns::Regex::with_pattern(&ns::String::with_str("\\"), Default::default())
                .expect_err("should be err");
            assert_eq!(err.code(), 2048);
            println!("err {:?}", err);
        });
    }
}
