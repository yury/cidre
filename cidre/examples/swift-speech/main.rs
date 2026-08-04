use cidre::{av, swift::speech};

#[tokio::main(flavor = "current_thread")]
async fn main() {
    if !speech::SpeechTranscriber::is_available() {
        eprintln!("The native Speech transcriber API is not available on this system.");
        return;
    }

    let locale = std::env::args()
        .nth(1)
        .unwrap_or_else(|| "en_US".to_owned())
        .replace('-', "_");
    let transcriber = speech::DictationTranscriber::with_locale_id(
        &locale,
        speech::DictationPreset::ProgressiveLongDictation,
    );
    transcriber.results(|result| match result {
        Ok(Some(text)) => eprint!("\r{text}"),
        Ok(None) => eprintln!("Transcription stream ended."),
        Err(err) => eprintln!("Transcription stream stopped: {}", err.localized_desc()),
    });

    let module = speech::SpeechModule::from(transcriber.as_ref());
    let analyzer = speech::SpeechAnalyzer::with_modules(&[module.clone()]);

    let audio = av::MediaType::audio();

    if !av::CaptureDevice::request_access_for_media_type(audio)
        .await
        .expect("microphone permission request failed")
    {
        eprintln!("Microphone permission denied.");
        return;
    }

    let device = av::CaptureDevice::with_type_media_and_pos(
        av::CaptureDeviceType::microphone(),
        Some(audio),
        av::CaptureDevicePos::Unspecified,
    )
    .expect("no microphone is available");

    let provider = speech::CaptureInputSequenceProvider::with_session(&device, &[module])
        .await
        .expect("could not configure microphone capture");

    analyzer.analyze_capture(&provider, |result| {
        if let Err(err) = result {
            eprintln!("Speech analysis stopped: {}", err.localized_desc());
        }
    });

    let mut session = provider.capture_session();
    session.start_running();
    println!(
        "Listening on {} for {locale}… Press Ctrl-C to stop.",
        device.localized_name()
    );

    tokio::signal::ctrl_c()
        .await
        .expect("Ctrl-C handler failed");
    session.stop_running();
}
