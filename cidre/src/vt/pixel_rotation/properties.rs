pub mod keys {
    use crate::cf;

    pub const fn rotation() -> &'static cf::String {
        unsafe { kVTPixelRotationPropertyKey_Rotation }
    }

    pub const fn flip_horizontal_orientation() -> &'static cf::String {
        unsafe { kVTPixelRotationPropertyKey_FlipHorizontalOrientation }
    }

    pub const fn flip_vertical_orientation() -> &'static cf::String {
        unsafe { kVTPixelRotationPropertyKey_FlipVerticalOrientation }
    }

    extern "C" {
        static kVTPixelRotationPropertyKey_Rotation: &'static cf::String;
        static kVTPixelRotationPropertyKey_FlipHorizontalOrientation: &'static cf::String;
        static kVTPixelRotationPropertyKey_FlipVerticalOrientation: &'static cf::String;
    }
}

pub mod rotation {
    use crate::cf;

    #[inline]
    pub const fn _0() -> &'static cf::String {
        unsafe { kVTRotation_0 }
    }

    #[inline]
    pub const fn cw_90() -> &'static cf::String {
        unsafe { kVTRotation_CW90 }
    }

    #[inline]
    pub const fn _180() -> &'static cf::String {
        unsafe { kVTRotation_180 }
    }

    #[inline]
    pub const fn ccw_90() -> &'static cf::String {
        unsafe { kVTRotation_CCW90 }
    }

    extern "C" {
        static kVTRotation_0: &'static cf::String;
        static kVTRotation_CW90: &'static cf::String;
        static kVTRotation_180: &'static cf::String;
        static kVTRotation_CCW90: &'static cf::String;
    }
}
