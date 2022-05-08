pub mod session;
pub use session::Session;

pub mod properties;
pub use properties::h264_entropy_mode;
pub use properties::hdr_metadata_insertion_mode;
pub use properties::keys;
pub use properties::profile_level;

#[cfg(test)]
mod tests {
    use crate::{cf, cm::VideoCodecType, cv};

    use super::*;

    #[test]
    fn session_new() {
        let mut image_attrs = cf::MutableDictionary::with_capacity(3).unwrap();

        let empty_dict = cf::Dictionary::new().unwrap();
        let pixel_format = cf::Number::from_four_char_code(
            cv::PixelFormatType::_420_YP_CB_CR8_BI_PLANAR_VIDEO_RANGE.0,
        );

        image_attrs.insert(cv::pixel_buffer::keys::io_surface_properties(), &empty_dict);
        image_attrs.insert(
            cv::pixel_buffer::keys::metal_compatability(),
            &cf::Boolean::value_true(),
        );
        image_attrs.insert(cv::pixel_buffer::keys::pixel_format_type(), &pixel_format);

        let session = Session::new(
            1920,
            1080,
            VideoCodecType::H264,
            None,
            Some(&image_attrs),
            None,
            std::ptr::null_mut(),
        )
        .expect("encoder");

        session.show();
    }
}
