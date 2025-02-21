use crate::{api, arc, define_obj_type, ns, objc, os, vn};

/// Person segmentation level options to favor speed over recognition accuracy.
/// Accurate is the default option.
#[derive(Debug, Eq, PartialEq)]
#[repr(usize)]
pub enum QualityLevel {
    Accurate = 0,
    Balanced,
    Fast,
}

define_obj_type!(
    #[doc(alias = "VNGeneratePersonSegmentationRequest")]
    pub GenPersonSegmentationRequest(vn::StatefulRequest),
    VN_GENERATE_PERSON_SEGMENTAION_REQUEST
);

impl GenPersonSegmentationRequest {
    pub const REVISION_1: usize = 1;

    #[objc::msg_send(qualityLevel)]
    pub fn quality_level(&self) -> QualityLevel;

    #[objc::msg_send(setQualityLevel:)]
    pub fn set_quality_level(&mut self, value: QualityLevel);

    #[objc::msg_send(outputPixelFormat)]
    pub fn output_pixel_format(&self) -> os::Type;

    #[objc::msg_send(setOutputPixelFormat:)]
    pub fn set_output_pixel_format(&mut self, value: os::Type);

    #[objc::msg_send(results)]
    pub fn results(&self) -> Option<arc::R<ns::Array<vn::PixelBufObservation>>>;

    #[objc::msg_send(supportedOutputPixelFormatsAndReturnError:)]
    #[api::available(
        macos = 15.0,
        ios = 18.0,
        maccatalyst = 18.0,
        tvos = 18.0,
        visionos = 2.0
    )]
    pub unsafe fn supported_output_pixel_formats_err<'ear>(
        err: *mut Option<&'ear ns::Error>,
    ) -> Option<arc::R<ns::Array<ns::Number>>>;

    #[api::available(
        macos = 15.0,
        ios = 18.0,
        maccatalyst = 18.0,
        tvos = 18.0,
        visionos = 2.0
    )]
    pub fn supported_output_pixel_formats<'ear>() -> ns::Result<'ear, arc::R<ns::Array<ns::Number>>>
    {
        ns::if_none(|err| unsafe { Self::supported_output_pixel_formats_err(err) })
    }
}

#[link(name = "vn", kind = "static")]
unsafe extern "C" {
    static VN_GENERATE_PERSON_SEGMENTAION_REQUEST:
        &'static objc::Class<GenPersonSegmentationRequest>;
}

#[cfg(test)]
mod tests {
    use crate::{
        cv::PixelFormat, vn::GenPersonSegmentationRequest,
        vn::generate_person_segmentation_request::QualityLevel,
    };

    #[test]
    fn basics() {
        let request = GenPersonSegmentationRequest::new();

        assert_eq!(request.quality_level(), QualityLevel::Accurate);

        assert_eq!(
            request.output_pixel_format(),
            PixelFormat::ONE_COMPONENT_8.0
        );

        assert!(request.results().is_none());
    }
}
