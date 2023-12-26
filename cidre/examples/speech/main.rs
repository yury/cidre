use cidre::{
    av, av::SpeechSynthesizerDelegate, av::SpeechSynthesizerDelegateImpl, define_obj_type, ns, objc,
};

define_obj_type!(
    SynthDelegate + av::SpeechSynthesizerDelegateImpl,
    (),
    SYNH_DELEGATE
);

impl SpeechSynthesizerDelegate for SynthDelegate {}

#[objc::add_methods]
impl SpeechSynthesizerDelegateImpl for SynthDelegate {
    extern "C" fn impl_speech_synthesizer_did_start_speech_utterance(
        &mut self,
        _cmd: Option<&objc::Sel>,
        _synthesizer: &av::SpeechSynthesizer,
        _utterance: &av::SpeechUtterance,
    ) {
        println!("did start");
    }

    extern "C" fn impl_speech_synthesizer_did_finish_speech_utterance(
        &mut self,
        _cmd: Option<&objc::Sel>,
        _synthesizer: &av::SpeechSynthesizer,
        _utterance: &av::SpeechUtterance,
    ) {
        println!("did finish");
        ns::Application::shared().terminate(None);
    }
}

fn main() {
    let text = ns::String::with_str("The quick brown fox jumped over the lazy dog.");
    let mut utterance = av::SpeechUtterance::with_string(&text);

    // Configure the utterance.
    utterance.set_rate(0.57);
    utterance.set_pitch_multiplier(0.8);
    utterance.set_post_utterance_delay(0.2);
    utterance.set_volume(0.8);

    // Retrieve the British English voice.
    let lang_code = ns::String::with_str("en-GB");
    let voice = av::SpeechSynthesisVoice::with_lang(Some(&lang_code)).unwrap();

    // Assign the voice to the utterance.
    utterance.set_voice(Some(&voice));

    let synth_delegate = SynthDelegate::new();

    // Create a speech synthesizer.
    let mut synth = av::SpeechSynthesizer::new();

    synth.set_delegate(Some(synth_delegate.as_ref()));

    // Tell the synthesizer to speak the utterance.
    synth.speak_utterance(&utterance).unwrap();

    ns::Application::shared().run();
}
