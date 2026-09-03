use std::ffi::c_void;

use crate::{arc, cf, cg, define_cf_type};

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

    #[doc(alias = "CGContextSetLineWidth")]
    #[inline]
    pub fn set_line_width(&mut self, val: cg::Float) {
        unsafe {
            CGContextSetLineWidth(self, val);
        }
    }

    #[doc(alias = "CGContextSetLineCap")]
    #[inline]
    pub fn set_line_cap(&mut self, val: cg::LineCap) {
        unsafe {
            CGContextSetLineCap(self, val);
        }
    }

    #[doc(alias = "CGContextSetLineJoin")]
    #[inline]
    pub fn set_line_join(&mut self, val: cg::LineJoin) {
        unsafe {
            CGContextSetLineJoin(self, val);
        }
    }

    /// Creates a bitmap-backed context that draws into `data` (or a CoreGraphics-owned buffer
    /// when `data` is null). `data` must hold at least `bytes_per_row * height` bytes.
    #[doc(alias = "CGBitmapContextCreate")]
    #[inline]
    pub fn new_bitmap(
        data: *mut c_void,
        width: usize,
        height: usize,
        bits_per_component: usize,
        bytes_per_row: usize,
        space: &cg::ColorSpace,
        bitmap_info: cg::BitmapInfo,
    ) -> Option<arc::R<Self>> {
        unsafe {
            CGBitmapContextCreate(
                data,
                width,
                height,
                bits_per_component,
                bytes_per_row,
                space,
                bitmap_info,
            )
        }
    }

    #[doc(alias = "CGContextClearRect")]
    #[inline]
    pub fn clear_rect(&mut self, rect: cg::Rect) {
        unsafe { CGContextClearRect(self, rect) }
    }

    #[doc(alias = "CGContextSetRGBFillColor")]
    #[inline]
    pub fn set_rgb_fill_color(&mut self, r: cg::Float, g: cg::Float, b: cg::Float, a: cg::Float) {
        unsafe { CGContextSetRGBFillColor(self, r, g, b, a) }
    }

    #[doc(alias = "CGContextFillRect")]
    #[inline]
    pub fn fill_rect(&mut self, rect: cg::Rect) {
        unsafe { CGContextFillRect(self, rect) }
    }

    #[doc(alias = "CGContextSetTextPosition")]
    #[inline]
    pub fn set_text_pos(&mut self, x: cg::Float, y: cg::Float) {
        unsafe { CGContextSetTextPosition(self, x, y) }
    }

    #[doc(alias = "CGContextSetTextMatrix")]
    #[inline]
    pub fn set_text_matrix(&mut self, matrix: cg::AffineTransform) {
        unsafe { CGContextSetTextMatrix(self, matrix) }
    }
}

unsafe extern "C" {
    fn CGContextGetTypeID() -> cf::TypeId;
    fn CGContextSaveGState(ctx: Option<&Context>);
    fn CGContextRestoreGState(ctx: Option<&Context>);
    fn CGContextScaleCTM(ctx: *mut Context, sx: cg::Float, sy: cg::Float);
    fn CGContextTranslateCTM(ctx: *mut Context, tx: cg::Float, ty: cg::Float);
    fn CGContextRotateCTM(ctx: *mut Context, angle: cg::Float);
    fn CGContextConcatCTM(ctx: *mut Context, transform: cg::AffineTransform);
    fn CGContextGetCTM(ctx: *const Context) -> cg::AffineTransform;
    fn CGContextSetLineWidth(ctx: *mut Context, val: cg::Float);
    fn CGContextSetLineCap(ctx: *mut Context, val: cg::LineCap);
    fn CGContextSetLineJoin(ctx: *mut Context, val: cg::LineJoin);

    fn CGBitmapContextCreate(
        data: *mut c_void,
        width: usize,
        height: usize,
        bits_per_component: usize,
        bytes_per_row: usize,
        space: &cg::ColorSpace,
        bitmap_info: cg::BitmapInfo,
    ) -> Option<arc::R<Context>>;
    fn CGContextClearRect(ctx: *mut Context, rect: cg::Rect);
    fn CGContextSetRGBFillColor(
        ctx: *mut Context,
        r: cg::Float,
        g: cg::Float,
        b: cg::Float,
        a: cg::Float,
    );
    fn CGContextFillRect(ctx: *mut Context, rect: cg::Rect);
    fn CGContextSetTextPosition(ctx: *mut Context, x: cg::Float, y: cg::Float);
    fn CGContextSetTextMatrix(ctx: *mut Context, matrix: cg::AffineTransform);
}
