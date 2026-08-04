use crate::swift::{
    self, SwiftMetadata, abi,
    value::{Optional, Storage, Value},
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

crate::define_swift_marker!(pub(crate) UuidValue = accessor uuid_metadata);

/// `Foundation.UUID`.
#[doc(alias = "UUID")]
pub struct Uuid {
    pub(super) value: Value<UuidValue>,
}

unsafe impl Send for Uuid {}
unsafe impl Sync for Uuid {}

impl Uuid {
    // Bridges for the framework modules that hand back these values.
    #[allow(dead_code)]
    #[inline]
    pub(crate) unsafe fn from_value(value: Value<UuidValue>) -> Self {
        Self { value }
    }

    #[allow(dead_code)]
    #[inline]
    pub(crate) fn as_ptr(&self) -> *const () {
        self.value.as_ptr()
    }

    /// Generates a new random identifier.
    #[doc(alias = "UUID.init()")]
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        unsafe {
            let mut storage = Storage::<UuidValue>::new();
            abi::call0_value(uuid_init as *const (), storage.as_mut_ptr());
            Self {
                value: storage.assume_init(),
            }
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
            let mut unwrapped = Storage::<UuidValue>::new();
            abi::initialize_with_copy(unwrapped.as_mut_ptr(), raw, UuidValue::metadata());
            Some(Self {
                value: unwrapped.assume_init(),
            })
        }
    }

    /// `UUID.uuidString`, the uppercase 36-character form.
    #[doc(alias = "UUID.uuidString")]
    pub fn to_swift_string(&self) -> swift::String {
        unsafe {
            swift::String::from_raw(abi::call_value_to_string(
                uuid_string as *const (),
                self.value.as_ptr(),
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

impl PartialEq for Uuid {
    /// A `UUID` is sixteen plain bytes, so comparing them needs no call into
    /// Swift.
    fn eq(&self, other: &Self) -> bool {
        unsafe {
            let size = abi::value_layout(UuidValue::metadata()).size;
            let lhs = core::slice::from_raw_parts(self.value.as_ptr().cast::<u8>(), size);
            let rhs = core::slice::from_raw_parts(other.value.as_ptr().cast::<u8>(), size);
            lhs == rhs
        }
    }
}

impl Eq for Uuid {}

impl Clone for Uuid {
    fn clone(&self) -> Self {
        unsafe {
            let mut storage = Storage::<UuidValue>::new();
            abi::initialize_with_copy(
                storage.as_mut_ptr(),
                self.value.as_ptr(),
                UuidValue::metadata(),
            );
            Self {
                value: storage.assume_init(),
            }
        }
    }
}
