use crate::{arc, define_obj_type, mtl, ns, objc};

define_obj_type!(Buf(mtl::Resource));

impl Buf {
    #[objc::msg_send(length)]
    pub fn len(&self) -> usize;

    #[inline]
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    #[objc::msg_send(contents)]
    pub fn contents(&self) -> *mut u8;

    #[inline]
    pub fn as_slice(&self) -> &[u8] {
        unsafe { std::slice::from_raw_parts(self.contents(), self.len()) }
    }

    #[inline]
    pub fn as_slice_mut(&mut self) -> &mut [u8] {
        unsafe { std::slice::from_raw_parts_mut(self.contents(), self.len()) }
    }

    /// Inform the device of the range of a buffer that the CPU has modified,
    /// allowing the implementation to invalidate  its caches of the buffer's content.
    #[objc::msg_send(didModifyRange:)]
    pub fn did_modify_range(&self, range: ns::Range);

    #[objc::msg_send(newTextureWithDescriptor:offset:bytesPerRow:)]
    pub fn new_texture_with_descriptor_ar(
        &self,
        descritptor: &mtl::TextureDescriptor,
        offset: usize,
        bytes_per_row: usize,
    ) -> Option<&'ar mtl::Texture>;

    #[objc::rar_retain()]
    pub fn new_texture_with_descriptor(
        &self,
        descritptor: &mtl::TextureDescriptor,
        offset: usize,
        bytes_per_row: usize,
    ) -> Option<arc::R<mtl::Texture>>;

    #[objc::msg_send(addDebugMarker:range:)]
    pub fn add_debug_marker(&mut self, marker: &ns::String, range: ns::Range);

    #[objc::msg_send(removeAllDebugMarkers)]
    pub fn remove_all_debug_markers(&mut self);

    /// Represents the GPU virtual address of a buffer resource
    #[objc::msg_send(gpuAddress)]
    pub fn gpu_address(&self) -> u64;
}

#[cfg(test)]
mod tests {
    use crate::mtl;

    #[test]
    fn basics() {
        let device = mtl::Device::default().unwrap();

        let mut buffer = device.new_buf_len_opts(10, Default::default()).unwrap();
        buffer.as_slice_mut()[1] = 10;

        assert_eq!(buffer.len(), 10);
    }
}
