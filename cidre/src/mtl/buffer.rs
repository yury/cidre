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

    #[objc::msg_send(gpuAddress)]
    pub fn gpu_address(&self) -> u64;
}

#[cfg(test)]
mod tests {
    use crate::mtl;
    #[test]
    fn basics() {
        let device = mtl::Device::default().unwrap();

        let buffer = device.new_buf_len_opts(10, Default::default()).unwrap();

        assert_eq!(buffer.len(), 10);
    }
}
