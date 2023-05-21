#[derive(Debug, Copy, Clone, Eq, PartialEq)]
#[repr(u32)]
pub enum FontOrientation {
    Default = 0,
    Horizontal = 1,
    Vertical = 2,
}
