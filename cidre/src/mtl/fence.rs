use crate::{arc, define_mtl, define_obj_type, mtl, ns, objc};

define_obj_type!(
    /// An object that can capture, track, and manage resource dependencies
    /// across command encoders.
    ///
    /// Apple Documentation:
    /// - [Objective-C](https://developer.apple.com/documentation/metal/mtlfence?language=objc)
    /// - [Swift](https://developer.apple.com/documentation/metal/mtlfence?language=swift)
    #[doc(alias = "MTLFence")]
    pub Fence(ns::Id)
);

impl Fence {
    define_mtl!(set_label);

    #[objc::msg_send(device)]
    pub fn device(&self) -> arc::R<mtl::Device>;

    #[objc::msg_send(label)]
    pub fn label(&self) -> Option<arc::R<ns::String>>;
}
