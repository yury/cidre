use crate::{arc, ca, define_obj_type, mtl, ns, objc};

define_obj_type!(MetalLayer(ca::Layer), CA_METAL_LAYER);

pub trait MetalDrawable<T: objc::Obj>: mtl::Drawable<T> {
    #[objc::msg_send(texture)]
    fn texture(&self) -> &mtl::Texture;

    #[objc::msg_send(layer)]
    fn layer(&self) -> &MetalLayer;
}

define_obj_type!(AnyMetalDrawable(ns::Id));

impl mtl::Drawable<ns::Id> for AnyMetalDrawable {}
impl MetalDrawable<ns::Id> for AnyMetalDrawable {}

impl MetalLayer {
    #[objc::msg_send(device)]
    pub fn device(&self) -> Option<&mtl::Device>;

    #[objc::msg_send(preferredDevice)]
    pub fn preferred_device(&self) -> Option<&mtl::Device>;

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
    use crate::{ca, ca::MetalDrawable, cg, ns};

    #[test]
    fn basics() {
        let mut metal_layer = ca::MetalLayer::new();
        metal_layer.set_name(Some(&ns::String::with_str("mtl_layer")));
        metal_layer.set_bounds(cg::Rect::with_size(100.0, 100.0));
        let device = metal_layer.preferred_device().unwrap().retained();
        metal_layer.set_device(Some(&device));
        let drawable = metal_layer.next_drawable().unwrap();
        let texture = drawable.texture();
        println!("{drawable:?} {texture:?}");
    }
}
