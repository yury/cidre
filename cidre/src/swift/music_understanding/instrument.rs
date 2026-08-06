use crate::{cm, define_swift_getter_enum, swift, swift::abi, swift::value::define_swift_value};

#[link(name = "MusicUnderstanding", kind = "framework")]
unsafe extern "C" {
    #[link_name = "$s18MusicUnderstanding24InstrumentActivityResultVMa"]
    fn instrument_activity_result_metadata();

    #[link_name = "$s18MusicUnderstanding24InstrumentActivityResultV6rangesSDyAC0C0VSaySo11CMTimeRangeaGGvg"]
    fn instrument_activity_result_ranges();

    #[link_name = "$s18MusicUnderstanding24InstrumentActivityResultV0C0VSHAAMc"]
    static INSTRUMENT_HASHABLE: u8;

}

define_swift_getter_enum!(
    /// `InstrumentActivityResult.Instrument`.
    pub Instrument in "MusicUnderstanding"
        = accessor "$s18MusicUnderstanding24InstrumentActivityResultV0C0VMa"
    {
        Vocal = "$s18MusicUnderstanding24InstrumentActivityResultV0C0V5vocalAEvgZ",
        Drum = "$s18MusicUnderstanding24InstrumentActivityResultV0C0V4drumAEvgZ",
        Bass = "$s18MusicUnderstanding24InstrumentActivityResultV0C0V4bassAEvgZ",
        Other = "$s18MusicUnderstanding24InstrumentActivityResultV0C0V5otherAEvgZ",
    }
);

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
