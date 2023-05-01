use crate::{arc, ca, define_obj_type, mtl, ns, objc};

define_obj_type!(MetalLayer(ca::Layer), CA_METAL_LAYER);

pub trait MetalDrawable<T: objc::Obj>: mtl::Drawable<T> {
    #[objc::msg_send(texture)]
    fn texture(&self);

    #[objc::msg_send(layer)]
    fn layer(&self) -> &MetalLayer;
}

define_obj_type!(AnyMetalDrawable(ns::Id));

impl mtl::Drawable<ns::Id> for AnyMetalDrawable {}
impl MetalDrawable<ns::Id> for AnyMetalDrawable {}

impl MetalLayer {
    #[objc::msg_send(device)]
    pub fn device(&self) -> Option<&mtl::Device>;

    #[objc::msg_send(setDevice:)]
    pub fn set_device(&mut self, value: Option<&mtl::Device>);

    #[objc::msg_send(nextDrawable)]
    pub fn next_drawable(&self) -> Option<&AnyMetalDrawable>;
}

#[link(name = "ca", kind = "static")]
extern "C" {
    static CA_METAL_LAYER: &'static objc::Class<MetalLayer>;
}

#[cfg(test)]
mod tests {
    use crate::ca;

    #[test]
    fn basics() {
        let metal_layer = ca::MetalLayer::new();
        let drawable = metal_layer.next_drawable();
        println!("{drawable:?}");
    }
}
