use crate::{define_obj_type, ns};

define_obj_type!(
    #[doc(alias = "NLLanguage")]
    pub Lang(ns::String)
);

impl Lang {
    #[inline]
    pub fn with_string(string: &ns::String) -> &Self {
        unsafe { std::mem::transmute(string) }
    }

    /// The unique identifier string for a language the Natural Language framework
    /// doesn’t recognize
    #[inline]
    pub fn undetermined() -> &'static Self {
        unsafe { NLLanguageUndetermined }
    }

    /// The unique identifier string for the Amharic language.
    #[inline]
    pub fn amharic() -> &'static Self {
        unsafe { NLLanguageAmharic }
    }

    /// The unique identifier string for the Arabic language.
    #[inline]
    pub fn arabic() -> &'static Self {
        unsafe { NLLanguageArabic }
    }

    /// The unique identifier string for the Armenian language.
    #[inline]
    pub fn armenian() -> &'static Self {
        unsafe { NLLanguageArmenian }
    }

    /// The unique identifier string for the Bengali language.
    #[inline]
    pub fn bengali() -> &'static Self {
        unsafe { NLLanguageBengali }
    }

    /// The unique identifier string for the Bulgarian language.
    #[inline]
    pub fn bulgarian() -> &'static Self {
        unsafe { NLLanguageBulgarian }
    }

    /// The unique identifier string for the Burmese language.
    #[inline]
    pub fn burmese() -> &'static Self {
        unsafe { NLLanguageBurmese }
    }

    /// The unique identifier string for the Catalan language.
    #[inline]
    pub fn catalan() -> &'static Self {
        unsafe { NLLanguageCatalan }
    }

    /// The unique identifier string for the Cherokee language.
    #[inline]
    pub fn cherokee() -> &'static Self {
        unsafe { NLLanguageCherokee }
    }

    /// The unique identifier string for the Croatian language.
    #[inline]
    pub fn croatian() -> &'static Self {
        unsafe { NLLanguageCroatian }
    }

    /// The unique identifier string for the Czech language.
    #[inline]
    pub fn czech() -> &'static Self {
        unsafe { NLLanguageCzech }
    }

    /// The unique identifier string for the Danish language.
    #[inline]
    pub fn danish() -> &'static Self {
        unsafe { NLLanguageDanish }
    }

    /// The unique identifier string for the Dutch language.
    #[inline]
    pub fn dutch() -> &'static Self {
        unsafe { NLLanguageDutch }
    }

    /// The unique identifier string for the English language.
    #[inline]
    pub fn english() -> &'static Self {
        unsafe { NLLanguageEnglish }
    }

    /// The unique identifier string for the Finnish language.
    #[inline]
    pub fn finnish() -> &'static Self {
        unsafe { NLLanguageFinnish }
    }

    /// The unique identifier string for the French language.
    #[inline]
    pub fn french() -> &'static Self {
        unsafe { NLLanguageFrench }
    }

    /// The unique identifier string for the Georgian language.
    #[inline]
    pub fn georgian() -> &'static Self {
        unsafe { NLLanguageGeorgian }
    }

    /// The unique identifier string for the German language.
    #[inline]
    pub fn german() -> &'static Self {
        unsafe { NLLanguageGerman }
    }

    /// The unique identifier string for the Greek language.
    #[inline]
    pub fn greek() -> &'static Self {
        unsafe { NLLanguageGreek }
    }

    /// The unique identifier string for the Gujarati language.
    #[inline]
    pub fn gujarati() -> &'static Self {
        unsafe { NLLanguageGujarati }
    }

    /// The unique identifier string for the Hebrew language.
    #[inline]
    pub fn hebrew() -> &'static Self {
        unsafe { NLLanguageHebrew }
    }

    /// The unique identifier string for the Hindi language.
    #[inline]
    pub fn hindi() -> &'static Self {
        unsafe { NLLanguageHindi }
    }

    /// The unique identifier string for the Hungarian language.
    #[inline]
    pub fn hungarian() -> &'static Self {
        unsafe { NLLanguageHungarian }
    }

    /// The unique identifier string for the Icelandic language.
    #[inline]
    pub fn icelandic() -> &'static Self {
        unsafe { NLLanguageIcelandic }
    }

    /// The unique identifier string for the Indonesian language.
    #[inline]
    pub fn indonesian() -> &'static Self {
        unsafe { NLLanguageIndonesian }
    }

    /// The unique identifier string for the Italian language.
    #[inline]
    pub fn italian() -> &'static Self {
        unsafe { NLLanguageItalian }
    }

    /// The unique identifier string for the Japanese language.
    #[inline]
    pub fn japanese() -> &'static Self {
        unsafe { NLLanguageJapanese }
    }

    /// The unique identifier string for the Kannada language.
    #[inline]
    pub fn kannada() -> &'static Self {
        unsafe { NLLanguageKannada }
    }

    /// The unique identifier string for the Khmer language.
    #[inline]
    pub fn khmer() -> &'static Self {
        unsafe { NLLanguageKhmer }
    }

    /// The unique identifier string for the Korean language.
    #[inline]
    pub fn korean() -> &'static Self {
        unsafe { NLLanguageKorean }
    }

    /// The unique identifier string for the Lao language.
    #[inline]
    pub fn lao() -> &'static Self {
        unsafe { NLLanguageLao }
    }

    /// The unique identifier string for the Malay language.
    #[inline]
    pub fn malay() -> &'static Self {
        unsafe { NLLanguageMalay }
    }

    /// The unique identifier string for the Malayalam language.
    #[inline]
    pub fn malayalam() -> &'static Self {
        unsafe { NLLanguageMalayalam }
    }

    /// The unique identifier string for the Marathi language.
    #[inline]
    pub fn marathi() -> &'static Self {
        unsafe { NLLanguageMarathi }
    }

    /// The unique identifier string for the Mongolian language.
    #[inline]
    pub fn mongolian() -> &'static Self {
        unsafe { NLLanguageMongolian }
    }

    /// The unique identifier string for the Norwegian language.
    #[inline]
    pub fn norwegian() -> &'static Self {
        unsafe { NLLanguageNorwegian }
    }

    /// The unique identifier string for the Oriya language.
    #[inline]
    pub fn oriya() -> &'static Self {
        unsafe { NLLanguageOriya }
    }

    /// The unique identifier string for the Persian language.
    #[inline]
    pub fn persian() -> &'static Self {
        unsafe { NLLanguagePersian }
    }

    /// The unique identifier string for the Polish language.
    #[inline]
    pub fn polish() -> &'static Self {
        unsafe { NLLanguagePolish }
    }

    /// The unique identifier string for the Portuguese language.
    #[inline]
    pub fn portuguese() -> &'static Self {
        unsafe { NLLanguagePortuguese }
    }

    /// The unique identifier string for the Punjabi language.
    #[inline]
    pub fn punjabi() -> &'static Self {
        unsafe { NLLanguagePunjabi }
    }

    /// The unique identifier string for the Romanian language.
    #[inline]
    pub fn romanian() -> &'static Self {
        unsafe { NLLanguageRomanian }
    }

    /// The unique identifier string for the Russian language.
    #[inline]
    pub fn russian() -> &'static Self {
        unsafe { NLLanguageRussian }
    }

    /// The unique identifier string for the Simplified Chinese language.
    #[inline]
    pub fn simplified_chinese() -> &'static Self {
        unsafe { NLLanguageSimplifiedChinese }
    }

    /// The unique identifier string for the Sinhalese language.
    #[inline]
    pub fn sinhalese() -> &'static Self {
        unsafe { NLLanguageSinhalese }
    }

    /// The unique identifier string for the Slovak language.
    #[inline]
    pub fn slovak() -> &'static Self {
        unsafe { NLLanguageSlovak }
    }

    /// The unique identifier string for the Spanish language.
    #[inline]
    pub fn spanish() -> &'static Self {
        unsafe { NLLanguageSpanish }
    }

    /// The unique identifier string for the Swedish language.
    #[inline]
    pub fn swedish() -> &'static Self {
        unsafe { NLLanguageSwedish }
    }

    /// The unique identifier string for the Tamil language.
    #[inline]
    pub fn tamil() -> &'static Self {
        unsafe { NLLanguageTamil }
    }

    /// The unique identifier string for the Telugu language.
    #[inline]
    pub fn telugu() -> &'static Self {
        unsafe { NLLanguageTelugu }
    }

    /// The unique identifier string for the Thai language.
    #[inline]
    pub fn thai() -> &'static Self {
        unsafe { NLLanguageThai }
    }

    /// The unique identifier string for the Tibetan language.
    #[inline]
    pub fn tibetan() -> &'static Self {
        unsafe { NLLanguageTibetan }
    }

    /// The unique identifier string for the Traditional Chinese language.
    #[inline]
    pub fn traditional_chinese() -> &'static Self {
        unsafe { NLLanguageTraditionalChinese }
    }

    /// The unique identifier string for the Turkish language.
    #[inline]
    pub fn turkish() -> &'static Self {
        unsafe { NLLanguageTurkish }
    }

    /// The unique identifier string for the Ukrainian language.
    #[inline]
    pub fn ukrainian() -> &'static Self {
        unsafe { NLLanguageUkrainian }
    }

    /// The unique identifier string for the Urdu language.
    #[inline]
    pub fn urdu() -> &'static Self {
        unsafe { NLLanguageUrdu }
    }

    /// The unique identifier string for the Vietnamese language.
    #[inline]
    pub fn vietnamese() -> &'static Self {
        unsafe { NLLanguageVietnamese }
    }

    /// The unique identifier string for the Kazakh language.
    #[inline]
    pub fn kazakh() -> &'static Self {
        unsafe { NLLanguageKazakh }
    }
}

#[link(name = "NaturalLanguage", kind = "framework")]
unsafe extern "C" {
    static NLLanguageUndetermined: &'static Lang;
    static NLLanguageAmharic: &'static Lang;
    static NLLanguageArabic: &'static Lang;
    static NLLanguageArmenian: &'static Lang;
    static NLLanguageBengali: &'static Lang;
    static NLLanguageBulgarian: &'static Lang;
    static NLLanguageBurmese: &'static Lang;
    static NLLanguageCatalan: &'static Lang;
    static NLLanguageCherokee: &'static Lang;
    static NLLanguageCroatian: &'static Lang;
    static NLLanguageCzech: &'static Lang;
    static NLLanguageDanish: &'static Lang;
    static NLLanguageDutch: &'static Lang;
    static NLLanguageEnglish: &'static Lang;
    static NLLanguageFinnish: &'static Lang;
    static NLLanguageFrench: &'static Lang;
    static NLLanguageGeorgian: &'static Lang;
    static NLLanguageGerman: &'static Lang;
    static NLLanguageGreek: &'static Lang;
    static NLLanguageGujarati: &'static Lang;
    static NLLanguageHebrew: &'static Lang;
    static NLLanguageHindi: &'static Lang;
    static NLLanguageHungarian: &'static Lang;
    static NLLanguageIcelandic: &'static Lang;
    static NLLanguageIndonesian: &'static Lang;
    static NLLanguageItalian: &'static Lang;
    static NLLanguageJapanese: &'static Lang;
    static NLLanguageKannada: &'static Lang;
    static NLLanguageKhmer: &'static Lang;
    static NLLanguageKorean: &'static Lang;
    static NLLanguageLao: &'static Lang;
    static NLLanguageMalay: &'static Lang;
    static NLLanguageMalayalam: &'static Lang;
    static NLLanguageMarathi: &'static Lang;
    static NLLanguageMongolian: &'static Lang;
    static NLLanguageNorwegian: &'static Lang;
    static NLLanguageOriya: &'static Lang;
    static NLLanguagePersian: &'static Lang;
    static NLLanguagePolish: &'static Lang;
    static NLLanguagePortuguese: &'static Lang;
    static NLLanguagePunjabi: &'static Lang;
    static NLLanguageRomanian: &'static Lang;
    static NLLanguageRussian: &'static Lang;
    static NLLanguageSimplifiedChinese: &'static Lang;
    static NLLanguageSinhalese: &'static Lang;
    static NLLanguageSlovak: &'static Lang;
    static NLLanguageSpanish: &'static Lang;
    static NLLanguageSwedish: &'static Lang;
    static NLLanguageTamil: &'static Lang;
    static NLLanguageTelugu: &'static Lang;
    static NLLanguageThai: &'static Lang;
    static NLLanguageTibetan: &'static Lang;
    static NLLanguageTraditionalChinese: &'static Lang;
    static NLLanguageTurkish: &'static Lang;
    static NLLanguageUkrainian: &'static Lang;
    static NLLanguageUrdu: &'static Lang;
    static NLLanguageVietnamese: &'static Lang;
    static NLLanguageKazakh: &'static Lang;

}
