use crate::{arc, define_obj_type, ns, objc, vn};

#[repr(isize)]
pub enum RecognitionLevel {
    Accurate = 0,
    Fast,
}

define_obj_type!(
    pub RecognizeTextRequest(vn::ImageBasedRequest),
    VN_RECOGNIZE_TEXT_REQUEST
);

impl RecognizeTextRequest {
    /// only supports English
    pub const REVISION_1: usize = 1;

    /// supports English, Chinese, Portuguese, French, Italian, German and Spanish
    /// in the accurate recognition level. The fast recognition level supports English,
    /// Portuguese, French, Italian, German and Spanish. Best practice is to use
    /// supportedRecognitionLanguagesForTextRecognitionLevel to check for supported languages.
    /// As the underlying engine has changed from VNRecognizeTextRequestRevision1, results can differ
    /// but are generally more accurate.
    pub const REVISION_2: usize = 2;

    /// supports all languages from previous revisions plus some new languages in the accurate recognition level.
    /// It also improves recognition capbilities for rotation and handwriting as well as support for
    /// the automatic language detection property. Best practice is to use
    /// supportedRecognitionLanguagesForTextRecognitionLevel to check for supported languages.
    /// As the underlying engine has changed from previous revsions, results can differ but are
    /// generally more accurate.
    pub const REVISION_3: usize = 3;

    #[objc::msg_send(results)]
    pub fn results(&self) -> Option<arc::R<ns::Array<vn::RecognizedTextObservation>>>;
}

#[link(name = "vn", kind = "static")]
extern "C" {
    static VN_RECOGNIZE_TEXT_REQUEST: &'static objc::Class<RecognizeTextRequest>;
}
