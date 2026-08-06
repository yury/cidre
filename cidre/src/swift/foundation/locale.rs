use crate::swift;

crate::define_swift!(
    #[swift::struct("Foundation.Locale", size(16), align(8), sendable)]
    pub Locale
);

impl Locale {
    /// `Locale(identifier:)`.
    #[doc(alias = "Locale.init(identifier:)")]
    pub fn with_id(identifier: &str) -> Self {
        Self::with_swift_id(swift::String::from(identifier))
    }

    /// `Locale(identifier:)` from an already-built Swift string, which lets a
    /// caller reuse a `const` literal.
    ///
    /// The initializer takes its argument at `+1`, so the string is surrendered
    /// to it rather than released here.
    #[swift::call("Foundation.Locale(struct).init(identifier: String)")]
    pub fn with_swift_id(identifier: swift::String) -> Self;

    #[swift::call("Foundation.Locale(struct).identifier: String { get }")]
    pub fn id(&self) -> swift::String;
}

impl std::fmt::Display for Locale {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(&self.id(), f)
    }
}

impl std::fmt::Debug for Locale {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Locale({})", self.id())
    }
}

impl From<&str> for Locale {
    fn from(identifier: &str) -> Self {
        Self::with_id(identifier)
    }
}
