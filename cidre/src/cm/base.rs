#[doc(alias = "CMItemCount")]
pub type ItemCount = crate::cf::Index;

#[doc(alias = "CMItemIndex")]
pub type ItemIndex = crate::cf::Index;

#[doc(alias = "CMPersistentTrackID")]
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
#[repr(transparent)]
pub struct PersistentTrackId(pub i32);

impl PersistentTrackId {
    #[doc(alias = "kCMPersistentTrackID_Invalid")]
    pub const INVALID: Self = Self(0);
}

impl PartialEq<i32> for PersistentTrackId {
    fn eq(&self, other: &i32) -> bool {
        self.0 == *other
    }
}
