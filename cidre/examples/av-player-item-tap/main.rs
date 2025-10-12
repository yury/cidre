/// Example of usage av::AudioMix and mt::AudioProcessingTap
/// from [WWDC 2012 Session 517: Real-Time Media Effects and Processing during Playback](https://nonstrict.eu/wwdcindex/wwdc2012/517/)
/// Speaker: Simon Goldrei
use cidre::{av, cat, cm, mt, ns};

fn main() {
    let url = ns::Url::with_str("https://samplelib.com/lib/preview/mp3/sample-15s.mp3").unwrap();

    let mut item = av::PlayerItem::with_url(&url);

    let callbacks = mt::AudioProcessingTapCbs::<1, ()> {
        prepare: Some(prepare),
        process,
        ..Default::default()
    };
    let tap = mt::AudioProcessingTap::with_callbacks(
        &callbacks,
        mt::AudioProcessingTapCreationFlags::POST_EFFECTS,
    )
    .unwrap();

    let mut params = av::AudioMixInputParamsMut::new();
    params.set_tap(Some(&tap));

    let mut mix = av::AudioMixMut::new();
    mix.set_input_params_mut(&ns::arr![params]);

    item.set_audio_mix(Some(&mix));

    let mut player = av::Player::with_player_item(Some(&item)).unwrap();
    player.play();
    ns::App::shared().run();
}

extern "C-unwind" fn prepare(
    _tap: &mut mt::AudioProcessingTap,
    frames_max: cm::ItemCount,
    processing_format: &cat::AudioStreamBasicDesc,
) {
    println!("{frames_max} processing {processing_format:#?}");
}

extern "C-unwind" fn process(
    tap: &mut mt::AudioProcessingTap,
    frames_n: cm::ItemCount,
    _flags: mt::AudioProcessingTapFlags,
    buf_list_in_out: &mut cat::AudioBufList,
    frames_n_out: &mut cm::ItemCount,
    flags_out: &mut mt::AudioProcessingTapFlags,
) {
    let mut time_range = cm::TimeRange::zero();
    tap.src_audio(
        frames_n,
        buf_list_in_out,
        flags_out,
        &mut time_range,
        frames_n_out,
    )
    .unwrap();

    // *frames_n_out = 0; // uncomment for silence
    println!("frames_n_out {frames_n_out}");
}
