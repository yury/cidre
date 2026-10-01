use crate::{arc, cg, define_obj_type, define_opts, ns, objc, ui};

define_opts!(
    #[doc(alias = "UIPopoverArrowDirection")]
    pub PopoverArrowDirection(usize)
);

impl PopoverArrowDirection {
    pub const UP: Self = Self(1 << 0);
    pub const DOWN: Self = Self(1 << 1);
    pub const LEFT: Self = Self(1 << 2);
    pub const RIGHT: Self = Self(1 << 3);
    pub const ANY: Self = Self(Self::UP.0 | Self::DOWN.0 | Self::LEFT.0 | Self::RIGHT.0);
    pub const UNKNOWN: Self = Self(usize::MAX);
}

/// What a popover can be anchored to: a view, a bar button item, an action's
/// presentation source.
#[objc::protocol(UIPopoverPresentationControllerSourceItem)]
pub trait PopoverPresentationControllerSrcItem: objc::Obj {}

define_obj_type!(
    pub AnyPopoverPresentationControllerSrcItem(ns::Id)
);

impl PopoverPresentationControllerSrcItem for AnyPopoverPresentationControllerSrcItem {}
impl PopoverPresentationControllerSrcItem for ui::View {}
impl PopoverPresentationControllerSrcItem for ui::BarButtonItem {}

define_obj_type!(
    #[doc(alias = "UIPopoverPresentationController")]
    pub PopoverPresentationController(ui::PresentationController)
);

impl PopoverPresentationController {
    #[objc::msg_send(permittedArrowDirections)]
    pub fn permitted_arrow_directions(&self) -> PopoverArrowDirection;

    /// The directions the arrow may point in. Defaults to `ANY`.
    #[objc::msg_send(setPermittedArrowDirections:)]
    pub fn set_permitted_arrow_directions(&mut self, val: PopoverArrowDirection);

    #[objc::msg_send(sourceView)]
    pub fn src_view(&self) -> Option<arc::R<ui::View>>;

    #[objc::msg_send(setSourceView:)]
    pub fn set_src_view(&mut self, val: Option<&ui::View>);

    /// The rectangle in `src_view`'s coordinates the popover points at.
    #[objc::msg_send(sourceRect)]
    pub fn src_rect(&self) -> cg::Rect;

    #[objc::msg_send(setSourceRect:)]
    pub fn set_src_rect(&mut self, val: cg::Rect);

    #[objc::msg_send(sourceItem)]
    #[objc::available(ios = 16.0)]
    pub fn src_item(&self) -> Option<arc::R<AnyPopoverPresentationControllerSrcItem>>;

    /// The item the popover is anchored to; takes precedence over `src_view`.
    #[objc::msg_send(setSourceItem:)]
    #[objc::available(ios = 16.0)]
    pub fn set_src_item<I: PopoverPresentationControllerSrcItem>(&mut self, val: Option<&I>);

    #[objc::msg_send(arrowDirection)]
    pub fn arrow_direction(&self) -> PopoverArrowDirection;

    #[objc::msg_send(canOverlapSourceViewRect)]
    pub fn can_overlap_src_view_rect(&self) -> bool;

    #[objc::msg_send(setCanOverlapSourceViewRect:)]
    pub fn set_can_overlap_src_view_rect(&mut self, val: bool);

    #[objc::msg_send(backgroundColor)]
    pub fn bg_color(&self) -> Option<arc::R<ui::Color>>;

    #[objc::msg_send(setBackgroundColor:)]
    pub fn set_bg_color(&mut self, val: Option<&ui::Color>);

    /// The sheet the popover becomes in a compact size class; configure its
    /// detents before presenting.
    #[objc::msg_send(adaptiveSheetPresentationController)]
    #[objc::available(ios = 15.0)]
    pub fn adaptive_sheet_presentation_controller(&self)
    -> arc::R<ui::SheetPresentationController>;
}
