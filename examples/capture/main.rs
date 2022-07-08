use cidre::{
    av::{self, CaptureDevicePosition, MediaType},
    objc::autorelease_pool,
};

fn main() {
    let session = av::capture::Session::new();
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
}
