use crate::define_obj_type;
use crate::mtl;
use crate::ns::{self, Id};

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
        unsafe { rsel_length(self) }
    }

    #[inline]
    pub fn len(&self) -> usize {
        self.length()
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

#[link(name = "mtl", kind = "static")]
extern "C" {
    fn rsel_length(id: &Id) -> ns::UInteger;
}
