#[derive(Debug, Eq, PartialEq, Copy, Clone)]
#[repr(usize)]
pub enum ResizeMode {
    Nearest = 0,
    Bilinear = 1,
}

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
#[repr(usize)]
pub enum ResizeNearestRoundingMode {
    RoundPreferCeil = 0,
    RoundPreferFloor = 1,
    Ceil = 2,
    Floor = 3,
}
