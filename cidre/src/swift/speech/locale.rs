//! Shared `Foundation.Locale` plumbing for the locale-dependent transcribers.
//!
//! `SpeechTranscriber` and `DictationTranscriber` both conform to
//! `LocaleDependentSpeechModule` and are both created by an
//! `init(locale:preset:)` taking two indirect `@owned` values. Only the symbols
//! differ, so the construction sequence lives here once.

use crate::{
    swift::foundation,
    swift::{SwiftMetadata, abi},
};

use crate::swift::value::{Storage, call_with_owned_values};

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
        let locale = foundation::Locale::with_id(locale_id);

        let mut preset_storage = Storage::<P>::new();
        abi::call0_value(preset_getter, preset_storage.as_mut_ptr());
        let preset = preset_storage.assume_init();

        let class_metadata = abi::call_int_to_int(class_metadata_accessor, 0) as *const ();
        call_with_owned_values(locale.into_value(), preset, |locale, preset| {
            abi::call_static_values_to_object(init, class_metadata, locale, preset)
        })
    }
}
