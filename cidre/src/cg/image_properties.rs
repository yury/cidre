// TODO: It is actually belongs to ImageIO

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
#[repr(u32)]
pub enum Orientation {
    /// 0th row at top,    0th column on left   - default orientation
    Up = 1,

    /// 0th row at top,    0th column on right  - horizontal flip
    UpMirrored,

    /// 0th row at bottom, 0th column on right  - 180 deg rotation
    Down,

    /// 0th row at bottom, 0th column on left   - vertical flip
    DownMirrored,

    /// 0th row on left,   0th column at top
    LeftMirrored,

    /// 0th row on right,  0th column at top    - 90 deg CW
    Right,

    /// 0th row on right,  0th column on bottom
    RightMirrored,

    /// 0th row on left,   0th column at bottom - 90 deg CCW
    Left,
}

impl Default for Orientation {
    fn default() -> Self {
        Self::Up
    }
}
