use crate::{arc, objc};

#[objc::protocol(NSAnimatablePropertyContainer)]
pub trait AnimatablePropContainer: objc::Obj
where
    Self: 'static,
{
    #[objc::msg_send(animator)]
    fn animator(&self) -> arc::R<Self>;
}
