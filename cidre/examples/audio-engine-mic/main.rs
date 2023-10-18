use cidre::{av, blocks};
use std::{thread::sleep, time::Duration};

fn main() {
    let mut engine = av::AudioEngine::new();
    let input = engine.input_node_mut();
    println!(
        "voice processing enabled? {}",
        input.is_voice_processing_enabled()
    );

    if !input.is_voice_processing_enabled() {
        input.set_voice_processing_enabled(true).unwrap();
    }

    let mut tap = blocks::mut2(|buffer: &av::AudioPcmBuf, when: &av::AudioTime| {
        println!("{buffer:?} {when:?} {:?}", buffer.format());
    });

    input.install_tap_on_bus(0, 1024, None, tap.escape());

    engine.start().unwrap();

    sleep(Duration::from_secs(10));

    engine.stop();
}
