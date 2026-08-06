use crate::{
    cm, swift,
    swift::value::define_swift_value,
    swift::{SwiftMetadata, ToSwift, abi},
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

unsafe impl SwiftMetadata for Instrument {
    #[inline]
    fn metadata() -> *const abi::TypeMetadata {
        InstrumentValue::metadata()
    }
}

/// Each case is a static property of the Swift type, so making the value means
/// calling its getter straight into the destination.
unsafe impl ToSwift for Instrument {
    #[inline]
    unsafe fn copy_to_swift(&self, dst: *mut ()) {
        unsafe { abi::call0_value(self.getter(), dst) }
    }
}

crate::impl_swift_hashable!(Instrument = descriptor(&raw const INSTRUMENT_HASHABLE).cast());

define_swift_value!(
    /// `MusicUnderstanding.InstrumentActivityResult`.
    pub InstrumentActivityResult, InstrumentActivityResultValue = optional accessor instrument_activity_result_metadata
);

crate::impl_swift_sendable!(InstrumentActivityResultValue);

impl InstrumentActivityResult {
    /// The time ranges an instrument is active in, or `None` when the analysis
    /// did not report that instrument.
    #[doc(alias = "InstrumentActivityResult.ranges")]
    pub fn ranges(&self, instrument: Instrument) -> Option<swift::Array<cm::TimeRange>> {
        self.ranges_by_instrument().get(&instrument)
    }

    /// `InstrumentActivityResult.ranges`, the whole dictionary the getter hands
    /// back at `+1`.
    #[doc(alias = "InstrumentActivityResult.ranges")]
    pub fn ranges_by_instrument(
        &self,
    ) -> swift::Dictionary<Instrument, swift::Array<cm::TimeRange>> {
        unsafe {
            swift::Dictionary::from_raw(abi::call_value_to_object(
                instrument_activity_result_ranges as *const (),
                self.as_ptr(),
            ))
        }
    }
}
