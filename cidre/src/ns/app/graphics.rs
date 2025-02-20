#[doc(alias = "NSWindowOrderingMode")]
#[derive(Debug, Eq, PartialEq, Copy, Clone)]
#[repr(isize)]
pub enum WindowOrderingMode {
    Above = 1,
    Below = -1,
    Out = 0,
}
