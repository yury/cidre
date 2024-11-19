use crate::{arc, blocks, define_cls, define_obj_type, mc, ns, objc};

define_obj_type!(
    #[doc(alias = "MCNearbyServiceAdvertiser")]
    pub NearbyServiceAdvertiser(ns::Id)
);

impl arc::A<NearbyServiceAdvertiser> {
    #[objc::msg_send(initWithPeer:discoveryInfo:serviceType:)]
    pub unsafe fn init_with_peer_throws(
        self,
        my_peer: &mc::PeerId,
        discorvery_info: Option<&ns::Dictionary<ns::String, ns::String>>,
        service_type: &ns::String,
    ) -> arc::R<NearbyServiceAdvertiser>;
}

impl NearbyServiceAdvertiser {
    define_cls!(MC_NEARBY_SERVICE_ADVERTISER);

    pub fn with_peer<'ear>(
        my_peer: &mc::PeerId,
        discorvery_info: Option<&ns::Dictionary<ns::String, ns::String>>,
        service_type: &ns::String,
    ) -> ns::ExResult<'ear, arc::R<Self>> {
        ns::try_catch(|| unsafe {
            Self::alloc().init_with_peer_throws(my_peer, discorvery_info, service_type)
        })
    }

    #[objc::msg_send(startAdvertisingPeer)]
    pub fn start_advertising_peer(&mut self);

    #[objc::msg_send(stopAdvertisingPeer)]
    pub fn stop_advertising_peer(&mut self);

    #[objc::msg_send(delegate)]
    pub fn delegate(&self) -> Option<arc::R<AnyDelegate>>;

    #[objc::msg_send(setDelegate:)]
    pub fn set_delegate<D: Delegate>(&mut self, val: Option<&D>);

    #[objc::msg_send(myPeerID)]
    pub fn my_peer_id(&self) -> arc::R<mc::PeerId>;

    #[objc::msg_send(discoveryInfo)]
    pub fn discovery_info(&self) -> Option<arc::R<ns::Dictionary<ns::String, ns::String>>>;

    #[objc::msg_send(serviceType)]
    pub fn service_type(&self) -> arc::R<ns::String>;
}

#[objc::protocol(MCNearbyServiceAdvertiserDelegate)]
pub trait Delegate: objc::Obj {
    /// Incoming invitation request. Call the invitation_handler block with true
    /// and a valid session to connect the inviting peer to the session.
    #[objc::msg_send(advertiser:didReceiveInvitationFromPeer:withContext:invitationHandler:)]
    fn advertiser_did_receive_invitation_from_peer(
        &mut self,
        advertiser: &NearbyServiceAdvertiser,
        peer: &mc::PeerId,
        context: Option<&ns::Data>,
        invitation_handler: &mut blocks::Block<fn(bool, Option<&mc::Session>)>,
    );

    #[objc::optional]
    #[objc::msg_send(advertiser:didNotStartAdvertisingPeer:)]
    fn advertiser_did_not_start_advertising_peer(
        &mut self,
        advertiser: &NearbyServiceAdvertiser,
        err: &ns::Error,
    );
}

define_obj_type!(
    pub AnyDelegate(ns::Id)
);

impl Delegate for AnyDelegate {}

#[link(name = "mc", kind = "static")]
extern "C" {
    static MC_NEARBY_SERVICE_ADVERTISER: &'static objc::Class<NearbyServiceAdvertiser>;
}
