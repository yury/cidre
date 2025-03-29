#[derive(Debug, Clone, Copy, Eq, PartialEq)]
#[repr(usize)]
pub enum ButtonType {
    #[doc(alias = "NSButtonTypeMomentaryLight")]
    MomentaryLight,

    #[doc(alias = "NSButtonTypePushOnPushOff")]
    PushOnPushOff,

    #[doc(alias = "NSButtonTypeToggle")]
    Toggle,

    #[doc(alias = "NSButtonTypeSwitch")]
    Switch,

    #[doc(alias = "NSButtonTypeRadio")]
    Radio,

    #[doc(alias = "NSButtonTypeMomentaryChange")]
    MomentaryChange,

    #[doc(alias = "NSButtonTypeOnOff")]
    OnOff,

    #[doc(alias = "NSButtonTypeMomentaryPushIn")]
    MomentaryPushIn,

    #[doc(alias = "NSButtonTypeAccelerator")]
    Accelerator,

    #[doc(alias = "NSButtonTypeMultiLevelAccelerator")]
    MultiLevelAccelerator,
}

// typedef NS_ENUM(NSUInteger, NSBezelStyle) {

//     /// A button with a circular bezel suitable for a small icon or single character.
//     NSBezelStyleCircular           = 7,

//     /// A circular button with a question mark providing the standard Help button appearance.
//     NSBezelStyleHelpButton         = 9,

//     /// A button with squared edges and flexible height.
//     NSBezelStyleSmallSquare        = 10,

//     /// A button style that is appropriate for use in a toolbar item.
//     NSBezelStyleToolbar            = 11,

//     /// A bezel style that is suitable for accessory and scope bars. This style is typically used for buttons that perform an action or for pop-up buttons.
//     NSBezelStyleAccessoryBarAction = 12,

//     /// A bezel style that is suitable for accessory and scope bars. This style is typically used for buttons with togglable state.
//     NSBezelStyleAccessoryBar       = 13,

//     /// A bezeled variant of NSBezelStyleDisclosure.
//     NSBezelStylePushDisclosure     = 14,

//     /// A bezel style that is typically used in table rows to display information about the row, such as a count.
//     NSBezelStyleBadge API_AVAILABLE(macos(10.7)) = 15,

#[doc(alias = "NSBezelStyle")]
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
#[non_exhaustive]
#[repr(usize)]
pub enum BezelStyle {
    Automatic = 0,
    Push = 1,
    FlexiblePush = 2,
    Disclosure = 5,
    Circular = 7,
    HelpButton = 9,
    SmallSquare = 10,
    Toolbar = 11,
    AccessoryBarAction = 12,
    AccessoryBar = 13,
    PushDisclosure = 14,
    Badge = 15,
}
