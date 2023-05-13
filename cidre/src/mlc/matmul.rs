use crate::{arc, define_cls, define_obj_type, mlc, ns, objc};

define_obj_type!(Descriptor(ns::Id));
impl Descriptor {
    define_cls!(MLC_MATMUL_DESCRIPTOR);

    #[objc::msg_send(alpha)]
    pub fn alpha(&self) -> f32;

    #[objc::msg_send(transposesX)]
    pub fn transposes_x(&self) -> bool;

    #[objc::msg_send(transposesY)]
    pub fn transposes_y(&self) -> bool;

    #[objc::cls_msg_send(descriptor)]
    pub fn descriptor_ar() -> arc::Rar<Self>;

    #[objc::cls_rar_retain]
    pub fn descriptor() -> arc::R<Self>;

    #[objc::cls_msg_send(descriptorWithAlpha:transposesX:transposesY:)]
    pub fn with_alpha_ar(
        alpha: f32,
        transposes_x: bool,
        transposes_y: bool,
    ) -> Option<arc::Rar<Self>>;

    #[objc::cls_rar_retain]
    pub fn with_alpha(alpha: f32, transposes_x: bool, transposes_y: bool) -> Option<arc::R<Self>>;

    #[objc::msg_send(descriptor)]
    pub fn descriptor_(&self) -> arc::R<Self>;

    #[inline]
    pub fn new() -> arc::R<Self> {
        Self::descriptor()
    }
}

define_obj_type!(Layer(mlc::Layer));
impl Layer {
    define_cls!(MLC_MATMUL_LAYER);

    #[objc::cls_msg_send(layerWithDescriptor:)]
    pub fn with_descriptor_ar(desc: &Descriptor) -> Option<arc::Rar<Self>>;

    #[objc::cls_rar_retain]
    pub fn with_descriptor(desc: &Descriptor) -> Option<arc::R<Self>>;

    #[objc::msg_send(descriptor)]
    pub fn descriptor(&self) -> &Descriptor;

    pub fn new() -> arc::R<Self> {
        Self::with_descriptor(&Descriptor::new()).unwrap()
    }
}

#[link(name = "mlc", kind = "static")]
extern "C" {
    static MLC_MATMUL_DESCRIPTOR: &'static objc::Class<Descriptor>;
    static MLC_MATMUL_LAYER: &'static objc::Class<Layer>;
}
