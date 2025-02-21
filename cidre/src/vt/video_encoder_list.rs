use crate::{arc, cf, cm, os};

#[doc(alias = "VTCopyVideoEncoderList")]
pub fn copy() -> os::Result<arc::R<cf::ArrayOf<cf::Dictionary>>> {
    unsafe { os::result_unchecked(|res| VTCopyVideoEncoderList(None, res)) }
}

#[doc(alias = "VTCopySupportedPropertyDictionaryForEncoder")]
pub fn supported_props_for_encoder(
    width: i32,
    height: i32,
    codec: cm::VideoCodec,
    encoder_spec: Option<&cf::Dictionary>,
    encoder_id_out: &mut Option<arc::R<cf::String>>,
    supported_props_out: &mut Option<arc::R<cf::Dictionary>>,
) -> os::Result<()> {
    unsafe {
        VTCopySupportedPropertyDictionaryForEncoder(
            width,
            height,
            codec,
            encoder_spec,
            encoder_id_out,
            supported_props_out,
        )
        .result()
    }
}

unsafe extern "C-unwind" {
    fn VTCopyVideoEncoderList(
        // Not currently supported. Pass NULL for this parameter.
        options: Option<&cf::Dictionary>,
        list_of_video_encoders_out: *mut Option<arc::R<cf::ArrayOf<cf::Dictionary>>>,
    ) -> os::Status;

    fn VTCopySupportedPropertyDictionaryForEncoder(
        width: i32,
        height: i32,
        codec: cm::VideoCodec,
        encoder_spec: Option<&cf::Dictionary>,
        encoder_id_out: *mut Option<arc::R<cf::String>>,
        supported_props_out: *mut Option<arc::R<cf::Dictionary>>,
    ) -> os::Status;
}

#[cfg(test)]
mod tests {
    use crate::cm;

    #[test]
    fn copy() {
        super::copy()
            .expect("failed to copy list of encoders")
            .show();

        let mut encoder_id = None;
        let mut supported_props = None;
        super::supported_props_for_encoder(
            1920,
            1080,
            cm::VideoCodec::H264,
            None,
            &mut encoder_id,
            &mut supported_props,
        )
        .unwrap();

        assert!(encoder_id.unwrap().len() > 0);
        assert!(supported_props.unwrap().len() > 0);
    }
}
