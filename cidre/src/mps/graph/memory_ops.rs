use crate::{arc, cf, define_obj_type, mps, mps::graph, ns, objc};

define_obj_type!(pub VariableOp(graph::Operation));

impl VariableOp {
    #[objc::msg_send(shape)]
    pub fn shape(&self) -> &mps::Shape;

    #[objc::msg_send(dataType)]
    pub fn data_type(&self) -> mps::DataType;
}

impl graph::Graph {
    #[objc::msg_send(placeholderWithShape:dataType:name:)]
    pub fn placeholder_with_shape_ar(
        &self,
        shape: Option<&mps::Shape>,
        data_type: mps::DataType,
        name: Option<&cf::String>,
    ) -> arc::Rar<graph::Tensor>;

    #[objc::rar_retain()]
    pub fn placeholder_with_shape(
        &self,
        shape: Option<&mps::Shape>,
        data_type: mps::DataType,
        name: Option<&cf::String>,
    ) -> arc::R<graph::Tensor>;

    #[objc::msg_send(constantWithData:shape:dataType:)]
    pub fn constant_with_data_shape_data_type_ar(
        &self,
        data: &ns::Data,
        shape: &mps::Shape,
        data_type: mps::DataType,
    ) -> arc::Rar<graph::Tensor>;

    #[objc::rar_retain]
    pub fn constant_with_data_shape_data_type(
        &self,
        data: &ns::Data,
        shape: &mps::Shape,
        data_type: mps::DataType,
    ) -> arc::R<graph::Tensor>;

    #[objc::msg_send(constantWithScalar:dataType:)]
    pub fn constant_ar(&self, scalar: f64, data_type: mps::DataType) -> arc::Rar<graph::Tensor>;

    #[objc::rar_retain]
    pub fn constant(&self, scalar: f64, data_type: mps::DataType) -> arc::R<graph::Tensor>;

    #[objc::msg_send(constantWithScalar:shape:dataType:)]
    pub fn constant_shape_ar(
        &self,
        scalar: f64,
        shape: &mps::Shape,
        data_type: mps::DataType,
    ) -> arc::Rar<graph::Tensor>;

    #[objc::rar_retain]
    pub fn constant_shape(
        &self,
        scalar: f64,
        shape: &mps::Shape,
        data_type: mps::DataType,
    ) -> arc::R<graph::Tensor>;
}

#[cfg(test)]
mod tests {
    use crate::{mps, mps::graph};

    #[test]
    pub fn lifetimes() {
        let tensor = {
            let gr = graph::Graph::new();
            let t = gr.placeholder_with_shape(None, mps::DataType::F32, None);
            t
        };
        assert_eq!(1, tensor.as_type_ref().retain_count());
        // this will crash, since we released graph. Same crash will be in Swift too.
        // We may add lifetime to tensor
        // assert_eq!("mps_placeholder", tensor.operation().name().to_string());
    }
    #[test]
    pub fn basics() {
        let gr = graph::Graph::new();
        let tensor = gr.placeholder_with_shape(None, mps::DataType::F32, None);
        assert_eq!("mps_placeholder", tensor.operation().name().to_string());
        assert!(tensor.shape().is_none());
    }
}
