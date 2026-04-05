use crate::{cf, cg, ct, define_cf_type};

define_cf_type!(
    #[doc(alias = "CTFrame")]
    Frame(cf::Type)
);

impl Frame {
    #[inline]
    pub fn type_id() -> cf::TypeId {
        unsafe { CTFrameGetTypeID() }
    }

    #[inline]
    pub fn lines(&self) -> &cf::ArrayOf<ct::Line> {
        unsafe { CTFrameGetLines(self) }
    }

    #[inline]
    pub fn line_origins(&self, range: cf::Range, origins: &mut [cg::Point]) {
        unsafe { CTFrameGetLineOrigins(self, range, origins.as_mut_ptr()) }
    }
}

unsafe extern "C-unwind" {
    fn CTFrameGetTypeID() -> cf::TypeId;
    fn CTFrameGetLines(frame: &Frame) -> &cf::ArrayOf<ct::Line>;
    fn CTFrameGetLineOrigins(frame: &Frame, range: cf::Range, origins: *mut cg::Point);
}

#[cfg(test)]
mod tests {
    use crate::{cf, cg, ct};

    #[test]
    fn basics() {
        let string = cf::AttrString::new(cf::str!(c"test"));
        let framesetter = ct::Framesetter::with_attr_string(&string);
        let path = cg::Path::with_rect(cg::Rect::new(0.0, 0.0, 100.0, 100.0), None);
        let frame = framesetter.create_frame(cf::Range::new(0, 0), &path, None);
        let lines = frame.lines();
        assert_eq!(lines.len(), 1);

        let mut origins = vec![cg::Point::zero(); lines.len()];
        frame.line_origins(cf::Range::new(0, 0), &mut origins);
    }
}
