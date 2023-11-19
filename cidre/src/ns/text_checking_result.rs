use crate::{define_obj_type, define_options, ns, objc};

define_options!(Type(usize));

impl Type {
    /// language identification
    #[doc(alias = "NSTextCheckingTypeOrthography")]
    pub const ORTHOGRAPHY: Self = Self(1 << 0);

    /// spell checking
    #[doc(alias = "NSTextCheckingTypeSpelling")]
    pub const SPELLING: Self = Self(1 << 1);

    /// grammar checking
    #[doc(alias = "NSTextCheckingTypeGrammar")]
    pub const GRAMMAR: Self = Self(1 << 2);

    /// date/time detection
    #[doc(alias = "NSTextCheckingTypeDate")]
    pub const DATE: Self = Self(1 << 3);

    /// address detection
    #[doc(alias = "NSTextCheckingTypeAddress")]
    pub const ADDRESS: Self = Self(1 << 4);

    /// link detection
    #[doc(alias = "NSTextCheckingTypeLink")]
    pub const LINK: Self = Self(1 << 5);

    /// smart quotes
    #[doc(alias = "NSTextCheckingTypeQuote")]
    pub const QUOTE: Self = Self(1 << 6);

    /// smart dashes
    #[doc(alias = "NSTextCheckingTypeDash")]
    pub const DASH: Self = Self(1 << 7);

    /// fixed replacements, such as copyright symbol for (c)
    #[doc(alias = "NSTextCheckingTypeReplacement")]
    pub const REPLACEMENT: Self = Self(1 << 8);

    /// autocorrection
    #[doc(alias = "NSTextCheckingTypeCorrection")]
    pub const CORRECTION: Self = Self(1 << 9);

    /// regular expression matches
    #[doc(alias = "NSTextCheckingTypeRegularExpression")]
    pub const REGULAR_EXPRESSION: Self = Self(1 << 10);

    /// phone number detection
    #[doc(alias = "NSTextCheckingTypePhoneNumber")]
    pub const PHONE_NUMBER: Self = Self(1 << 11);

    /// transit (e.g. flight) info detection
    #[doc(alias = "NSTextCheckingTypeTransitInformation")]
    pub const TRANSIT_INFORMATION: Self = Self(1 << 12);
}

define_obj_type!(pub TextCheckingResult(ns::Id));

impl TextCheckingResult {
    #[objc::msg_send(resultType)]
    pub fn result_type(&self) -> Type;

    #[objc::msg_send(range)]
    pub fn range(&self) -> ns::Range;
}
