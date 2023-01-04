use crate::{cf, define_obj_type, define_options, ns};

define_options!(Options(usize));

impl Options {
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

define_options!(MatchingOptions(usize));

impl MatchingOptions {
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

define_options!(MatchingFlags(usize));

impl MatchingFlags {
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

define_obj_type!(RegularExpression(ns::Id));

impl RegularExpression {
    #[inline]
    pub fn with_pattern_error(
        pattern: &cf::String,
        options: Options,
        error: &mut Option<&cf::Error>,
    ) -> Option<cf::Retained<Self>> {
        unsafe {
            NSRegularExpression_regularExpressionWithPattern_options_error(pattern, options, error)
        }
    }

    #[inline]
    pub fn with_pattern(
        pattern: &cf::String,
        options: Options,
    ) -> Result<cf::Retained<Self>, &cf::Error> {
        let mut error = None;
        unsafe {
            let res = Self::with_pattern_error(pattern, options, &mut error);
            match res {
                Some(res) => Ok(res),
                None => Err(error.unwrap_unchecked()),
            }
        }
    }
}

#[link(name = "ns", kind = "static")]
extern "C" {
    fn NSRegularExpression_regularExpressionWithPattern_options_error(
        pattern: &cf::String,
        options: Options,
        error: &mut Option<&cf::Error>,
    ) -> Option<cf::Retained<RegularExpression>>;
}

#[cfg(test)]
mod tests {
    pub use crate::{cf, ns};
    #[test]
    fn basics() {
        let pat =
            ns::RegularExpression::with_pattern(&cf::String::from_str(".*"), Default::default())
                .unwrap();
        println!("pat {:?}", pat);
    }
}
