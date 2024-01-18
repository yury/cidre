use crate::{arc, define_cls, define_obj_type, mc, ns, nw::AdvertiseDesc, objc};

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
    ) -> Result<arc::R<Self>, &'ear ns::Exception> {
        ns::try_catch(|| unsafe { Self::alloc().init_service_type_throws(peer, service_type) })
    }

    #[objc::msg_send(start)]
    pub fn start(&mut self);

    #[objc::msg_send(stop)]
    pub fn stop(&mut self);

    #[objc::msg_send(session)]
    pub fn session(&self) -> &mc::Session;

    #[objc::msg_send(discoveryInfo)]
    pub fn discovery_info(&self) -> Option<&ns::Dictionary<ns::String, ns::String>>;

    #[objc::msg_send(serviceType)]
    pub fn service_type(&self) -> &ns::String;

    #[objc::msg_send(setDelegate:)]
    pub fn set_delegate<D: Delegate>(&mut self, val: &D);

    #[objc::msg_send(delegate)]
    pub fn delegate(&mut self) -> &AnyDelegate;
}

#[objc::obj_trait]
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

#[link(name = "mc", kind = "framework")]
extern "C" {
    static MC_ADVERTISER_ASSISTANT: &'static objc::Class<AdvertiserAssistant>;
}
