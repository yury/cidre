/// Example of usage av::AudioMix and mt::AudioProcessingTap
/// from [WWDC 2012 Session 517: Real-Time Media Effects and Processing during Playback](https://nonstrict.eu/wwdcindex/wwdc2012/517/)
/// Speaker: Simon Goldrei
use cidre::{arc, av, ca, cat, cm, define_obj_type, mt, ns, objc};

fn main() {
    let url = ns::Url::with_str("https://test-streams.mux.dev/x36xhzz/x36xhzz.m3u8").unwrap();

    let mut item = av::PlayerItem::with_url(&url);

    let callbacks = mt::AudioProcessingTapCbs::<()> {
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

    let mut video_output = av::PlayerItemVideoOutput::new();
    video_output.set_suppresses_player_rendering(true);
    item.add_output(&video_output);

    let link_target = Link::with(video_output);

    let display_link = ns::Screen::main()
        .unwrap()
        .display_link_with_target(link_target.as_ref());

    display_link.add_to_run_loop_for_mode(ns::RunLoop::current(), ns::RunLoopMode::common());

    let mut player = av::Player::with_player_item(Some(&item)).unwrap();
    player.set_src_clock(Some(cm::Clock::host_time_clock()));
    player.play();
    player.set_rate(1.0);
    ns::App::shared().run();
}

extern "C-unwind" fn prepare(
    _tap: &mut mt::AudioProcessingTap,
    frames_max: cm::ItemCount,
    processing_format: &cat::AudioStreamBasicDesc,
) {
    println!();
    println!("audio processing format: {processing_format:#?}");
    println!("audio processing frames_max: {frames_max}");
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
    println!("frames_n_out {frames_n_out}",);
}

define_obj_type!(
    Link + ca::DisplayLinkTargetImpl,
    arc::R<av::PlayerItemVideoOutput>,
    DISPLAY_LINK_TARGET_CLS
);

impl ca::DisplayLinkTarget for Link {}

impl ca::DisplayLinkTargetImpl for Link {
    extern "C" fn impl_on_display_link(
        &mut self,
        _cmd: Option<&objc::Sel>,
        link: &mut ca::DisplayLink,
    ) {
        let output = self.inner_mut();
        let item_ts = output.item_time_for_host_time(link.target_ts());
        if output.has_new_pixel_buf_for_item_time(item_ts) {
            if let Some(pixel_buf) = output.pixel_buf_for_item_time(item_ts) {
                let w = pixel_buf.width();
                let h = pixel_buf.height();
                print!("{w}x{h}.");
            }
        } else {
            print!(".");
        }
    }
}
