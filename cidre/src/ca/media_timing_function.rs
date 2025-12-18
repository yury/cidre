use crate::{arc, define_cls, define_obj_type, ns, objc};

define_obj_type!(
    #[doc(alias = "CAMediaTimingFunctionName")]
    pub Name(ns::String)
);

impl Name {
    #[doc(alias = "kCAMediaTimingFunctionLinear")]
    pub fn linear() -> &'static Self {
        unsafe { kCAMediaTimingFunctionLinear }
    }

    #[doc(alias = "kCAMediaTimingFunctionEaseIn")]
    pub fn ease_in() -> &'static Self {
        unsafe { kCAMediaTimingFunctionEaseIn }
    }

    #[doc(alias = "kCAMediaTimingFunctionEaseOut")]
    pub fn ease_out() -> &'static Self {
        unsafe { kCAMediaTimingFunctionEaseOut }
    }

    #[doc(alias = "kCAMediaTimingFunctionEaseInEaseOut")]
    pub fn ease_in_ease_out() -> &'static Self {
        unsafe { kCAMediaTimingFunctionEaseInEaseOut }
    }

    #[doc(alias = "kCAMediaTimingFunctionDefault")]
    pub fn default() -> &'static Self {
        unsafe { kCAMediaTimingFunctionDefault }
    }
}

impl arc::A<MediaTimingFn> {
    #[objc::msg_send(initWithControlPoints::::)]
    pub fn init_with_ctrl_points(
        self,
        c1x: f32,
        c1y: f32,
        c2x: f32,
        c2y: f32,
    ) -> arc::R<MediaTimingFn>;
}

define_obj_type!(pub MediaTimingFn(ns::Id));
impl MediaTimingFn {
    define_cls!(CA_MEDIA_TIMING_FUNCTION);

    #[objc::msg_send(functionWithName:)]
    pub fn with_name(name: &Name) -> arc::R<Self>;

    #[inline]
    pub fn with_ctrl_points(c1x: f32, c1y: f32, c2x: f32, c2y: f32) -> arc::R<Self> {
        Self::alloc().init_with_ctrl_points(c1x, c1y, c2x, c2y)
    }
}

#[link(name = "QuartzCore", kind = "framework")]
unsafe extern "C" {
    static kCAMediaTimingFunctionLinear: &'static Name;
    static kCAMediaTimingFunctionEaseIn: &'static Name;
    static kCAMediaTimingFunctionEaseOut: &'static Name;
    static kCAMediaTimingFunctionEaseInEaseOut: &'static Name;
    static kCAMediaTimingFunctionDefault: &'static Name;
}

#[link(name = "ca", kind = "static")]
unsafe extern "C" {
    static CA_MEDIA_TIMING_FUNCTION: &'static objc::Class<MediaTimingFn>;
}

#[cfg(test)]
mod tests {
    use crate::ca;

    #[test]
    fn basics() {
        let tfn = ca::MediaTimingFn::with_name(ca::MediaTimingFnName::ease_out());
        println!("{tfn:?}");
        let tfn = ca::MediaTimingFn::with_ctrl_points(0.0, 0.5, 0.3, 0.6);
        println!("{tfn:?}");
    }
}
