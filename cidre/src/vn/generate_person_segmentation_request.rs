use crate::{arc, define_obj_type, ns, objc, os, vn};

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
    GeneratePersonSegmentationRequest(vn::StatefulRequest),
    VN_GENERATE_PERSON_SEGMENTAION_REQUEST
);

impl GeneratePersonSegmentationRequest {
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
    pub fn results(&self) -> Option<&ns::Array<vn::PixelBufferObservation>>;
}

#[link(name = "vn", kind = "static")]
extern "C" {
    static VN_GENERATE_PERSON_SEGMENTAION_REQUEST:
        &'static objc::Class<GeneratePersonSegmentationRequest>;
}

#[cfg(test)]
mod tests {
    use crate::{
        cv::PixelFormat, vn::generate_person_segmentation_request::QualityLevel,
        vn::GeneratePersonSegmentationRequest,
    };

    #[test]
    fn basics() {
        let request = GeneratePersonSegmentationRequest::new();

        assert_eq!(request.quality_level(), QualityLevel::Accurate);

        assert_eq!(
            request.output_pixel_format(),
            PixelFormat::ONE_COMPONENT_8.0
        );

        assert!(request.results().is_none());
    }
}
