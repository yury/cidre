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
