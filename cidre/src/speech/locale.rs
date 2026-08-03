//! Shared `Foundation.Locale` plumbing for the locale-dependent transcribers.
//!
//! `SpeechTranscriber` and `DictationTranscriber` both conform to
//! `LocaleDependentSpeechModule` and are both created by an
//! `init(locale:preset:)` taking two indirect `@owned` values. Only the symbols
//! differ, so the construction sequence lives here once.

use crate::{
    swift,
    swift::{SwiftMetadata, abi},
};

use super::value::{Storage, Value, call_with_owned_values};

#[link(name = "swiftFoundation")]
unsafe extern "C" {
    #[link_name = "$s10Foundation6LocaleVMa"]
    fn foundation_locale_metadata();

    #[link_name = "$s10Foundation6LocaleV10identifierACSS_tcfC"]
    fn foundation_locale_init();
}

pub(super) struct FoundationLocale;

unsafe impl SwiftMetadata for FoundationLocale {
    fn metadata() -> *const abi::TypeMetadata {
        unsafe {
            abi::call_int_to_int(foundation_locale_metadata as *const (), 0)
                as *const abi::TypeMetadata
        }
    }
}

/// Builds `Foundation.Locale(identifier:)`.
pub(super) fn with_id(locale_id: &str) -> Value<FoundationLocale> {
    unsafe {
        let mut storage = Storage::<FoundationLocale>::new();
        let locale_id = swift::String::from(locale_id).into_raw();
        abi::call_string_to_value(
            foundation_locale_init as *const (),
            locale_id,
            storage.as_mut_ptr(),
        );
        storage.assume_init()
    }
}

/// Creates a transcriber through `init(locale:preset:)`.
///
/// `preset_getter` is one of the preset type's static getters, which returns its
/// value indirectly; `P` is that preset type. Both the locale and the preset are
/// consumed by the initializer.
///
/// # Safety
///
/// The symbols must belong to one type: `class_metadata_accessor` and `init`
/// must name a transcriber whose `Preset` is `P`, and `preset_getter` must be
/// one of that `Preset`'s static getters.
pub(super) unsafe fn transcriber_with_id_and_preset<P: SwiftMetadata>(
    locale_id: &str,
    preset_getter: *const (),
    class_metadata_accessor: *const (),
    init: *const (),
) -> *mut () {
    unsafe {
        let locale = with_id(locale_id);

        let mut preset_storage = Storage::<P>::new();
        abi::call0_value(preset_getter, preset_storage.as_mut_ptr());
        let preset = preset_storage.assume_init();

        let class_metadata = abi::call_int_to_int(class_metadata_accessor, 0) as *const ();
        call_with_owned_values(locale, preset, |locale, preset| {
            abi::call_static_values_to_object(init, class_metadata, locale, preset)
        })
    }
}
