use cidre::av::audio;
use std::time::Duration;

fn main() {
    let mut engine = audio::Engine::new();
    let mut down_mixer = audio::MixerNode::new();
    engine.attach_node(&down_mixer);

    down_mixer
        .install_tap_on_bus(0, 2048, None, |buf, time| {
            println!("{time:?} {buf:?}");
        })
        .expect("tap installed");

    let input_node = engine.input_node();
    let output_node = engine.output_node();
    let format = input_node.input_format_for_bus(0);
    let format_16kh_mono = audio::Format::with_common_format_sample_rate_channels_interleaved(
        audio::CommonFormat::PcmI16,
        16_000.0,
        1,
        true,
    )
    .expect("valid format");

    engine.connect_node_to_node(&input_node, &down_mixer, Some(&format));
    engine.connect_node_to_node(&down_mixer, &output_node, Some(&format_16kh_mono));

    engine.prepare();
    engine.start().unwrap();

    std::thread::sleep(Duration::from_secs(10));
}
