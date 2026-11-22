use crate::{api, arc, cg, define_obj_type, ns, objc, ui};

#[doc(alias = "UIEditMenuArrowDirection")]
#[derive(Debug, Copy, Clone, Eq, PartialEq, Default)]
#[repr(isize)]
pub enum EditMenuArrowDirection {
    #[default]
    Automatic = 0,
    Up = 1,
    Down = 2,
    Left = 3,
    Right = 4,
}

define_obj_type!(
    /// Where and which edit menu to show: its id, and the point (in the interaction's view) it
    /// comes from.
    #[doc(alias = "UIEditMenuConfiguration")]
    pub EditMenuCfg(ns::Id)
);

impl EditMenuCfg {
    #[api::available(ios = 16.0)]
    crate::define_cls!(UI_EDIT_MENU_CONFIGURATION);

    #[objc::msg_send(configurationWithIdentifier:sourcePoint:)]
    #[objc::available(ios = 16.0)]
    pub fn with_id_src_point(id: Option<&ns::Id>, src_point: cg::Point) -> arc::R<Self>;

    #[objc::msg_send(identifier)]
    pub fn id(&self) -> Option<arc::R<ns::Id>>;

    #[objc::msg_send(sourcePoint)]
    pub fn src_point(&self) -> cg::Point;

    #[objc::msg_send(preferredArrowDirection)]
    pub fn preferred_arrow_direction(&self) -> EditMenuArrowDirection;

    #[objc::msg_send(setPreferredArrowDirection:)]
    pub fn set_preferred_arrow_direction(&mut self, val: EditMenuArrowDirection);
}

define_obj_type!(
    /// Shows a menu at a point of its view, as text editing does: a bar of actions with touch, a
    /// menu with a pointer. The delegate builds the menu.
    #[doc(alias = "UIEditMenuInteraction")]
    pub EditMenuInteraction(ns::Id)
);

impl ui::Interaction for EditMenuInteraction {}

impl EditMenuInteraction {
    #[api::available(ios = 16.0)]
    crate::define_cls!(UI_EDIT_MENU_INTERACTION);

    #[objc::init(initWithDelegate:)]
    pub fn init_with_delegate<D: EditMenuInteractionDelegate>(
        self,
        delegate: Option<&D>,
    ) -> arc::R<EditMenuInteraction>;

    /// The interaction, `delegate` held weakly.
    #[objc::available(ios = 16.0)]
    pub fn with_delegate<D: EditMenuInteractionDelegate>(delegate: Option<&D>) -> arc::R<Self> {
        Self::alloc().init_with_delegate(delegate)
    }

    #[objc::msg_send(delegate)]
    pub fn delegate(&self) -> Option<arc::R<AnyEditMenuInteractionDelegate>>;

    #[objc::msg_send(presentEditMenuWithConfiguration:)]
    pub fn present_edit_menu(&mut self, cfg: &EditMenuCfg);

    #[objc::msg_send(dismissMenu)]
    pub fn dismiss_menu(&mut self);

    #[objc::msg_send(reloadVisibleMenu)]
    pub fn reload_visible_menu(&mut self);

    #[objc::msg_send(updateVisibleMenuPositionAnimated:)]
    pub fn update_visible_menu_pos(&mut self, animated: bool);

    #[objc::msg_send(locationInView:)]
    pub fn location_in_view(&self, view: Option<&ui::View>) -> cg::Point;
}

#[objc::protocol(UIEditMenuInteractionDelegate)]
pub trait EditMenuInteractionDelegate: objc::Obj {
    /// The menu to show for `cfg`; `suggested` are the system's actions for the responder
    /// chain. `None` shows none.
    #[objc::optional]
    #[objc::msg_send(editMenuInteraction:menuForConfiguration:suggestedActions:)]
    fn edit_menu_interaction_menu_for_cfg(
        &mut self,
        interaction: &mut EditMenuInteraction,
        cfg: &EditMenuCfg,
        suggested: &ns::Array<ui::MenuElement>,
    ) -> Option<arc::R<ui::Menu>>;

    /// The rect (in the interaction's view) the menu keeps clear of.
    #[objc::optional]
    #[objc::msg_send(editMenuInteraction:targetRectForConfiguration:)]
    fn edit_menu_interaction_target_rect_for_cfg(
        &mut self,
        interaction: &mut EditMenuInteraction,
        cfg: &EditMenuCfg,
    ) -> cg::Rect;

    #[objc::optional]
    #[objc::msg_send(editMenuInteraction:willPresentMenuForConfiguration:animator:)]
    fn edit_menu_interaction_will_present_menu_for_cfg(
        &mut self,
        interaction: &mut EditMenuInteraction,
        cfg: &EditMenuCfg,
        animator: &ns::Id,
    );

    #[objc::optional]
    #[objc::msg_send(editMenuInteraction:willDismissMenuForConfiguration:animator:)]
    fn edit_menu_interaction_will_dismiss_menu_for_cfg(
        &mut self,
        interaction: &mut EditMenuInteraction,
        cfg: &EditMenuCfg,
        animator: &ns::Id,
    );
}

define_obj_type!(
    pub AnyEditMenuInteractionDelegate(ns::Id)
);

impl EditMenuInteractionDelegate for AnyEditMenuInteractionDelegate {}

unsafe extern "C" {
    static UI_EDIT_MENU_CONFIGURATION: &'static objc::Class<EditMenuCfg>;
    static UI_EDIT_MENU_INTERACTION: &'static objc::Class<EditMenuInteraction>;
}

#[cfg(test)]
mod tests {
    use crate::{cg, ui};

    #[test]
    fn basics() {
        let interaction =
            ui::EditMenuInteraction::with_delegate::<ui::AnyEditMenuInteractionDelegate>(None);
        let point = cg::Point::new(10.0, 20.0);
        #[cfg(feature = "ios_16_0")]
        let cfg = ui::EditMenuCfg::with_id_src_point(None, point);

        #[cfg(not(feature = "ios_16_0"))]
        let (interaction, cfg) = {
            assert_eq!(interaction.is_some(), ui::EditMenuCfg::cls().is_some());
            let Some(interaction) = interaction else {
                return;
            };
            // Both classes were introduced in iOS 16 and are present on this runtime.
            (interaction, unsafe {
                ui::EditMenuCfg::with_id_src_point(None, point)
            })
        };

        assert_eq!(cfg.src_point(), point);
        assert!(cfg.id().is_some());
        assert!(interaction.delegate().is_none());
    }
}
