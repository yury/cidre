#[doc(alias = "AudioDeviceID")]
#[derive(Debug, Eq, PartialEq, Copy, Clone)]
#[repr(transparent)]
pub struct AudioObjId(pub u32);

#[doc(alias = "AudioClassID")]
#[derive(Debug, Eq, PartialEq, Copy, Clone)]
#[repr(transparent)]
pub struct AudioObjClassId(pub u32);

#[doc(alias = "AudioObjectPropertySelector")]
#[derive(Debug, Eq, PartialEq, Copy, Clone)]
#[repr(transparent)]
pub struct AudioObjPropSelector(pub u32);

#[doc(alias = "AudioObjectPropertyScope")]
#[derive(Debug, Eq, PartialEq, Copy, Clone)]
#[repr(transparent)]
pub struct AudioObjPropScope(pub u32);

#[doc(alias = "AudioObjectPropertyElement")]
#[derive(Debug, Eq, PartialEq, Copy, Clone)]
#[repr(transparent)]
pub struct AudioObjPropElement(pub u32);

#[doc(alias = "AudioObjectPropertyAddress")]
#[derive(Debug)]
#[repr(C)]
pub struct AudioObjPropAddr {
    pub selector: AudioObjPropSelector,
    pub scope: AudioObjPropScope,
    pub element: AudioObjPropElement,
}

impl AudioObjId {
    /// This is the sentinel value. No object will have an ID whose value is 0.
    #[doc(alias = "kAudioObjectUnknown")]
    pub const UNKNOWN: Self = Self(0);
}

impl AudioObjPropScope {
    /// The AudioObjectPropertyScope for properties that apply to the object as a
    /// whole. All objects have a global scope and for most it is their only scope.
    #[doc(alias = "kAudioObjectPropertyScopeGlobal")]
    pub const GLOBAL: Self = Self(u32::from_be_bytes(*b"glob"));

    /// The AudioObjectPropertyScope for properties that apply to the input side of
    /// an object.
    #[doc(alias = "kAudioObjectPropertyScopeInput")]
    pub const INPUT: Self = Self(u32::from_be_bytes(*b"inpt"));

    /// The AudioObjectPropertyScope for properties that apply to the output side of
    /// an object.
    #[doc(alias = "kAudioObjectPropertyScopeOutput")]
    pub const OUTPUT: Self = Self(u32::from_be_bytes(*b"outp"));

    /// The AudioObjectPropertyScope for properties that apply to the play through
    /// side of an object.
    #[doc(alias = "kAudioObjectPropertyScopePlayThrough")]
    pub const PLAY_THROUGH: Self = Self(u32::from_be_bytes(*b"ptru"));

    #[doc(alias = "kAudioObjectPropertyScopeWildcard")]
    pub const WILDCARD: Self = Self(u32::from_be_bytes(*b"****"));
}

impl AudioObjPropElement {
    /// The AudioObjectPropertyElement value for properties that apply to the main
    /// element or to the entire scope.
    #[doc(alias = "kAudioObjectPropertyElementMaster")] // replaced with main
    #[doc(alias = "kAudioObjectPropertyElementMain")]
    pub const MAIN: Self = Self(0);

    #[doc(alias = "kAudioObjectPropertyElementWildcard")]
    pub const WILDCARD: Self = Self(0xFFFFFFFF);
}

impl AudioObjClassId {
    #[doc(alias = "kAudioObjectClassIDWildcard")]
    pub const WILDCARD: Self = Self(u32::from_be_bytes(*b"****"));

    #[doc(alias = "kAudioObjectClassID")]
    pub const OBJECT: Self = Self(u32::from_be_bytes(*b"aobj"));

    /// The AudioClassId that identifies the AudioPlugIn class.
    #[doc(alias = "kAudioPlugInClassID")]
    pub const PLUG_IN: Self = Self(u32::from_be_bytes(*b"aplg"));

    /// The AudioClassId that identifies the AudioTransportManager class.
    #[doc(alias = "kAudioTransportManagerClassID")]
    pub const TRANSPORT_MANAGER: Self = Self(u32::from_be_bytes(*b"trpm"));

    #[doc(alias = "kAudioBoxClassID")]
    pub const BOX: Self = Self(u32::from_be_bytes(*b"abox"));

    #[doc(alias = "kAudioDeviceClassID")]
    pub const DEVICE: Self = Self(u32::from_be_bytes(*b"adev"));
}

impl AudioObjPropSelector {
    #[doc(alias = "kAudioObjectPropertySelectorWildcard")]
    pub const WILDCARD: Self = Self(u32::from_be_bytes(*b"****"));

    /// An AudioClassId that identifies the class from which the class of the
    /// AudioObject is derived. This value must always be one of the standard
    /// classes.
    #[doc(alias = "kAudioObjectPropertyBaseClass")]
    pub const BASE_CLASS: Self = Self(u32::from_be_bytes(*b"bcls"));

    /// An AudioClassId that identifies the class of the AudioObject.
    #[doc(alias = "kAudioObjectPropertyClass")]
    pub const CLASS: Self = Self(u32::from_be_bytes(*b"clas"));

    /// An AudioObjectId that identifies the the AudioObject that owns the given
    /// AudioObject. Note that all AudioObjects are owned by some other AudioObject.
    /// The only exception is the AudioSystemObject, for which the value of this
    /// property is AudioObject::UNKNOWN.
    #[doc(alias = "kAudioObjectPropertyOwner")]
    pub const OWNER: Self = Self(u32::from_be_bytes(*b"stdv"));

    /// A cf::String that contains the human readable name of the object. The caller
    /// is responsible for releasing the returned cf::Object.
    #[doc(alias = "kAudioObjectPropertyName")]
    pub const NAME: Self = Self(u32::from_be_bytes(*b"lnam"));

    /// A cf::String that contains the human readable model name of the object. The
    /// model name differs from kAudioObjectPropertyName in that two objects of the
    /// same model will have the same value for this property but may have different
    /// values for kAudioObjectPropertyName.
    #[doc(alias = "kAudioObjectPropertyModelName")]
    pub const MODEL_NAME: Self = Self(u32::from_be_bytes(*b"lmod"));

    /// A cf::String that contains the human readable name of the manufacturer of the
    /// hardware the AudioObject is a part of. The caller is responsible for
    /// releasing the returned CFObject.
    #[doc(alias = "kAudioObjectPropertyManufacturer")]
    pub const MANUFACTURER: Self = Self(u32::from_be_bytes(*b"lmak"));

    /// A cf::String that contains a human readable name for the given element in the
    /// given scope. The caller is responsible for releasing the returned CFObject.
    #[doc(alias = "kAudioObjectPropertyElementName")]
    pub const ELEMENT_NAME: Self = Self(u32::from_be_bytes(*b"lchn"));

    /// A cf::String that contains a human readable name for the category of the given
    /// element in the given scope. The caller is responsible for releasing the
    /// returned CFObject.
    #[doc(alias = "kAudioObjectPropertyElementCategoryName")]
    pub const ELEMENT_CATEGORY_NAME: Self = Self(u32::from_be_bytes(*b"lccn"));

    /// A cf::String that contains a human readable name for the number of the given
    /// element in the given scope. The caller is responsible for releasing the
    /// returned CFObject.
    #[doc(alias = "kAudioObjectPropertyElementNumberName")]
    pub const ELEMENT_NUMBER_NAME: Self = Self(u32::from_be_bytes(*b"lcnn"));

    /// An array of AudioObjectIDs that represent all the AudioObjects owned by the
    /// given object. The qualifier is an array of AudioClassIDs. If it is
    /// non-empty, the returned array of AudioObjectIDs will only refer to objects
    /// whose class is in the qualifier array or whose is a subclass of one in the
    /// qualifier array.
    #[doc(alias = "kAudioObjectPropertyOwnedObjects")]
    pub const OWNED_OBJS: Self = Self(u32::from_be_bytes(*b"ownd"));

    /// A u32 where a value of one indicates that the object's hardware is
    /// drawing attention to itself, typically by flashing or lighting up its front
    /// panel display. A value of 0 indicates that this function is turned off. This
    /// makes it easy for a user to associate the physical hardware with its
    /// representation in an application. Typically, this property is only supported
    /// by AudioDevices and AudioBoxes.
    #[doc(alias = "kAudioObjectPropertyIdentify")]
    pub const IDENTITY: Self = Self(u32::from_be_bytes(*b"iden"));

    /// A cf::String that contains the human readable serial number for the object.
    /// This property will typically be implemented by AudioBox and AudioDevice
    /// objects. Note that the serial number is not defined to be unique in the same
    /// way that an AudioBox's or AudioDevice's UID property are defined. This is
    /// purely an informational value. The caller is responsible for releasing the
    /// returned CFObject.
    #[doc(alias = "kAudioObjectPropertySerialNumber")]
    pub const SERIAL_NUMBER: Self = Self(u32::from_be_bytes(*b"snum"));

    /// A cf::String that contains the human readable firmware version for the object.
    /// This property will typically be implemented by AudioBox and AudioDevice
    /// objects. Note that this is purely an informational value. The caller is
    /// responsible for releasing the returned CFObject.
    #[doc(alias = "kAudioObjectPropertyFirmwareVersion")]
    pub const FIRMWARE_VERSION: Self = Self(u32::from_be_bytes(*b"fwvn"));
}

/// AudioPlugIn Properties
impl AudioObjPropSelector {
    #[doc(alias = "kAudioPlugInPropertyBundleID")]
    pub const PLUG_IN_BUNDLE_ID: Self = Self(u32::from_be_bytes(*b"piid"));

    #[doc(alias = "kAudioPlugInPropertyDeviceList")]
    pub const PLUG_IN_DEVICE_LIST: Self = Self(u32::from_be_bytes(*b"dev#"));

    #[doc(alias = "kAudioPlugInPropertyTranslateUIDToDevice")]
    pub const PLUG_IN_TRANSLATE_UID_TO_DEVICE: Self = Self(u32::from_be_bytes(*b"uidd"));

    #[doc(alias = "kAudioPlugInPropertyBoxList")]
    pub const PLUG_IN_BOX_LIST: Self = Self(u32::from_be_bytes(*b"box#"));

    #[doc(alias = "kAudioPlugInPropertyTranslateUIDToBox")]
    pub const PLUG_IN_TRANSLATE_UID_TO_BOX: Self = Self(u32::from_be_bytes(*b"uidb"));

    #[doc(alias = "kAudioPlugInPropertyClockDeviceList")]
    pub const PLUG_IN_CLOCK_DEVICE_LIST: Self = Self(u32::from_be_bytes(*b"clk#"));

    #[doc(alias = "kAudioPlugInPropertyTranslateUIDToClockDevice")]
    pub const PLUG_IN_TRANSLATE_UID_TO_CLOCK_DEVICE: Self = Self(u32::from_be_bytes(*b"uidc"));
}

/// AudioTransportManager Properties
impl AudioObjPropSelector {
    #[doc(alias = "kAudioTransportManagerPropertyEndPointList")]
    pub const TRANSPORT_MANAGER_END_POINT_LIST: Self = Self(u32::from_be_bytes(*b"end#"));

    #[doc(alias = "kAudioTransportManagerPropertyTranslateUIDToEndPoint")]
    pub const TRANSPORT_MANAGER_TRANSLATE_UID_TO_END_POINT: Self =
        Self(u32::from_be_bytes(*b"uide"));

    #[doc(alias = "kAudioTransportManagerPropertyTransportType")]
    pub const TRANSPORT_MANAGER_TRANSPORT_TYPE: Self = Self(u32::from_be_bytes(*b"tran"));
}

/// AudioObjectPropertySelector values provided by the AudioBox class.
impl AudioObjPropSelector {
    #[doc(alias = "kAudioBoxPropertyBoxUID")]
    pub const BOX_UID: Self = Self(u32::from_be_bytes(*b"buid"));

    #[doc(alias = "kAudioBoxPropertyTransportType")]
    pub const BOX_TRANSPORT_TYPE: Self = Self(u32::from_be_bytes(*b"tran"));

    #[doc(alias = "kAudioBoxPropertyHasAudio")]
    pub const BOX_HAS_AUDIO: Self = Self(u32::from_be_bytes(*b"bhau"));

    #[doc(alias = "kAudioBoxPropertyHasVideo")]
    pub const BOX_HAS_VIDEO: Self = Self(u32::from_be_bytes(*b"bhvi"));

    #[doc(alias = "kAudioBoxPropertyHasMIDI")]
    pub const BOX_HAS_MIDI: Self = Self(u32::from_be_bytes(*b"bhmi"));

    #[doc(alias = "kAudioBoxPropertyIsProtected")]
    pub const BOX_IS_PROTECTED: Self = Self(u32::from_be_bytes(*b"bpro"));

    #[doc(alias = "kAudioBoxPropertyAcquired")]
    pub const BOX_ACQUIRED: Self = Self(u32::from_be_bytes(*b"bxon"));

    #[doc(alias = "kAudioBoxPropertyAcquisitionFailed")]
    pub const BOX_ACQUISITION_FAILED: Self = Self(u32::from_be_bytes(*b"bxof"));

    #[doc(alias = "kAudioBoxPropertyDeviceList")]
    pub const BOX_DEVICE_LIST: Self = Self(u32::from_be_bytes(*b"bdv#"));

    #[doc(alias = "kAudioBoxPropertyClockDeviceList")]
    pub const BOX_CLOCK_DEVICE_LIST: Self = Self(u32::from_be_bytes(*b"bcl#"));
}

pub struct AudioDeviceTransportType(pub u32);

impl AudioDeviceTransportType {
    #[doc(alias = "kAudioDeviceTransportTypeUnknown")]
    pub const UNKNOWN: Self = Self(0);

    #[doc(alias = "kAudioDeviceTransportTypeBuiltIn")]
    pub const BUILT_IN: Self = Self(u32::from_be_bytes(*b"bltn"));

    #[doc(alias = "kAudioDeviceTransportTypeAggregate")]
    pub const AGGREGATE: Self = Self(u32::from_be_bytes(*b"grup"));

    #[doc(alias = "kAudioDeviceTransportTypeVirtual")]
    pub const VIRTUAL: Self = Self(u32::from_be_bytes(*b"virt"));

    #[doc(alias = "kAudioDeviceTransportTypePCI")]
    pub const PCI: Self = Self(u32::from_be_bytes(*b"pci "));

    #[doc(alias = "kAudioDeviceTransportTypeUSB")]
    pub const USB: Self = Self(u32::from_be_bytes(*b"usb "));

    #[doc(alias = "kAudioDeviceTransportTypeFireWire")]
    pub const FIRE_WIRE: Self = Self(u32::from_be_bytes(*b"1394"));

    #[doc(alias = "kAudioDeviceTransportTypeBluetooth")]
    pub const BLUETOOTH: Self = Self(u32::from_be_bytes(*b"blue"));

    #[doc(alias = "kAudioDeviceTransportTypeBluetoothLE")]
    pub const BLUETOOTH_LE: Self = Self(u32::from_be_bytes(*b"blea"));

    #[doc(alias = "kAudioDeviceTransportTypeHDMI")]
    pub const HDMI: Self = Self(u32::from_be_bytes(*b"hdmi"));

    #[doc(alias = "kAudioDeviceTransportTypeDisplayPort")]
    pub const DISPLAY_PORT: Self = Self(u32::from_be_bytes(*b"dprt"));

    #[doc(alias = "kAudioDeviceTransportTypeAirPlay")]
    pub const AIR_PLAY: Self = Self(u32::from_be_bytes(*b"airp"));

    #[doc(alias = "kAudioDeviceTransportTypeAVB")]
    pub const AVB: Self = Self(u32::from_be_bytes(*b"eavb"));

    #[doc(alias = "kAudioDeviceTransportTypeThunderbolt")]
    pub const THUNDERBOLT: Self = Self(u32::from_be_bytes(*b"thun"));

    #[doc(alias = "kAudioDeviceTransportTypeContinuityCaptureWired")]
    pub const CONTINUITY_CAPTURE_WIRED: Self = Self(u32::from_be_bytes(*b"ccwd"));

    #[doc(alias = "kAudioDeviceTransportTypeContinuityCaptureWireless")]
    pub const CONTINUITY_CAPTURE_WIRELESS: Self = Self(u32::from_be_bytes(*b"ccwl"));
}
