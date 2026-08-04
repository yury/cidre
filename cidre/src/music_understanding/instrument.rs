use crate::{
    cm, swift,
    swift::value::{Optional, Storage, Value},
    swift::{SwiftMetadata, abi},
};

#[link(name = "MusicUnderstanding", kind = "framework")]
unsafe extern "C" {
    #[link_name = "$s18MusicUnderstanding24InstrumentActivityResultVMa"]
    fn instrument_activity_result_metadata();

    #[link_name = "$s18MusicUnderstanding24InstrumentActivityResultV6rangesSDyAC0C0VSaySo11CMTimeRangeaGGvg"]
    fn instrument_activity_result_ranges();

    #[link_name = "$s18MusicUnderstanding24InstrumentActivityResultV0C0VMa"]
    fn instrument_metadata();

    #[link_name = "$s18MusicUnderstanding24InstrumentActivityResultV0C0VSHAAMc"]
    static INSTRUMENT_HASHABLE: u8;

    #[link_name = "$s18MusicUnderstanding24InstrumentActivityResultV0C0V5vocalAEvgZ"]
    fn vocal();

    #[link_name = "$s18MusicUnderstanding24InstrumentActivityResultV0C0V4drumAEvgZ"]
    fn drum();

    #[link_name = "$s18MusicUnderstanding24InstrumentActivityResultV0C0V4bassAEvgZ"]
    fn bass();

    #[link_name = "$s18MusicUnderstanding24InstrumentActivityResultV0C0V5otherAEvgZ"]
    fn other();
}

crate::define_swift_marker!(pub(super) InstrumentActivityResultValue = accessor instrument_activity_result_metadata);

crate::define_swift_marker!(InstrumentValue = accessor instrument_metadata);

/// `InstrumentActivityResult.Instrument`.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
#[non_exhaustive]
pub enum Instrument {
    Vocal,
    Drum,
    Bass,
    Other,
}

impl Instrument {
    fn getter(self) -> *const () {
        match self {
            Self::Vocal => vocal as _,
            Self::Drum => drum as _,
            Self::Bass => bass as _,
            Self::Other => other as _,
        }
    }
}

/// `MusicUnderstanding.InstrumentActivityResult`.
pub struct InstrumentActivityResult {
    pub(super) value: Value<Optional<InstrumentActivityResultValue>>,
}

unsafe impl Send for InstrumentActivityResult {}

impl InstrumentActivityResult {
    /// The time ranges an instrument is active in, or `None` when the analysis
    /// did not report that instrument.
    ///
    /// `ranges` is a Swift `Dictionary`, so the lookup carries both the key and
    /// value metadata plus the key's `Hashable` conformance.
    #[doc(alias = "InstrumentActivityResult.ranges")]
    pub fn ranges(&self, instrument: Instrument) -> Option<swift::Array<cm::TimeRange>> {
        unsafe {
            // The getter hands back an owned dictionary.
            let dictionary = RawDictionary(abi::call_value_to_int(
                instrument_activity_result_ranges as *const (),
                self.value.as_ptr(),
            ) as *mut ());

            let mut key = Storage::<InstrumentValue>::new();
            abi::call0_value(instrument.getter(), key.as_mut_ptr());
            let key = key.assume_init();

            let key_metadata = InstrumentValue::metadata();
            let witness = abi::witness_table((&raw const INSTRUMENT_HASHABLE).cast(), key_metadata);
            assert!(!witness.is_null(), "Hashable witness table must exist");

            let mut out = Storage::<Optional<swift::Array<cm::TimeRange>>>::new();
            abi::dictionary_get(
                key.as_ptr(),
                dictionary.0,
                key_metadata,
                swift::Array::<cm::TimeRange>::metadata(),
                witness,
                out.as_mut_ptr(),
            );
            let out = out.assume_init();
            if !out.is_some() {
                return None;
            }

            // The array now belongs to the caller, so release the optional's
            // storage without destroying what it held.
            let raw = out.as_ptr().cast::<*mut ()>().read();
            out.assume_consumed();
            Some(swift::Array::from_raw(raw))
        }
    }
}

/// Releases the dictionary the getter returned at +1.
struct RawDictionary(*mut ());

impl Drop for RawDictionary {
    fn drop(&mut self) {
        unsafe { abi::bridge_object_release(self.0 as usize) }
    }
}
