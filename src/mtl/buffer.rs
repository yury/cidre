use crate::{define_obj_type, msg_send, mtl};

define_obj_type!(Buffer(mtl::Resource));

impl Buffer {
    /// ```
    /// use cidre::mtl;
    ///
    /// let device = mtl::Device::default().unwrap();
    ///
    /// let buffer = device.buffer_with_length_and_options(10, Default::default()).unwrap();
    ///
    /// assert_eq!(buffer.length(), 10);
    /// assert_eq!(buffer.len(), 10);
    ///
    /// ```
    #[inline]
    pub fn length(&self) -> usize {
        msg_send!("common", self, sel_length)
    }

    #[inline]
    pub fn len(&self) -> usize {
        self.length()
    }

    #[inline]
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    #[inline]
    pub fn contents(&self) -> *mut u8 {
        msg_send!("mtl", self, sel_content)
    }

    #[inline]
    pub fn gpu_address(&self) -> u64 {
        msg_send!("mtl", self, sel_gpuAddress)
    }
}
