use crate::{api, arc, define_obj_type, mtl, ns, objc};

define_obj_type!(
    #[doc(alias = "MTL4ArgumentTableDescriptor")]
    pub ArgTableDesc(ns::Id),
    MTL4_ARGUMENT_TABLE_DESCRIPTOR,
    #[api::available(macos = 26.0, ios = 26.0, tvos = 26.0, visionos = 26.0)]
);

impl ArgTableDesc {
    /// Determines the number of buffer-binding slots for the argument table.
    #[objc::msg_send(maxBufferBindCount)]
    pub fn max_buf_bind_count(&self) -> usize;

    /// The maximum value of this parameter is 31.
    #[objc::msg_send(setMaxBufferBindCount:)]
    pub fn set_max_buf_bind_count(&mut self, val: usize);

    /// Determines the number of texture-binding slots for the argument table.
    #[objc::msg_send(maxTextureBindCount)]
    pub fn max_texture_bind_count(&self) -> usize;

    /// The maximum value of this parameter is 128.
    #[objc::msg_send(setMaxTextureBindCount:)]
    pub fn set_max_texture_bind_count(&mut self, val: usize);

    #[objc::msg_send(maxSamplerStateBindCount)]
    pub fn max_sample_state_bind_count(&self) -> usize;

    #[objc::msg_send(setMaxSamplerStateBindCount:)]
    pub fn set_max_sample_state_bind_count(&mut self, val: usize);

    #[objc::msg_send(initializeBindings)]
    pub fn initialize_bindings(&self) -> bool;

    #[objc::msg_send(setInitializeBindings:)]
    pub fn set_initialize_bindings(&mut self, val: bool);

    #[objc::msg_send(supportAttributeStrides)]
    pub fn support_attr_strides(&self) -> bool;

    #[objc::msg_send(setSupportAttributeStrides:)]
    pub fn set_support_attr_strides(&mut self, val: bool);

    #[objc::msg_send(label)]
    pub fn label(&self) -> Option<arc::R<ns::String>>;

    #[objc::msg_send(setLabel:)]
    pub fn set_label(&mut self, val: Option<&ns::String>);
}

define_obj_type!(
    #[doc(alias = "MTL4ArgumentTable")]
    pub ArgTable(ns::Id)
);

impl ArgTable {
    #[objc::msg_send(setAddress:atIndex:)]
    pub fn set_address(&mut self, gpu_address: usize, binding_index: usize);

    #[objc::msg_send(setAddress:attributeStride:atIndex:)]
    pub fn set_address_attr_stride(
        &mut self,
        gpu_address: usize,
        stride: usize,
        binding_index: usize,
    );

    #[objc::msg_send(setResource:atBufferIndex:)]
    pub fn set_res(&mut self, res_id: mtl::ResId, binding_index: usize);

    #[objc::msg_send(setTexture:atIndex:)]
    pub fn set_texture(&mut self, res_id: mtl::ResId, binding_index: usize);

    #[objc::msg_send(setSamplerState:atIndex:)]
    pub fn set_sampler_state(&mut self, res_id: mtl::ResId, binding_index: usize);

    #[objc::msg_send(device)]
    pub fn device(&self) -> arc::R<mtl::Device>;

    #[objc::msg_send(label)]
    pub fn label(&self) -> Option<arc::R<ns::String>>;
}

#[link(name = "mtl", kind = "static")]
unsafe extern "C" {
    static MTL4_ARGUMENT_TABLE_DESCRIPTOR: &'static objc::Class<ArgTableDesc>;
}
