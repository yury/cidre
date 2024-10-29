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
}

#[link(name = "ns", kind = "static")]
extern "C" {
    static NS_BUNDLE: &'static objc::Class<Bundle>;
}

#[cfg(test)]
mod tests {
    use crate::ns;

    #[test]
    fn basics() {
        let _bundle = ns::Bundle::main();
    }
}
