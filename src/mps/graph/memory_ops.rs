use crate::{cf, define_obj_type, mps, mps::graph, msg_send, ns};

define_obj_type!(VariableOp(graph::Operation));

impl VariableOp {
    #[inline]
    pub fn shape(&self) -> &mps::Shape {
        msg_send!("mpsg", self, sel_shape)
    }

    #[inline]
    pub fn data_type(&self) -> mps::DataType {
        msg_send!("mpsg", self, sel_dataType)
    }
}

impl graph::Graph {
    pub fn placeholder_with_shape(
        &self,
        shape: Option<&mps::Shape>,
        data_type: mps::DataType,
        name: Option<&cf::String>,
    ) -> cf::Retained<graph::Tensor> {
        unsafe { rsel_placeholderWithShape_dataType_name(self, shape, data_type, name) }
    }
}

#[link(name = "mpsg", kind = "static")]
extern "C" {
    fn rsel_placeholderWithShape_dataType_name(
        id: &ns::Id,
        shape: Option<&mps::Shape>,
        data_type: mps::DataType,
        name: Option<&cf::String>,
    ) -> cf::Retained<graph::Tensor>;
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
        //assert_eq!("mps_placeholder", tensor.operation().name().to_string());
    }
    #[test]
    pub fn basics() {
        let gr = graph::Graph::new();
        let tensor = gr.placeholder_with_shape(None, mps::DataType::F32, None);
        assert_eq!("mps_placeholder", tensor.operation().name().to_string());
        assert!(tensor.shape().is_none());
    }
}
