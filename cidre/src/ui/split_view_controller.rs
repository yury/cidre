use crate::{arc, cg, define_obj_type, objc, ui};

define_obj_type!(
    #[doc(alias = "UISplitViewController")]
    pub SplitViewController(ui::ViewController),
    UI_SPLIT_VIEW_CONTROLLER
);

impl SplitViewController {
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

    #[objc::msg_send(setPreferredDisplayMode:)]
    pub fn set_preferred_display_mode(&mut self, val: DisplayMode);

    #[objc::msg_send(setPreferredPrimaryColumnWidth:)]
    #[objc::available(ios = 14.0)]
    pub fn set_preferred_primary_column_width(&mut self, val: cg::Float);

    #[objc::msg_send(setMinimumPrimaryColumnWidth:)]
    pub fn set_min_primary_column_width(&mut self, val: cg::Float);

    #[objc::msg_send(setMaximumPrimaryColumnWidth:)]
    pub fn set_max_primary_column_width(&mut self, val: cg::Float);

    #[objc::msg_send(showColumn:)]
    #[objc::available(ios = 14.0)]
    pub fn show_column(&mut self, column: Column);

    #[objc::msg_send(hideColumn:)]
    #[objc::available(ios = 14.0)]
    pub fn hide_column(&mut self, column: Column);
}

unsafe extern "C" {
    static UI_SPLIT_VIEW_CONTROLLER: &'static objc::Class<SplitViewController>;
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
