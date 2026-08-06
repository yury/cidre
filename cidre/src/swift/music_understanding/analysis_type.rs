use crate::define_swift_getter_enum;

#[link(name = "MusicUnderstanding", kind = "framework")]
unsafe extern "C" {
    /// Exported as a descriptor only; the witness table itself is built at
    /// runtime by whoever needs the conformance.
    #[link_name = "$s18MusicUnderstanding12AnalysisTypeVSHAAMc"]
    static ANALYSIS_TYPE_HASHABLE: u8;
}

define_swift_getter_enum!(
    /// `MusicUnderstanding.AnalysisType`.
    pub AnalysisType in "MusicUnderstanding"
        = swift "MusicUnderstanding.AnalysisType" {
        InstrumentActivity = "$s18MusicUnderstanding12AnalysisTypeV18instrumentActivityACvgZ",
        Loudness = "$s18MusicUnderstanding12AnalysisTypeV8loudnessACvgZ",
        Pace = "$s18MusicUnderstanding12AnalysisTypeV4paceACvgZ",
        Rhythm = "$s18MusicUnderstanding12AnalysisTypeV6rhythmACvgZ",
        Structure = "$s18MusicUnderstanding12AnalysisTypeV9structureACvgZ",
        Key = "$s18MusicUnderstanding12AnalysisTypeV3keyACvgZ",
    }
);

crate::impl_swift_hashable!(AnalysisType = descriptor(&raw const ANALYSIS_TYPE_HASHABLE).cast());

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        swift,
        swift::{SwiftMetadata, ToSwift, abi, value::Storage},
    };

    const ALL: [AnalysisType; 6] = [
        AnalysisType::InstrumentActivity,
        AnalysisType::Loudness,
        AnalysisType::Pace,
        AnalysisType::Rhythm,
        AnalysisType::Structure,
        AnalysisType::Key,
    ];

    /// Every case must reach its own Swift static, so a mistyped mangled name
    /// cannot silently alias another analysis.
    #[test]
    fn every_analysis_type_reads_a_distinct_value() {
        let metadata = AnalysisType::metadata();
        let size = unsafe { abi::value_layout(metadata) }.size;

        let values: Vec<Vec<u8>> = ALL
            .iter()
            .map(|ty| unsafe {
                let mut storage = Storage::<AnalysisType>::new();
                ty.copy_to_swift(storage.as_mut_ptr());
                let bytes =
                    core::slice::from_raw_parts(storage.as_ptr().cast::<u8>(), size).to_vec();
                storage.destroy();
                bytes
            })
            .collect();

        for (i, a) in values.iter().enumerate() {
            for (j, b) in values.iter().enumerate().skip(i + 1) {
                assert_ne!(a, b, "{:?} and {:?} read the same value", ALL[i], ALL[j]);
            }
        }
    }

    /// Exercises the witness-table instantiation and `Set.init(arrayLiteral:)`.
    #[test]
    fn set_is_built_from_the_hashable_conformance() {
        let set = swift::Set::from_slice(&ALL);
        assert!(!set.as_raw().is_null());

        let duplicates = swift::Set::from_slice(&[AnalysisType::Rhythm, AnalysisType::Rhythm]);
        assert!(!duplicates.as_raw().is_null());
    }
}
