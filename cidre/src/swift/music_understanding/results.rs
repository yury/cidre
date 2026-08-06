use super::instrument::{InstrumentActivityResult, InstrumentActivityResultValue};
use crate::{
    cm, swift,
    swift::value::{Optional, Storage, define_swift_value},
    swift::{SwiftMetadata, abi},
};

#[link(name = "MusicUnderstanding", kind = "framework")]
unsafe extern "C" {
    #[link_name = "$s18MusicUnderstanding0aB7SessionC0C6ResultVMa"]
    fn session_result_metadata();

    #[link_name = "$s18MusicUnderstanding0aB7SessionC0C6ResultV6rhythmAA06RhythmD0VSgvg"]
    fn session_result_rhythm();

    #[link_name = "$s18MusicUnderstanding12RhythmResultVMa"]
    fn rhythm_result_metadata();

    #[link_name = "$s18MusicUnderstanding12RhythmResultV14beatsPerMinuteSfSgvg"]
    fn rhythm_result_beats_per_minute();

    #[link_name = "$s18MusicUnderstanding12RhythmResultV5beatsSaySo6CMTimeaGvg"]
    fn rhythm_result_beats();

    #[link_name = "$s18MusicUnderstanding12RhythmResultV4barsSaySo6CMTimeaGvg"]
    fn rhythm_result_bars();

    #[link_name = "$s18MusicUnderstanding0aB7SessionC0C6ResultV8loudnessAA08LoudnessD0VSgvg"]
    fn session_result_loudness();

    #[link_name = "$s18MusicUnderstanding0aB7SessionC0C6ResultV18instrumentActivityAA010InstrumentfD0VSgvg"]
    fn session_result_instrument_activity();

    #[link_name = "$s18MusicUnderstanding14LoudnessResultVMa"]
    fn loudness_result_metadata();

    #[link_name = "$s18MusicUnderstanding14LoudnessResultV10integratedAA0aB7SessionC10TimedValueVy_SfGvg"]
    fn loudness_result_integrated();

    #[link_name = "$s18MusicUnderstanding14LoudnessResultV4peakAA0aB7SessionC10TimedValueVy_SfGvg"]
    fn loudness_result_peak();

    #[link_name = "$s18MusicUnderstanding0aB7SessionC10TimedValueV5valuexvg"]
    fn timed_value_value();

    #[link_name = "$s18MusicUnderstanding0aB7SessionC10TimedValueV4timeSo6CMTimeavg"]
    fn timed_value_time();
}

define_swift_value!(
    /// `MusicUnderstandingSession.SessionResult`.
    ///
    /// A Swift value type whose layout is only known at runtime, so it is kept
    /// in its Swift representation and read through the framework's getters.
    pub SessionResult, SessionResultValue = accessor session_result_metadata
);

crate::impl_swift_sendable!(SessionResultValue);

impl SessionResult {
    /// `SessionResult.rhythm`, present only when rhythm analysis ran.
    #[doc(alias = "SessionResult.rhythm")]
    pub fn rhythm(&self) -> Option<RhythmResult> {
        unsafe {
            let mut storage = Storage::<Optional<RhythmResultValue>>::new();
            abi::call::value_to_value(
                session_result_rhythm as *const (),
                self.as_ptr(),
                storage.as_mut_ptr(),
            );
            RhythmResult::from_optional_storage(storage)
        }
    }

    /// `SessionResult.loudness`, present only when loudness analysis ran.
    #[doc(alias = "SessionResult.loudness")]
    pub fn loudness(&self) -> Option<LoudnessResult> {
        unsafe {
            let mut storage = Storage::<Optional<LoudnessResultValue>>::new();
            abi::call::value_to_value(
                session_result_loudness as *const (),
                self.as_ptr(),
                storage.as_mut_ptr(),
            );
            LoudnessResult::from_optional_storage(storage)
        }
    }

    /// `SessionResult.instrumentActivity`.
    #[doc(alias = "SessionResult.instrumentActivity")]
    pub fn instrument_activity(&self) -> Option<InstrumentActivityResult> {
        unsafe {
            let mut storage = Storage::<Optional<InstrumentActivityResultValue>>::new();
            abi::call::value_to_value(
                session_result_instrument_activity as *const (),
                self.as_ptr(),
                storage.as_mut_ptr(),
            );
            InstrumentActivityResult::from_optional_storage(storage)
        }
    }
}

define_swift_value!(
    /// `MusicUnderstanding.RhythmResult`.
    pub RhythmResult, RhythmResultValue = optional accessor rhythm_result_metadata
);

crate::impl_swift_sendable!(RhythmResultValue);

impl RhythmResult {
    /// `RhythmResult.beatsPerMinute`.
    ///
    /// Swift returns this `Float?` directly in a register rather than through
    /// an indirect result, so the word is decoded through the runtime instead
    /// of by assuming where the tag sits.
    #[doc(alias = "RhythmResult.beatsPerMinute")]
    pub fn beats_per_minute(&self) -> Option<f32> {
        unsafe {
            let word =
                abi::call::value_to_int(rhythm_result_beats_per_minute as *const (), self.as_ptr());

            let mut storage = Storage::<Optional<f32>>::new();
            let out = storage.as_mut_ptr().cast::<usize>();
            out.write(word as usize);
            storage.assume_init().take()
        }
    }

    /// `RhythmResult.beats`.
    #[doc(alias = "RhythmResult.beats")]
    pub fn beats(&self) -> swift::Array<cm::Time> {
        unsafe { self.times(rhythm_result_beats as *const ()) }
    }

    /// `RhythmResult.bars`.
    #[doc(alias = "RhythmResult.bars")]
    pub fn bars(&self) -> swift::Array<cm::Time> {
        unsafe { self.times(rhythm_result_bars as *const ()) }
    }

    /// Reads a `[CMTime]` getter, which hands back the array's one-word
    /// representation directly rather than through an indirect result.
    unsafe fn times(&self, getter: *const ()) -> swift::Array<cm::Time> {
        unsafe {
            let raw = abi::call::value_to_int(getter, self.as_ptr()) as *mut ();
            swift::Array::from_raw(raw)
        }
    }
}

define_swift_value!(
    /// `MusicUnderstanding.LoudnessResult`.
    pub LoudnessResult, LoudnessResultValue = optional accessor loudness_result_metadata
);

crate::impl_swift_sendable!(LoudnessResultValue);

impl LoudnessResult {
    /// `LoudnessResult.integrated`, the whole track's loudness.
    #[doc(alias = "LoudnessResult.integrated")]
    pub fn integrated(&self) -> TimedValue {
        unsafe { self.timed(loudness_result_integrated as *const ()) }
    }

    /// `LoudnessResult.peak`.
    #[doc(alias = "LoudnessResult.peak")]
    pub fn peak(&self) -> TimedValue {
        unsafe { self.timed(loudness_result_peak as *const ()) }
    }

    unsafe fn timed(&self, getter: *const ()) -> TimedValue {
        unsafe {
            let mut storage = Storage::<TimedValueF32>::new();
            abi::call::value_to_value(getter, self.as_ptr(), storage.as_mut_ptr());
            TimedValue::from_value(storage.assume_init())
        }
    }
}

define_swift_value!(
    /// A `TimedValue<Float>`: a measurement and the time it applies to.
    ///
    /// `TimedValue`'s generic parameter is constrained to Decodable, Encodable
    /// and Equatable, so its metadata accessor wants the argument metadata plus
    /// three witness tables — more arguments than an accessor passes in
    /// registers. Resolving the mangled name instead lets the runtime assemble
    /// the conformances.
    pub TimedValue, TimedValueF32 = mangled "18MusicUnderstanding0aB7SessionC10TimedValueVy_SfG"
);

crate::impl_swift_sendable!(TimedValueF32);

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
                TimedValueF32::metadata(),
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
                TimedValueF32::metadata(),
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

        let md = TimedValueF32::metadata();
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
        let md = LoudnessResultValue::metadata();
        assert!(!md.is_null());
        let layout = unsafe { abi::value_layout(md) };
        println!(
            "LoudnessResult: size {} stride {}",
            layout.size, layout.stride
        );
    }
}
