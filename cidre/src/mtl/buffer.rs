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
    #[objc::msg_send(didModifyRange:)]
    pub fn did_modify_range(&self, range: ns::Range);

    #[inline]
    pub fn did_modify<T: Sized>(&self, range: std::ops::Range<usize>) {
        let start = range.start * std::mem::size_of::<T>();
        let end = range.end * std::mem::size_of::<T>();
        self.did_modify_range(ns::Range {
            loc: start,
            len: end - start,
        })
    }

    #[objc::msg_send(newTextureWithDescriptor:offset:bytesPerRow:)]
    pub fn new_texture(
        &self,
        descritptor: &mtl::TextureDesc,
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

        let mut buffer = device.new_buf(10, Default::default()).unwrap();
        buffer.as_mut_slice()[1] = 10;

        assert_eq!(buffer.len(), 10);
    }
}
