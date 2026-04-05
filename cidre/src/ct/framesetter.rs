use crate::{arc, cf, cg, ct, define_cf_type};

define_cf_type!(
    #[doc(alias = "CTFramesetter")]
    Framesetter(cf::Type)
);

impl Framesetter {
    #[inline]
    pub fn with_attr_string(attr_string: &cf::AttrString) -> arc::R<Self> {
        unsafe { CTFramesetterCreateWithAttributedString(attr_string) }
    }

    #[inline]
    pub fn suggest_frame_size_with_constraints(
        &self,
        string_range: cf::Range,
        frame_attributes: Option<&cf::Dictionary>,
        constraints: cg::Size,
        fit_range: Option<&mut cf::Range>,
    ) -> cg::Size {
        unsafe {
            CTFramesetterSuggestFrameSizeWithConstraints(
                self,
                string_range,
                frame_attributes,
                constraints,
                fit_range,
            )
        }
    }

    #[inline]
    pub fn create_frame(
        &self,
        string_range: cf::Range,
        path: &cg::Path,
        frame_attributes: Option<&cf::Dictionary>,
    ) -> arc::R<ct::Frame> {
        unsafe { CTFramesetterCreateFrame(self, string_range, path, frame_attributes) }
    }
}

unsafe extern "C-unwind" {
    fn CTFramesetterCreateWithAttributedString(string: &cf::AttrString) -> arc::R<Framesetter>;
    fn CTFramesetterCreateFrame(
        framesetter: &Framesetter,
        string_range: cf::Range,
        path: &cg::Path,
        frame_attributes: Option<&cf::Dictionary>,
    ) -> arc::R<ct::Frame>;
    fn CTFramesetterSuggestFrameSizeWithConstraints(
        framesetter: &Framesetter,
        string_range: cf::Range,
        frame_attributes: Option<&cf::Dictionary>,
        constraints: cg::Size,
        fit_range: Option<&mut cf::Range>,
    ) -> cg::Size;
}

#[cfg(test)]
mod tests {
    use crate::{cf, cg, ct};

    #[test]
    fn basics() {
        let string = cf::AttrString::new(cf::str!(c"test"));
        let framesetter = ct::Framesetter::with_attr_string(&string);
        let size = framesetter.suggest_frame_size_with_constraints(
            cf::Range::new(0, 0),
            None,
            cg::Size::new(1000.0, 1000.0),
            None,
        );
        assert!(size.width > 0.0);
        assert!(size.height >= 0.0);
    }
}
