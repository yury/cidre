use crate::{arc, define_cls, define_obj_type, mc, ns, objc};

define_obj_type!(
    pub AdvertiserAssistant(ns::Id)
);

impl arc::A<AdvertiserAssistant> {
    #[objc::msg_send(initwithpeer:servicetype:)]
    pub unsafe fn init_service_type_throws(
        self,
        peer: &mc::PeerId,
        service_type: &ns::String,
    ) -> arc::R<AdvertiserAssistant>;
}

impl AdvertiserAssistant {
    define_cls!(MC_ADVERTISER_ASSISTANT);

    pub fn with_service_type<'ear>(
        peer: &mc::PeerId,
        service_type: &ns::String,
    ) -> ns::ExResult<'ear, arc::R<Self>> {
        ns::try_catch(|| unsafe { Self::alloc().init_service_type_throws(peer, service_type) })
    }

    #[objc::msg_send(start)]
    pub fn start(&mut self);

    #[objc::msg_send(stop)]
    pub fn stop(&mut self);

    #[objc::msg_send(session)]
    pub fn session(&self) -> arc::R<mc::Session>;

    #[objc::msg_send(discoveryInfo)]
    pub fn discovery_info(&self) -> Option<arc::R<ns::Dictionary<ns::String, ns::String>>>;

    #[objc::msg_send(serviceType)]
    pub fn service_type(&self) -> arc::R<ns::String>;

    #[objc::msg_send(setDelegate:)]
    pub fn set_delegate<D: Delegate>(&mut self, val: &D);

    #[objc::msg_send(delegate)]
    pub fn delegate(&self) -> arc::R<AnyDelegate>;
}

#[objc::protocol(MCAdvertiserAssistantDelegate)]
pub trait Delegate {
    #[objc::msg_send(advertiserAssistantWillPresentInvitation:)]
    fn assistant_will_present_invitation(&mut self, assistant: &AdvertiserAssistant);

    #[objc::msg_send(advertiserAssistantDidDismissInvitation:)]
    fn assistant_did_dismiss_invitation(&mut self, assistant: &AdvertiserAssistant);
}

define_obj_type!(
    pub AnyDelegate(ns::Id)
);

impl Delegate for AnyDelegate {}

#[link(name = "mc", kind = "static")]
unsafe extern "C" {
    static MC_ADVERTISER_ASSISTANT: &'static objc::Class<AdvertiserAssistant>;
}
