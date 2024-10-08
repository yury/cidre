use cidre::{
    av::{self, capture::device},
    ns,
};

fn main() {
    let device_type = device::Type::built_in_wide_angle_camera();
    let media_type = av::MediaType::video();
    let pos = device::Pos::Front;
    let device = device::Device::with_type_media_and_pos(device_type, Some(media_type), pos)
        .expect("device");

    let mut types = ns::ArrayMut::with_capacity(1);
    types.push(device::Type::built_in_wide_angle_camera());
    let discrover_session = device::DiscoverySession::with_device_types_media_and_pos(
        &types,
        Some(av::MediaType::video()),
        device::Pos::Front,
    );

    println!("devices {:?}", discrover_session.devices());

    let device_id = device.unique_id();
    println!("device id: {:?}", device_id);
    let formats = device.formats();
    for f in formats.iter() {
        let format_description = f.format_desc();
        let resolution = format_description.dimensions();
        println!("resolution: {:?}", resolution);

        let ranges = f.video_supported_frame_rate_ranges();
        println!("autofocus {:?}", f.auto_focus_sys());
        for r in ranges.iter() {
            println!(
                "  frame_rate {:?}-{:?}",
                r.min_frame_rate(),
                r.max_frame_rate()
            );
            println!(
                "  frame_duration {:?}-{:?}",
                r.min_frame_duration(),
                r.max_frame_duration()
            );
        }
    }

    let session = av::CaptureSession::new();
    if session.can_set_session_preset(av::CaptureSessionPreset::_1920x1080()) {
        session
            .set_session_preset(av::CaptureSessionPreset::_1920x1080())
            .expect("failed to set preseet");
    }
    session.as_type_ref().show();
}
