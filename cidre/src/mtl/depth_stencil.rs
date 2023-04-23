use crate::{arc, define_mtl, define_obj_type, ns, objc};

#[doc(alias = "MTLCompareFunction")]
#[derive(Debug, Eq, PartialEq)]
#[repr(usize)]
pub enum CompareFn {
    Never = 0,
    Less = 1,
    Equal = 2,
    LessEqual = 3,
    Greater = 4,
    NotEqual = 5,
    GreaterEqual = 6,
    Always = 7,
}

#[doc(alias = "MTLStencilOperation")]
#[derive(Debug, Eq, PartialEq)]
#[repr(usize)]
pub enum StencilOp {
    Keep = 0,
    Zero = 1,
    Replace = 2,
    IncrementClamp = 3,
    DecrementClamp = 4,
    Invert = 5,
    IncrementWrap = 6,
    DecrementWrap = 7,
}

define_obj_type!(StencilDescriptor(ns::Id), MTL_STENCIL_DESCRIPTOR);

impl StencilDescriptor {
    #[objc::msg_send(stencilCompareFunction)]
    pub fn compare_fn(&self) -> CompareFn;

    #[objc::msg_send(setStencilCompareFunction:)]
    pub fn set_compare_fn(&mut self, value: CompareFn);

    #[objc::msg_send(stencilFailureOperation)]
    pub fn failure_op(&self) -> StencilOp;

    #[objc::msg_send(setStencilFailureOperation:)]
    pub fn set_failure_op(&mut self, value: StencilOp);

    #[objc::msg_send(depthFailureOperation)]
    pub fn depth_failure_op(&self) -> StencilOp;

    #[objc::msg_send(setDepthFailureOperation:)]
    pub fn set_depth_failure_op(&mut self, value: StencilOp);

    #[objc::msg_send(depthStencilPassOperation)]
    pub fn depth_stencil_op(&self) -> StencilOp;

    #[objc::msg_send(setDepthStencilPassOperation:)]
    pub fn set_depth_stencil_op(&mut self, value: StencilOp);

    #[objc::msg_send(readMask)]
    pub fn read_mask(&self) -> u32;

    #[objc::msg_send(setReadMask:)]
    pub fn set_read_mask(&mut self, value: u32);

    #[objc::msg_send(writeMask)]
    pub fn write_mask(&self) -> u32;

    #[objc::msg_send(setWriteMask:)]
    pub fn set_write_mask(&mut self, value: u32);
}

define_obj_type!(DepthStencilDescriptor(ns::Id), MTL_DEPTH_STENCIL_DESCRIPTOR);

impl DepthStencilDescriptor {
    /// Defaults to mtl::CompareFnAlways, which effectively skips the depth test
    #[objc::msg_send(depthCompareFunction)]
    pub fn compare_fn(&self) -> CompareFn;

    #[objc::msg_send(setDepthCompareFunction:)]
    pub fn set_compare_fn(&mut self, value: CompareFn);

    /// Defaults to false, so no depth writes are performed
    #[objc::msg_send(isDepthWriteEnabled)]
    pub fn is_depth_write_enabled(&self) -> bool;

    #[objc::msg_send(setDepthWriteEnabled:)]
    pub fn set_depth_write_enabled(&mut self, value: bool);

    #[objc::msg_send(frontFaceStencil)]
    pub fn front_face_stenil(&self) -> &StencilDescriptor;

    #[objc::msg_send(setFrontFaceStencil:)]
    fn _set_front_face_stencil(&mut self, value: Option<&StencilDescriptor>);

    #[inline]
    pub fn set_front_face_stencil(&mut self, value: &StencilDescriptor) {
        self._set_front_face_stencil(Some(value))
    }

    #[inline]
    pub fn reset_front_face_stencil(&mut self) {
        self._set_front_face_stencil(None)
    }

    #[objc::msg_send(backFaceStencil)]
    pub fn back_face_stencil(&self) -> &StencilDescriptor;

    #[objc::msg_send(setBackFaceStencil:)]
    fn _set_back_face_stencil(&mut self, value: Option<&StencilDescriptor>);

    #[inline]
    pub fn set_back_face_stencil(&mut self, value: &StencilDescriptor) {
        self._set_back_face_stencil(Some(value))
    }

    #[inline]
    pub fn reset_back_face_stencil(&mut self) {
        self._set_back_face_stencil(None)
    }

    define_mtl!(label, set_label);
}

define_obj_type!(State(ns::Id));

impl State {
    define_mtl!(label, device);
}

#[link(name = "mtl", kind = "static")]
extern "C" {
    static MTL_STENCIL_DESCRIPTOR: &'static objc::Class<StencilDescriptor>;
    static MTL_DEPTH_STENCIL_DESCRIPTOR: &'static objc::Class<DepthStencilDescriptor>;
}
