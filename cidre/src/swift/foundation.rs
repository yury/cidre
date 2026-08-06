//! Foundation's Swift-native value types.
//!
//! These are the Swift `struct`s — `Locale`, `UUID`, `AttributedString` — not
//! the Objective-C classes in [`crate::ns`]. Their layouts are only known at
//! runtime, so each keeps its Swift representation and is read through the
//! framework's own accessors.

mod attr_string;
mod date;
mod locale;
mod uuid;

pub use attr_string::AttrString;
#[allow(unused_imports)]
pub(crate) use attr_string::AttrStringValue;
pub use date::Date;
#[allow(unused_imports)]
pub(crate) use date::DateValue;
pub use locale::Locale;
pub use uuid::Uuid;
#[allow(unused_imports)]
pub(crate) use uuid::UuidValue;

#[link(name = "Foundation", kind = "framework")]
unsafe extern "C" {}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::swift;

    #[test]
    fn uuid_generates_and_round_trips() {
        let a = Uuid::new();
        let b = Uuid::new();
        assert_ne!(a, b, "two generated identifiers must differ");

        let text = a.to_swift_string().to_string();
        assert_eq!(36, text.len(), "{text}");
        assert_eq!(text, text.to_uppercase(), "Swift renders them uppercase");

        let parsed = Uuid::with_str(&text).expect("the rendered form must parse");
        assert_eq!(a, parsed);
        assert_eq!(a, a.clone());
    }

    /// `Hash` must agree with `Eq`, since both are used to key collections.
    #[test]
    fn uuid_hashes_consistently_with_equality() {
        use std::collections::HashSet;

        let a = Uuid::new();
        let same = Uuid::with_str(&a.to_swift_string().to_string()).unwrap();
        let other = Uuid::new();

        let mut set = HashSet::new();
        assert!(set.insert(a.clone()));
        assert!(
            !set.insert(same),
            "an equal identifier must not be inserted twice"
        );
        assert!(set.insert(other));
        assert_eq!(2, set.len());
        assert!(set.contains(&a));
    }

    #[test]
    fn uuid_rejects_malformed_input() {
        assert!(Uuid::with_str("").is_none());
        assert!(Uuid::with_str("not-a-uuid").is_none());
        assert!(Uuid::with_str("00000000-0000-0000-0000-00000000000").is_none());
        assert!(Uuid::with_str("00000000-0000-0000-0000-000000000000").is_some());
    }

    #[test]
    fn locale_round_trips_its_identifier() {
        for id in ["en_US", "fr_FR", "zh-Hans"] {
            let locale = Locale::with_id(id);
            assert_eq!(id, locale.id().to_string(), "{id}");
        }

        // A `const` literal is the zero-cost path, so it must agree.
        const EN: swift::String = swift::String::with_ascii_literal("en_GB");
        assert_eq!("en_GB", Locale::with_swift_id(EN).id().to_string());
    }

    /// Cloning goes through `initializeWithCopy`, so the copy has to own its
    /// own contents: both must still read correctly after either is dropped.
    #[test]
    fn a_swift_value_clone_owns_its_contents() {
        let locale = Locale::with_id("fr_FR");
        let copy = locale.clone();
        assert_eq!("fr_FR", copy.id().to_string());

        // If the copy had aliased rather than retained, this would leave the
        // survivor reading freed storage.
        drop(locale);
        assert_eq!("fr_FR", copy.id().to_string());

        let second = copy.clone();
        drop(copy);
        assert_eq!("fr_FR", second.id().to_string());
    }

    /// Whether a wrapper may cross threads now follows from the Swift type
    /// being `Sendable`, so this is the statement that it still reaches the
    /// wrapper — and everything built over it.
    #[test]
    fn sendable_swift_values_stay_send_and_sync() {
        const fn assert_send_sync<T: Send + Sync>() {}

        assert_send_sync::<swift::String>();
        assert_send_sync::<Uuid>();
        assert_send_sync::<Locale>();
        assert_send_sync::<Date>();
        assert_send_sync::<AttrString>();
        assert_send_sync::<swift::Array<Uuid>>();
    }

    /// A wrapper around a runtime-laid-out value has to work as an array
    /// element like any other, which is what replaced the hand-written
    /// `[UUID]` these bindings used to carry.
    #[test]
    fn an_array_holds_values_whose_layout_is_only_known_at_runtime() {
        let first = Uuid::new();
        let second = Uuid::new();
        let array = swift::Array::from_slice(&[first.clone(), second.clone()]);

        assert_eq!(2, array.len());
        assert_eq!(Some(first.clone()), array.get(0));
        assert_eq!(Some(second), array.get(1));
        assert_eq!(None, array.get(2));

        // The array owns copies, so the originals stay usable and nothing is
        // released twice.
        drop(array);
        assert_eq!(36, first.to_swift_string().to_string().len());
    }

    #[test]
    fn attr_string_is_reachable_through_speech_results() {
        // `AttrString` has no public constructor yet; this pins the metadata so
        // a symbol change is caught here rather than inside a transcription.
        use crate::swift::SwiftMetadata;
        let metadata = AttrStringValue::metadata();
        assert!(!metadata.is_null(), "AttributedString metadata must exist");
    }
}
