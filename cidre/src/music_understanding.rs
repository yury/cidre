//! MusicUnderstanding.framework native Swift ABI bindings.
//!
//! Like [`crate::speech`], these call framework and Swift runtime symbols
//! directly. No C or Objective-C wrapper functions are used.

mod analysis_type;
pub mod err;
mod instrument;
mod results;
mod session;

pub use analysis_type::AnalysisType;
pub use instrument::{Instrument, InstrumentActivityResult};
pub use results::{LoudnessResult, RhythmResult, SessionResult, TimedValue};
pub use session::MusicUnderstandingSession;

#[link(name = "MusicUnderstanding", kind = "framework")]
unsafe extern "C" {}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{arc, av, ns};
    use std::sync::mpsc;

    fn asset(path: &str) -> arc::R<av::UrlAsset> {
        let url = ns::Url::with_fs_path_str(path, false);
        av::UrlAsset::with_url(&url, None).expect("asset")
    }

    /// A file that is not audio must be rejected. This runs the whole async
    /// path and, because `MusicUnderstandingError` is a native Swift enum
    /// rather than an `NSError` subclass, it also exercises the error bridge on
    /// a case Speech never produced.
    #[test]
    #[allow(unused_unsafe)]
    fn invalid_asset_reports_a_bridged_swift_error() {
        let asset = asset("/System/Library/CoreServices/SystemVersion.plist");
        let (tx, rx) = mpsc::channel();

        unsafe {
            MusicUnderstandingSession::with_asset_handler(&asset, move |res| {
                let _ = tx.send(res.map(|_| ()));
            });
        }

        let result = rx
            .recv_timeout(std::time::Duration::from_secs(30))
            .expect("session callback");

        let err = result.expect_err("a plist is not an audio asset");
        println!(
            "domain {:?} code {} :: {}",
            err.domain(),
            err.code(),
            err.localized_desc()
        );
        assert_eq!(err::domain(), err.domain().as_ref());
        assert_eq!(err::code::INVALID_ASSET, err.code());
    }

    /// Looks instruments up in a Swift `Dictionary`, which needs both key and
    /// value metadata plus the key's `Hashable` conformance.
    #[test]
    #[allow(unused_unsafe)]
    fn instrument_activity_reads_a_dictionary() {
        let Ok(path) = std::env::var("CIDRE_MU_TEST_TRACK") else {
            eprintln!("skipping: set CIDRE_MU_TEST_TRACK to an audio file");
            return;
        };
        let asset = asset(&path);

        let (tx, rx) = mpsc::channel();
        unsafe {
            MusicUnderstandingSession::with_asset_handler(&asset, move |res| {
                let _ = tx.send(res);
            });
        }
        let session = rx
            .recv_timeout(std::time::Duration::from_secs(60))
            .expect("session callback")
            .expect("session");

        let result = futures_lite_block_on(unsafe {
            session.analyze_for(&[AnalysisType::InstrumentActivity])
        })
        .expect("analysis");

        let activity = result
            .instrument_activity()
            .expect("an instrument activity result");

        let mut found = 0;
        for instrument in [
            Instrument::Vocal,
            Instrument::Drum,
            Instrument::Bass,
            Instrument::Other,
        ] {
            if let Some(ranges) = activity.ranges(instrument) {
                println!("{instrument:?}: {} ranges", ranges.len());
                found += 1;
            } else {
                println!("{instrument:?}: absent");
            }
        }
        assert!(
            found > 0,
            "expected at least one instrument in the dictionary"
        );
    }

    /// Reads a generic `TimedValue<Float>` out of a loudness result, which
    /// exercises generic metadata instantiation and calling a member of a
    /// generic type.
    #[test]
    #[allow(unused_unsafe)]
    fn loudness_reads_a_generic_timed_value() {
        let Ok(path) = std::env::var("CIDRE_MU_TEST_TRACK") else {
            eprintln!("skipping: set CIDRE_MU_TEST_TRACK to an audio file");
            return;
        };
        let asset = asset(&path);

        let (tx, rx) = mpsc::channel();
        unsafe {
            MusicUnderstandingSession::with_asset_handler(&asset, move |res| {
                let _ = tx.send(res);
            });
        }
        let session = rx
            .recv_timeout(std::time::Duration::from_secs(60))
            .expect("session callback")
            .expect("session");

        let result =
            futures_lite_block_on(unsafe { session.analyze_for(&[AnalysisType::Loudness]) })
                .expect("analysis");

        let loudness = result.loudness().expect("a loudness result");
        let integrated = loudness.integrated();
        let peak = loudness.peak();
        let time = integrated.time();

        println!(
            "integrated {} LUFS at {}s, peak {}",
            integrated.value(),
            time.value as f64 / time.scale as f64,
            peak.value()
        );

        // A 20s tone is well above silence and below clipping.
        assert!(
            integrated.value() > -60.0 && integrated.value() < 0.0,
            "implausible integrated loudness {}",
            integrated.value()
        );
        assert!(time.scale > 0, "timed value must carry a valid time");
    }

    /// Requesting only rhythm must still produce a rhythm result, and exercises
    /// `analyze(for:)` with a Swift `Set` built from Rust.
    #[test]
    #[allow(unused_unsafe)]
    fn analyze_for_rhythm_only() {
        let Ok(path) = std::env::var("CIDRE_MU_TEST_TRACK") else {
            eprintln!("skipping: set CIDRE_MU_TEST_TRACK to an audio file");
            return;
        };
        let asset = asset(&path);

        let (tx, rx) = mpsc::channel();
        unsafe {
            MusicUnderstandingSession::with_asset_handler(&asset, move |res| {
                let _ = tx.send(res);
            });
        }
        let session = rx
            .recv_timeout(std::time::Duration::from_secs(60))
            .expect("session callback")
            .expect("session");

        let bpm = futures_lite_block_on(unsafe { session.analyze_for(&[AnalysisType::Rhythm]) })
            .expect("analysis")
            .rhythm()
            .and_then(|r| r.beats_per_minute())
            .expect("a tempo");

        println!("analyze(for: [.rhythm]) reported {bpm} BPM");
        assert!((bpm - 120.0).abs() < 6.0 || (bpm - 60.0).abs() < 6.0 || (bpm - 240.0).abs() < 6.0);
    }

    /// Analyzes a synthetic click track whose tempo is known, so the reported
    /// value can be checked rather than merely observed.
    #[test]
    #[allow(unused_unsafe)]
    fn analyze_reports_the_tempo_of_a_known_track() {
        let Ok(path) = std::env::var("CIDRE_MU_TEST_TRACK") else {
            eprintln!("skipping: set CIDRE_MU_TEST_TRACK to an audio file");
            return;
        };
        let asset = asset(&path);

        let (tx, rx) = mpsc::channel();
        unsafe {
            MusicUnderstandingSession::with_asset_handler(&asset, move |res| {
                let _ = tx.send(res);
            });
        }
        let session = rx
            .recv_timeout(std::time::Duration::from_secs(60))
            .expect("session callback")
            .expect("session");

        let (tx, rx) = mpsc::channel();
        unsafe {
            session.analyze_handler(move |res| {
                let _ = tx.send(res.map(|r| {
                    r.rhythm().map(|r| {
                        let beats: Vec<f64> = r
                            .beats()
                            .iter()
                            .map(|t| t.value as f64 / t.scale as f64)
                            .collect();
                        (r.beats_per_minute(), beats)
                    })
                }));
            });
        }
        let (bpm, beats) = rx
            .recv_timeout(std::time::Duration::from_secs(300))
            .expect("analyze callback")
            .expect("analysis")
            .expect("a rhythm result");

        let bpm = bpm.expect("a tempo");
        println!("reported {bpm} BPM over {} beats", beats.len());
        assert!(
            (bpm - 120.0).abs() < 6.0 || (bpm - 60.0).abs() < 6.0 || (bpm - 240.0).abs() < 6.0,
            "expected a tempo near 120 (or its octave), got {bpm}"
        );

        // `beats` comes back as a Swift `[CMTime]`, so this also checks that
        // `swift::Array` copies an imported C struct correctly.
        assert!(beats.len() > 30, "expected a beat per half second over 20s");
        let gaps: Vec<f64> = beats.windows(2).map(|w| w[1] - w[0]).collect();
        let mean = gaps.iter().sum::<f64>() / gaps.len() as f64;
        assert!(
            (mean - 0.5).abs() < 0.05,
            "expected beats about 0.5s apart, got {mean}"
        );
    }
}

#[cfg(test)]
/// Minimal executor so the async APIs can be tested without pulling in a runtime.
fn futures_lite_block_on<F: std::future::Future>(mut fut: F) -> F::Output {
    use std::sync::{Arc, Condvar, Mutex};
    use std::task::{Context, Poll, Wake, Waker};

    struct Signal(Mutex<bool>, Condvar);
    impl Wake for Signal {
        fn wake(self: Arc<Self>) {
            *self.0.lock().unwrap() = true;
            self.1.notify_all();
        }
    }

    let signal = Arc::new(Signal(Mutex::new(false), Condvar::new()));
    let waker = Waker::from(signal.clone());
    let mut cx = Context::from_waker(&waker);
    let mut fut = unsafe { std::pin::Pin::new_unchecked(&mut fut) };
    loop {
        if let Poll::Ready(out) = fut.as_mut().poll(&mut cx) {
            return out;
        }
        let mut ready = signal.0.lock().unwrap();
        while !*ready {
            ready = signal.1.wait(ready).unwrap();
        }
        *ready = false;
    }
}
