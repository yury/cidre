use std::ops::DerefMut;

use crate::{arc, define_mtl, define_obj_type, define_options, ns, objc};

define_options!(
    /// Describes how a resource will be used by a shader through an argument buffer
    #[doc(alias = "MTLResourceUsage")]
    ResourceUsage(usize)
);

impl ResourceUsage {
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

define_options!(
    /// Describes the types of resources that the a barrier operates on
    #[doc(alias = "MTLBarrierScope")]
    BarrierScope(usize)
);

impl BarrierScope {
    /// The barrier affects any buffer objects.
    pub const BUFFERS: Self = Self(1 << 0);

    /// The barrier affects textures.
    pub const TEXTURES: Self = Self(1 << 1);

    /// The barrier affects any render targets.
    pub const RENDER_TARGETS: Self = Self(1 << 2);
}

define_obj_type!(
    /// An encoder that writes GPU commands into a command buffer.
    #[doc(alias = "MTLCommandEncoder")]
    CmdEncoder(ns::Id)
);

impl CmdEncoder {
    define_mtl!(device, label, set_label, push_debug_group, pop_debug_group);

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
        let device = mtl::Device::default().unwrap();
        let queue = device.new_cmd_queue().unwrap();
        let buf = queue.new_cmd_buf().unwrap();
        let enc = buf.new_blit_cmd_enc().unwrap();
        enc.end();
    }
}
