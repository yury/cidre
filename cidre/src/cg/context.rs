use crate::{cf, cg, define_cf_type};

define_cf_type!(
    #[doc(alias = "CGContext")]
    #[doc(alias = "CGContextRef")]
    Context(cf::Type)
);

impl Context {
    #[doc(alias = "CGContextGetTypeID")]
    #[inline]
    pub fn type_id() -> cf::TypeId {
        unsafe { CGContextGetTypeID() }
    }

    #[doc(alias = "CGContextSaveGState")]
    #[inline]
    pub fn save(&mut self) {
        unsafe {
            CGContextSaveGState(Some(self));
        }
    }

    #[doc(alias = "CGContextRestoreGState")]
    #[inline]
    pub fn restore(&mut self) {
        unsafe {
            CGContextRestoreGState(Some(self));
        }
    }

    #[doc(alias = "CGContextScaleCTM")]
    #[inline]
    pub fn scale(&mut self, sx: cg::Float, sy: cg::Float) {
        unsafe {
            CGContextScaleCTM(self, sx, sy);
        }
    }

    #[doc(alias = "CGContextTranslateCTM")]
    #[inline]
    pub fn translate(&mut self, tx: cg::Float, ty: cg::Float) {
        unsafe {
            CGContextTranslateCTM(self, tx, ty);
        }
    }

    #[doc(alias = "CGContextRotateCTM")]
    #[inline]
    pub fn rotate(&mut self, angle: cg::Float) {
        unsafe {
            CGContextRotateCTM(self, angle);
        }
    }

    #[doc(alias = "CGContextConcatCTM")]
    #[inline]
    pub fn concat(&mut self, transform: cg::AffineTransform) {
        unsafe {
            CGContextConcatCTM(self, transform);
        }
    }

    #[doc(alias = "CGContextGetCTM")]
    #[inline]
    pub fn ctm(&self) -> cg::AffineTransform {
        unsafe { CGContextGetCTM(self) }
    }
}

#[link(name = "CoreGraphics", kind = "framework")]
unsafe extern "C" {
    fn CGContextGetTypeID() -> cf::TypeId;
    fn CGContextSaveGState(ctx: Option<&Context>);
    fn CGContextRestoreGState(ctx: Option<&Context>);
    fn CGContextScaleCTM(ctx: *mut Context, sx: cg::Float, sy: cg::Float);
    fn CGContextTranslateCTM(ctx: *mut Context, tx: cg::Float, ty: cg::Float);
    fn CGContextRotateCTM(ctx: *mut Context, angle: cg::Float);
    fn CGContextConcatCTM(ctx: *mut Context, transform: cg::AffineTransform);
    fn CGContextGetCTM(ctx: *const Context) -> cg::AffineTransform;
}
