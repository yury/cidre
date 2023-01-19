use crate::{define_obj_type, mtl, objc};

define_obj_type!(Buffer(mtl::Resource));

impl Buffer {
    /// ```no_run
    /// use cidre::mtl;
    ///
    /// let device = mtl::Device::default().unwrap();
    ///
    /// let buffer = device.buffer_with_length_and_options(10, Default::default()).unwrap();
    ///
    /// assert_eq!(buffer.len(), 10);
    ///
    /// ```
    #[objc::msg_send2(length)]
    pub fn len(&self) -> usize;

    #[inline]
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
    #[objc::msg_send2(contents)]
    pub fn contents(&self) -> *mut u8;

    #[objc::msg_send2(gpuAddress)]
    pub fn gpu_address(&self) -> u64;
}

#[cfg(test)]
mod tests {
    use crate::mtl;
    #[test]
    fn basics() {
        let device = mtl::Device::default().unwrap();

        let buffer = device
            .buffer_with_length_and_options(10, Default::default())
            .unwrap();

        assert_eq!(buffer.len(), 10);
    }
}
