use crate::{
    cf::{self, Retained},
    os,
};

pub fn copy(
    options: Option<&cf::Dictionary>,
) -> Result<Retained<cf::ArrayOf<cf::Dictionary>>, os::Status> {
    unsafe {
        let mut list_of_video_encoders_out = None;
        VTCopyVideoEncoderList(options, &mut list_of_video_encoders_out)
            .to_result(list_of_video_encoders_out)
    }
}

extern "C" {
    fn VTCopyVideoEncoderList<'a>(
        options: Option<&cf::Dictionary>,
        list_of_video_encoders_out: &mut Option<Retained<cf::ArrayOf<cf::Dictionary>>>,
    ) -> os::Status;
}

#[cfg(test)]
mod tests {

    #[test]
    fn copy() {
        super::copy(None).expect("list").show()
    }
}
