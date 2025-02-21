use crate::{arc, define_cls, define_obj_type, ns, objc};

define_obj_type!(
    #[doc(alias = "GCColor")]
    pub Color(ns::Id)
);

impl arc::A<Color> {
    #[objc::msg_send(initWithRed:green:blue:)]
    pub fn init_with_rgb(self, r: f32, g: f32, b: f32) -> arc::R<Color>;
}

impl Color {
    define_cls!(GC_COLOR);

    pub fn with_rgb(r: f32, g: f32, b: f32) -> arc::R<Self> {
        Self::alloc().init_with_rgb(r, g, b)
    }

    #[objc::msg_send(red)]
    pub fn r(&self) -> f32;

    #[objc::msg_send(green)]
    pub fn g(&self) -> f32;

    #[objc::msg_send(blue)]
    pub fn b(&self) -> f32;
}

#[link(name = "gc", kind = "static")]
unsafe extern "C" {
    static GC_COLOR: &'static objc::Class<Color>;
}

#[cfg(test)]
mod tests {
    use crate::gc;

    #[test]
    fn basics() {
        let c = gc::Color::with_rgb(0.4, 0.8, 0.9);
        assert_eq!(0.4, c.r());
        assert_eq!(0.8, c.g());
        assert_eq!(0.9, c.b());
    }
}
