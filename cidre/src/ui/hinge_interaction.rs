use crate::{api, arc, define_obj_type, ns, objc, ui};

#[cfg(feature = "blocks")]
use crate::blocks;

/// Called with the initial hinge state and on each subsequent update for the interaction.
#[cfg(feature = "blocks")]
pub type HingeInteractionUpdateHandler =
    blocks::EscBlock<fn(interaction: &mut HingeInteraction, update: &HingeInteractionUpdate)>;

define_obj_type!(
    /// An update for a [`HingeInteraction`].
    #[doc(alias = "UIHingeInteractionUpdate")]
    #[doc(alias = "UIHingeInteraction.Update")]
    pub HingeInteractionUpdate(ns::Id)
);

impl ns::Copying for HingeInteractionUpdate {}

impl HingeInteractionUpdate {
    /// The current hinge state for the interaction, or `None` when the interaction
    /// leaves a hierarchy that provides hinge updates.
    #[objc::msg_send(hinge)]
    #[objc::available(ios = 27.1, tvos = 27.1, visionos = 27.1)]
    pub fn hinge(&self) -> Option<arc::R<ui::Hinge>>;
}

define_obj_type!(
    /// An interaction for observing the hinge state associated with the view's hierarchy.
    ///
    /// The handler is called when the hinge state changes, or when the interaction
    /// moves between hierarchies.
    #[doc(alias = "UIHingeInteraction")]
    pub HingeInteraction(ns::Id)
);

impl ui::Interaction for HingeInteraction {}

impl HingeInteraction {
    #[api::available(ios = 27.1, tvos = 27.1, visionos = 27.1)]
    crate::define_cls!(UI_HINGE_INTERACTION);

    #[cfg(feature = "blocks")]
    #[objc::init(initWithUpdateHandler:)]
    pub fn init_with_update_handler(
        self,
        handler: &mut HingeInteractionUpdateHandler,
    ) -> arc::R<HingeInteraction>;

    /// The handler is stored and escapes, so take care to avoid retain cycles.
    #[cfg(feature = "blocks")]
    #[objc::available(ios = 27.1, tvos = 27.1, visionos = 27.1)]
    pub fn with_update_handler(
        handler: impl FnMut(&mut HingeInteraction, &HingeInteractionUpdate) + 'static,
    ) -> arc::R<Self> {
        Self::alloc().init_with_update_handler(&mut HingeInteractionUpdateHandler::new2(handler))
    }

    /// While disabled, the handler is not called for hinge updates, and any updates
    /// that occur are not queued. When re-enabled, the handler is called with
    /// the current hinge state if one is available.
    #[objc::msg_send(isEnabled)]
    #[objc::available(ios = 27.1, tvos = 27.1, visionos = 27.1)]
    pub fn is_enabled(&self) -> bool;

    #[objc::msg_send(setEnabled:)]
    #[objc::available(ios = 27.1, tvos = 27.1, visionos = 27.1)]
    pub fn set_enabled(&mut self, val: bool);
}

unsafe extern "C" {
    static UI_HINGE_INTERACTION: &'static objc::Class<HingeInteraction>;
}
