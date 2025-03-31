use crate::{arc, cg, define_obj_type, define_opts, ns, objc};

define_opts!(
    #[doc(alias = "UIFontDescriptorSymbolicTraits")]
    pub FontDescSymbolicTraits(u32)
);
impl FontDescSymbolicTraits {
    pub const TRAIT_ITALIC: Self = Self(1 << 0);
    pub const TRAIT_BOLD: Self = Self(1 << 1);
    pub const TRAIT_EXPANDED: Self = Self(1 << 5);
    pub const TRAIT_CONDENSED: Self = Self(1 << 6);
    pub const TRAIT_MONOSPACE: Self = Self(1 << 10);
    pub const TRAIT_VERTICAL: Self = Self(1 << 11);
    pub const TRAIT_UI_OPTIMIZED: Self = Self(1 << 12);
    pub const TRAIT_TIGHT_LEADING: Self = Self(1 << 15);
    pub const TRAIT_LOOSE_LEADING: Self = Self(1 << 16);
    pub const CLASS_MASK: Self = Self(0xF0000000);
    pub const CLASS_UNKNOWN: Self = Self(0 << 28);
    pub const CLASS_OLD_STYLE_SERIFS: Self = Self(1 << 28);
    pub const CLASS_TRANSITIONAL_SERIFS: Self = Self(2 << 28);
    pub const CLASS_MODERN_SERIFS: Self = Self(3 << 28);
    pub const CLASS_CLARENDON_SERIFS: Self = Self(4 << 28);
    pub const CLASS_SLAB_SERIFS: Self = Self(5 << 28);
    pub const CLASS_FREEFORM_SERIFS: Self = Self(7 << 28);
    pub const CLASS_SANS_SERIF: Self = Self(8 << 28);
    pub const CLASS_ORNAMENTALS: Self = Self(9 << 28);
    pub const CLASS_SCRIPTS: Self = Self(10 << 28);
    pub const CLASS_SYMBOLIC: Self = Self(12 << 28);
}

pub type FontDescClass = ns::UInteger;
define_obj_type!(
    #[doc(alias = "UIFontTextStyle")]
    pub TextStyle(ns::String)
);

impl TextStyle {
    pub fn large_title() -> &'static Self {
        unsafe { UIFontTextStyleLargeTitle }
    }

    pub fn extra_large_title() -> &'static Self {
        unsafe { UIFontTextStyleExtraLargeTitle }
    }

    pub fn extra_large_title2() -> &'static Self {
        unsafe { UIFontTextStyleExtraLargeTitle2 }
    }

    pub fn title1() -> &'static Self {
        unsafe { UIFontTextStyleTitle1 }
    }

    pub fn title2() -> &'static Self {
        unsafe { UIFontTextStyleTitle2 }
    }

    pub fn title3() -> &'static Self {
        unsafe { UIFontTextStyleTitle3 }
    }

    pub fn headline() -> &'static Self {
        unsafe { UIFontTextStyleHeadline }
    }

    pub fn subheadline() -> &'static Self {
        unsafe { UIFontTextStyleSubheadline }
    }

    pub fn body() -> &'static Self {
        unsafe { UIFontTextStyleBody }
    }

    pub fn callout() -> &'static Self {
        unsafe { UIFontTextStyleCallout }
    }

    pub fn footnote() -> &'static Self {
        unsafe { UIFontTextStyleFootnote }
    }

    pub fn caption1() -> &'static Self {
        unsafe { UIFontTextStyleCaption1 }
    }

    pub fn caption2() -> &'static Self {
        unsafe { UIFontTextStyleCaption2 }
    }
}

#[link(name = "UIKit", kind = "framework")]
unsafe extern "C" {
    static UIFontTextStyleLargeTitle: &'static TextStyle;
    static UIFontTextStyleExtraLargeTitle: &'static TextStyle;
    static UIFontTextStyleExtraLargeTitle2: &'static TextStyle;
    static UIFontTextStyleTitle1: &'static TextStyle;
    static UIFontTextStyleTitle2: &'static TextStyle;
    static UIFontTextStyleTitle3: &'static TextStyle;
    static UIFontTextStyleHeadline: &'static TextStyle;
    static UIFontTextStyleSubheadline: &'static TextStyle;
    static UIFontTextStyleBody: &'static TextStyle;
    static UIFontTextStyleCallout: &'static TextStyle;
    static UIFontTextStyleFootnote: &'static TextStyle;
    static UIFontTextStyleCaption1: &'static TextStyle;
    static UIFontTextStyleCaption2: &'static TextStyle;
}

#[derive(Clone, Copy, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct FontWeight(pub cg::Float);

impl FontWeight {
    pub fn ultra_light() -> Self {
        unsafe { UIFontWeightUltraLight }
    }

    pub fn thin() -> Self {
        unsafe { UIFontWeightThin }
    }

    pub fn light() -> Self {
        unsafe { UIFontWeightLight }
    }

    pub fn regular() -> Self {
        unsafe { UIFontWeightRegular }
    }

    pub fn medium() -> Self {
        unsafe { UIFontWeightMedium }
    }

    pub fn semibold() -> Self {
        unsafe { UIFontWeightSemibold }
    }

    pub fn bold() -> Self {
        unsafe { UIFontWeightBold }
    }

    pub fn heavy() -> Self {
        unsafe { UIFontWeightHeavy }
    }

    pub fn black() -> Self {
        unsafe { UIFontWeightBlack }
    }
}

#[link(name = "UIKit", kind = "framework")]
unsafe extern "C" {
    static UIFontWeightUltraLight: FontWeight;
    static UIFontWeightThin: FontWeight;
    static UIFontWeightLight: FontWeight;
    static UIFontWeightRegular: FontWeight;
    static UIFontWeightMedium: FontWeight;
    static UIFontWeightSemibold: FontWeight;
    static UIFontWeightBold: FontWeight;
    static UIFontWeightHeavy: FontWeight;
    static UIFontWeightBlack: FontWeight;
}

#[derive(Clone, Copy, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct FontWidth(pub cg::Float);

impl FontWidth {
    pub fn condensed() -> Self {
        unsafe { UIFontWidthCondensed }
    }

    pub fn standard() -> Self {
        unsafe { UIFontWidthStandard }
    }

    pub fn expanded() -> Self {
        unsafe { UIFontWidthExpanded }
    }

    pub fn compressed() -> Self {
        unsafe { UIFontWidthCompressed }
    }
}

#[link(name = "UIKit", kind = "framework")]
unsafe extern "C" {

    static UIFontWidthCondensed: FontWidth;
    static UIFontWidthStandard: FontWidth;
    static UIFontWidthExpanded: FontWidth;
    static UIFontWidthCompressed: FontWidth;
}

define_obj_type!(
    #[doc(alias = "UIFontDescriptor")]
    pub FontDesc(ns::Id),
    UI_FONT_DESCRIPTOR
);

unsafe extern "C" {
    static UI_FONT_DESCRIPTOR: &'static objc::Class<FontDesc>;
}
