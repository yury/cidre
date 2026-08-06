use crate::swift::{SwiftMetadata, ToSwift, abi};

#[link(name = "MusicUnderstanding", kind = "framework")]
unsafe extern "C" {
    #[link_name = "$s18MusicUnderstanding12AnalysisTypeVMa"]
    fn analysis_type_metadata();

    /// Exported as a descriptor only; the witness table itself is built at
    /// runtime by whoever needs the conformance.
    #[link_name = "$s18MusicUnderstanding12AnalysisTypeVSHAAMc"]
    static ANALYSIS_TYPE_HASHABLE: u8;

    #[link_name = "$s18MusicUnderstanding12AnalysisTypeV18instrumentActivityACvgZ"]
    fn instrument_activity();

    #[link_name = "$s18MusicUnderstanding12AnalysisTypeV8loudnessACvgZ"]
    fn loudness();

    #[link_name = "$s18MusicUnderstanding12AnalysisTypeV4paceACvgZ"]
    fn pace();

    #[link_name = "$s18MusicUnderstanding12AnalysisTypeV6rhythmACvgZ"]
    fn rhythm();

    #[link_name = "$s18MusicUnderstanding12AnalysisTypeV9structureACvgZ"]
    fn structure();

    #[link_name = "$s18MusicUnderstanding12AnalysisTypeV3keyACvgZ"]
    fn key();
}

crate::define_swift_marker!(pub(super) AnalysisTypeValue = accessor analysis_type_metadata);

/// `MusicUnderstanding.AnalysisType`.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
#[non_exhaustive]
pub enum AnalysisType {
    InstrumentActivity,
    Loudness,
    Pace,
    Rhythm,
    Structure,
    Key,
}

impl AnalysisType {
    fn getter(self) -> *const () {
        match self {
            Self::InstrumentActivity => instrument_activity as _,
            Self::Loudness => loudness as _,
            Self::Pace => pace as _,
            Self::Rhythm => rhythm as _,
            Self::Structure => structure as _,
            Self::Key => key as _,
        }
    }
}

unsafe impl SwiftMetadata for AnalysisType {
    #[inline]
    fn metadata() -> *const abi::TypeMetadata {
        AnalysisTypeValue::metadata()
    }
}

/// Each case is a static property of the Swift type rather than a tag this
/// binding could write itself, so making the value means calling its getter
/// straight into the destination.
unsafe impl ToSwift for AnalysisType {
    #[inline]
    unsafe fn copy_to_swift(&self, dst: *mut ()) {
        unsafe { abi::call0_value(self.getter(), dst) }
    }
}

crate::impl_swift_hashable!(AnalysisType = descriptor(&raw const ANALYSIS_TYPE_HASHABLE).cast());

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{swift, swift::value::Storage};

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
        let metadata = AnalysisTypeValue::metadata();
        let size = unsafe { abi::value_layout(metadata) }.size;

        let values: Vec<Vec<u8>> = ALL
            .iter()
            .map(|ty| unsafe {
                let mut storage = Storage::<AnalysisTypeValue>::new();
                abi::call0_value(ty.getter(), storage.as_mut_ptr());
                let value = storage.assume_init();
                core::slice::from_raw_parts(value.as_ptr().cast::<u8>(), size).to_vec()
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
