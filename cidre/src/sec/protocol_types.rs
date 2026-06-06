use crate::{define_obj_type, ns};

define_obj_type!(
    #[doc(alias = "sec_trust")]
    #[doc(alias = "sec_trust_t")]
    pub ProtocolTrust(ns::Id)
);

#[cfg(feature = "blocks")]
pub type ProtocolVerifyComplete = crate::blocks::NoEscBlock<fn(result: bool)>;

#[cfg(feature = "blocks")]
pub type ProtocolVerify = crate::blocks::EscBlock<
    fn(
        metadata: &crate::sec::ProtocolMetadata,
        trust: &ProtocolTrust,
        complete: &mut ProtocolVerifyComplete,
    ),
>;
