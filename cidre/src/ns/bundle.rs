use crate::{arc, define_cls, define_obj_type, ns, objc};

define_obj_type!(
    #[doc(alias = "NSBundle")]
    pub Bundle(ns::Id)
);

impl arc::A<Bundle> {
    #[objc::msg_send(initWithPath:)]
    pub fn init_with_path(self, path: &ns::String) -> Option<arc::R<Bundle>>;
}

impl Bundle {
    define_cls!(NS_BUNDLE);

    #[objc::msg_send(mainBundle)]
    pub fn main() -> arc::R<Self>;

    pub fn with_path(path: &ns::String) -> Option<arc::R<Self>> {
        Self::alloc().init_with_path(path)
    }

    #[objc::msg_send(allBundles)]
    pub fn all_bundles() -> arc::R<ns::Array<Self>>;

    #[objc::msg_send(allFrameworks)]
    pub fn all_frameworks() -> arc::R<ns::Array<Self>>;

    #[objc::msg_send(isLoaded)]
    pub fn is_loaded(&self) -> bool;

    #[objc::msg_send(load)]
    pub fn load(&self) -> bool;

    #[objc::msg_send(unload)]
    pub fn unload(&self) -> bool;

    #[objc::msg_send(bundleURL)]
    pub fn bundle_url(&self) -> arc::R<ns::Url>;

    #[objc::msg_send(bundlePath)]
    pub fn bundle_path(&self) -> arc::R<ns::String>;

    #[objc::msg_send(bundleIdentifier)]
    pub fn bundle_id(&self) -> Option<arc::R<ns::String>>;
}

#[link(name = "ns", kind = "static")]
unsafe extern "C" {
    static NS_BUNDLE: &'static objc::Class<Bundle>;
}

#[cfg(test)]
mod tests {
    use crate::ns;

    #[test]
    fn basics() {
        let bundle = ns::Bundle::main();

        assert!(bundle.bundle_id().is_none());
        assert!(!bundle.bundle_path().is_empty());

        let bundles = ns::Bundle::all_frameworks();
        assert!(!bundles.is_empty());

        let bundles = ns::Bundle::all_bundles();
        assert!(!bundles.is_empty());
    }
}
