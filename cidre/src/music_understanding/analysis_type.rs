use crate::swift::{SwiftMetadata, abi};

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

/// An owned Swift `Set<AnalysisType>`.
///
/// Building one needs the element's `Hashable` witness table, which the
/// framework does not export, so it is instantiated from the conformance
/// descriptor at runtime.
pub(super) struct AnalysisTypeSet(*mut ());

unsafe impl Send for AnalysisTypeSet {}

impl AnalysisTypeSet {
    pub(super) fn new(types: &[AnalysisType]) -> Self {
        unsafe {
            let metadata = AnalysisTypeValue::metadata();
            assert!(!metadata.is_null(), "AnalysisType metadata must exist");

            // Each case reads its value through a static getter, so the array is
            // filled element by element rather than copied from a Rust slice.
            let (array, elements) = abi::allocate_uninitialized_array(types.len(), metadata);
            let stride = abi::value_layout(metadata).stride;
            for (index, ty) in types.iter().enumerate() {
                abi::call0_value(
                    ty.getter(),
                    elements.cast::<u8>().add(index * stride).cast(),
                );
            }

            let witness = abi::witness_table((&raw const ANALYSIS_TYPE_HASHABLE).cast(), metadata);
            assert!(!witness.is_null(), "Hashable witness table must exist");

            Self(abi::set_from_array(array, metadata, witness))
        }
    }

    #[inline]
    pub(super) fn as_raw(&self) -> *mut () {
        self.0
    }
}

impl Drop for AnalysisTypeSet {
    fn drop(&mut self) {
        unsafe { abi::bridge_object_release(self.0 as usize) }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::swift::value::Storage;

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
        let set = AnalysisTypeSet::new(&ALL);
        assert!(!set.as_raw().is_null());

        let duplicates = AnalysisTypeSet::new(&[AnalysisType::Rhythm, AnalysisType::Rhythm]);
        assert!(!duplicates.as_raw().is_null());
    }
}
