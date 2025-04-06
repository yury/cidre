use crate::{arc, define_mtl, define_obj_type, mtl, ns, objc};

#[doc(alias = "MTLCompareFunction")]
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
#[repr(usize)]
pub enum CompareFn {
    /// A new value never passes the comparison test.
    Never = 0,
    /// A new value passes the comparison test if it is less than the existing value.
    Less = 1,
    /// A new value passes the comparison test if it is equal to the existing value.
    Equal = 2,
    /// A new value passes the comparison test if it is less than or equal to the existing value.
    LessEqual = 3,
    /// A new value passes the comparison test if it is greater than the existing value.
    Greater = 4,
    /// A new value passes the comparison test if it is not equal to the existing value.
    NotEqual = 5,
    /// A new value passes the comparison test if it is greater than or equal to the existing value.
    GreaterEqual = 6,
    /// A new value always passes the comparison test.
    Always = 7,
}

#[doc(alias = "MTLStencilOperation")]
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
#[repr(usize)]
pub enum StencilOp {
    /// Keep the current stencil value.
    Keep = 0,
    /// Set the stencil value to zero.
    Zero = 1,
    /// Replace the stencil value with the stencil reference value, which is set by
    /// the set_stencil_reference_value method of mtl::RenderCmdEncoder.
    Replace = 2,
    /// If the current stencil value is not the maximum representable value, increase the stencil
    /// value by one. Otherwise, if the current stencil value is the maximum representable value,
    /// do not change the stencil value.
    IncrementClamp = 3,
    /// If the current stencil value is not zero, decrease the stencil value by one. Otherwise,
    /// if the current stencil value is zero, do not change the stencil value.
    DecrementClamp = 4,
    /// Perform a logical bitwise invert operation on the current stencil value.
    Invert = 5,
    /// If the current stencil value is not the maximum representable value, increase
    /// the stencil value by one. Otherwise, if the current stencil value is the maximum
    /// representable value, set the stencil value to zero.
    IncrementWrap = 6,
    /// If the current stencil value is not zero, decrease the stencil value by one.
    /// Otherwise, if the current stencil value is zero, set the stencil value to the maximum
    /// representable value.
    DecrementWrap = 7,
}

define_obj_type!(pub StencilDesc(ns::Id), MTL_STENCIL_DESCRIPTOR);

impl StencilDesc {
    /// The comparison that is performed between the masked reference value and a
    /// masked value in the stencil attachment.
    #[objc::msg_send(stencilCompareFunction)]
    pub fn compare_fn(&self) -> CompareFn;

    #[objc::msg_send(setStencilCompareFunction:)]
    pub fn set_compare_fn(&mut self, val: CompareFn);

    /// The operation that is performed to update the values in the stencil attachment
    /// when the stencil test fails.
    #[objc::msg_send(stencilFailureOperation)]
    pub fn failure_op(&self) -> StencilOp;

    #[objc::msg_send(setStencilFailureOperation:)]
    pub fn set_failure_op(&mut self, val: StencilOp);

    /// The operation that is performed to update the values in the stencil attachment
    /// when the stencil test passes, but the depth test fails.
    #[objc::msg_send(depthFailureOperation)]
    pub fn depth_failure_op(&self) -> StencilOp;

    #[objc::msg_send(setDepthFailureOperation:)]
    pub fn set_depth_failure_op(&mut self, val: StencilOp);

    /// The operation that is performed to update the values in the stencil attachment
    /// when both the stencil test and the depth test pass.
    #[objc::msg_send(depthStencilPassOperation)]
    pub fn depth_stencil_op(&self) -> StencilOp;

    #[objc::msg_send(setDepthStencilPassOperation:)]
    pub fn set_depth_stencil_op(&mut self, val: StencilOp);

    /// A bitmask that determines from which bits that stencil comparison tests can read.
    #[objc::msg_send(readMask)]
    pub fn read_mask(&self) -> u32;

    #[objc::msg_send(setReadMask:)]
    pub fn set_read_mask(&mut self, val: u32);

    /// A bitmask that determines to which bits that stencil operations can write.
    #[objc::msg_send(writeMask)]
    pub fn write_mask(&self) -> u32;

    #[objc::msg_send(setWriteMask:)]
    pub fn set_write_mask(&mut self, val: u32);
}

define_obj_type!(pub DepthStencilDesc(ns::Id), MTL_DEPTH_STENCIL_DESCRIPTOR);

impl DepthStencilDesc {
    /// Defaults to mtl::CompareFnAlways, which effectively skips the depth test
    #[objc::msg_send(depthCompareFunction)]
    pub fn compare_fn(&self) -> CompareFn;

    #[objc::msg_send(setDepthCompareFunction:)]
    pub fn set_compare_fn(&mut self, val: CompareFn);

    /// Defaults to false, so no depth writes are performed
    #[objc::msg_send(isDepthWriteEnabled)]
    pub fn is_depth_write_enabled(&self) -> bool;

    #[objc::msg_send(setDepthWriteEnabled:)]
    pub fn set_depth_write_enabled(&mut self, val: bool);

    #[objc::msg_send(frontFaceStencil)]
    pub fn front_face_stenil(&self) -> arc::R<StencilDesc>;

    #[objc::msg_send(setFrontFaceStencil:)]
    fn _set_front_face_stencil(&mut self, val: Option<&StencilDesc>);

    #[inline]
    pub fn set_front_face_stencil(&mut self, val: &StencilDesc) {
        self._set_front_face_stencil(Some(val))
    }

    #[inline]
    pub fn reset_front_face_stencil(&mut self) {
        self._set_front_face_stencil(None)
    }

    #[objc::msg_send(backFaceStencil)]
    pub fn back_face_stencil(&self) -> arc::R<StencilDesc>;

    #[objc::msg_send(setBackFaceStencil:)]
    fn _set_back_face_stencil(&mut self, val: Option<&StencilDesc>);

    #[inline]
    pub fn set_back_face_stencil(&mut self, val: &StencilDesc) {
        self._set_back_face_stencil(Some(val))
    }

    #[inline]
    pub fn reset_back_face_stencil(&mut self) {
        self._set_back_face_stencil(None)
    }

    define_mtl!(set_label);

    #[objc::msg_send(label)]
    pub fn label(&self) -> Option<arc::R<ns::String>>;
}

define_obj_type!(
    /// A depth and stencil state object that specifies the depth and stencil
    /// configuration and operations used in a render pass.
    pub State(ns::Id)
);

impl State {
    #[objc::msg_send(device)]
    pub fn device(&self) -> arc::R<mtl::Device>;

    #[objc::msg_send(label)]
    pub fn label(&self) -> Option<arc::R<ns::String>>;
}

#[link(name = "mtl", kind = "static")]
unsafe extern "C" {
    static MTL_STENCIL_DESCRIPTOR: &'static objc::Class<StencilDesc>;
    static MTL_DEPTH_STENCIL_DESCRIPTOR: &'static objc::Class<DepthStencilDesc>;
}
