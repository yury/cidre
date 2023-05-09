use crate::{arc, define_cls, define_obj_type, mlc, objc};

define_obj_type!(ArithmeticLayer(mlc::Layer));

impl ArithmeticLayer {
    define_cls!(MLC_ARITHMETIC_LAYER);

    #[objc::msg_send(operation)]
    pub fn operation(&self) -> mlc::ArithmeticOp;

    #[objc::cls_msg_send(layerWithOperation:)]
    pub fn with_op_ar(op: mlc::ArithmeticOp) -> arc::Rar<Self>;

    #[objc::cls_rar_retain]
    pub fn with_op(op: mlc::ArithmeticOp) -> arc::R<Self>;
}

#[link(name = "mlc", kind = "static")]
extern "C" {
    static MLC_ARITHMETIC_LAYER: &'static objc::Class<ArithmeticLayer>;
}

#[cfg(test)]
mod tests {
    use crate::mlc;

    #[test]
    fn basics() {
        let layer = mlc::ArithmeticLayer::with_op(mlc::ArithmeticOp::Add);
        println!("layer {layer:?}");
    }
}
