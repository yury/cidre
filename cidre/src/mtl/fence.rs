use crate::{define_mtl, define_obj_type, ns};

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
    define_mtl!(device, label, set_label);
}
