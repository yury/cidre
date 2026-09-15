#[doc(alias = "NSLineBreakMode")]
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
#[repr(isize)]
pub enum LineBreakMode {
    WordWrapping = 0,
    CharWrapping = 1,
    Clipping = 2,
    TruncatingHead = 3,
    TruncatingTail = 4,
    TruncatingMiddle = 5,
}
