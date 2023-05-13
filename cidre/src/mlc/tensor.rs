use crate::{arc, define_cls, define_obj_type, mlc, ns, objc};

pub type Shape = ns::Array<ns::Number>;

impl<const N: usize> From<[i32; N]> for arc::R<ns::Array<ns::Number>> {
    fn from(value: [i32; N]) -> Self {
        let mut vals = [std::ptr::null(); N];
        for (i, v) in value.iter().enumerate() {
            vals[i] = ns::Number::tagged_i32(*v);
        }
        ns::Array::from_slice(unsafe { std::mem::transmute(&vals[..]) })
    }
}

define_obj_type!(Tensor(ns::Id));
impl Tensor {
    define_cls!(MLC_TENSOR);

    /// A unique number to identify each tensor.  Assigned when the tensor is created.
    #[objc::msg_send(tensorID)]
    pub fn tensor_id(&self) -> usize;

    #[objc::msg_send(descriptor)]
    pub fn descriptor(&self) -> &mlc::TensorDescriptor;

    #[objc::msg_send(data)]
    pub fn data(&self) -> Option<&ns::Data>;

    #[objc::msg_send(label)]
    pub fn label(&self) -> &ns::String;

    #[objc::msg_send(setLabel:)]
    pub fn set_label(&mut self, value: &ns::String);

    #[objc::msg_send(device)]
    pub fn device(&self) -> Option<&mlc::Device>;

    #[objc::cls_msg_send(tensorWithShape:dataType:)]
    pub fn with_shape_data_type_ar(shape: &Shape, data_type: mlc::DataType) -> arc::Rar<Self>;

    #[objc::cls_rar_retain]
    pub fn with_shape_data_type(shape: &Shape, data_type: mlc::DataType) -> arc::R<Self>;

    pub fn with_shape_dt<S: Into<arc::R<Shape>>>(
        shape: S,
        data_type: mlc::DataType,
    ) -> arc::R<Self> {
        Self::with_shape_data_type(&shape.into(), data_type)
    }
}

#[link(name = "mlc", kind = "static")]
extern "C" {
    static MLC_TENSOR: &'static objc::Class<Tensor>;
}
