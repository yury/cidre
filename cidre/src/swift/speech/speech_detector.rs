use crate::{
    api, arc, swift,
    swift::{SwiftMetadata, abi},
};

use crate::swift::value::Storage;

crate::define_swift!(#[swift::class("Speech.SpeechDetector")] pub SpeechDetector);

/// `SpeechDetector.SensitivityLevel`.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum SensitivityLevel {
    Low,
    #[default]
    Medium,
    High,
}

crate::define_swift!(
    #[swift::enum("Speech.SpeechDetector(class).SensitivityLevel")]
    SensitivityLevelValue
);

crate::define_swift!(
    #[swift::struct("Speech.SpeechDetector(class).DetectionOptions")]
    DetectionOptions
);

impl DetectionOptions {
    #[swift::call(
        "Speech.SpeechDetector(class).DetectionOptions(struct).init(\
         sensitivityLevel: Speech.SpeechDetector(class).SensitivityLevel(enum))"
    )]
    fn init(sensitivity_level: Storage<SensitivityLevelValue>) -> Storage<Self>;
}

impl SpeechDetector {
    #[doc(alias = "SpeechDetector.init")]
    #[api::available(
        macos = 26.0,
        ios = 26.0,
        maccatalyst = 26.0,
        tvos = 26.0,
        visionos = 26.0
    )]
    #[swift::call("Speech.SpeechDetector(class).init()")]
    pub fn new() -> arc::R<Self>;

    #[doc(alias = "SpeechDetector.init(detectionOptions:reportResults:)")]
    #[api::available(
        macos = 26.0,
        ios = 26.0,
        maccatalyst = 26.0,
        tvos = 26.0,
        visionos = 26.0
    )]
    pub fn with_sensitivity(sensitivity: SensitivityLevel, report_results: bool) -> arc::R<Self> {
        let mut level = Storage::<SensitivityLevelValue>::new();
        // `SensitivityLevel` is a resilient enum, so its layout is not
        // guaranteed to stay one byte wide. Cases carry no payload, and their
        // tags follow declaration order.
        unsafe {
            abi::destructive_inject_enum_tag(
                level.as_mut_ptr(),
                match sensitivity {
                    SensitivityLevel::Low => 0,
                    SensitivityLevel::Medium => 1,
                    SensitivityLevel::High => 2,
                },
                SensitivityLevelValue::metadata(),
            );
        }
        Self::init(DetectionOptions::init(level), report_results)
    }

    #[swift::call(
        "Speech.SpeechDetector(class).init(\
         detectionOptions: Speech.SpeechDetector(class).DetectionOptions(struct), \
         reportResults: Bool)"
    )]
    fn init(detection_options: Storage<DetectionOptions>, report_results: bool) -> arc::R<Self>;
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Pins the value witness index used to write `SensitivityLevel` cases by
    /// reading each tag back through the enum's `getEnumTag` witness.
    #[test]
    fn sensitivity_tags_round_trip_through_enum_value_witnesses() {
        let metadata = SensitivityLevelValue::metadata();
        let get_tag: unsafe extern "C" fn(*const (), *const abi::TypeMetadata) -> u32 =
            unsafe { std::mem::transmute(*abi::value_witness_table(metadata).add(11)) };

        for tag in 0..3 {
            let mut storage = Storage::<SensitivityLevelValue>::new();
            unsafe {
                abi::destructive_inject_enum_tag(storage.as_mut_ptr(), tag, metadata);
                assert_eq!(tag, get_tag(storage.as_ptr(), metadata));
            }
        }
    }
}
