use cidre::{av, av::capture::*, cm, define_obj_type, dispatch, ns, objc};

define_obj_type!(
    OutputDelegate + AudioDataOutputSampleBufDelegateImpl,
    (), // InnerType
    OUTPUT_DELEGATE
);

impl AudioDataOutputSampleBufDelegate for OutputDelegate {}

#[objc::add_methods]
impl AudioDataOutputSampleBufDelegateImpl for OutputDelegate {
    extern "C" fn impl_capture_output_did_output_sample_buf_from_connection(
        &mut self,
        _cmd: Option<&cidre::objc::Sel>,
        _output: &av::CaptureOutput,
        sample_buf: &cm::SampleBuf,
        _connection: &av::CaptureConnection,
    ) {
        // println!("sample buf: {:?}", sample_buf.num_samples());
        println!("sample buf: {:?}", sample_buf);
    }
}

fn main() {
    let mic = {
        let device_types =
            ns::Array::from_slice(&[DeviceType::built_in_microphone(), DeviceType::external()]);
        let discovery_session = DiscoverySession::with_device_types_media_and_pos(
            &device_types,
            Some(av::MediaType::audio()),
            DevicePos::Unspecified,
        );
        let devices = discovery_session.devices();
        for d in devices.iter() {
            eprintln!("device: {:?}", d);
            eprintln!("formats {:?}", d.formats());
        }
        devices.first().unwrap().retained()
    };

    let input = DeviceInput::with_device(&mic).unwrap();
    let queue = dispatch::Queue::new();
    let delegate = OutputDelegate::new();
    let mut output = av::capture::AudioDataOutput::new();
    output.set_sample_buf_delegate(Some(delegate.as_ref()), Some(&queue));

    let mut session = Session::new();

    session.configure(|s| {
        if s.can_add_input(&input) {
            s.add_input(&input);
        } else {
            panic!("can't add input");
        }

        s.add_output(&output);
    });

    session.start_running();

    std::thread::sleep(std::time::Duration::from_secs(5));

    session.stop_running();
}
