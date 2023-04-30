use crate::{arc, define_obj_type, ns, objc, vn};

define_obj_type!(
    DetectBarcodesRequest(vn::ImageBasedRequest),
    VN_DETECT_BARCODES_REQUEST
);

impl DetectBarcodesRequest {
    pub const REVISION_1: usize = 1;
    pub const REVISION_2: usize = 2;
    pub const REVISION_3: usize = 3;

    #[objc::msg_send(results)]
    pub fn results(&self) -> Option<&ns::Array<vn::BarcodeObservation>>;

    #[objc::msg_send(symbologies)]
    pub fn symbologies(&self) -> &ns::Array<vn::BarcodeSymbology>;

    #[objc::msg_send(setSymbologies:)]
    pub fn set_symbologies(&mut self, value: &ns::Array<vn::BarcodeSymbology>);

    #[objc::msg_send(supportedSymbologiesAndReturnError:)]
    fn supported_symbologies_and_return_err_ar(
        &self,
        error: &mut Option<&ns::Error>,
    ) -> Option<&'ar ns::Array<vn::BarcodeSymbology>>;

    #[objc::rar_retain()]
    fn supported_symbologies_and_return_err(
        &self,
        error: &mut Option<&ns::Error>,
    ) -> Option<arc::R<ns::Array<vn::BarcodeSymbology>>>;

    pub fn supported_symbologies<'ar>(
        &self,
    ) -> Result<arc::R<ns::Array<vn::BarcodeSymbology>>, &'ar ns::Error> {
        let mut error = None;
        let res = self.supported_symbologies_and_return_err(&mut error);
        if let Some(r) = res {
            Ok(r)
        } else {
            Err(error.unwrap())
        }
    }
}

#[link(name = "vn", kind = "static")]
extern "C" {
    static VN_DETECT_BARCODES_REQUEST: &'static objc::Class<DetectBarcodesRequest>;
}

#[cfg(test)]
mod tests {
    use crate::vn;

    #[test]
    fn basics() {
        let request = vn::DetectBarcodesRequest::new();
        let symbologies = request.symbologies();

        assert!(!symbologies.is_empty());

        let supported_symbologies = request.supported_symbologies().unwrap();

        assert!(supported_symbologies.contains(vn::BarcodeSymbology::qr()));
        assert!(symbologies.contains(vn::BarcodeSymbology::qr()));
    }
}
