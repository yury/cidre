use crate::{arc, cg, define_obj_type, objc, ui};

define_obj_type!(
    #[doc(alias = "UISplitViewController")]
    pub SplitViewController(ui::ViewController),
    sym UISplitViewController
);

impl SplitViewController {
    #[objc::msg_send(setDelegate:)]
    pub fn set_delegate<D: Delegate>(&mut self, val: Option<&D>);

    #[objc::msg_send(setPresentsWithGesture:)]
    pub fn set_presents_with_gesture(&mut self, val: bool);

    #[objc::msg_send(displayModeButtonVisibility)]
    #[objc::available(ios = 14.5)]
    pub fn display_mode_button_visibility(&self) -> DisplayModeButtonVisibility;

    #[objc::msg_send(setDisplayModeButtonVisibility:)]
    #[objc::available(ios = 14.5)]
    pub fn set_display_mode_button_visibility(&mut self, val: DisplayModeButtonVisibility);

    #[objc::init(initWithStyle:)]
    #[objc::available(ios = 14.0)]
    pub fn init_with_style(self, style: Style) -> arc::R<SplitViewController>;

    #[objc::available(ios = 14.0)]
    pub fn with_style(style: Style) -> arc::R<Self> {
        Self::alloc().init_with_style(style)
    }

    #[objc::msg_send(setViewController:forColumn:)]
    #[objc::available(ios = 14.0)]
    pub fn set_vc_for_column(&mut self, vc: Option<&ui::ViewController>, column: Column);

    #[objc::msg_send(viewControllerForColumn:)]
    #[objc::available(ios = 14.0)]
    pub fn vc_for_column(&self, column: Column) -> Option<arc::R<ui::ViewController>>;

    #[objc::msg_send(primaryColumnWidth)]
    pub fn primary_column_width(&self) -> cg::Float;

    #[objc::msg_send(isCollapsed)]
    pub fn is_collapsed(&self) -> bool;

    #[objc::msg_send(isShowingColumn:)]
    #[objc::available(ios = 26.0)]
    pub fn is_showing_column(&self, column: Column) -> bool;

    #[objc::msg_send(setPrimaryBackgroundStyle:)]
    #[objc::available(ios = 13.0)]
    pub fn set_primary_background_style(&mut self, val: BackgroundStyle);

    #[objc::msg_send(setPreferredInspectorColumnWidth:)]
    #[objc::available(ios = 26.0)]
    pub fn set_preferred_inspector_column_width(&mut self, val: cg::Float);

    #[objc::msg_send(setMinimumInspectorColumnWidth:)]
    #[objc::available(ios = 26.0)]
    pub fn set_min_inspector_column_width(&mut self, val: cg::Float);

    #[objc::msg_send(setMaximumInspectorColumnWidth:)]
    #[objc::available(ios = 26.0)]
    pub fn set_max_inspector_column_width(&mut self, val: cg::Float);

    #[objc::msg_send(setPreferredDisplayMode:)]
    pub fn set_preferred_display_mode(&mut self, val: DisplayMode);

    #[objc::msg_send(setPreferredPrimaryColumnWidth:)]
    #[objc::available(ios = 14.0)]
    pub fn set_preferred_primary_column_width(&mut self, val: cg::Float);

    /// The primary column's width as a fraction of the whole, 0 to 1.
    #[objc::msg_send(setPreferredPrimaryColumnWidthFraction:)]
    pub fn set_preferred_primary_column_width_fraction(&mut self, val: cg::Float);

    #[objc::msg_send(setMinimumPrimaryColumnWidth:)]
    pub fn set_min_primary_column_width(&mut self, val: cg::Float);

    #[objc::msg_send(setMinimumSecondaryColumnWidth:)]
    #[objc::available(ios = 26.0, tvos = 26.0, visionos = 26.0)]
    pub fn set_min_secondary_column_width(&mut self, val: cg::Float);

    #[objc::msg_send(setPreferredSupplementaryColumnWidth:)]
    #[objc::available(ios = 14.0)]
    pub fn set_preferred_supplementary_column_width(&mut self, val: cg::Float);

    /// The supplementary column's width as a fraction of the whole, 0 to 1.
    #[objc::msg_send(setPreferredSupplementaryColumnWidthFraction:)]
    #[objc::available(ios = 14.0)]
    pub fn set_preferred_supplementary_column_width_fraction(&mut self, val: cg::Float);

    #[objc::msg_send(setMinimumSupplementaryColumnWidth:)]
    #[objc::available(ios = 14.0)]
    pub fn set_min_supplementary_column_width(&mut self, val: cg::Float);

    #[objc::msg_send(setMaximumSupplementaryColumnWidth:)]
    #[objc::available(ios = 14.0)]
    pub fn set_max_supplementary_column_width(&mut self, val: cg::Float);

    #[objc::msg_send(setMaximumPrimaryColumnWidth:)]
    pub fn set_max_primary_column_width(&mut self, val: cg::Float);

    #[objc::msg_send(showColumn:)]
    #[objc::available(ios = 14.0)]
    pub fn show_column(&mut self, column: Column);

    #[objc::msg_send(hideColumn:)]
    #[objc::available(ios = 14.0)]
    pub fn hide_column(&mut self, column: Column);
}

#[doc(alias = "UISplitViewControllerStyle")]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[repr(isize)]
pub enum Style {
    DoubleColumn = 1,
    TripleColumn = 2,
}

#[doc(alias = "UISplitViewControllerColumn")]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[repr(isize)]
pub enum Column {
    Primary,
    Supplementary,
    Secondary,
    Compact,
    Inspector,
}

#[doc(alias = "UISplitViewControllerBackgroundStyle")]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[repr(isize)]
pub enum BackgroundStyle {
    None,
    Sidebar,
}

#[doc(alias = "UISplitViewControllerDisplayModeButtonVisibility")]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[repr(isize)]
pub enum DisplayModeButtonVisibility {
    Automatic,
    Never,
    Always,
}

#[doc(alias = "UISplitViewControllerDisplayMode")]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[repr(isize)]
pub enum DisplayMode {
    Automatic,
    SecondaryOnly,
    OneBesideSecondary,
    OneOverSecondary,
    TwoBesideSecondary,
    TwoOverSecondary,
    TwoDisplaceSecondary,
}

#[objc::protocol(UISplitViewControllerDelegate)]
pub trait Delegate: objc::Obj {
    #[objc::optional]
    #[objc::msg_send(splitViewController:willShowColumn:)]
    fn will_show_column(&mut self, controller: &mut SplitViewController, column: Column);

    #[objc::optional]
    #[objc::msg_send(splitViewController:willHideColumn:)]
    fn will_hide_column(&mut self, controller: &mut SplitViewController, column: Column);

    /// iOS/tvOS/visionOS 26 callback. System-driven column presentations; programmatic show_column does not send this.
    #[objc::optional]
    #[objc::msg_send(splitViewController:didShowColumn:)]
    fn did_show_column(&mut self, controller: &mut SplitViewController, column: Column);

    /// iOS/tvOS/visionOS 26 callback. System-driven column dismissals; programmatic hide_column does not send this.
    #[objc::optional]
    #[objc::msg_send(splitViewController:didHideColumn:)]
    fn did_hide_column(&mut self, controller: &mut SplitViewController, column: Column);

    #[objc::optional]
    #[objc::msg_send(splitViewControllerInteractivePresentationGestureWillBegin:)]
    fn interactive_presentation_gesture_will_begin(&mut self, controller: &mut SplitViewController);

    #[objc::optional]
    #[objc::msg_send(splitViewControllerInteractivePresentationGestureDidEnd:)]
    fn interactive_presentation_gesture_did_end(&mut self, controller: &mut SplitViewController);

    #[objc::optional]
    #[objc::msg_send(splitViewController:topColumnForCollapsingToProposedTopColumn:)]
    fn top_column_for_collapsing(
        &mut self,
        controller: &mut SplitViewController,
        proposed: Column,
    ) -> Column;

    /// The interface collapsed into one column (compact width).
    #[objc::optional]
    #[objc::msg_send(splitViewControllerDidCollapse:)]
    fn split_view_controller_did_collapse(&mut self, controller: &mut SplitViewController);

    /// The interface expanded into its columns (regular width).
    #[objc::optional]
    #[objc::msg_send(splitViewControllerDidExpand:)]
    fn split_view_controller_did_expand(&mut self, controller: &mut SplitViewController);
}
