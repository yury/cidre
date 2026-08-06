use crate::swift::{
    self, SwiftMetadata, abi,
    value::{Optional, Storage, define_swift_value},
};

#[link(name = "Foundation", kind = "framework")]
unsafe extern "C" {
    #[link_name = "$s10Foundation4UUIDVMa"]
    fn uuid_metadata();

    #[link_name = "$s10Foundation4UUIDVACycfC"]
    fn uuid_init();

    #[link_name = "$s10Foundation4UUIDV10uuidStringACSgSSh_tcfC"]
    fn uuid_init_with_string();

    #[link_name = "$s10Foundation4UUIDV10uuidStringSSvg"]
    fn uuid_string();

}

define_swift_value!(
    /// `Foundation.UUID`.
    #[doc(alias = "UUID")]
    pub Uuid, UuidValue = accessor uuid_metadata
);

crate::impl_swift_sendable!(UuidValue);

impl Uuid {
    /// Generates a new random identifier.
    #[doc(alias = "UUID.init()")]
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        unsafe {
            let mut storage = Self::storage();
            abi::call0_value(uuid_init as *const (), storage.as_mut_ptr());
            Self::from_storage(storage)
        }
    }

    /// Parses the standard 36-character form, or `None` when it does not parse.
    #[doc(alias = "UUID.init(uuidString:)")]
    pub fn with_str(str: &str) -> Option<Self> {
        unsafe {
            // `init(uuidString:)` takes its argument `__shared`, so the string
            // stays ours to release.
            let text = swift::String::from(str);
            let mut storage = Storage::<Optional<UuidValue>>::new();
            abi::call_string_to_value(
                uuid_init_with_string as *const (),
                text.as_raw(),
                storage.as_mut_ptr(),
            );
            let value = storage.assume_init();
            if !value.is_some() {
                return None;
            }

            // An optional's payload starts at offset 0, so the storage already
            // holds the unwrapped value.
            let raw = value.as_ptr();
            let mut unwrapped = Self::storage();
            abi::initialize_with_copy(unwrapped.as_mut_ptr(), raw, UuidValue::metadata());
            Some(Self::from_storage(unwrapped))
        }
    }

    /// `UUID.uuidString`, the uppercase 36-character form.
    #[doc(alias = "UUID.uuidString")]
    pub fn to_swift_string(&self) -> swift::String {
        unsafe {
            swift::String::from_raw(abi::call_value_to_string(
                uuid_string as *const (),
                self.as_ptr(),
            ))
        }
    }
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
        unsafe {
            let size = abi::value_layout(UuidValue::metadata()).size;
            core::slice::from_raw_parts(self.as_ptr().cast::<u8>(), size)
        }
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
