use crate::{arc, cf, define_obj_type, msg_send, vn};

#[repr(isize)]
pub enum RecognitionLevel {
    Accurate = 0,
    Fast,
}

define_obj_type!(RecognizeTextRequest(vn::ImageBasedRequest));

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

    #[inline]
    pub fn results(&self) -> Option<&cf::ArrayOf<vn::RecognizedTextObservation>> {
        msg_send!("vn", self, sel_results)
    }

    pub fn new() -> arc::R<Self> {
        unsafe { VNRecognizeTextRequest_new() }
    }
}

#[link(name = "vn", kind = "static")]
extern "C" {
    fn VNRecognizeTextRequest_new() -> arc::R<RecognizeTextRequest>;
}
