use crate::{cm, define_swift_getter_enum, swift};

#[link(name = "MusicUnderstanding", kind = "framework")]
unsafe extern "C" {
    #[link_name = "$s18MusicUnderstanding24InstrumentActivityResultV0C0VSHAAMc"]
    static INSTRUMENT_HASHABLE: u8;
}

define_swift_getter_enum!(
    /// `InstrumentActivityResult.Instrument`.
    pub Instrument in "MusicUnderstanding"
        = swift "MusicUnderstanding.InstrumentActivityResult(struct).Instrument" {
        Vocal = "$s18MusicUnderstanding24InstrumentActivityResultV0C0V5vocalAEvgZ",
        Drum = "$s18MusicUnderstanding24InstrumentActivityResultV0C0V4drumAEvgZ",
        Bass = "$s18MusicUnderstanding24InstrumentActivityResultV0C0V4bassAEvgZ",
        Other = "$s18MusicUnderstanding24InstrumentActivityResultV0C0V5otherAEvgZ",
    }
);

crate::impl_swift_hashable!(Instrument = descriptor(&raw const INSTRUMENT_HASHABLE).cast());

crate::define_swift!(
    #[swift::struct("MusicUnderstanding.InstrumentActivityResult", size(16), align(8), sendable)]
    /// `MusicUnderstanding.InstrumentActivityResult`.
    pub InstrumentActivityResult
);

impl InstrumentActivityResult {
    /// The time ranges an instrument is active in, or `None` when the analysis
    /// did not report that instrument.
    #[doc(alias = "InstrumentActivityResult.ranges")]
    pub fn ranges(&self, instrument: Instrument) -> Option<swift::Array<cm::TimeRange>> {
        self.ranges_by_instrument().get(&instrument)
    }

    /// `InstrumentActivityResult.ranges`, the whole dictionary the getter hands
    /// back at `+1`.
    /// The dictionary's key is the enclosing type's own nested `Instrument`,
    /// which the symbol reaches by back reference rather than by name, so this
    /// one keeps its mangled symbol.
    #[swift::call(
        sym = "$s18MusicUnderstanding24InstrumentActivityResultV6rangesSDyAC0C0VSaySo11CMTimeRangeaGGvg"
    )]
    #[doc(alias = "InstrumentActivityResult.ranges")]
    pub fn ranges_by_instrument(
        &self,
    ) -> swift::Dictionary<Instrument, swift::Array<cm::TimeRange>>;
}
