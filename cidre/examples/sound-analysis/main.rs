use cidre::{
    av, define_obj_type, dispatch,
    objc::{self, Obj},
    sn::{self, ResultsObserving},
};

define_obj_type!(
    ResultsObs + sn::ResultsObservingImpl,
    (),
    EXAMPLE_RESULTS_OBSERVER
);

impl ResultsObserving for ResultsObs {}

#[objc::add_methods]
impl sn::ResultsObservingImpl for ResultsObs {
    extern "C" fn impl_request_did_produce_result(
        &mut self,
        _cmd: Option<&objc::Sel>,
        _request: &sn::Request,
        result: &sn::Result,
    ) {
        let res = result.try_cast(sn::ClassificationResult::cls()).unwrap();
        eprintln!("--- {:?}", res.time_range().start);
        for c in res.classifications().iter().take(3) {
            eprintln!("{c:?}");
        }
    }

    extern "C" fn impl_request_did_fail_with_err(
        &mut self,
        _cmd: Option<&objc::Sel>,
        _request: &sn::Request,
        error: &cidre::ns::Error,
    ) {
        eprintln!("{error:?}");
    }
}

#[tokio::main(flavor = "current_thread")]
async fn main() {
    let mut engine = av::AudioEngine::new();

    let mut input = engine.input_node();
    println!("voice processing enabled? {}", input.is_vp_enabled());

    // this is heavy on CPU
    // if !input.is_vp_enabled() {
    //     input.set_vp_enabled(true).unwrap();
    //     input.set_vp_other_audio_ducking_cfg(av::AudioVPOtherAudioDuckingCfg {
    //         enable_advanced_ducking: true,
    //         ducking_level: av::AudioVPOtherAudioDuckingLevel::Min,
    //     });
    // }
    let format = input.input_format_for_bus(0);
    let mut analysis = sn::AudioStreamAnalyzer::with_format(format);

    let obs = ResultsObs::new();
    let req = sn::ClassifySoundRequest::v1().unwrap();
    println!("knowns classes {:?}", req.known_classifications().len());

    analysis
        .add_request_with_observer(&req, obs.as_ref())
        .expect("Failed to add request with observer");

    let queue = dispatch::Queue::new();

    let sa = analysis.retained();

    let tap = move |buf: &av::AudioPcmBuf, when: &av::AudioTime| {
        queue.async_mut({
            let pos = when.sample_time();
            let buf = buf.retained();
            let mut sa = sa.retained();
            move || sa.analyze_audio_buf_at_pos(&buf, pos)
        });
    };

    input.install_tap_on_bus(0, 8192, None, tap);

    engine.start().unwrap();

    tokio::signal::ctrl_c().await.unwrap();

    eprintln!("stopping");

    engine.stop();

    analysis.complete();

    eprintln!("done");
}
