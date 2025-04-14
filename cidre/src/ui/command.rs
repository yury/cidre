use crate::define_opts;

define_opts!(
    #[doc(alias = "UIKeyModifierFlags")]
    pub KeyModFlags(isize)
);

impl KeyModFlags {
    /// This bit indicates CapsLock
    pub const ALPHA_SHIFT: Self = Self(1 << 16);
    pub const SHIFT: Self = Self(1 << 17);
    pub const CTRL: Self = Self(1 << 18);
    pub const ALT: Self = Self(1 << 19);
    pub const CMD: Self = Self(1 << 20);
    pub const NUM_PAD: Self = Self(1 << 21);
}
