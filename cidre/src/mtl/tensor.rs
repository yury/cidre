use crate::{api, arc, define_obj_type, define_opts, mtl, ns, objc};

/// The possible data types for the elements of a tensor.
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
#[repr(isize)]
pub enum TensorDType {
    #[doc(alias = "MTLTensorDataTypeNone")]
    None = mtl::DType::None as _,
    #[doc(alias = "MTLTensorDataTypeFloat32")]
    F32 = mtl::DType::F32 as _,
    #[doc(alias = "MTLTensorDataTypeFloat16")]
    F16 = mtl::DType::F16 as _,
    #[doc(alias = "MTLTensorDataTypeBFloat16")]
    Bf16 = mtl::DType::Bf16 as _,
    #[doc(alias = "MTLTensorDataTypeInt8")]
    I8 = mtl::DType::I8 as _,
    #[doc(alias = "MTLTensorDataTypeUInt8")]
    U8 = mtl::DType::U8 as _,
    #[doc(alias = "MTLTensorDataTypeInt16")]
    I16 = mtl::DType::I16 as _,
    #[doc(alias = "MTLTensorDataTypeUInt16")]
    U16 = mtl::DType::U16 as _,
    #[doc(alias = "MTLTensorDataTypeInt32")]
    I32 = mtl::DType::I32 as _,
    #[doc(alias = "MTLTensorDataTypeUInt32")]
    U32 = mtl::DType::U32 as _,
}

/// The largest rank a tensor can have.
#[doc(alias = "MTL_TENSOR_MAX_RANK")]
pub const TENSOR_MAX_RANK: usize = 16;

define_obj_type!(
    #[doc(alias = "MTLTensorExtents")]
    pub TensorExtents(ns::Id),
    MTL_TENSOR_EXTENTS,
    #[api::available(macos = 26.0, ios = 26.0, tvos = 26.0, visionos = 26.0)]
);

impl arc::A<TensorExtents> {
    #[objc::msg_send(initWithRank:values:)]
    #[api::available(macos = 26.0, ios = 26.0, tvos = 26.0, visionos = 26.0)]
    pub fn init_with_rank(self, rank: usize, values: *const usize)
    -> Option<arc::R<TensorExtents>>;
}

#[cfg(any(
    feature = "macos_26_0",
    feature = "ios_26_0",
    feature = "tvos_26_0",
    feature = "visionos_26_0"
))]
impl TensorExtents {
    #[allow(unused_unsafe)]
    pub fn with_dims(dims: &[usize]) -> Option<arc::R<Self>> {
        if dims.is_empty() {
            unsafe { Self::alloc().init_with_rank(0, std::ptr::null()) }
        } else {
            unsafe { Self::alloc().init_with_rank(dims.len(), dims.as_ptr()) }
        }
    }
}

impl TensorExtents {
    #[objc::msg_send(rank)]
    pub fn rank(&self) -> usize;

    #[objc::msg_send(extentAtDimensionIndex:)]
    pub fn extent_at_dim(&self, index: usize) -> isize;
}

define_opts!(
    #[doc(alias = "MTLTensorUsage")]
    pub TensorUsage(isize)
);

impl TensorUsage {
    #[doc(alias = "MTLTensorUsageCompute")]
    pub const COMPUTE: Self = Self(1 << 0);

    #[doc(alias = "MTLTensorUsageRender")]
    pub const RENDER: Self = Self(1 << 1);

    #[doc(alias = "MTLTensorUsageMachineLearning")]
    pub const ML: Self = Self(1 << 2);
}

define_obj_type!(
    #[doc(alias = "MTLTensorDescriptor")]
    pub TensorDesc(ns::Id),
    MTL_TENSOR_DESCRIPTOR,
    #[api::available(macos = 26.0, ios = 26.0, tvos = 26.0, visionos = 26.0)]
);

impl ns::Copying for TensorDesc {}

impl TensorDesc {
    #[objc::msg_send(dimensions)]
    pub fn dims(&self) -> arc::R<mtl::TensorExtents>;

    #[objc::msg_send(setDimensions:)]
    pub fn set_dims(&mut self, val: &mtl::TensorExtents);

    #[objc::msg_send(strides)]
    pub fn strides(&self) -> arc::R<mtl::TensorExtents>;

    #[objc::msg_send(setStrides:)]
    pub fn set_strides(&mut self, val: &mtl::TensorExtents);

    #[objc::msg_send(dataType)]
    pub fn d_type(&self) -> mtl::TensorDType;

    #[objc::msg_send(setDataType:)]
    pub fn set_d_type(&mut self, val: mtl::TensorDType);

    #[objc::msg_send(usage)]
    pub fn usage(&self) -> mtl::TensorUsage;

    #[objc::msg_send(setUsage:)]
    pub fn set_usage(&mut self, val: mtl::TensorUsage);

    #[objc::msg_send(resourceOptions)]
    pub fn res_opts(&self) -> mtl::ResOpts;

    #[objc::msg_send(setResourceOptions:)]
    pub fn set_res_opts(&mut self, val: mtl::ResOpts);

    #[objc::msg_send(cpuCacheMode)]
    pub fn cpu_cache_mode(&self) -> mtl::CpuCacheMode;

    #[objc::msg_send(setCpuCacheMode:)]
    pub fn set_cpu_cache_mode(&mut self, val: mtl::CpuCacheMode);

    #[objc::msg_send(storageMode)]
    pub fn storage_mode(&self) -> mtl::StorageMode;

    #[objc::msg_send(setStorageMode:)]
    pub fn set_storage_mode(&mut self, val: mtl::StorageMode);

    #[objc::msg_send(hazardTrackingMode)]
    pub fn hazard_tracking_mode(&self) -> mtl::HazardTrackingMode;

    #[objc::msg_send(setHazardTrackingMode:)]
    pub fn set_hazard_tracking_mode(&mut self, val: mtl::HazardTrackingMode);
}

define_obj_type!(
    #[doc(alias = "MTLTensor")]
    pub Tensor(mtl::Res),
    MTL_TENSOR,
    #[api::available(macos = 26.0, ios = 26.0, tvos = 26.0, visionos = 26.0)]
);

impl Tensor {
    #[objc::msg_send(gpuResourceID)]
    pub fn gpu_res_id(&self) -> mtl::ResId;

    #[objc::msg_send(buffer)]
    pub fn buf(&self) -> Option<arc::R<mtl::Buf>>;

    #[objc::msg_send(bufferOffset)]
    pub fn buf_offset(&self) -> usize;

    #[objc::msg_send(strides)]
    pub fn strides(&self) -> arc::R<mtl::TensorExtents>;

    #[objc::msg_send(dimensions)]
    pub fn dims(&self) -> arc::R<mtl::TensorExtents>;

    #[objc::msg_send(dataType)]
    pub fn d_type(&self) -> mtl::TensorDType;

    #[objc::msg_send(usage)]
    pub fn usage(&self) -> mtl::TensorUsage;

    #[objc::msg_send(replaceSliceOrigin:sliceDimensions:withBytes:strides:)]
    pub unsafe fn replace_slice(
        &mut self,
        slice_origin: &mtl::TensorExtents,
        slice_dims: &mtl::TensorExtents,
        bytes: *const u8,
        strides: &mtl::TensorExtents,
    );

    #[objc::msg_send(getBytes:strides:fromSliceOrigin:sliceDimensions:)]
    pub unsafe fn copy_bytes(
        &self,
        buf: *mut u8,
        strides: &mtl::TensorExtents,
        slice_origin: &mtl::TensorExtents,
        slice_dims: &mtl::TensorExtents,
    );
}

#[link(name = "mtl", kind = "static")]
unsafe extern "C" {
    static MTL_TENSOR_EXTENTS: &'static objc::Class<TensorExtents>;
    static MTL_TENSOR_DESCRIPTOR: &'static objc::Class<TensorDesc>;
    static MTL_TENSOR: &'static objc::Class<Tensor>;
}
