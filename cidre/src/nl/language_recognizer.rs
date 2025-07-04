use crate::{arc, define_obj_type, nl, ns, objc};

define_obj_type!(
    #[doc(alias = "NLLanguageRecognizer")]
    pub LangRecognizer(ns::Id),
    NL_LANGUAGE_RECOGNIZER
);

impl LangRecognizer {
    #[objc::msg_send(dominantLanguageForString:)]
    pub fn dominant_lang_for_string(str: &ns::String) -> Option<arc::R<nl::Lang>>;

    #[objc::msg_send(processString:)]
    pub fn process_string(&mut self, str: &ns::String);

    #[objc::msg_send(reset)]
    pub fn reset(&mut self);

    #[objc::msg_send(dominantLanguage)]
    pub fn dominant_lang(&self) -> Option<arc::R<nl::Lang>>;

    #[objc::msg_send(languageHypothesesWithMaximum:)]
    pub fn lang_hypotheses(&self, max: usize) -> arc::R<ns::Dictionary<nl::Lang, ns::Number>>;

    #[objc::msg_send(languageHints)]
    pub fn lang_hints(&self) -> arc::R<ns::Dictionary<nl::Lang, ns::Number>>;

    #[objc::msg_send(setLanguageHints:)]
    pub fn set_lang_hints(&mut self, val: &ns::Dictionary<nl::Lang, ns::Number>);

    #[objc::msg_send(languageConstraints)]
    pub fn lang_constraints(&self) -> arc::R<ns::Array<nl::Lang>>;

    #[objc::msg_send(setLanguageConstraints:)]
    pub fn set_lang_constraints(&mut self, val: &ns::Array<nl::Lang>);
}

#[link(name = "nl", kind = "static")]
unsafe extern "C" {
    static NL_LANGUAGE_RECOGNIZER: &'static objc::Class<LangRecognizer>;
}

#[cfg(test)]
mod tests {
    use crate::{api, nl, ns};
    #[test]
    fn basics() {
        assert!(nl::LangRecognizer::dominant_lang_for_string(ns::str!(c"")).is_none());

        let lang = nl::LangRecognizer::dominant_lang_for_string(ns::str!(c"cidre"))
            .expect("failed to recognize language");
        let fr_str = nl::Lang::with_string(ns::str!(c"fr"));
        let nl_str = nl::Lang::with_string(ns::str!(c"nl"));

        if api::macos_available("26.0") {
            assert_eq!(nl_str, &lang);
        } else {
            assert_eq!(fr_str, &lang);
        }

        let mut recognizer = nl::LangRecognizer::new();

        assert!(recognizer.dominant_lang().is_none());

        assert!(recognizer.lang_hypotheses(10).is_empty());

        recognizer.process_string(ns::str!(c"cidre"));
        if api::macos_available("26.0") {
            assert_eq!(nl_str, &recognizer.dominant_lang().unwrap());
        } else {
            assert_eq!(fr_str, &recognizer.dominant_lang().unwrap());
        }
        assert_eq!(10, recognizer.lang_hypotheses(10).len());

        assert!(recognizer.lang_hints().is_empty());
    }
}
