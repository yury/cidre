use crate::{
    api, arc, swift,
    swift::{SwiftMetadata, abi},
};

use crate::swift::value::{Storage, call_with_owned_value};

crate::define_swift_class!(pub SpeechDetector = accessor speech_detector_metadata);

/// `SpeechDetector.SensitivityLevel`.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum SensitivityLevel {
    Low,
    #[default]
    Medium,
    High,
}

#[link(name = "Speech", kind = "framework")]
unsafe extern "C" {
    #[link_name = "$s6Speech0A8DetectorCMa"]
    fn speech_detector_metadata();


    #[link_name = "$s6Speech0A8DetectorC16SensitivityLevelOMa"]
    fn sensitivity_level_metadata();

    #[link_name = "$s6Speech0A8DetectorC16DetectionOptionsVMa"]
    fn detection_options_metadata();

    #[link_name = "$s6Speech0A8DetectorC16DetectionOptionsV16sensitivityLevelAeC011SensitivityF0O_tcfC"]
    fn detection_options_init();

    #[link_name = "$s6Speech0A8DetectorC16detectionOptions13reportResultsA2C09DetectionD0V_SbtcfC"]
    fn speech_detector_init();
}

crate::define_swift_marker!(SensitivityLevelValue = accessor sensitivity_level_metadata);

crate::define_swift_marker!(DetectionOptions = accessor detection_options_metadata);

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
        unsafe {
            let mut sensitivity_storage = Storage::<SensitivityLevelValue>::new();
            // `SensitivityLevel` is a resilient enum, so its layout is not
            // guaranteed to stay one byte wide. Cases carry no payload, and
            // their tags follow declaration order.
            abi::destructive_inject_enum_tag(
                sensitivity_storage.as_mut_ptr(),
                match sensitivity {
                    SensitivityLevel::Low => 0,
                    SensitivityLevel::Medium => 1,
                    SensitivityLevel::High => 2,
                },
                SensitivityLevelValue::metadata(),
            );
            let sensitivity_value = sensitivity_storage.assume_init();

            let mut options_storage = Storage::<DetectionOptions>::new();
            call_with_owned_value(sensitivity_value, |sensitivity| {
                abi::call::value_to_value(
                    detection_options_init as *const (),
                    sensitivity.cast_const(),
                    options_storage.as_mut_ptr(),
                );
            });
            let options = options_storage.assume_init();

            let detector_metadata =
                <SpeechDetector as crate::swift::SwiftMetadata>::metadata().cast();
            let object = call_with_owned_value(options, |options| {
                abi::call::static_value_bool_to_object(
                    speech_detector_init as *const (),
                    detector_metadata,
                    options,
                    report_results,
                )
            });
            arc::R::from_raw(object.cast())
        }
    }
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
