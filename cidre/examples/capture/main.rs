use cidre::{
    av::{self, CaptureDevicePosition, MediaType},
    ns,
};

extern "C" fn exception_handler(exception: &ns::Exception) {
    println!("{:?}", exception);
}

fn main() {
    unsafe { ns::set_uncaught_exception_handler(exception_handler as _) };

    // ns::Exception::raise(&cf::String::from_str("str"));

    let mut session = av::capture::Session::new();
    let device = av::capture::Device::with_device_type_media_and_position(
        av::CaptureDeviceType::built_in_wide_angle_camera(),
        Some(MediaType::video()),
        CaptureDevicePosition::Front,
    )
    .expect("front wide angle camera");

    let mut video_output = av::capture::VideoDataOutput::new();
    video_output.set_always_discard_late_video_frames(true);

    assert!(video_output.sample_buffer_callback_queue().is_none());

    println!("{:?}", video_output.available_video_cv_pixel_format_types());
    println!("{:?}", video_output.available_video_codec_types());
    println!("{:?}", video_output.video_settings());
    println!(
        "{:?}",
        video_output
            .recommended_video_settings_for_asset_writer_with_output_file_type(av::FileType::mp4())
    );
    println!(
        "{:?}",
        video_output.recommended_video_settings_for_video_codec_type_asset_writer_output_file_type(
            av::VideoCodecType::h264(),
            av::FileType::mp4()
        )
    );

    session.begin_configuration();

    if session.can_add_output(&video_output) {
        session.add_output(&video_output);
    }

    let device_input = av::CaptureDeviceInput::with_device(&device).expect("intput");
    println!("{:?}", device_input);
    if session.can_add_input(&device_input) {
        session.add_input(&device_input)
    }
    println!("{:?}", video_output.available_video_cv_pixel_format_types());
    println!("{:?}", video_output.available_video_codec_types());
    println!("{:?}", video_output.video_settings());
    println!(
        "{:?}",
        video_output
            .recommended_video_settings_for_asset_writer_with_output_file_type(av::FileType::mp4())
    );
    println!(
        "{:?}",
        video_output.recommended_video_settings_for_video_codec_type_asset_writer_output_file_type(
            av::VideoCodecType::h264(),
            av::FileType::mp4()
        )
    );

    session.commit_configuration();
    session.start_running();
}
