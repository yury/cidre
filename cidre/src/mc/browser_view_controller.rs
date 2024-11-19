use crate::{arc, define_cls, define_obj_type, mc, ns, objc};

#[cfg(any(
    target_os = "ios",
    all(target_os = "ios", target_abi = "macabi",),
    target_os = "tvos"
))]
use crate::ui;

#[cfg(target_os = "macos")]
define_obj_type!(
     pub BrowserViewController(ns::ViewController)
);

#[cfg(any(
    target_os = "ios",
    all(target_os = "ios", target_abi = "macabi",),
    target_os = "tvos"
))]
define_obj_type!(
     pub BrowserViewController(ui::ViewController)
);

impl mc::NearbyServiceBrowserDelegate for BrowserViewController {}

impl arc::A<BrowserViewController> {
    #[objc::msg_send(initWithServiceType:session:)]
    pub unsafe fn init_with_service_type_throws(
        self,
        service_type: &ns::String,
        session: &mc::Session,
    ) -> arc::R<BrowserViewController>;

    #[objc::msg_send(initWithBrowser:session:)]
    pub unsafe fn init_with_browser_throws(
        self,
        browser: &mc::NearbyServiceBrowser,
        session: &mc::Session,
    ) -> arc::R<BrowserViewController>;
}

impl BrowserViewController {
    define_cls!(MC_BROWSER_VIEW_CONTROLLER);

    pub fn with_service_type<'ear>(
        service_type: &ns::String,
        session: &mc::Session,
    ) -> ns::ExResult<'ear, arc::R<Self>> {
        ns::try_catch(|| unsafe {
            Self::alloc().init_with_service_type_throws(service_type, session)
        })
    }

    pub fn with_browser<'ear>(
        browser: &mc::NearbyServiceBrowser,
        session: &mc::Session,
    ) -> ns::ExResult<'ear, arc::R<Self>> {
        ns::try_catch(|| unsafe { Self::alloc().init_with_browser_throws(browser, session) })
    }

    #[objc::msg_send(browser)]
    pub fn browser(&self) -> Option<arc::R<mc::NearbyServiceBrowser>>;

    #[objc::msg_send(session)]
    pub fn session(&self) -> arc::R<mc::Session>;

    #[objc::msg_send(delegate)]
    pub fn delegate(&self) -> Option<arc::R<AnyDelegate>>;

    #[objc::msg_send(setDelegate:)]
    pub fn set_delegate<D: Delegate>(&mut self, val: Option<&D>);

    #[objc::msg_send(minimumNumberOfPeers)]
    pub fn min_number_of_peers(&self) -> usize;

    #[objc::msg_send(setMinimumNumberOfPeers:)]
    pub fn set_min_number_of_peers(&mut self, val: usize);

    #[objc::msg_send(maximumNumberOfPeers)]
    pub fn max_number_of_peers(&self) -> usize;

    #[objc::msg_send(setMaximumNumberOfPeers:)]
    pub fn set_max_number_of_peers(&mut self, val: usize);
}

#[objc::protocol(MCBrowserViewControllerDelegate)]
pub trait Delegate: objc::Obj {
    /// Notifies the delegate, when the user taps the done button.
    #[objc::msg_send(browserViewControllerDidFinish:)]
    fn browser_vc_did_finish(&mut self, vc: &BrowserViewController);

    /// Notifies delegate that the user taps the cancel button.
    #[objc::msg_send(browserViewControllerWasCancelled:)]
    fn browser_vc_was_cancelled(&mut self, vc: &BrowserViewController);

    #[objc::optional]
    #[objc::msg_send(browserViewController:shouldPresentNearbyPeer:withDiscoveryInfo:)]
    fn browser_vc_should_present_nearby_peer(
        &mut self,
        vc: &BrowserViewController,
        peer: &mc::PeerId,
        discovery_info: Option<&ns::Dictionary<ns::String, ns::String>>,
    ) -> bool;
}

define_obj_type!(
    pub AnyDelegate(ns::Id)
);

impl Delegate for AnyDelegate {}

#[link(name = "mc", kind = "static")]
extern "C" {
    static MC_BROWSER_VIEW_CONTROLLER: &'static objc::Class<BrowserViewController>;
}

#[cfg(test)]
mod tests {
    use crate::{mc, ns};

    #[test]
    fn basics() {
        let peer_name = ns::str!(c"Peer1");
        let peer = mc::PeerId::with_display_name(peer_name).unwrap();
        let session = mc::Session::with_peer(&peer).unwrap();
        let service_type = ns::str!(c"cidre-test");
        let mut bvc = mc::BrowserViewController::with_service_type(service_type, &session).unwrap();
        assert_eq!(bvc.min_number_of_peers(), 2);
        assert_eq!(bvc.max_number_of_peers(), 8);
        bvc.set_max_number_of_peers(10);
        assert_eq!(bvc.max_number_of_peers(), 8);
        assert!(bvc.delegate().is_none());
    }
}
