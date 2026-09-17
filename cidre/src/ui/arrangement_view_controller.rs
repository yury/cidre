use crate::{api, arc, define_obj_type, ns, objc, ui};

/// Where a child of an arrangement view controller sits.
#[doc(alias = "UIArrangementViewControllerViewPlacement")]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[repr(isize)]
pub enum ViewPlacement {
    None,
    Primary,
    Secondary,
}

define_obj_type!(
    #[doc(alias = "UIArrangementViewState")]
    pub ArrangementViewState(ns::Id)
);

/// How an arrangement view controller currently shows a placement.
impl ArrangementViewState {
    #[objc::msg_send(zIndex)]
    #[objc::available(ios = 27.1, tvos = 27.1, visionos = 27.1)]
    pub fn z_index(&self) -> isize;

    #[objc::msg_send(splitAxis)]
    #[objc::available(ios = 27.1, tvos = 27.1, visionos = 27.1)]
    pub fn split_axis(&self) -> ui::Axis;

    #[objc::msg_send(isHidden)]
    #[objc::available(ios = 27.1, tvos = 27.1, visionos = 27.1)]
    pub fn is_hidden(&self) -> bool;
}

define_obj_type!(
    /// The base of `ui::SplitArrangement` and `ui::OverlayArrangement`.
    #[doc(alias = "UIArrangement")]
    pub Arrangement(ns::Id)
);

define_obj_type!(
    #[doc(alias = "UIArrangementViewController")]
    pub ArrangementViewController(ui::ViewController)
);

/// A container that lays out a primary and a secondary child according to
/// an arrangement: side by side (`ui::SplitArrangement`) or one over the
/// other (`ui::OverlayArrangement`).
impl ArrangementViewController {
    #[api::available(ios = 27.1, tvos = 27.1, visionos = 27.1)]
    crate::define_cls!(UI_ARRANGEMENT_VIEW_CONTROLLER);

    #[objc::init(init)]
    pub fn init(self) -> arc::R<ArrangementViewController>;

    #[api::available(ios = 27.1, tvos = 27.1, visionos = 27.1)]
    pub fn new() -> arc::R<Self> {
        Self::alloc().init()
    }

    #[objc::msg_send(updateArrangement:animated:)]
    #[objc::available(ios = 27.1, tvos = 27.1, visionos = 27.1)]
    pub fn update_arrangement_animated(&mut self, arrangement: &Arrangement, animated: bool);

    #[objc::msg_send(updateArrangement:)]
    #[objc::available(ios = 27.1, tvos = 27.1, visionos = 27.1)]
    pub fn update_arrangement(&mut self, arrangement: &Arrangement);

    #[objc::msg_send(stateForPlacement:)]
    #[objc::available(ios = 27.1, tvos = 27.1, visionos = 27.1)]
    pub fn state_for_placement(
        &self,
        placement: ViewPlacement,
    ) -> Option<arc::R<ArrangementViewState>>;

    #[objc::msg_send(viewControllerForPlacement:)]
    #[objc::available(ios = 27.1, tvos = 27.1, visionos = 27.1)]
    pub fn vc_for_placement(&self, placement: ViewPlacement) -> Option<arc::R<ui::ViewController>>;

    #[objc::msg_send(placementForViewController:)]
    #[objc::available(ios = 27.1, tvos = 27.1, visionos = 27.1)]
    pub fn placement_for_vc(&self, vc: &ui::ViewController) -> ViewPlacement;

    #[objc::msg_send(setViewController:forPlacement:animated:)]
    #[objc::available(ios = 27.1, tvos = 27.1, visionos = 27.1)]
    pub fn set_vc_for_placement_animated(
        &mut self,
        vc: Option<&ui::ViewController>,
        placement: ViewPlacement,
        animated: bool,
    );

    #[objc::msg_send(setViewController:forPlacement:)]
    #[objc::available(ios = 27.1, tvos = 27.1, visionos = 27.1)]
    pub fn set_vc_for_placement(
        &mut self,
        vc: Option<&ui::ViewController>,
        placement: ViewPlacement,
    );
}

/// UIViewController (UIArrangementViewController)
impl ui::ViewController {
    /// The nearest arrangement view controller this one is a child of.
    #[objc::msg_send(arrangementViewController)]
    #[objc::available(ios = 27.1, tvos = 27.1, visionos = 27.1)]
    pub fn arrangement_vc(&self) -> Option<arc::R<ArrangementViewController>>;
}

unsafe extern "C" {
    static UI_ARRANGEMENT_VIEW_CONTROLLER: &'static objc::Class<ArrangementViewController>;
}
