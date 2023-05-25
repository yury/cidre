use crate::define_options;

define_options!(FontSymbolicTraits(u32));
impl FontSymbolicTraits {
    pub const ITALIC: Self = Self(1 << 0); // Additional detail available via kCTFontSlantTrait
    pub const BOLD: Self = Self(1 << 1); // Additional detail available via kCTFontWeightTrait
    pub const EXPANDED: Self = Self(1 << 5); // Expanded and condensed traits are mutually exclusive
    pub const CONDENSED: Self = Self(1 << 6); // Additional detail available via kCTFontWidthTrait
    pub const MONO_SPACE: Self = Self(1 << 10); // Use fixed-pitch glyphs if available. May have multiple glyph advances (most CJK glyphs may contain two spaces)
    pub const VERTICAL: Self = Self(1 << 11); // Use vertical glyph variants and metrics
    pub const UI_OPTIMIZED: Self = Self(1 << 12); // Synthesize appropriate attributes for UI rendering such as control titles if necessary
    pub const COLOR_GLYPHS: Self = Self(1 << 13); // Color glyphs ('sbix', 'COLR', or 'SVG ') are available.
    pub const COMPOSITE: Self = Self(1 << 14); // The font is a CFR (Composite font reference), a cascade list is expected per font.

    pub const CLASS_MASK: Self = Self(15u32 << CLASS_MASK_SHIFT);
}

define_options!(FontStylisticClass(u32));
impl FontStylisticClass {
    pub const UNKNOWN: Self = Self(0 << CLASS_MASK_SHIFT);
    pub const OLD_STYLE_SERIFS: Self = Self(1 << CLASS_MASK_SHIFT);
    pub const TRANSITIONAL_SERIFS: Self = Self(2 << CLASS_MASK_SHIFT);
    pub const MODERN_SERIFS: Self = Self(3 << CLASS_MASK_SHIFT);
    pub const CLARENDON_SERIFS: Self = Self(4 << CLASS_MASK_SHIFT);
    pub const SLAB_SERIFS: Self = Self(5 << CLASS_MASK_SHIFT);
    pub const FREEFORM_SERIFS: Self = Self(7 << CLASS_MASK_SHIFT);
    pub const SANS_SERIF: Self = Self(8u32 << CLASS_MASK_SHIFT);
    pub const ORNAMENTALS: Self = Self(9u32 << CLASS_MASK_SHIFT);
    pub const SCRIPTS: Self = Self(10u32 << CLASS_MASK_SHIFT);
    pub const SYMBOLIC: Self = Self(12u32 << CLASS_MASK_SHIFT);
}

pub const CLASS_MASK_SHIFT: usize = 28;
