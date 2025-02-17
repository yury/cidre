pub mod session;
pub use session::Session;

pub mod properties;
pub use properties::h264_entropy_mode;
pub use properties::hdr_metadata_insertion_mode;
pub use properties::keys;
pub use properties::profile_level;

#[cfg(test)]
mod tests {
    use crate::{cf, cm::VideoCodec, cv};

    use super::*;

    #[test]
    fn session_new() {
        let mut image_attrs = cf::DictionaryMut::with_capacity(3);

        let empty_dict = cf::Dictionary::new();
        let pixel_format = cv::PixelFormat::_420V.to_cf_number();

        image_attrs.insert(cv::pixel_buffer::keys::io_surf_props(), &empty_dict);
        image_attrs.insert(
            cv::pixel_buffer::keys::metal_compatability(),
            &cf::Boolean::value_true(),
        );
        image_attrs.insert(cv::pixel_buffer::keys::pixel_format(), &pixel_format);

        let mut session = Session::new::<std::ffi::c_void>(
            1920,
            1080,
            VideoCodec::H264,
            None,
            Some(&image_attrs),
            None,
            None,
            std::ptr::null_mut(),
        )
        .expect("encoder");

        let bool_true = cf::Boolean::value_true();
        let bool_false = cf::Boolean::value_false();
        let expected_fr = cf::Number::from_i32(60);
        let frame_delay_count = cf::Number::from_i32(0);

        let mut props = cf::DictionaryMut::with_capacity(10);
        props.insert(keys::real_time(), &bool_true);
        props.insert(keys::allow_frame_reordering(), &bool_false);
        props.insert(keys::profile_lvl(), profile_level::h264::main_auto_lvl());
        props.insert(keys::allow_open_gop(), &bool_false);
        props.insert(keys::h264_entropy_mode(), h264_entropy_mode::cabac());
        props.insert(keys::expected_frame_rate(), &expected_fr);
        props.insert(keys::max_frame_delay_count(), &frame_delay_count);

        session.set_props(&props).unwrap();
        session.prepare().unwrap();

        let pool = session.pixel_buf_pool().expect("pool");
        pool.show();

        session.supported_props().expect("props").show();

        session.show();
    }
}
