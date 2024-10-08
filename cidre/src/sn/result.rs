use crate::{define_obj_type, ns, objc, sn};

define_obj_type!(
    #[doc(alias = "SNResult")]
    pub Result(ns::Id)
);

#[objc::protocol(SNResultsObserving)]
pub trait ResultsObserving: objc::Obj {
    #[objc::msg_send(request:didProduceResult:)]
    fn request_did_produce_result(&mut self, request: &sn::Request, result: &sn::Result);

    #[objc::optional]
    #[objc::msg_send(request:didFailWithError:)]
    fn request_did_fail_with_err(&mut self, request: &sn::Request, error: &ns::Error);

    #[objc::optional]
    #[objc::msg_send(requestDidComplete:)]
    fn request_did_complete(&mut self, request: &sn::Request);
}
