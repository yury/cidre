use crate::{arc, define_cls, define_obj_type, mlc, ns, objc};

pub type Shape = ns::Array<ns::Number>;

define_obj_type!(pub Tensor(ns::Id));
impl Tensor {
    define_cls!(MLC_TENSOR);

    /// A unique number to identify each tensor.  Assigned when the tensor is created.
    #[objc::msg_send(tensorID)]
    pub fn tensor_id(&self) -> usize;

    #[objc::msg_send(descriptor)]
    pub fn desc(&self) -> &mlc::TensorDesc;

    #[objc::msg_send(data)]
    pub fn data(&self) -> Option<&ns::Data>;

    #[objc::msg_send(label)]
    pub fn label(&self) -> &ns::String;

    #[objc::msg_send(setLabel:)]
    pub fn set_label(&mut self, val: &ns::String);

    #[objc::msg_send(device)]
    pub fn device(&self) -> Option<&mlc::Device>;

    #[objc::cls_msg_send(tensorWithShape:dataType:)]
    pub fn with_shape_dtype_ar(shape: &Shape, data_type: mlc::DType) -> arc::Rar<Self>;

    #[objc::cls_rar_retain]
    pub fn with_shape_dtype(shape: &Shape, data_type: mlc::DType) -> arc::R<Self>;

    pub fn with_shape_dt<S: Into<arc::R<Shape>>>(shape: S, data_type: mlc::DType) -> arc::R<Self> {
        Self::with_shape_dtype(&shape.into(), data_type)
    }

    #[objc::msg_send(copyDataFromDeviceMemoryToBytes:length:synchronizeWithDevice:)]
    pub fn copy_from_device_mem_to_bytes(
        &self,
        buf: *mut u8,
        length: usize,
        sync_with_device: bool,
    ) -> bool;

    #[inline]
    pub fn copy_from_device_mem_to_buf<T: Sized>(
        &self,
        buf: &mut [T],
        sync_with_device: bool,
    ) -> Result<(), ()> {
        if self.copy_from_device_mem_to_bytes(
            buf.as_mut_ptr() as _,
            std::mem::size_of_val(buf),
            sync_with_device,
        ) {
            Ok(())
        } else {
            Err(())
        }
    }
}

#[link(name = "mlc", kind = "static")]
extern "C" {
    static MLC_TENSOR: &'static objc::Class<Tensor>;
}
