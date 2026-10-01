use crate::{arc, ca, cf, cg, define_obj_type, ns, objc};

define_obj_type!(
    #[doc(alias = "CATextLayerTruncationMode")]
    pub TruncationMode(ns::String)
);

impl TruncationMode {
    #[doc(alias = "kCATruncationNone")]
    #[inline]
    pub fn none() -> &'static Self {
        unsafe { kCATruncationNone }
    }

    #[doc(alias = "kCATruncationStart")]
    #[inline]
    pub fn start() -> &'static Self {
        unsafe { kCATruncationStart }
    }

    #[doc(alias = "kCATruncationEnd")]
    #[inline]
    pub fn end() -> &'static Self {
        unsafe { kCATruncationEnd }
    }

    #[doc(alias = "kCATruncationMiddle")]
    #[inline]
    pub fn middle() -> &'static Self {
        unsafe { kCATruncationMiddle }
    }
}

define_obj_type!(
    #[doc(alias = "CATextLayerAlignmentMode")]
    pub AlignmentMode(ns::String)
);

impl AlignmentMode {
    #[doc(alias = "kCAAlignmentNatural")]
    #[inline]
    pub fn natural() -> &'static Self {
        unsafe { kCAAlignmentNatural }
    }

    #[doc(alias = "kCAAlignmentLeft")]
    #[inline]
    pub fn left() -> &'static Self {
        unsafe { kCAAlignmentLeft }
    }

    #[doc(alias = "kCAAlignmentRight")]
    #[inline]
    pub fn right() -> &'static Self {
        unsafe { kCAAlignmentRight }
    }

    #[doc(alias = "kCAAlignmentCenter")]
    #[inline]
    pub fn center() -> &'static Self {
        unsafe { kCAAlignmentCenter }
    }

    #[doc(alias = "kCAAlignmentJustified")]
    #[inline]
    pub fn justified() -> &'static Self {
        unsafe { kCAAlignmentJustified }
    }
}

define_obj_type!(
    /// Renders plain or attributed text in its bounds.
    #[doc(alias = "CATextLayer")]
    pub TextLayer(ca::Layer),
    sym CATextLayer
);

impl TextLayer {
    /// The text: an `ns::String` or an `ns::AttributedString`.
    #[objc::msg_send(string)]
    pub fn string(&self) -> Option<arc::R<ns::Id>>;

    #[objc::msg_send(setString:)]
    pub fn set_string(&mut self, val: Option<&ns::String>);

    #[objc::msg_send(setString:)]
    pub fn set_attributed_string(&mut self, val: Option<&ns::AttrString>);

    /// The font of plain text: a `ct::Font`, `cg::Font` or a font name.
    /// Defaults to Helvetica.
    #[objc::msg_send(font)]
    pub fn font(&self) -> Option<arc::R<ns::Id>>;

    #[objc::msg_send(setFont:)]
    pub fn set_font_name(&mut self, val: Option<&ns::String>);

    #[objc::msg_send(setFont:)]
    pub fn set_cf_font(&mut self, val: Option<&cf::Type>);

    /// The font size of plain text. Defaults to 36.
    #[objc::msg_send(fontSize)]
    pub fn font_size(&self) -> cg::Float;

    #[objc::msg_send(setFontSize:)]
    pub fn set_font_size(&mut self, val: cg::Float);

    /// The color of plain text. Defaults to opaque white.
    #[objc::msg_send(foregroundColor)]
    pub fn foreground_color(&self) -> Option<&cg::Color>;

    #[objc::msg_send(setForegroundColor:)]
    pub fn set_foreground_color(&mut self, val: Option<&cg::Color>);

    #[objc::msg_send(isWrapped)]
    pub fn is_wrapped(&self) -> bool;

    #[objc::msg_send(setWrapped:)]
    pub fn set_wrapped(&mut self, val: bool);

    #[objc::msg_send(truncationMode)]
    pub fn truncation_mode(&self) -> arc::R<TruncationMode>;

    #[objc::msg_send(setTruncationMode:)]
    pub fn set_truncation_mode(&mut self, val: &TruncationMode);

    #[objc::msg_send(alignmentMode)]
    pub fn alignment_mode(&self) -> arc::R<AlignmentMode>;

    #[objc::msg_send(setAlignmentMode:)]
    pub fn set_alignment_mode(&mut self, val: &AlignmentMode);

    #[objc::msg_send(allowsFontSubpixelQuantization)]
    pub fn allows_font_subpixel_quantization(&self) -> bool;

    #[objc::msg_send(setAllowsFontSubpixelQuantization:)]
    pub fn set_allows_font_subpixel_quantization(&mut self, val: bool);
}

unsafe extern "C" {
    static kCATruncationNone: &'static TruncationMode;
    static kCATruncationStart: &'static TruncationMode;
    static kCATruncationEnd: &'static TruncationMode;
    static kCATruncationMiddle: &'static TruncationMode;

    static kCAAlignmentNatural: &'static AlignmentMode;
    static kCAAlignmentLeft: &'static AlignmentMode;
    static kCAAlignmentRight: &'static AlignmentMode;
    static kCAAlignmentCenter: &'static AlignmentMode;
    static kCAAlignmentJustified: &'static AlignmentMode;
}

#[cfg(test)]
mod tests {
    use crate::{ca, cg, ns};

    #[test]
    fn basics() {
        let mut layer = ca::TextLayer::new();
        assert!(layer.string().is_none());
        layer.set_string(Some(ns::str!(c"-60db")));
        assert!(layer.string().is_some());
        layer.set_font_size(8.0);
        assert_eq!(layer.font_size(), 8.0);
        layer.set_font_name(Some(ns::str!(c"Menlo")));
        assert!(layer.font().is_some());
        layer.set_alignment_mode(ca::AlignmentMode::right());
        assert!(layer.alignment_mode().is_equal(ca::AlignmentMode::right()));
        layer.set_truncation_mode(ca::TruncationMode::end());
        assert!(layer.truncation_mode().is_equal(ca::TruncationMode::end()));
        layer.set_wrapped(true);
        assert!(layer.is_wrapped());
        layer.set_foreground_color(Some(&cg::Color::generic_gray(0.5, 1.0)));
        assert!(layer.foreground_color().is_some());
    }
}
