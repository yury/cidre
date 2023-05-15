use std::time::Duration;

use cidre::{
    av, av::capture::AudioDataOutputSampleBufferDelegate,
    av::capture::AudioDataOutputSampleBufferDelegateImpl, av::capture::*, cm, define_obj_type,
    dispatch, ns, objc,
};

#[repr(C)]
pub struct OutputDelegateInner {}

define_obj_type!(
    OutputDelegate + AudioDataOutputSampleBufferDelegateImpl,
    OutputDelegateInner,
    OUTPUT_DELEGATE
);

impl AudioDataOutputSampleBufferDelegate for OutputDelegate {}

#[objc::add_methods]
impl AudioDataOutputSampleBufferDelegateImpl for OutputDelegate {
    extern "C" fn impl_capture_output_did_output_sample_buffer_from_connection(
        &mut self,
        _cmd: Option<&cidre::objc::Sel>,
        _output: &av::CaptureOutput,
        sample_buffer: &cm::SampleBuffer,
        _connection: &av::CaptureConnection,
    ) {
        println!("sample buffer: {:?}", sample_buffer.num_samples());
    }
}

fn main() {
    let mic = {
        let device_types = ns::Array::from_slice(&[
            DeviceType::built_in_microphone(),
            DeviceType::external_unknown(),
        ]);
        let discovery_session = DiscoverySession::with_device_types_media_and_position(
            &device_types,
            Some(av::MediaType::audio()),
            DevicePosition::Unspecified,
        );
        discovery_session.devices().last().unwrap().retained()
    };

    let input = DeviceInput::with_device(mic.as_ref()).unwrap();

    let mut session = av::CaptureSession::new();

    if session.can_add_input(&input) {
        session.add_input(&input);
    } else {
        panic!("can't add input");
    }

    let queue = dispatch::Queue::new();
    let delegate = OutputDelegate::with(OutputDelegateInner {});
    let mut output = av::capture::AudioDataOutput::new();
    output.set_sample_buffer_delegate(Some(delegate.as_ref()), Some(&queue));

    session.add_output(&output);

    session.start_running();

    std::thread::sleep(Duration::from_secs(5));

    session.stop_running();
}
