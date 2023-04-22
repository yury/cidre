use crate::{define_obj_type, mtl, objc};

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
