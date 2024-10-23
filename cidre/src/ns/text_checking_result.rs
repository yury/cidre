use crate::{define_obj_type, define_opts, ns, objc};

define_opts!(
    #[doc(alias = "NSTextCheckingType")]
    pub Type(usize)
);

impl Type {
    /// Language identification
    #[doc(alias = "NSTextCheckingTypeOrthography")]
    pub const ORTHOGRAPHY: Self = Self(1 << 0);

    /// Spell checking
    #[doc(alias = "NSTextCheckingTypeSpelling")]
    pub const SPELLING: Self = Self(1 << 1);

    /// Grammar checking
    #[doc(alias = "NSTextCheckingTypeGrammar")]
    pub const GRAMMAR: Self = Self(1 << 2);

    /// Date/time detection
    #[doc(alias = "NSTextCheckingTypeDate")]
    pub const DATE: Self = Self(1 << 3);

    /// Address detection
    #[doc(alias = "NSTextCheckingTypeAddress")]
    pub const ADDR: Self = Self(1 << 4);

    /// Link detection
    #[doc(alias = "NSTextCheckingTypeLink")]
    pub const LINK: Self = Self(1 << 5);

    /// Smart quotes
    #[doc(alias = "NSTextCheckingTypeQuote")]
    pub const QUOTE: Self = Self(1 << 6);

    /// Smart dashes
    #[doc(alias = "NSTextCheckingTypeDash")]
    pub const DASH: Self = Self(1 << 7);

    /// Fixed replacements, such as copyright symbol for (c)
    #[doc(alias = "NSTextCheckingTypeReplacement")]
    pub const REPLACEMENT: Self = Self(1 << 8);

    /// Autocorrection
    #[doc(alias = "NSTextCheckingTypeCorrection")]
    pub const CORRECTION: Self = Self(1 << 9);

    /// Regular expression matches
    #[doc(alias = "NSTextCheckingTypeRegularExpression")]
    pub const REGULAR_EXPRESSION: Self = Self(1 << 10);

    /// Phone number detection
    #[doc(alias = "NSTextCheckingTypePhoneNumber")]
    pub const PHONE_NUMBER: Self = Self(1 << 11);

    /// Transit (e.g. flight) info detection
    #[doc(alias = "NSTextCheckingTypeTransitInformation")]
    pub const TRANSIT_INFORMATION: Self = Self(1 << 12);
}

define_obj_type!(
    #[doc(alias = "NSTextCheckingResult")]
    pub TextCheckingResult(ns::Id)
);

impl TextCheckingResult {
    #[objc::msg_send(resultType)]
    pub fn result_type(&self) -> Type;

    #[objc::msg_send(range)]
    pub fn range(&self) -> ns::Range;
}
