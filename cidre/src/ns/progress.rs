use crate::{define_obj_type, ns, objc};

define_obj_type!(
    #[doc(alias = "NSProgress")]
    pub Progress(ns::Id)
);

impl Progress {
    #[objc::msg_send(isCancelled)]
    pub fn is_cancelled(&self) -> bool;

    #[objc::msg_send(isPaused)]
    pub fn is_paused(&self) -> bool;

    #[objc::msg_send(isIndeterminate)]
    pub fn is_indeterminate(&self) -> bool;

    #[objc::msg_send(fractionCompleted)]
    pub fn fraction_completed(&self) -> f64;

    #[objc::msg_send(isFinished)]
    pub fn is_finished(&self) -> bool;

    #[objc::msg_send(cancel)]
    pub fn cancel(&mut self);

    #[objc::msg_send(pause)]
    pub fn pause(&mut self);

    #[objc::msg_send(resume)]
    pub fn resume(&mut self);
}
