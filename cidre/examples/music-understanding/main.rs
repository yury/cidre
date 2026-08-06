//! Analyzes an audio file with MusicUnderstanding.framework.
//!
//! Usage: `music-understanding <audio-file>`
//! `cargo r --features="av,async,cm,dispatch,macos_27_0" --example music-understanding <audio-file>`

use cidre::{av, cm, ns, swift::music_understanding as mu};

#[tokio::main(flavor = "current_thread")]
async fn main() {
    let Some(path) = std::env::args().nth(1) else {
        eprintln!("usage: music-understanding <audio-file>");
        return;
    };

    let url = ns::Url::with_fs_path_str(&path, false);
    let Some(asset) = av::UrlAsset::with_url(&url, None) else {
        eprintln!("could not open {path}");
        return;
    };

    let session = match mu::MusicUnderstandingSession::with_asset(&asset).await {
        Ok(session) => session,
        Err(err) => return report(&err),
    };

    // Requesting only what is printed below is much faster than running every
    // analysis the framework offers.
    println!("Analyzing {path}…");
    let result = match session
        .analyze_for(&[
            mu::AnalysisType::Rhythm,
            mu::AnalysisType::Loudness,
            mu::AnalysisType::InstrumentActivity,
        ])
        .await
    {
        Ok(result) => result,
        Err(err) => return report(&err),
    };

    if let Some(loudness) = result.loudness() {
        println!(
            "Loud:   {:.1} LUFS integrated, {:.1} peak",
            loudness.integrated().value(),
            loudness.peak().value()
        );
    }

    if let Some(activity) = result.instrument_activity() {
        let present: Vec<String> = [
            (mu::Instrument::Vocal, "vocal"),
            (mu::Instrument::Drum, "drum"),
            (mu::Instrument::Bass, "bass"),
            (mu::Instrument::Other, "other"),
        ]
        .into_iter()
        .filter_map(|(instrument, name)| {
            activity
                .ranges(instrument)
                .map(|ranges| format!("{name} ({})", ranges.len()))
        })
        .collect();
        if !present.is_empty() {
            println!("Instr:  {}", present.join(", "));
        }
    }

    let Some(rhythm) = result.rhythm() else {
        println!("No rhythm was detected.");
        return;
    };

    match rhythm.beats_per_minute() {
        Some(bpm) => println!("Tempo:  {bpm:.1} BPM"),
        None => println!("Tempo:  not detected"),
    }

    let beats = rhythm.beats();
    let bars = rhythm.bars();
    println!("Beats:  {}", beats.len());
    println!("Bars:   {}", bars.len());

    if !beats.is_empty() {
        let times: Vec<String> = beats
            .iter()
            .take(8)
            .map(|time| format!("{:.2}s", seconds(time)))
            .collect();
        let ellipsis = if beats.len() > 8 { ", …" } else { "" };
        println!("First:  {}{ellipsis}", times.join(", "));
    }
}

fn seconds(time: cm::Time) -> f64 {
    if time.scale == 0 {
        return f64::NAN;
    }
    time.value as f64 / time.scale as f64
}

#[cold]
fn report(err: &ns::Error) {
    if err.domain().as_ref() != mu::err::domain() {
        eprintln!("Analysis failed: {}", err.localized_desc());
        return;
    }

    use mu::err::code;
    let reason = match err.code() {
        code::SESSION_IN_PROGRESS => "another analysis is already running",
        code::EMPTY_ANALYSIS_SET => "no analyses were requested",
        code::INVALID_ASSET => "the file is not usable audio",
        code::HAS_PROTECTED_CONTENT => "the audio is protected",
        code::INTERNAL_ERROR => "the framework reported an internal error",
        other => {
            eprintln!("Analysis failed: MusicUnderstanding error {other}");
            return;
        }
    };
    eprintln!("Analysis failed: {reason}");
}
