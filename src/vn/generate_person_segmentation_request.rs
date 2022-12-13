use crate::{cf, define_obj_type, msg_send, ns, os, vn};

/// Person segmentation level options to favor speed over recognition accuracy.
/// Accurate is the default option.
#[derive(Debug, Eq, PartialEq)]
#[repr(usize)]
pub enum QualityLevel {
    Accurate = 0,
    Balanced,
    Fast,
}

define_obj_type!(GeneratePersonSegmentationRequest(vn::StatefulRequest));

impl GeneratePersonSegmentationRequest {
    pub const REVISION_1: usize = 1;

    #[inline]
    pub fn quality_level(&self) -> QualityLevel {
        unsafe { rsel_qualityLevel(self) }
    }

    #[inline]
    pub fn set_quality_level(&mut self, value: QualityLevel) {
        unsafe { wsel_setQualityLevel(self, value) }
    }

    #[inline]
    pub fn output_pixel_format(&self) -> os::Type {
        unsafe { rsel_outputPixelFormat(self) }
    }

    #[inline]
    pub fn set_output_pixel_format(&mut self, value: os::Type) {
        unsafe { wsel_setOutputPixelFormat(self, value) }
    }

    #[inline]
    pub fn results(&self) -> Option<&cf::ArrayOf<vn::PixelBufferObservation>> {
        msg_send!("vn", self, sel_results)
    }

    #[inline]
    pub fn new() -> cf::Retained<GeneratePersonSegmentationRequest> {
        unsafe { VNGeneratePersonSegmentationRequest_new() }
    }
}

#[link(name = "vn", kind = "static")]
extern "C" {
    fn rsel_qualityLevel(id: &ns::Id) -> QualityLevel;
    fn wsel_setQualityLevel(id: &mut ns::Id, value: QualityLevel);

    fn rsel_outputPixelFormat(id: &ns::Id) -> os::Type;
    fn wsel_setOutputPixelFormat(id: &mut ns::Id, value: os::Type);

    fn VNGeneratePersonSegmentationRequest_new() -> cf::Retained<GeneratePersonSegmentationRequest>;
}

#[cfg(test)]
mod tests {
    use crate::{
        cv::PixelFormatType, vn::generate_person_segmentation_request::QualityLevel,
        vn::GeneratePersonSegmentationRequest,
    };

    #[test]
    fn basics() {
        let request = GeneratePersonSegmentationRequest::new();

        assert_eq!(request.quality_level(), QualityLevel::Accurate);

        assert_eq!(
            request.output_pixel_format(),
            PixelFormatType::ONE_COMPONENT_8.0
        );

        assert!(request.results().is_none());
    }
}
