use crate::{arc, define_obj_type, ns, objc, ui};

#[cfg(feature = "blocks")]
use crate::blocks;

define_obj_type!(
    /// The token of a registration made with `register_for_trait_changes`;
    /// hand it back to `unregister_for_trait_changes` to end it.
    #[doc(alias = "UITraitChangeRegistration")]
    pub TraitChangeRegistration(ns::Id)
);

/// Runs when any of the registered traits changes: the environment whose
/// traits changed, and the collection before the change.
#[cfg(feature = "blocks")]
pub type TraitChangeHandler = blocks::EscBlock<fn(env: &ns::Id, previous: &ui::TraitCollection)>;

/// The trait classes to register for, as `UITraitEnvironment` takes them.
pub mod traits {
    use crate::{ns, objc};

    #[doc(alias = "UITraitHorizontalSizeClass")]
    pub fn horizontal_size_class() -> &'static objc::Class<ns::Id> {
        unsafe { UI_TRAIT_HORIZONTAL_SIZE_CLASS }
    }

    #[doc(alias = "UITraitVerticalSizeClass")]
    pub fn vertical_size_class() -> &'static objc::Class<ns::Id> {
        unsafe { UI_TRAIT_VERTICAL_SIZE_CLASS }
    }

    #[doc(alias = "UITraitUserInterfaceStyle")]
    pub fn ui_style() -> &'static objc::Class<ns::Id> {
        unsafe { UI_TRAIT_USER_INTERFACE_STYLE }
    }

    #[doc(alias = "UITraitLayoutDirection")]
    pub fn layout_direction() -> &'static objc::Class<ns::Id> {
        unsafe { UI_TRAIT_LAYOUT_DIRECTION }
    }

    #[doc(alias = "UITraitDisplayScale")]
    pub fn display_scale() -> &'static objc::Class<ns::Id> {
        unsafe { UI_TRAIT_DISPLAY_SCALE }
    }

    /// iOS 26; `None` before that.
    #[doc(alias = "UITraitTabAccessoryEnvironment")]
    pub fn tab_accessory_env() -> Option<&'static objc::Class<ns::Id>> {
        let cls: *const objc::Class<ns::Id> = unsafe { UI_TRAIT_TAB_ACCESSORY_ENVIRONMENT };
        unsafe { cls.as_ref() }
    }

    unsafe extern "C" {
        static UI_TRAIT_HORIZONTAL_SIZE_CLASS: &'static objc::Class<ns::Id>;
        static UI_TRAIT_VERTICAL_SIZE_CLASS: &'static objc::Class<ns::Id>;
        static UI_TRAIT_USER_INTERFACE_STYLE: &'static objc::Class<ns::Id>;
        static UI_TRAIT_LAYOUT_DIRECTION: &'static objc::Class<ns::Id>;
        static UI_TRAIT_DISPLAY_SCALE: &'static objc::Class<ns::Id>;
        static UI_TRAIT_TAB_ACCESSORY_ENVIRONMENT: *const objc::Class<ns::Id>;
    }
}

/// `UITraitEnvironment`: views and view controllers register for the
/// changes of the traits they care about, instead of overriding
/// `traitCollectionDidChange:`.
macro_rules! trait_env {
    ($t:ty) => {
        impl $t {
            #[cfg(feature = "blocks")]
            #[objc::msg_send(registerForTraitChanges:withHandler:)]
            #[objc::available(ios = 17.0, tvos = 17.0)]
            pub fn register_for_trait_changes_block(
                &mut self,
                traits: &ns::Array<objc::Class<ns::Id>>,
                handler: &mut TraitChangeHandler,
            ) -> arc::R<TraitChangeRegistration>;

            /// Runs `handler` whenever one of `traits` changes on this environment.
            #[cfg(feature = "blocks")]
            #[objc::available(ios = 17.0, tvos = 17.0)]
            pub fn register_for_trait_changes(
                &mut self,
                traits: &[&objc::Class<ns::Id>],
                handler: impl FnMut(&ns::Id, &ui::TraitCollection) + 'static,
            ) -> arc::R<TraitChangeRegistration> {
                let mut handler = TraitChangeHandler::new2(handler);
                let traits: &[&ns::Id] = unsafe { std::mem::transmute(traits) };
                let traits: arc::R<ns::Array<objc::Class<ns::Id>>> =
                    unsafe { std::mem::transmute(ns::Array::<ns::Id>::from_slice(traits)) };
                self.register_for_trait_changes_block(&traits, &mut handler)
            }

            #[objc::msg_send(unregisterForTraitChanges:)]
            #[objc::available(ios = 17.0, tvos = 17.0)]
            pub fn unregister_for_trait_changes(&mut self, registration: &TraitChangeRegistration);
        }
    };
}

trait_env!(ui::View);
trait_env!(ui::ViewController);
