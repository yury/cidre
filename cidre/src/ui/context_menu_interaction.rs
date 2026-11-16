#[cfg(feature = "blocks")]
use crate::{arc, blocks, ui};
use crate::{define_obj_type, ns, objc};

define_obj_type!(
    #[doc(alias = "UIContextMenuInteraction")]
    pub ContextMenuInteraction(ns::Id)
);

impl ContextMenuInteraction {
    /// Dismisses the currently presented menu, if any.
    #[objc::msg_send(dismissMenu)]
    #[objc::available(ios = 13.0)]
    pub fn dismiss_menu(&mut self);

    /// Replaces the menu on screen, if any, with the one `block` returns for it, in place.
    #[cfg(feature = "blocks")]
    #[objc::msg_send(updateVisibleMenuWithBlock:)]
    #[objc::available(ios = 14.0)]
    pub fn update_visible_menu_block(
        &mut self,
        block: &mut blocks::NoEscBlock<fn(&ui::Menu) -> arc::Rar<ui::Menu>>,
    );

    /// See [`Self::update_visible_menu_block`].
    #[cfg(feature = "blocks")]
    #[objc::available(ios = 14.0)]
    pub fn update_visible_menu(&mut self, mut f: impl FnMut(&ui::Menu) -> arc::R<ui::Menu>) {
        let mut update = |visible: &ui::Menu| -> arc::Rar<ui::Menu> {
            let menu = f(visible);
            crate::return_ar!(menu)
        };
        let mut block = unsafe {
            blocks::NoEscBlock::<fn(&ui::Menu) -> arc::Rar<ui::Menu>>::stack1(&mut update)
        };
        self.update_visible_menu_block(&mut block);
    }
}

#[objc::protocol(UIContextMenuInteractionAnimating)]
pub trait ContextMenuInteractionAnimating: objc::Obj {
    #[cfg(feature = "blocks")]
    #[objc::msg_send(addCompletion:)]
    fn add_completion(&self, completion: &mut blocks::EscBlock<fn()>);
}

define_obj_type!(
    pub AnyContextMenuInteractionAnimating(ns::Id)
);

impl ContextMenuInteractionAnimating for AnyContextMenuInteractionAnimating {}

#[objc::protocol(UIContextMenuInteractionCommitAnimating)]
pub trait ContextMenuInteractionCommitAnimating: objc::Obj {}

define_obj_type!(
    pub AnyContextMenuInteractionCommitAnimating(ns::Id)
);

impl ContextMenuInteractionCommitAnimating for AnyContextMenuInteractionCommitAnimating {}
