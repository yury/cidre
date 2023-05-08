use crate::{define_cls, define_obj_type, mps, ns, objc};

define_obj_type!(NDArray(ns::Id));
impl NDArray {
    define_cls!(MPS_NDARRAY);
    /// Copy bytes from NDArray into buffer
    /// The dimensionality and size of the copy region is given by the size of the NDArray
    /// For subregions, use a NDArray view.
    #[doc(alias = "readBytes:strideBytes:")]
    #[objc::msg_send(readBytes:strideBytes:)]
    pub fn read_bytes(&self, bytes: *mut u8, stride_bytes: *mut isize);

    #[objc::msg_send(descriptor)]
    pub fn descriptor(&self) -> &NDArrayDescriptor;
}

define_obj_type!(NDArrayDescriptor(ns::Id));
impl NDArrayDescriptor {
    define_cls!(MPS_NDARRAY_DESCRIPTOR);

    #[objc::msg_send(dataType)]
    pub fn data_type(&self) -> mps::DataType;

    #[objc::msg_send(setDataType:)]
    pub fn set_data_type(&mut self, value: mps::DataType);

    #[objc::msg_send(numberOfDimensions)]
    pub fn ndim(&self) -> usize;

    #[objc::msg_send(setNumberOfDimensions:)]
    pub fn set_ndim(&mut self, value: usize);

    #[objc::msg_send(lengthOfDimension:)]
    pub fn dim_len(&self, dim_index: usize) -> usize;

    #[objc::msg_send(sliceRangeForDimension:)]
    pub fn dim_slice_range(&self, dim_index: usize) -> mps::DimensionSlice;
}

#[link(name = "mps", kind = "static")]
extern "C" {
    static MPS_NDARRAY: &'static objc::Class<NDArray>;
    static MPS_NDARRAY_DESCRIPTOR: &'static objc::Class<NDArrayDescriptor>;
}
