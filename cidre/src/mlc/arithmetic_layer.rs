use crate::{arc, define_cls, define_obj_type, mlc, objc};

define_obj_type!(pub ArithmeticLayer(mlc::Layer));

impl ArithmeticLayer {
    define_cls!(sym MLCArithmeticLayer);

    #[objc::msg_send(operation)]
    pub fn operation(&self) -> mlc::ArithmeticOp;

    #[objc::msg_send(layerWithOperation:)]
    pub fn with_op(op: mlc::ArithmeticOp) -> arc::R<Self>;
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
