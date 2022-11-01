use crate::cm;

#[derive(Debug, PartialEq, Eq)]
#[repr(C)]
pub struct Range {
    pub start: cm::Time,
    pub duration: cm::Time,
}

impl Range {
    #[inline]
    pub const fn is_valid(&self) -> bool {
        self.start.is_valid()
            && self.duration.is_valid()
            && self.duration.epoch == 0
            && self.duration.value >= 0
    }
}
