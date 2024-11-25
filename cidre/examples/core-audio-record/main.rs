use std::time::Duration;

use ca::aggregate_device_keys as agg_keys;
use ca::sub_device_keys as sub_keys;
use cidre::{cat, cf, core_audio as ca, ns, os};

fn main() {
    let output_device = ca::System::default_output_device().unwrap();
    let output_uid = output_device.uid().unwrap();

    let sub_device =
        cf::DictionaryOf::with_keys_values(&[sub_keys::uid()], &[output_uid.as_type_ref()]);

    let tap_desc = ca::TapDesc::with_stereo_global_tap_excluding_processes(&ns::Array::new());
    let tap = tap_desc.create_process_tap().unwrap();

    let sub_tap =
        cf::DictionaryOf::with_keys_values(&[sub_keys::uid()], &[tap.uid().unwrap().as_type_ref()]);

    // magic configuration for aggregate device
    let dict = cf::DictionaryOf::with_keys_values(
        &[
            agg_keys::is_private(),
            agg_keys::is_stacked(),
            agg_keys::tap_auto_start(),
            agg_keys::name(),
            agg_keys::main_sub_device(),
            agg_keys::uid(),
            agg_keys::sub_device_list(),
            agg_keys::tap_list(),
        ],
        &[
            cf::Boolean::value_true().as_type_ref(),
            cf::Boolean::value_false(),
            cf::Boolean::value_true(),
            cf::str!(c"Tap"),
            &output_uid,
            &cf::Uuid::new().to_cf_string(),
            &cf::ArrayOf::from_slice(&[sub_device.as_ref()]),
            &cf::ArrayOf::from_slice(&[sub_tap.as_ref()]),
        ],
    );
    let agg_device = ca::AggregateDevice::with_desc(&dict).unwrap();

    extern "C" fn proc(
        _device: ca::Device,
        _now: &cat::AudioTimeStamp,
        input_data: &cat::AudioBufList<1>,
        _input_time: &cat::AudioTimeStamp,
        _output_data: &mut cat::AudioBufList<1>,
        _output_time: &cat::AudioTimeStamp,
        _client_data: Option<&mut std::ffi::c_void>,
    ) -> os::Status {
        println!("{input_data:?}");
        Default::default()
    }

    {
        let proc_id = agg_device.create_io_proc_id(proc, None).unwrap();
        let _started_device = ca::device_start(agg_device, Some(proc_id)).unwrap();
        std::thread::sleep(Duration::from_secs(10));
    }
}
