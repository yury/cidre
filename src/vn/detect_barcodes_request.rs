use std::mem::transmute;

use crate::{cf, define_obj_type, objc, vn};

define_obj_type!(DetectBarcodesRequest(vn::ImageBasedRequest));

impl DetectBarcodesRequest {
    pub const REVISION_1: usize = 1;
    pub const REVISION_2: usize = 2;
    pub const REVISION_3: usize = 3;

    pub fn results(&self) -> Option<&cf::ArrayOf<vn::BarcodeObservation>> {
        unsafe { transmute(rsel_results(self)) }
    }

    pub fn new() -> cf::Retained<Self> {
        unsafe { VNDetectBarcodesRequest_new() }
    }

    pub fn symbologies(&self) -> &cf::ArrayOf<vn::BarcodeSymbology> {
        unsafe { rsel_symbologies(self) }
    }

    pub fn set_symbologies(&mut self, value: &cf::ArrayOf<vn::BarcodeSymbology>) {
        unsafe { wsel_setSymbologies(self, value) }
    }
}

#[link(name = "vn", kind = "static")]
extern "C" {
    fn rsel_results(id: &objc::Id) -> Option<&cf::Array>;

    fn VNDetectBarcodesRequest_new() -> cf::Retained<DetectBarcodesRequest>;

    fn rsel_symbologies(id: &objc::Id) -> &cf::ArrayOf<vn::BarcodeSymbology>;

    fn wsel_setSymbologies(id: &mut objc::Id, value: &cf::ArrayOf<vn::BarcodeSymbology>);
}

#[cfg(test)]
mod tests {
    use crate::vn;
    #[test]
    fn basics() {
        let request = vn::DetectBarcodesRequest::new();
        let symbologies = request.symbologies();
        assert!(!symbologies.is_empty());
    }
}
