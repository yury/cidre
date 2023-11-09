use crate::{arc, cf, os};

pub fn copy(
    options: Option<&cf::Dictionary>,
) -> Result<arc::R<cf::ArrayOf<cf::Dictionary>>, os::Status> {
    unsafe {
        let mut list_of_video_encoders_out = None;
        VTCopyVideoEncoderList(options, &mut list_of_video_encoders_out)
            .to_result_unchecked(list_of_video_encoders_out)
    }
}

extern "C" {
    fn VTCopyVideoEncoderList(
        options: Option<&cf::Dictionary>,
        list_of_video_encoders_out: *mut Option<arc::R<cf::ArrayOf<cf::Dictionary>>>,
    ) -> os::Status;
}

#[cfg(test)]
mod tests {

    #[test]
    fn copy() {
        super::copy(None).expect("list").show()
    }
}
