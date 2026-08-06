use super::instrument::InstrumentActivityResult;
use crate::{
    cm, swift,
    swift::{SwiftMetadata, abi},
};

#[link(name = "MusicUnderstanding", kind = "framework")]
unsafe extern "C" {
    #[link_name = "$s18MusicUnderstanding0aB7SessionC10TimedValueV5valuexvg"]
    fn timed_value_value();

    #[link_name = "$s18MusicUnderstanding0aB7SessionC10TimedValueV4timeSo6CMTimeavg"]
    fn timed_value_time();
}

crate::define_swift!(
    #[swift::struct("MusicUnderstanding.MusicUnderstandingSession(class).SessionResult", size(296), align(8), sendable)]
    /// `MusicUnderstandingSession.SessionResult`.
    ///
    /// A Swift value type whose layout is only known at runtime, so it is kept
    /// in its Swift representation and read through the framework's getters.
    pub SessionResult
);

impl SessionResult {
    /// `SessionResult.rhythm`, present only when rhythm analysis ran.
    #[swift::call(sym = "$s18MusicUnderstanding0aB7SessionC0C6ResultV6rhythmAA06RhythmD0VSgvg")]
    #[doc(alias = "SessionResult.rhythm")]
    pub fn rhythm(&self) -> Option<RhythmResult>;

    /// `SessionResult.loudness`, present only when loudness analysis ran.
    #[swift::call(sym = "$s18MusicUnderstanding0aB7SessionC0C6ResultV8loudnessAA08LoudnessD0VSgvg")]
    #[doc(alias = "SessionResult.loudness")]
    pub fn loudness(&self) -> Option<LoudnessResult>;

    #[swift::call(
        sym = "$s18MusicUnderstanding0aB7SessionC0C6ResultV18instrumentActivityAA010InstrumentfD0VSgvg"
    )]
    #[doc(alias = "SessionResult.instrumentActivity")]
    pub fn instrument_activity(&self) -> Option<InstrumentActivityResult>;
}

crate::define_swift!(
    #[swift::struct("MusicUnderstanding.RhythmResult", size(24), align(8), sendable)]
    /// `MusicUnderstanding.RhythmResult`.
    pub RhythmResult
);

impl RhythmResult {
    /// `RhythmResult.beatsPerMinute`.
    ///
    /// Swift returns this `Float?` in a register rather than through an
    /// indirect result, so the word is decoded through the runtime instead of
    /// by assuming where the tag sits.
    #[swift::call("MusicUnderstanding.RhythmResult(struct).beatsPerMinute: Float? { get }")]
    pub fn beats_per_minute(&self) -> Option<f32>;

    /// `RhythmResult.beats`, whose array arrives as its one-word representation
    /// rather than through an indirect result.
    #[swift::call("MusicUnderstanding.RhythmResult(struct).beats: [__C.CMTime] { get }")]
    pub fn beats(&self) -> swift::Array<cm::Time>;

    /// `RhythmResult.bars`.
    #[swift::call("MusicUnderstanding.RhythmResult(struct).bars: [__C.CMTime] { get }")]
    pub fn bars(&self) -> swift::Array<cm::Time>;
}

crate::define_swift!(
    #[swift::struct("MusicUnderstanding.LoudnessResult", size(80), align(8), sendable)]
    /// `MusicUnderstanding.LoudnessResult`.
    pub LoudnessResult
);

impl LoudnessResult {
    /// `LoudnessResult.integrated`, the whole track's loudness.
    #[swift::call(
        sym = "$s18MusicUnderstanding14LoudnessResultV10integratedAA0aB7SessionC10TimedValueVy_SfGvg"
    )]
    #[doc(alias = "LoudnessResult.integrated")]
    pub fn integrated(&self) -> TimedValue;

    /// `LoudnessResult.peak`.
    #[swift::call(
        sym = "$s18MusicUnderstanding14LoudnessResultV4peakAA0aB7SessionC10TimedValueVy_SfGvg"
    )]
    #[doc(alias = "LoudnessResult.peak")]
    pub fn peak(&self) -> TimedValue;
}

crate::define_swift!(
    #[swift::mangled(
        "18MusicUnderstanding0aB7SessionC10TimedValueVy_SfG",
        size(28), align(4), trivial, sendable
    )]
    /// A `TimedValue<Float>`: a measurement and the time it applies to.
    ///
    /// `TimedValue`'s generic parameter is constrained to Decodable, Encodable
    /// and Equatable, so its metadata accessor wants the argument metadata plus
    /// three witness tables — more arguments than an accessor passes in
    /// registers. Resolving the mangled name instead lets the runtime assemble
    /// the conformances.
    pub TimedValue
);

impl TimedValue {
    /// `TimedValue.value`.
    ///
    /// A member of a generic type, so the call also carries the instantiated
    /// metadata as its generic context.
    #[doc(alias = "TimedValue.value")]
    pub fn value(&self) -> f32 {
        unsafe {
            let mut out = core::mem::MaybeUninit::<f32>::uninit();
            abi::call::generic_value_to_value(
                timed_value_value as *const (),
                self.as_ptr(),
                <TimedValue as SwiftMetadata>::metadata(),
                out.as_mut_ptr().cast(),
            );
            out.assume_init()
        }
    }

    /// `TimedValue.time`.
    #[doc(alias = "TimedValue.time")]
    pub fn time(&self) -> cm::Time {
        unsafe {
            let words = abi::call::generic_value_to_words3(
                timed_value_time as *const (),
                self.as_ptr(),
                <TimedValue as SwiftMetadata>::metadata(),
            );
            core::mem::transmute::<(u64, u64, u64), cm::Time>(words)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn generic_timed_value_metadata_instantiates() {
        let float = f32::metadata();
        assert!(!float.is_null(), "Float metadata");

        let md = <TimedValue as SwiftMetadata>::metadata();
        assert!(!md.is_null(), "TimedValue<Float> metadata");

        let layout = unsafe { abi::value_layout(md) };
        println!(
            "TimedValue<Float>: size {} stride {}",
            layout.size, layout.stride
        );
        assert!(layout.size >= core::mem::size_of::<cm::Time>() + core::mem::size_of::<f32>());
    }

    #[test]
    fn loudness_result_metadata_resolves() {
        let md = LoudnessResult::metadata();
        assert!(!md.is_null());
        let layout = unsafe { abi::value_layout(md) };
        println!(
            "LoudnessResult: size {} stride {}",
            layout.size, layout.stride
        );
    }
}
