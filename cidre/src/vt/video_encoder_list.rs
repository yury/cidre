use crate::{arc, cf, os};

pub fn copy() -> Result<arc::R<cf::ArrayOf<cf::Dictionary>>, os::Status> {
    unsafe {
        let mut list_of_video_encoders_out = None;
        VTCopyVideoEncoderList(None, &mut list_of_video_encoders_out)
            .to_result_unchecked(list_of_video_encoders_out)
    }
}

extern "C" {
    fn VTCopyVideoEncoderList(
        // Not currently supported. Pass NULL for this parameter.
        options: Option<&cf::Dictionary>,
        list_of_video_encoders_out: *mut Option<arc::R<cf::ArrayOf<cf::Dictionary>>>,
    ) -> os::Status;
}

#[cfg(test)]
mod tests {

    #[test]
    fn copy() {
        super::copy().expect("list").show()
    }
}
