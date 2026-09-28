use crate::{api, arc, swift};

use super::SpeechModule;
use crate::swift::value::{Optional, Storage};

crate::define_swift!(#[swift::class("Speech.SpeechAnalyzer")] pub SpeechAnalyzer);

crate::define_swift!(#[swift::struct("Speech.SpeechAnalyzer(class).Options")] AnalyzerOptions);

impl SpeechAnalyzer {
    /// Creates an analyzer with `options: nil`.
    #[doc(alias = "SpeechAnalyzer.init(modules:options:)")]
    #[api::available(
        macos = 26.0,
        ios = 26.0,
        maccatalyst = 26.0,
        tvos = 26.0,
        visionos = 26.0
    )]
    pub fn with_modules(modules: &[SpeechModule]) -> arc::R<Self> {
        Self::init(swift::Array::from_slice(modules), Storage::none())
    }

    #[swift::call(
        "Speech.SpeechAnalyzer(class).init(modules: [any Speech.SpeechModule], \
         options: Speech.SpeechAnalyzer(class).Options(struct)?)"
    )]
    fn init(
        modules: swift::Array<SpeechModule>,
        options: Storage<Optional<AnalyzerOptions>>,
    ) -> arc::R<Self>;
}
