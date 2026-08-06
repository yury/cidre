//! Shared `Foundation.Locale` plumbing for the locale-dependent transcribers.
//!
//! `SpeechTranscriber` and `DictationTranscriber` both conform to
//! `LocaleDependentSpeechModule` and are both created by an
//! `init(locale:preset:)` taking two indirect `@owned` values. Only the symbols
//! differ, so the construction sequence lives here once.

use crate::{
    swift::foundation,
    swift::{ToSwift, abi},
};

use crate::swift::value::{Storage, call_with_owned_values};

/// Creates a transcriber through `init(locale:preset:)`.
///
/// Both the locale and the preset are consumed by the initializer.
///
/// # Safety
///
/// The symbols must belong to one type: `class_metadata_accessor` and `init`
/// must name a transcriber whose `Preset` is `P`.
pub(super) unsafe fn transcriber_with_id_and_preset<P: ToSwift>(
    locale_id: &str,
    preset: P,
    class_metadata_accessor: *const (),
    init: *const (),
) -> *mut () {
    unsafe {
        let locale = foundation::Locale::with_id(locale_id);

        let mut preset_storage = Storage::<P>::new();
        preset.copy_to_swift(preset_storage.as_mut_ptr());
        let preset = preset_storage.assume_init();

        let class_metadata = abi::call_int_to_int(class_metadata_accessor, 0) as *const ();
        call_with_owned_values(locale.into_value(), preset, |locale, preset| {
            abi::call_static_values_to_object(init, class_metadata, locale, preset)
        })
    }
}
