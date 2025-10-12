use cidre::{av, cat, cm, mt, ns};

fn main() {
    let url = ns::Url::with_string(ns::str!(
        c"https://samplelib.com/lib/preview/mp3/sample-15s.mp3"
    ))
    .unwrap();

    let mut item = av::PlayerItem::with_url(&url);

    let mut mix = av::AudioMixMut::new();

    let callbacks = mt::AudioProcessingTapCbs::<1, std::ffi::c_void> {
        version: mt::AudioProcessingTapCbsVersion::V0,
        client_info: std::ptr::null_mut(),
        init: None,
        finalize: None,
        prepare: Some(prepare),
        unprepare: None,
        process,
    };
    let tap = mt::AudioProcessingTap::with_callbacks(
        &callbacks,
        mt::AudioProcessingTapCreationFlags::POST_EFFECTS,
    )
    .unwrap();

    let mut params = av::AudioMixInputParamsMut::new();
    params.set_audio_tap_processor(Some(&tap));
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
