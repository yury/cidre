use crate::{arc, define_cls, define_obj_type, mps, mtl, ns, objc};

define_obj_type!(
    #[doc(alias = "MPSNDArrayAllocator")]
    pub NdArrayAllocator(ns::Id)
);

define_obj_type!(
    #[doc(alias = "MPSNDArray")]
    pub NdArray(ns::Id)
);
impl NdArray {
    define_cls!(MPS_NDARRAY);

    #[objc::msg_send(defaultAllocator)]
    pub fn default_allocator() -> arc::R<NdArrayAllocator>;

    #[objc::msg_send(label)]
    pub fn label(&self) -> Option<arc::R<ns::String>>;

    #[objc::msg_send(setLabel:)]
    pub fn set_label(&mut self, value: Option<&ns::String>);

    #[objc::msg_send(dataType)]
    pub fn data_type(&self) -> mps::DType;

    /// Copy bytes from NdArray into buffer
    /// The dimensionality and size of the copy region is given by the size of the NdArray
    /// For subregions, use a NdArray view.
    #[objc::msg_send(readBytes:strideBytes:)]
    pub fn read_bytes(&self, bytes: *mut u8, stride_bytes: *mut isize);

    #[objc::msg_send(dataTypeSize)]
    pub fn data_type_size(&self) -> usize;

    #[objc::msg_send(numberOfDimensions)]
    pub fn ndim(&self) -> usize;

    #[objc::msg_send(lengthOfDimension:)]
    pub fn dim_len(&self, dimension_index: usize) -> usize;

    #[objc::msg_send(device)]
    pub fn device(&self) -> &mtl::Device;

    #[objc::msg_send(descriptor)]
    pub fn descriptor(&self) -> &NdArrayDesc;
}

define_obj_type!(
    #[doc(alias = "MPSNDArrayDescriptor")]
    pub NdArrayDesc(ns::Id)
);

impl NdArrayDesc {
    define_cls!(MPS_NDARRAY_DESCRIPTOR);

    #[objc::msg_send(dataType)]
    pub fn data_type(&self) -> mps::DType;

    #[objc::msg_send(setDataType:)]
    pub fn set_data_type(&mut self, value: mps::DType);

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
unsafe extern "C" {
    static MPS_NDARRAY: &'static objc::Class<NdArray>;
    static MPS_NDARRAY_DESCRIPTOR: &'static objc::Class<NdArrayDesc>;
}
