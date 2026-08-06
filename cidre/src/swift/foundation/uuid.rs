use crate::swift;

crate::define_swift!(
    #[swift::struct("Foundation.UUID", size(16), align(1), trivial, sendable)]
    pub Uuid
);

impl Uuid {
    /// Generates a new random identifier.
    #[allow(clippy::new_without_default)]
    #[swift::call("Foundation.UUID(struct).init()")]
    pub fn new() -> Self;

    /// Parses the standard 36-character form, or `None` when it does not parse.
    ///
    /// The initializer takes its argument `__shared`, which the borrow here is
    /// the Rust side of: the string stays the caller's.
    #[swift::call("Foundation.UUID(struct).init?(uuidString: __shared String)")]
    pub fn with_swift_str(text: &swift::String) -> Option<Self>;

    pub fn with_str(str: &str) -> Option<Self> {
        Self::with_swift_str(&swift::String::from(str))
    }

    /// `UUID.uuidString`, the uppercase 36-character form.
    #[swift::call("Foundation.UUID(struct).uuidString: String { get }")]
    pub fn to_swift_string(&self) -> swift::String;
}

impl std::fmt::Display for Uuid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(&self.to_swift_string(), f)
    }
}

impl std::fmt::Debug for Uuid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Uuid({})", self.to_swift_string())
    }
}

impl Uuid {
    /// The identifier's raw bytes.
    ///
    /// A `UUID` is plain data, so comparing and hashing it needs no call into
    /// Swift.
    #[inline]
    fn bytes(&self) -> &[u8] {
        // The declared size is Swift's own, checked when the metadata resolved.
        unsafe { core::slice::from_raw_parts(self.as_ptr().cast::<u8>(), size_of::<Self>()) }
    }
}

impl PartialEq for Uuid {
    fn eq(&self, other: &Self) -> bool {
        self.bytes() == other.bytes()
    }
}

impl Eq for Uuid {}

impl std::hash::Hash for Uuid {
    /// Hashes the same bytes [`PartialEq`] compares, so equal identifiers hash
    /// equally.
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.bytes().hash(state);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn generated_calls_match_the_hand_written_ones() {
        let uuid = Uuid::new();
        let text = uuid.to_swift_string().to_string();
        assert_eq!(36, text.len(), "{text}");
        assert_eq!(text, text.to_uppercase());

        let parsed = Uuid::with_str(&text).expect("round trips");
        assert_eq!(uuid, parsed);
        assert_eq!(text, parsed.to_swift_string().to_string());

        assert!(Uuid::with_str("not a uuid").is_none());
        assert_ne!(Uuid::new(), Uuid::new());
    }

    #[test]
    fn a_date_reads_its_interval() {
        let now = crate::swift::foundation::Date::now();
        let a = now.time_interval_since_reference_date();
        assert!(a > 700_000_000.0, "{a}");
    }
}
