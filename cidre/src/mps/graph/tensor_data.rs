#[allow(unused_imports)]
use crate::api;
use crate::{arc, define_obj_type, mps, mps::graph, mtl, ns, objc};

define_obj_type!(
    #[doc(alias = "MPSGraphTensorData")]
    pub TensorData(ns::Id),
    MPS_GRAPH_TENSOR_DATA
);

impl arc::A<TensorData> {
    #[objc::msg_send(initWithDevice:data:shape:dataType:)]
    pub fn init_with_device_data_shape_data_type(
        self,
        device: &graph::Device,
        data: &ns::Data,
        shape: &mps::Shape,
        data_type: mps::DType,
    ) -> arc::R<TensorData>;

    #[objc::msg_send(initWithMTLBuffer:shape:dataType:)]
    pub fn init_with_mtl_buffer_shape_data_type(
        self,
        buffer: &mtl::Buf,
        shape: &mps::Shape,
        data_type: mps::DType,
    ) -> arc::R<TensorData>;

    #[objc::msg_send(initWithMTLBuffer:shape:dataType:rowBytes:)]
    #[api::available(macos = 12.3, ios = 15.4, tvos = 15.4)]
    pub fn init_with_mtl_buffer_shape_data_type_row_bytes(
        self,
        buffer: &mtl::Buf,
        shape: &mps::Shape,
        data_type: mps::DType,
        row_bytes: usize,
    ) -> arc::R<TensorData>;

    #[objc::msg_send(initWithMPSNDArray:)]
    pub fn init_with_mps_nd_array(self, ndarray: &mps::NdArray) -> arc::R<TensorData>;
}

impl TensorData {
    #[inline]
    pub fn with_data(
        device: &graph::Device,
        data: &ns::Data,
        shape: &mps::Shape,
        data_type: mps::DType,
    ) -> arc::R<Self> {
        Self::alloc().init_with_device_data_shape_data_type(device, data, shape, data_type)
    }

    #[inline]
    pub fn with_mtl_buffer(
        buffer: &mtl::Buf,
        shape: &mps::Shape,
        data_type: mps::DType,
    ) -> arc::R<Self> {
        Self::alloc().init_with_mtl_buffer_shape_data_type(buffer, shape, data_type)
    }

    #[inline]
    #[api::available(macos = 12.3, ios = 15.4, tvos = 15.4)]
    pub fn with_mtl_buffer_row_bytes(
        buffer: &mtl::Buf,
        shape: &mps::Shape,
        data_type: mps::DType,
        row_bytes: usize,
    ) -> arc::R<Self> {
        Self::alloc()
            .init_with_mtl_buffer_shape_data_type_row_bytes(buffer, shape, data_type, row_bytes)
    }

    #[inline]
    pub fn with_mps_nd_array(ndarray: &mps::NdArray) -> arc::R<Self> {
        Self::alloc().init_with_mps_nd_array(ndarray)
    }

    #[objc::msg_send(shape)]
    pub fn shape(&self) -> arc::R<mps::Shape>;

    #[objc::msg_send(dataType)]
    pub fn data_type(&self) -> mps::DType;

    #[objc::msg_send(device)]
    pub fn device(&self) -> &mps::graph::Device;

    #[objc::msg_send(mpsndarray)]
    pub fn mps_nd_array(&self) -> arc::R<mps::NdArray>;
}

#[link(name = "mpsg", kind = "static")]
unsafe extern "C" {
    static MPS_GRAPH_TENSOR_DATA: &'static objc::Class<TensorData>;
}
