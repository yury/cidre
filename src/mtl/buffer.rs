use crate::{
    define_obj_type, mtl,
    objc::{msg_send, Obj},
};

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
    #[inline]
    pub fn len(&self) -> usize {
        unsafe { self.call0(msg_send::length) }
    }

    #[inline]
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    #[inline]
    pub fn contents(&self) -> *mut u8 {
        unsafe { self.call0(mtl::msg_send::contents) }
    }

    #[inline]
    pub fn gpu_address(&self) -> u64 {
        unsafe { self.call0(mtl::msg_send::gpu_address) }
    }
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
