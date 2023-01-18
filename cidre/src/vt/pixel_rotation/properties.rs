pub mod keys {
    use crate::cf;

    pub fn rotation() -> &'static cf::String {
        unsafe { kVTPixelRotationPropertyKey_Rotation }
    }

    pub fn flip_horizontal_orientation() -> &'static cf::String {
        unsafe { kVTPixelRotationPropertyKey_FlipHorizontalOrientation }
    }

    pub fn flip_vertical_orientation() -> &'static cf::String {
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

    pub fn _0() -> &'static cf::String {
        unsafe { kVTRotation_0 }
    }

    pub fn _cw_90() -> &'static cf::String {
        unsafe { kVTRotation_CW90 }
    }

    pub fn _180() -> &'static cf::String {
        unsafe { kVTRotation_180 }
    }

    pub fn _ccw_90() -> &'static cf::String {
        unsafe { kVTRotation_CCW90 }
    }

    extern "C" {
        static kVTRotation_0: &'static cf::String;
        static kVTRotation_CW90: &'static cf::String;
        static kVTRotation_180: &'static cf::String;
        static kVTRotation_CCW90: &'static cf::String;
    }
}
