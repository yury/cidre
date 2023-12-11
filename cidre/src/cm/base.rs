pub type ItemCount = crate::cf::Index;
pub type ItemIndex = crate::cf::Index;

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
#[repr(transparent)]
pub struct PersistentTrackId(pub i32);

impl PersistentTrackId {
    pub const INVALID: Self = Self(0);
}

impl PartialEq<i32> for PersistentTrackId {
    fn eq(&self, other: &i32) -> bool {
        self.0 == *other
    }
}
