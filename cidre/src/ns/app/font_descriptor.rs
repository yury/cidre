use crate::{define_obj_type, ns};

define_obj_type!(
    #[doc(alias = "NSFontTextStyle")]
    pub FontTextStyle(ns::String)
);

/// Text styles, for `ns::Font::preferred_font_for_text_style`.
impl FontTextStyle {
    #[doc(alias = "NSFontTextStyleLargeTitle")]
    pub fn large_title() -> &'static Self {
        unsafe { NSFontTextStyleLargeTitle }
    }

    #[doc(alias = "NSFontTextStyleTitle1")]
    pub fn title1() -> &'static Self {
        unsafe { NSFontTextStyleTitle1 }
    }

    #[doc(alias = "NSFontTextStyleTitle2")]
    pub fn title2() -> &'static Self {
        unsafe { NSFontTextStyleTitle2 }
    }

    #[doc(alias = "NSFontTextStyleTitle3")]
    pub fn title3() -> &'static Self {
        unsafe { NSFontTextStyleTitle3 }
    }

    #[doc(alias = "NSFontTextStyleHeadline")]
    pub fn headline() -> &'static Self {
        unsafe { NSFontTextStyleHeadline }
    }

    #[doc(alias = "NSFontTextStyleSubheadline")]
    pub fn subheadline() -> &'static Self {
        unsafe { NSFontTextStyleSubheadline }
    }

    #[doc(alias = "NSFontTextStyleBody")]
    pub fn body() -> &'static Self {
        unsafe { NSFontTextStyleBody }
    }

    #[doc(alias = "NSFontTextStyleCallout")]
    pub fn callout() -> &'static Self {
        unsafe { NSFontTextStyleCallout }
    }

    #[doc(alias = "NSFontTextStyleFootnote")]
    pub fn footnote() -> &'static Self {
        unsafe { NSFontTextStyleFootnote }
    }

    #[doc(alias = "NSFontTextStyleCaption1")]
    pub fn caption1() -> &'static Self {
        unsafe { NSFontTextStyleCaption1 }
    }

    #[doc(alias = "NSFontTextStyleCaption2")]
    pub fn caption2() -> &'static Self {
        unsafe { NSFontTextStyleCaption2 }
    }
}

unsafe extern "C" {
    static NSFontTextStyleLargeTitle: &'static FontTextStyle;
    static NSFontTextStyleTitle1: &'static FontTextStyle;
    static NSFontTextStyleTitle2: &'static FontTextStyle;
    static NSFontTextStyleTitle3: &'static FontTextStyle;
    static NSFontTextStyleHeadline: &'static FontTextStyle;
    static NSFontTextStyleSubheadline: &'static FontTextStyle;
    static NSFontTextStyleBody: &'static FontTextStyle;
    static NSFontTextStyleCallout: &'static FontTextStyle;
    static NSFontTextStyleFootnote: &'static FontTextStyle;
    static NSFontTextStyleCaption1: &'static FontTextStyle;
    static NSFontTextStyleCaption2: &'static FontTextStyle;
}
