use crate::{arc, blocks, define_cls, define_obj_type, mc, ns, objc};

#[doc(alias = "MCSessionSendDataMode")]
#[derive(Debug, Eq, PartialEq, Copy, Clone)]
#[repr(isize)]
pub enum SendDataMode {
    /// Guaranteed reliable and in-order delivery.
    ///
    /// The framework should guarantee delivery of each message, enqueueing
    /// and retransmitting data as needed, and ensuring in-order delivery
    ///
    /// Use this message type for application-critical data.
    #[doc(alias = "MCSessionSendDataReliable")]
    Reliable,

    /// Sent immediately without queuing, no guaranteed delivery.
    ///
    /// Messages to peers should be sent immediately without socket-level queueing.
    /// If a message cannot be sent immediately, it should be dropped. The order of
    /// messages is not guaranteed.
    ///
    /// Use this message type for data that ceases to be relevant if delayed,
    /// such as real-time gaming data.
    #[doc(alias = "MCSessionSendDataUnreliable")]
    Unreliable,
}

#[doc(alias = "MCSessionState")]
#[derive(Debug, Eq, PartialEq, Copy, Clone)]
#[repr(isize)]
pub enum State {
    /// Not connected to the session.
    ///
    /// The peer is not (or is no longer) in this session.
    #[doc(alias = "MCSessionStateNotConnected")]
    NotConnected,

    /// Peer is connecting to the session.
    ///
    /// A connection to the peer is currently being established.
    #[doc(alias = "MCSessionStateConnecting")]
    Connecting,

    /// Peer is connected to the session.
    #[doc(alias = "MCSessionStateConnected")]
    Connected,
}

#[doc(alias = "MCEncryptionPreference")]
#[derive(Debug, Eq, PartialEq, Copy, Clone)]
#[repr(isize)]
pub enum EncryptionPreference {
    /// Session prefers encryption but will accept unencrypted connections.
    Optional = 0,

    /// Session requires encryption.
    Required = 1,

    /// Session should not be encrypted.
    None = 2,
}

define_obj_type!(
    /// An [`mc::Session`] object enables and manages communication among all
    /// peers in a Multipeer Connectivity session.
    #[doc(alias = "MCSession")]
    pub Session(ns::Id)
);

impl arc::A<Session> {
    #[objc::msg_send(initWithPeer:)]
    pub unsafe fn init_with_peer_throws(self, peer: &mc::PeerId) -> arc::R<Session>;

    #[objc::msg_send(initWithPeer:securityIdentity:encryptionPreference:)]
    pub unsafe fn init_with_encryption_throws(
        self,
        peer: &mc::PeerId,
        sec_identity: Option<&ns::Array<ns::Id>>,
        encryption_preference: mc::EncryptionPreference,
    ) -> arc::R<Session>;
}

impl Session {
    define_cls!(MC_SESSION);

    /// Create a session with an MCPeerID for the local peer.
    #[inline]
    pub fn with_peer<'ear>(peer: &mc::PeerId) -> ns::ExResult<'ear, arc::R<Self>> {
        ns::try_catch(|| unsafe { Self::alloc().init_with_peer_throws(peer) })
    }

    #[inline]
    pub fn with_encryption<'ear>(
        peer: &mc::PeerId,
        sec_identity: Option<&ns::Array<ns::Id>>,
        encryption_preference: mc::EncryptionPreference,
    ) -> ns::ExResult<'ear, arc::R<Self>> {
        ns::try_catch(|| unsafe {
            Self::alloc().init_with_encryption_throws(peer, sec_identity, encryption_preference)
        })
    }

    #[objc::msg_send(sendData:toPeers:withMode:error:)]
    pub unsafe fn send_data_err<'ear>(
        &mut self,
        data: &ns::Data,
        to_peers: &ns::Array<mc::PeerId>,
        mode: mc::SessionSendDataMode,
        err: *mut Option<&'ear ns::Error>,
    ) -> bool;

    /// Send a data message to a list of destination peers.
    pub fn send_data<'ear>(
        &mut self,
        data: &ns::Data,
        to_peers: &ns::Array<mc::PeerId>,
        mode: mc::SessionSendDataMode,
    ) -> ns::Result<'ear> {
        ns::if_false(|err| unsafe { self.send_data_err(data, to_peers, mode, err) })
    }

    /// Disconnect from the session.
    #[objc::msg_send(disconnect)]
    pub fn disconnect(&mut self);

    #[objc::msg_send(sendResourceAtURL:withName:toPeer:withCompletionHandler:)]
    pub fn send_resource_at_url_ch_block(
        &mut self,
        resource_url: &ns::Url,
        resource_name: &ns::String,
        to_peer: &mc::PeerId,
        completion: &blocks::ErrCh,
    ) -> Option<arc::R<ns::Progress>>;

    pub fn send_resource_at_url_ch(
        &mut self,
        resource_url: &ns::Url,
        resource_name: &ns::String,
        to_peer: &mc::PeerId,
        completion: impl FnMut(Option<&ns::Error>) + 'static,
    ) -> Option<arc::R<ns::Progress>> {
        let mut completion = blocks::ErrCh::new1(completion);
        self.send_resource_at_url_ch_block(resource_url, resource_name, to_peer, &mut completion)
    }

    #[objc::msg_send(startStreamWithName:toPeer:error:)]
    pub unsafe fn start_stream_err<'ear>(
        &mut self,
        stream_name: &ns::String,
        to_peer: &mc::PeerId,
        err: *mut Option<&'ear ns::Error>,
    ) -> Option<arc::R<ns::OutputStream>>;

    pub fn start_stream<'ear>(
        &mut self,
        stream_name: &ns::String,
        to_peer: &mc::PeerId,
    ) -> ns::Result<'ear, arc::R<ns::OutputStream>> {
        ns::if_none(|err| unsafe { self.start_stream_err(stream_name, to_peer, err) })
    }

    #[objc::msg_send(myPeerID)]
    pub fn my_peer_id(&self) -> &mc::PeerId;

    #[objc::msg_send(securityIdentity)]
    pub fn security_identity(&self) -> Option<arc::R<ns::Array<ns::Id>>>;

    #[objc::msg_send(encryptionPreference)]
    pub fn encryption_preference(&self) -> mc::EncryptionPreference;

    #[objc::msg_send(connectedPeers)]
    pub fn connected_peers(&self) -> arc::R<ns::Array<mc::PeerId>>;
}

#[objc::protocol(MCSessionDelegate)]
pub trait Delegate: objc::Obj {
    /// Remote peer changed state.
    #[objc::msg_send(session:peer:didChangeState:)]
    fn session_peer_did_change_state(
        &mut self,
        session: &mc::Session,
        peer: &mc::PeerId,
        state: mc::SessionState,
    );

    /// Received data from remote peer.
    #[objc::msg_send(session:didReceiveData:fromPeer:)]
    fn session_did_receive_data_from_peer(
        &mut self,
        session: &mc::Session,
        data: &ns::Data,
        from_peer: &mc::PeerId,
    );

    /// Received a byte stream from remote peer.
    #[objc::msg_send(session:didReceiveStream:withName:fromPeer:)]
    fn session_did_receive_stream(
        &mut self,
        session: &mc::Session,
        stream: &ns::InputStream,
        stream_name: &ns::String,
        from_peer: &mc::PeerId,
    );

    /// Start receiving a resource from remote peer.
    #[objc::msg_send(session:didStartReceivingResourceWithName:fromPeer:withProgress:)]
    fn session_did_start_receiving_resource(
        &mut self,
        session: &mc::Session,
        resource_name: &ns::String,
        from_peer: &mc::PeerId,
        progress: &ns::Progress,
    );

    /// Finished receiving a resource from remote peer and saved the content
    /// in a temporary location - the app is responsible for moving the file
    /// to a permanent location within its sandbox.
    #[objc::msg_send(session:didFinishReceivingResourceWithName:fromPeer:atURL:withError:)]
    fn session_did_finish_receiving_resource(
        &mut self,
        session: &mc::Session,
        resource_name: &ns::String,
        from_peer: &mc::PeerId,
        at_url: Option<&ns::Url>,
        err: Option<&ns::Error>,
    );

    #[objc::optional]
    #[objc::msg_send(session:didReceiveCertificate:fromPeer:certificateHandler:)]
    fn session_did_receive_cert(
        &mut self,
        session: &mc::Session,
        cert: Option<&ns::Array<ns::Id>>,
        from_peer: &mc::PeerId,
        handler: &mut blocks::Block<fn(bool)>,
    );
}

/// MCSessionCustomDiscovery
impl Session {
    /// Gets the connection data for a remote peer.
    #[objc::msg_send(nearbyConnectionDataForPeer:withCompletionHandler:)]
    pub fn nearby_data_for_peer_ch_block(
        &self,
        peer: &mc::PeerId,
        handler: &mut blocks::ResultCh<ns::Data>,
    );

    pub fn nearby_data_for_peer_ch(
        &self,
        peer: &mc::PeerId,
        handler: impl FnMut(Option<&ns::Data>, Option<&ns::Error>) + 'static,
    ) {
        let mut handler = blocks::ResultCh::new2(handler);
        self.nearby_data_for_peer_ch_block(peer, &mut handler);
    }

    /// Connect a peer to the session once connection data is received.
    #[objc::msg_send(connectPeer:withNearbyConnectionData:)]
    pub fn connect_peer_with_nearby_data(&mut self, peer: &mc::PeerId, data: &ns::Data);

    /// Cancel connection attempt with a peer.    
    #[objc::msg_send(cancelConnectPeer:)]
    pub fn cancel_connect_peer(&mut self, peer: &mc::PeerId);
}

unsafe extern "C" {
    static MC_SESSION: &'static objc::Class<Session>;
}

#[cfg(test)]
mod tests {
    use crate::{mc, ns};

    #[test]
    fn basics() {
        let name = ns::str!(c"Peer1");
        let peer = mc::PeerId::with_display_name(name).unwrap();
        eprintln!("{peer:?}");
        let session = mc::Session::with_peer(&peer).unwrap();

        assert_eq!(
            mc::EncryptionPreference::Required,
            session.encryption_preference()
        );
        assert_eq!(session.my_peer_id(), &peer);
    }
}
