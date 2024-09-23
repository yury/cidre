use crate::{arc, cf, os};

pub fn copy() -> os::Result<arc::R<cf::ArrayOf<cf::Dictionary>>> {
    unsafe { os::result_unchecked(|res| VTCopyVideoEncoderList(None, res)) }
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
        super::copy()
            .expect("failed to copy list of encoders")
            .show()
    }
}
