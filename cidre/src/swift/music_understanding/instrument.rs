use crate::{cm, define_swift_getter_enum, swift};

define_swift_getter_enum!(
    /// `InstrumentActivityResult.Instrument`.
    pub Instrument = swift "MusicUnderstanding.InstrumentActivityResult(struct).Instrument" {
        Vocal = "vocal",
        Drum = "drum",
        Bass = "bass",
        Other = "other",
    }
);

crate::impl_swift_hashable!(
    Instrument = descriptor swift::conformance!(
        "MusicUnderstanding.InstrumentActivityResult(struct).Instrument(struct): Hashable"
    )
);

crate::define_swift!(
    #[swift::struct("MusicUnderstanding.InstrumentActivityResult", size(16), align(8), sendable)]
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
    #[swift::call(
        "MusicUnderstanding.InstrumentActivityResult(struct).ranges: \
         [MusicUnderstanding.InstrumentActivityResult(struct).Instrument(struct): \
         [__C.CMTimeRange]] { get }"
    )]
    #[doc(alias = "InstrumentActivityResult.ranges")]
    pub fn ranges_by_instrument(
        &self,
    ) -> swift::Dictionary<Instrument, swift::Array<cm::TimeRange>>;
}
