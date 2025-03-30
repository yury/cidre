use crate::define_opts;

#[doc = "NSLineBreakMode"]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(usize)]
pub enum LineBreakMode {
    WordWrapping = 0,
    CharWrapping = 1,
    Clipping = 2,
    TruncatingHead = 3,
    TruncatingTail = 4,
    TruncatingMiddle = 5,
}

define_opts!(
    #[doc(alias = "NSLineBreakStrategy")]
    pub LineBreakStrategy(usize)
);

impl LineBreakStrategy {
    pub const NONE: Self = Self(0);
    pub const PUSH_OUT: Self = Self(1 << 0);
    pub const HANGUL_WORD_PRIORITY: Self = Self(1 << 1);
    pub const STANDARD: Self = Self(0xFFFF);
}
