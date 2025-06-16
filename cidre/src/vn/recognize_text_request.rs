use crate::{arc, define_obj_type, ns, objc, vn};

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
#[repr(isize)]
pub enum RecognitionLevel {
    Accurate = 0,
    Fast,
}

define_obj_type!(
    pub RecognizeTextRequest(vn::ImageBasedRequest),
    VN_RECOGNIZE_TEXT_REQUEST
);

impl vn::RequestProgressProviding for RecognizeTextRequest {}

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

    #[objc::msg_send(supportedRecognitionLanguagesAndReturnError:)]
    #[objc::available(
        macos = 12.0,
        maccatalyst = 15.0,
        ios = 15.0,
        tvos = 15.0,
        visionos = 1.0
    )]
    pub unsafe fn supported_recognition_langs_err<'ear>(
        &self,
        err: *mut Option<&'ear ns::Error>,
    ) -> Option<arc::R<ns::Array<ns::String>>>;

    #[objc::available(
        macos = 12.0,
        maccatalyst = 15.0,
        ios = 15.0,
        tvos = 15.0,
        visionos = 1.0
    )]
    pub fn supported_recognition_langs<'ear>(
        &self,
    ) -> ns::Result<'ear, arc::R<ns::Array<ns::String>>> {
        ns::if_none(|err| unsafe { self.supported_recognition_langs_err(err) })
    }

    #[objc::msg_send(recognitionLanguages)]
    pub fn recognition_langs(&self) -> arc::R<ns::Array<ns::String>>;

    #[objc::msg_send(setRecognitionLanguages:)]
    pub fn set_recognition_langs(&mut self, val: &ns::Array<ns::String>);

    #[objc::msg_send(customWords)]
    pub fn custom_words(&self) -> arc::R<ns::Array<ns::String>>;

    #[objc::msg_send(setCustomWords:)]
    pub fn set_custom_words(&mut self, val: &ns::Array<ns::String>);

    #[objc::msg_send(recognitionLevel)]
    pub fn recognition_level(&self) -> RecognitionLevel;

    #[objc::msg_send(setRecognitionLevel:)]
    pub fn set_recognition_level(&mut self, val: RecognitionLevel);

    #[objc::msg_send(usesLanguageCorrection)]
    pub fn uses_lang_correction(&self) -> bool;

    #[objc::msg_send(setUsesLanguageCorrection:)]
    pub fn set_uses_lang_correction(&mut self, val: bool);

    #[objc::msg_send(automaticallyDetectsLanguage)]
    #[objc::available(
        macos = 13.0,
        maccatalyst = 16.0,
        ios = 16.0,
        tvos = 16.0,
        visionos = 1.0
    )]
    pub fn automatically_detects_lang(&self) -> bool;

    #[objc::msg_send(setAutomaticallyDetectsLanguage:)]
    #[objc::available(
        macos = 13.0,
        maccatalyst = 16.0,
        ios = 16.0,
        tvos = 16.0,
        visionos = 1.0
    )]
    pub fn set_automatically_detects_lang(&mut self, val: bool);

    #[objc::msg_send(minimumTextHeight)]
    pub fn min_text_height(&self) -> f32;

    #[objc::msg_send(setMinimumTextHeight:)]
    pub fn set_min_text_height(&mut self, val: f32);
}

#[link(name = "vn", kind = "static")]
unsafe extern "C" {
    static VN_RECOGNIZE_TEXT_REQUEST: &'static objc::Class<RecognizeTextRequest>;
}

#[cfg(test)]
mod tests {
    use crate::{ns, vn, vn::RequestProgressProviding};

    #[test]
    fn basics() {
        let mut request = vn::RecognizeTextRequest::new();
        request.set_min_text_height(10.0);
        let langs = request.supported_recognition_langs().unwrap();
        assert!(!langs.is_empty());

        request.set_uses_lang_correction(true);
        request.set_recognition_level(vn::RequestTextRecognitionLevel::Accurate);

        assert!(!request.automatically_detects_lang());
        request.set_automatically_detects_lang(true);
        assert!(request.automatically_detects_lang());

        assert!(request.custom_words().is_empty());

        assert!(request.progress_handler().is_none());
        assert!(!request.indeterminate());

        // test if api throws...

        request.set_min_text_height(-10.0);
        assert_eq!(request.min_text_height(), -10.0);

        request.set_recognition_langs(&ns::Array::new());
        assert!(request.recognition_langs().is_empty());
    }
}
