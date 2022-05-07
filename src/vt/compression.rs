pub mod session;
pub use session::Session;

pub mod properties;
pub use properties::h264_entropy_mode;
pub use properties::hdr_metadata_insertion_mode;
pub use properties::keys;
pub use properties::profile_level;

#[cfg(test)]
mod tests {
    use crate::cm::VideoCodecType;

    use super::*;

    #[test]
    fn session_new() {
        let session = Session::new(
            1920,
            1080,
            VideoCodecType::H264,
            None,
            None,
            None,
            std::ptr::null_mut(),
        )
        .expect("encoder");

        session.show();
    }
}
