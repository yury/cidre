use crate::{arc, define_obj_type, ns, objc};

define_obj_type!(Name(ns::String));
impl Name {
    pub fn linear() -> &'static Self {
        unsafe { kCAMediaTimingFunctionLinear }
    }

    pub fn ease_in() -> &'static Self {
        unsafe { kCAMediaTimingFunctionEaseIn }
    }

    pub fn ease_out() -> &'static Self {
        unsafe { kCAMediaTimingFunctionEaseOut }
    }

    pub fn ease_in_out() -> &'static Self {
        unsafe { kCAMediaTimingFunctionEaseInOut }
    }

    pub fn default() -> &'static Self {
        unsafe { kCAMediaTimingFunctionDefault }
    }
}

define_obj_type!(MediaTimingFn(ns::Id), CA_MEDIA_TIMING_FUNCTION);
impl MediaTimingFn {}

#[link(name = "QuartzCore", kind = "framework")]
extern "C" {
    static kCAMediaTimingFunctionLinear: &'static Name;
    static kCAMediaTimingFunctionEaseIn: &'static Name;
    static kCAMediaTimingFunctionEaseOut: &'static Name;
    static kCAMediaTimingFunctionEaseInOut: &'static Name;
    static kCAMediaTimingFunctionDefault: &'static Name;

}

#[link(name = "ca", kind = "static")]
extern "C" {
    static CA_MEDIA_TIMING_FUNCTION: &'static objc::Class<MediaTimingFn>;
}
