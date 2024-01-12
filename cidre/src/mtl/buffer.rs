use crate::{arc, define_obj_type, mtl, ns, objc};

define_obj_type!(
    /// A typeless allocation accessible by both the CPU and the GPU (MTLDevice) or by only the GPU
    /// when the storage mode is MTLResourceStorageModePrivate.
    ///
    /// Unlike in OpenGL and OpenCL, access to buffers is not synchronized. The caller may use the CPU
    /// to modify the data at any time but is also responsible for ensuring synchronization and coherency.
    ///
    /// The contents become undefined if both the CPU and GPU write to the same buffer without
    /// a synchronizing action between those writes. This is true even when the regions written do not overlap.
    #[doc(alias = "MTLBuffer")]
    pub Buf(mtl::Res)
);

impl Buf {
    /// The length of the buffer in bytes.
    #[objc::msg_send(length)]
    pub fn len(&self) -> usize;

    #[inline]
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Returns the data pointer of this buffer's shared copy.
    #[objc::msg_send(contents)]
    pub unsafe fn contents(&self) -> *mut u8;

    #[inline]
    pub fn as_slice(&self) -> &[u8] {
        unsafe { std::slice::from_raw_parts(self.contents(), self.len()) }
    }

    #[inline]
    pub fn as_slice_of<T: Sized>(&self) -> &[T] {
        let len = self.len() / std::mem::size_of::<T>();
        unsafe { std::slice::from_raw_parts(self.contents() as _, len) }
    }

    #[inline]
    pub fn as_mut_slice(&mut self) -> &mut [u8] {
        unsafe { std::slice::from_raw_parts_mut(self.contents(), self.len()) }
    }

    #[inline]
    pub fn as_mut_slice_of<T: Sized>(&mut self) -> &mut [T] {
        let len = self.len() / std::mem::size_of::<T>();
        unsafe { std::slice::from_raw_parts_mut(self.contents() as _, len) }
    }

    /// Inform the device of the range of a buffer that the CPU has modified,
    /// allowing the implementation to invalidate  its caches of the buffer's content.
    #[cfg(target_os = "macos")]
    #[objc::msg_send(didModifyRange:)]
    pub fn did_modify_range(&self, range: ns::Range);

    #[cfg(target_os = "macos")]
    #[inline]
    pub fn did_modify<T: Sized>(&self, range: std::ops::Range<usize>) {
        let start = range.start * std::mem::size_of::<T>();
        let end = range.end * std::mem::size_of::<T>();
        self.did_modify_range(ns::Range {
            loc: start,
            len: end - start,
        })
    }

    /// Create a 2D texture or texture buffer that shares storage with this buffer.
    #[objc::msg_send(newTextureWithDescriptor:offset:bytesPerRow:)]
    pub fn new_texture(
        &self,
        descritptor: &mtl::TextureDesc,
        offset: usize,
        bytes_per_row: usize,
    ) -> Option<arc::R<mtl::Texture>>;

    #[objc::msg_send(addDebugMarker:range:)]
    pub fn add_debug_marker(&mut self, marker: &ns::String, range: ns::Range);

    /// Removes all debug markers from a buffer.
    #[objc::msg_send(removeAllDebugMarkers)]
    pub fn remove_all_debug_markers(&mut self);

    /// Represents the GPU virtual address of a buffer resource
    #[objc::msg_send(gpuAddress)]
    pub fn gpu_addr(&self) -> u64;
}

#[cfg(test)]
mod tests {
    use crate::mtl;

    #[test]
    fn basics() {
        let device = mtl::Device::sys_default().unwrap();

        let mut buffer = device.new_buf(10, Default::default()).unwrap();
        buffer.as_mut_slice()[1] = 10;

        assert_eq!(buffer.len(), 10);
    }
}
