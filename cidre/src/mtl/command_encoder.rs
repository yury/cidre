use std::ops::DerefMut;

use crate::{arc, define_mtl, define_obj_type, define_opts, mtl, ns, objc};

define_opts!(
    /// Describes how a resource will be used by a shader through an argument buffer
    #[doc(alias = "MTLResourceUsage")]
    pub ResUsage(usize)
);

impl ResUsage {
    /// An option that enables reading from the resource.
    pub const READ: Self = Self(1 << 0);

    /// An option that enables writing to the resource.
    pub const WRITE: Self = Self(1 << 1);

    /*
    /// An option that enables sampling from the resource.
    // #[deprecated(since = "0.1.0", note = "use mtl::ResourceUsage::READ instead")]
    // pub const SAMPLE: Self = Self(1 << 2);
     */
}

define_opts!(
    /// Describes the types of resources that the a barrier operates on
    #[doc(alias = "MTLBarrierScope")]
    pub BarrierScope(usize)
);

impl BarrierScope {
    /// The barrier affects any buffer objects.
    pub const BUFFERS: Self = Self(1 << 0);

    /// The barrier affects textures.
    pub const TEXTURES: Self = Self(1 << 1);

    /// The barrier affects any render targets.
    pub const RENDER_TARGETS: Self = Self(1 << 2);
}

define_opts!(
    #[doc(alias = "MTLStages")]
    pub Stages(usize)
);

impl Stages {
    /// Represents all vertex shader stage work in a render pass.
    pub const VERTEX: Self = Self(1 << 0);

    /// Represents all fragment shader stage work in a render pass.
    pub const FRAGMENT: Self = Self(1 << 1);

    /// Represents all tile shading stage work in a render pass.
    pub const TILE: Self = Self(1 << 2);

    /// Represents all object shader stage work in a render pass.
    pub const OBJ: Self = Self(1 << 3);

    /// Represents all mesh shader stage work work in a render pass.
    pub const MESH: Self = Self(1 << 4);

    /// Represents all sparse and placement sparse resource mapping updates.
    pub const RES_STATE: Self = Self(1 << 26);

    /// Represents all compute dispatches in a compute pass.
    pub const DISPATCH: Self = Self(1 << 27);

    /// Represents all blit operations in a pass.
    pub const BLIT: Self = Self(1 << 28);

    /// Represents all acceleration structure operations.
    pub const ACCELERATION_STRUCTURE: Self = Self(1 << 29);

    /// Represents all machine learning network dispatch operations.
    pub const ML: Self = Self(1 << 30);

    /// Convenience mask representing all stages of GPU work.
    pub const ALL: Self = Self(usize::MAX);
}

define_obj_type!(
    /// An encoder that writes GPU commands into a command buffer.
    #[doc(alias = "MTLCommandEncoder")]
    pub CmdEncoder(ns::Id)
);

impl CmdEncoder {
    define_mtl!(set_label, push_debug_group, pop_debug_group);

    #[objc::msg_send(device)]
    pub fn device(&self) -> arc::R<mtl::Device>;

    #[objc::msg_send(label)]
    pub fn label(&self) -> Option<arc::R<ns::String>>;

    /// Declares that all command generation from the encoder is completed.
    ///
    /// After endEncoding is called, the command encoder has no further use.
    /// You cannot encode any other commands with this encoder.
    ///
    /// # Safety
    ///
    /// Use `end()` function instead
    #[objc::msg_send(endEncoding)]
    pub unsafe fn end_encoding(&mut self);

    #[objc::msg_send(insertDebugSignpost:)]
    pub fn insert_debug_signpost(&mut self, signpost: &ns::String);
}

impl<T> arc::R<T>
where
    T: objc::Obj + DerefMut<Target = CmdEncoder>,
{
    pub fn end(mut self) {
        unsafe { self.end_encoding() }
    }
}

#[cfg(test)]
mod tests {
    use crate::mtl;

    #[test]
    fn basics() {
        let device = mtl::Device::sys_default().unwrap();
        let queue = device.new_cmd_queue().unwrap();
        let buf = queue.new_cmd_buf().unwrap();
        let enc = buf.new_blit_cmd_enc().unwrap();
        enc.end();
    }
}
