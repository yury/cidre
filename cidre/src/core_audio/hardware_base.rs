use crate::four_cc_to_str;

#[doc(alias = "AudioDeviceID")]
#[doc(alias = "AudioObjectID")]
#[derive(Debug, Eq, PartialEq, Copy, Clone)]
#[repr(transparent)]
pub struct Obj(pub u32);

#[doc(alias = "AudioClassID")]
#[derive(Eq, PartialEq, Copy, Clone)]
#[repr(transparent)]
pub struct Class(pub u32);

impl std::fmt::Debug for Class {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut fcc = self.0.to_be_bytes();
        f.debug_struct("AudioClass")
            .field("raw", &self.0)
            .field("fcc", &four_cc_to_str(&mut fcc))
            .finish()
    }
}

#[doc(alias = "AudioObjectPropertySelector")]
#[derive(Eq, PartialEq, Copy, Clone)]
#[repr(transparent)]
pub struct PropSelector(pub u32);

impl PropSelector {
    #[inline]
    pub const fn global_addr(self) -> PropAddr {
        PropAddr {
            selector: self,
            scope: PropScope::GLOBAL,
            element: PropElement::MAIN,
        }
    }

    #[inline]
    pub const fn addr(self, scope: PropScope, element: PropElement) -> PropAddr {
        PropAddr {
            selector: self,
            scope,
            element,
        }
    }
}

impl std::fmt::Debug for PropSelector {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut fcc = self.0.to_be_bytes();
        f.debug_struct("AudioObjPropSelector")
            .field("raw", &self.0)
            .field("fcc", &four_cc_to_str(&mut fcc))
            .finish()
    }
}

#[doc(alias = "AudioObjectPropertyScope")]
#[derive(Eq, PartialEq, Copy, Clone)]
#[repr(transparent)]
pub struct PropScope(pub u32);

impl std::fmt::Debug for PropScope {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut fcc = self.0.to_be_bytes();
        f.debug_struct("AudioObjPropScope")
            .field("raw", &self.0)
            .field("fcc", &four_cc_to_str(&mut fcc))
            .finish()
    }
}

#[doc(alias = "AudioObjectPropertyElement")]
#[derive(Debug, Eq, PartialEq, Copy, Clone)]
#[repr(transparent)]
pub struct PropElement(pub u32);

#[doc(alias = "AudioObjectPropertyAddress")]
#[derive(Debug)]
#[repr(C)]
pub struct PropAddr {
    pub selector: PropSelector,
    pub scope: PropScope,
    pub element: PropElement,
}

impl Default for PropAddr {
    fn default() -> Self {
        Self {
            selector: PropSelector::WILDCARD,
            scope: Default::default(),
            element: Default::default(),
        }
    }
}

impl Obj {
    /// This is the sentinel value. No object will have an ID whose value is 0.
    #[doc(alias = "kAudioObjectUnknown")]
    pub const UNKNOWN: Self = Self(0);
}

impl Default for Obj {
    fn default() -> Self {
        Self::UNKNOWN
    }
}

impl PropScope {
    /// The AudioObjectPropertyScope for properties that apply to the object as a
    /// whole. All objects have a global scope and for most it is their only scope.
    #[doc(alias = "kAudioObjectPropertyScopeGlobal")]
    pub const GLOBAL: Self = Self(u32::from_be_bytes(*b"glob"));

    /// The AudioObjectPropertyScope for properties that apply to the input side of
    /// an object.
    #[doc(alias = "kAudioDevicePropertyScopeInput")]
    #[doc(alias = "kAudioObjectPropertyScopeInput")]
    pub const INPUT: Self = Self(u32::from_be_bytes(*b"inpt"));

    /// The AudioObjectPropertyScope for properties that apply to the output side of
    /// an object.
    #[doc(alias = "kAudioDevicePropertyScopeOutput")]
    #[doc(alias = "kAudioObjectPropertyScopeOutput")]
    pub const OUTPUT: Self = Self(u32::from_be_bytes(*b"outp"));

    /// The AudioObjectPropertyScope for properties that apply to the play through
    /// side of an object.
    #[doc(alias = "kAudioDevicePropertyScopePlayThrough")]
    #[doc(alias = "kAudioObjectPropertyScopePlayThrough")]
    pub const PLAY_THROUGH: Self = Self(u32::from_be_bytes(*b"ptru"));

    #[doc(alias = "kAudioObjectPropertyScopeWildcard")]
    pub const WILDCARD: Self = Self(u32::from_be_bytes(*b"****"));
}

impl Default for PropScope {
    fn default() -> Self {
        Self::GLOBAL
    }
}

impl PropElement {
    /// The AudioObjectPropertyElement value for properties that apply to the main
    /// element or to the entire scope.
    #[doc(alias = "kAudioObjectPropertyElementMaster")] // replaced with main
    #[doc(alias = "kAudioObjectPropertyElementMain")]
    pub const MAIN: Self = Self(0);

    #[doc(alias = "kAudioObjectPropertyElementWildcard")]
    pub const WILDCARD: Self = Self(0xFFFFFFFF);
}

impl Default for PropElement {
    fn default() -> Self {
        Self::MAIN
    }
}

impl Class {
    #[doc(alias = "kAudioObjectClassIDWildcard")]
    pub const WILDCARD: Self = Self(u32::from_be_bytes(*b"****"));

    #[doc(alias = "kAudioObjectClassID")]
    pub const OBJECT: Self = Self(u32::from_be_bytes(*b"aobj"));

    #[doc(alias = "kAudioSystemObjectClassID")]
    pub const SYSTEM: Self = Self(u32::from_be_bytes(*b"asys"));

    /// The AudioClassId that identifies the AudioPlugIn class.
    #[doc(alias = "kAudioPlugInClassID")]
    pub const PLUG_IN: Self = Self(u32::from_be_bytes(*b"aplg"));

    /// The AudioClassId that identifies the AudioTransportManager class.
    #[doc(alias = "kAudioTransportManagerClassID")]
    pub const TRANSPORT_MANAGER: Self = Self(u32::from_be_bytes(*b"trpm"));

    /// The AudioClassId that identifies the AudioBox class.
    #[doc(alias = "kAudioBoxClassID")]
    pub const BOX: Self = Self(u32::from_be_bytes(*b"abox"));

    /// The AudioClassId that identifies the AudioDevice class.
    #[doc(alias = "kAudioDeviceClassID")]
    pub const DEVICE: Self = Self(u32::from_be_bytes(*b"adev"));

    /// The AudioClassId that identifies the AudioSubDevice class.
    #[doc(alias = "kAudioSubDeviceClassID")]
    pub const SUB_DEVICE: Self = Self(u32::from_be_bytes(*b"asub"));

    /// The AudioClassId that identifies the Tap class.
    ///
    /// The Tap class contains a list of input streams that originate from the output
    /// stream(s) of one or more processes.
    #[doc(alias = "kAudioTapClassID")]
    pub const TAP: Self = Self(u32::from_be_bytes(*b"tcls"));

    /// The AudioClassID that identifies the AudioSubTap class.
    #[doc(alias = "kAudioSubTapClassID")]
    pub const SUB_TAP: Self = Self(u32::from_be_bytes(*b"stap"));

    #[doc(alias = "kAudioEndPointClassID")]
    pub const END_POINT: Self = Self(u32::from_be_bytes(*b"endp"));

    /// The AudioClassId that identifies the AudioStream class.
    #[doc(alias = "kAudioStreamClassID")]
    pub const STREAM: Self = Self(u32::from_be_bytes(*b"astr"));

    /// The AudioClassId that identifies the AudioControl class.
    #[doc(alias = "kAudioControlClassID")]
    pub const CONTROL: Self = Self(u32::from_be_bytes(*b"actl"));

    /// The AudioClassId that identifies the AudioSliderControl class.
    #[doc(alias = "kAudioSliderControlClassID")]
    pub const SLIDER_CONTROL: Self = Self(u32::from_be_bytes(*b"sldr"));

    /// The AudioClassId that identifies the LevelControl class.
    #[doc(alias = "kAudioLevelControlClassID")]
    pub const LEVEL_CONTROL: Self = Self(u32::from_be_bytes(*b"levl"));

    /// A subclass of the LevelControl class that implements a general
    /// gain/attenuation stage.
    #[doc(alias = "kAudioVolumeControlClassID")]
    pub const VOLUME_CONTROL: Self = Self(u32::from_be_bytes(*b"vlme"));

    /// A subclass of the LevelControl class for an LFE channel that results from
    /// bass management. Note that LFE channels that are represented as normal audio
    /// channels must use kAudioVolumeControlClassID to manipulate the level.
    #[doc(alias = "kAudioLFEVolumeControlClassID")]
    pub const LFE_VOLUME_CONTROL: Self = Self(u32::from_be_bytes(*b"subv"));

    /// The AudioClassID that identifies the BooleanControl class.
    #[doc(alias = "kAudioBooleanControlClassID")]
    pub const BOOLEAN_CONTROL: Self = Self(u32::from_be_bytes(*b"togl"));

    /// A subclass of the AudioBooleanControl class where a true value means that
    /// mute is enabled making that element inaudible.
    #[doc(alias = "kAudioMuteControlClassID")]
    pub const MUTE_CONTROL: Self = Self(u32::from_be_bytes(*b"mute"));

    /// A subclass of the AudioBooleanControl class where a true value means that
    /// solo is enabled making just that element audible and the other elements
    /// inaudible.
    #[doc(alias = "kAudioSoloControlClassID")]
    pub const SOLO_CONTROL: Self = Self(u32::from_be_bytes(*b"solo"));

    /// A subclass of the AudioBooleanControl class where a true value means
    /// something is plugged into that element.
    #[doc(alias = "kAudioJackControlClassID")]
    pub const JACK_CONTROL: Self = Self(u32::from_be_bytes(*b"jack"));

    /// A subclass of the AudioBooleanControl class where true means that mute is
    /// enabled making that LFE element inaudible. This control is for LFE channels
    /// that result from bass management. Note that LFE channels that are
    /// represented as normal audio channels must use an AudioMuteControl.
    #[doc(alias = "kAudioLFEMuteControlClassID")]
    pub const LFE_MUTE_CONTROL: Self = Self(u32::from_be_bytes(*b"subm"));

    /// A subclass of the AudioBooleanControl class where true means that the
    /// element's hardware has phantom power enabled.
    #[doc(alias = "kAudioPhantomPowerControlClassID")]
    pub const PHANTOM_POWER_CONTROL: Self = Self(u32::from_be_bytes(*b"phan"));

    /// A subclass of the AudioBooleanControl class where true means that the phase
    /// of the signal on the given element is being inverted by 180 degrees.
    #[doc(alias = "kAudioPhaseInvertControlClassID")]
    pub const PHASE_INVERT_CONTROL: Self = Self(u32::from_be_bytes(*b"phsi"));

    /// A subclass of the AudioBooleanControl class where true means that the signal
    /// for the element has exceeded the sample range. Once a clip light is turned
    /// on, it is to stay on until either the value of the control is set to false
    /// or the current IO session stops and a new IO session starts.
    #[doc(alias = "kAudioClipLightControlClassID")]
    pub const CLIP_LIGHT_CONTROL: Self = Self(u32::from_be_bytes(*b"clip"));

    /// An AudioBooleanControl where true means that the talkback channel is
    /// enabled. This control is for talkback channels that are handled outside of
    /// the regular IO channels. If the talkback channel is among the normal IO
    /// channels, it will use AudioMuteControl.
    #[doc(alias = "kAudioTalkbackControlClassID")]
    pub const TALKBACK_CONTROL: Self = Self(u32::from_be_bytes(*b"talb"));

    /// An AudioBooleanControl where true means that the listenback channel is
    /// audible. This control is for listenback channels that are handled outside of
    /// the regular IO channels. If the listenback channel is among the normal IO
    /// channels, it will use AudioMuteControl.
    #[doc(alias = "kAudioListenbackControlClassID")]
    pub const LISTENBACK_CONTROL: Self = Self(u32::from_be_bytes(*b"lsnb"));

    /// The AudioClassID that identifies the AudioSelectorControl class.
    #[doc(alias = "kAudioSelectorControlClassID")]
    pub const SELECTOR_CONTROL: Self = Self(u32::from_be_bytes(*b"slct"));

    /// A subclass of the AudioSelectorControl class that identifies where the data
    /// for the element is coming from.
    #[doc(alias = "kAudioDataSourceControlClassID")]
    pub const DATA_SRC_CONTROL: Self = Self(u32::from_be_bytes(*b"dsrc"));

    /// A subclass of the AudioSelectorControl class that identifies where the data
    /// for the element is going.
    #[doc(alias = "kAudioDataDestinationControlClassID")]
    pub const DATA_DST_CONTROL: Self = Self(u32::from_be_bytes(*b"dest"));

    /// A subclass of the AudioSelectorControl class that identifies where the
    /// timing info for the object is coming from.
    #[doc(alias = "kAudioClockSourceControlClassID")]
    pub const CLOCK_SRC_CONTROL: Self = Self(u32::from_be_bytes(*b"clck"));

    /// A subclass of the AudioSelectorControl class that identifies the nominal
    /// line level for the element. Note that this is not a gain stage but rather
    /// indicating the voltage standard (if any) used for the element, such as
    /// +4dBu, -10dBV, instrument, etc.
    #[doc(alias = "kAudioLineLevelControlClassID")]
    pub const LINE_LEVEL_CONTROL: Self = Self(u32::from_be_bytes(*b"nlvl"));

    /// A subclass of the AudioSelectorControl class that indicates the setting for
    /// the high pass filter on the given element.
    #[doc(alias = "kAudioHighPassFilterControlClassID")]
    pub const HIGH_PASS_FILTER_CONTROL: Self = Self(u32::from_be_bytes(*b"hipf"));

    /// The AudioClassId that identifies the StereoPanControl class.
    #[doc(alias = "kAudioStereoPanControlClassID")]
    pub const STEREO_PAN_CONTROL: Self = Self(u32::from_be_bytes(*b"span"));

    /// The AudioClassId that identifies the AudioAggregateDevice class.
    #[doc(alias = "kAudioAggregateDeviceClassID")]
    pub const AGGREGATE_DEVICE: Self = Self(u32::from_be_bytes(*b"aagg"));
}

impl PropSelector {
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
    pub const IDENTIFY: Self = Self(u32::from_be_bytes(*b"iden"));

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
impl PropSelector {
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
impl PropSelector {
    #[doc(alias = "kAudioTransportManagerPropertyEndPointList")]
    pub const TRANSPORT_MANAGER_END_POINT_LIST: Self = Self(u32::from_be_bytes(*b"end#"));

    #[doc(alias = "kAudioTransportManagerPropertyTranslateUIDToEndPoint")]
    pub const TRANSPORT_MANAGER_TRANSLATE_UID_TO_END_POINT: Self =
        Self(u32::from_be_bytes(*b"uide"));

    #[doc(alias = "kAudioTransportManagerPropertyTransportType")]
    pub const TRANSPORT_MANAGER_TRANSPORT_TYPE: Self = Self(u32::from_be_bytes(*b"tran"));
}

/// AudioObjectPropertySelector values provided by the AudioBox class.
impl PropSelector {
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

pub struct DeviceTransportType(pub u32);

impl DeviceTransportType {
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

/// Values provided by the AudioDevice class
impl PropSelector {
    /// A cf::String that contains the bundle ID for an application that provides a
    /// GUI for configuring the AudioDevice.
    ///
    /// By default, the value of this property
    /// is the bundle ID for Audio MIDI Setup. The caller is responsible for
    /// releasing the returned cf::Obj.
    #[doc(alias = "kAudioDevicePropertyConfigurationApplication")]
    pub const DEVICE_CONFIGURATION_APP: Self = Self(u32::from_be_bytes(*b"capp"));

    /// A cf::String that contains a persistent identifier for the AudioDevice. An
    /// AudioDevice's UID is persistent across boots. The content of the UID string
    /// is a black box and may contain information that is unique to a particular
    /// instance of an AudioDevice's hardware or unique to the CPU. Therefore they
    /// are not suitable for passing between CPUs or for identifying similar models
    /// of hardware. The caller is responsible for releasing the returned cf::Obj.
    #[doc(alias = "kAudioDevicePropertyDeviceUID")]
    pub const DEVICE_UID: Self = Self(u32::from_be_bytes(*b"uid "));

    /// A cf::String that contains a persistent identifier for the model of an
    /// AudioDevice. The identifier is unique such that the identifier from two
    /// AudioDevices are equal if and only if the two AudioDevices are the exact
    /// same model from the same manufacturer. Further, the identifier has to be the
    /// same no matter on what machine the AudioDevice appears. The caller is
    /// responsible for releasing the returned cf::Obj.
    #[doc(alias = "kAudioDevicePropertyModelUID")]
    pub const DEVICE_MODEL_UID: Self = Self(u32::from_be_bytes(*b"muid"));

    /// A u32 whose value indicates how the AudioDevice is connected to the CPU.
    /// Constants for some of the values for this property can be found in the enum
    /// in the AudioDevice Constants section of this file.
    #[doc(alias = "kAudioDevicePropertyTransportType")]
    pub const DEVICE_TRANSPORT_TYPE: Self = Self(u32::from_be_bytes(*b"tran"));

    ///  An array of AudioDeviceIDs for devices related to the AudioDevice. For
    /// IOAudio-based devices, AudioDevices are related if they share the same
    /// IOAudioDevice object.
    #[doc(alias = "kAudioDevicePropertyRelatedDevices")]
    pub const DEVICE_RELATED_DEVICES: Self = Self(u32::from_be_bytes(*b"akin"));

    /// A u32 whose value indicates the clock domain to which this AudioDevice
    /// belongs. AudioDevices that have the same value for this property are able to
    /// be synchronized in hardware. However, a value of 0 indicates that the clock
    /// domain for the device is unspecified and should be assumed to be separate
    /// from every other device's clock domain, even if they have the value of 0 as
    /// their clock domain as well.
    #[doc(alias = "kAudioDevicePropertyClockDomain")]
    pub const DEVICE_CLOCK_DOMAIN: Self = Self(u32::from_be_bytes(*b"clkd"));

    /// A u32 where a value of 1 means the device is ready and available and 0
    /// means the device is unusable and will most likely go away shortly.
    #[doc(alias = "kAudioDevicePropertyDeviceIsAlive")]
    pub const DEVICE_IS_ALIVE: Self = Self(u32::from_be_bytes(*b"livn"));

    ///  A u32 where a value of 0 means the AudioDevice is not performing IO and
    /// a value of 1 means that it is. Note that the device can be running even if
    /// there are no active IOProcs such as by calling AudioDeviceStart() and
    /// passing a NULL IOProc. Note that the notification for this property is
    /// usually sent from the AudioDevice's IO thread.
    #[doc(alias = "kAudioDevicePropertyDeviceIsRunning")]
    pub const DEVICE_IS_RUNNING: Self = Self(u32::from_be_bytes(*b"goin"));

    /// A u32 where 1 means that the AudioDevice is a possible selection for
    /// kAudioHardwarePropertyDefaultInputDevice or
    /// kAudioHardwarePropertyDefaultOutputDevice depending on the scope.
    #[doc(alias = "kAudioDevicePropertyDeviceCanBeDefaultDevice")]
    pub const DEVICE_CAN_BE_DEFAULT_DEVICE: Self = Self(u32::from_be_bytes(*b"dflt"));

    /// A u32 where 1 means that the AudioDevice is a possible selection for
    /// kAudioHardwarePropertyDefaultSystemOutputDevice.
    #[doc(alias = "kAudioDevicePropertyDeviceCanBeDefaultSystemDevice")]
    pub const DEVICE_CAN_BE_DEFAULT_SYS_DEVICE: Self = Self(u32::from_be_bytes(*b"sflt"));

    /// A u32 containing the number of frames of latency in the AudioDevice. Note
    /// that input and output latency may differ. Further, the AudioDevice's
    /// AudioStreams may have additional latency so they should be queried as well.
    /// If both the device and the stream say they have latency, then the total
    /// latency for the stream is the device latency summed with the stream latency.
    #[doc(alias = "kAudioDevicePropertyLatency")]
    pub const DEVICE_LATENCY: Self = Self(u32::from_be_bytes(*b"ltnc"));

    /// An array of AudioStreamIDs that represent the AudioStreams of the
    /// AudioDevice. Note that if a notification is received for this property, any
    /// cached AudioStreamIDs for the device become invalid and need to be
    /// re-fetched.
    #[doc(alias = "kAudioDevicePropertyStreams")]
    pub const DEVICE_STREAMS: Self = Self(u32::from_be_bytes(*b"stm#"));

    /// An array of AudioObjectIDs that represent the AudioControls of the
    /// AudioDevice. Note that if a notification is received for this property, any
    /// cached AudioObjectIDs for the device become invalid and need to be
    /// re-fetched.
    #[doc(alias = "kAudioObjectPropertyControlList")]
    pub const CTRL_LIST: Self = Self(u32::from_be_bytes(*b"ctrl"));

    /// A u32 whose value indicates the number for frames in ahead (for output)
    /// or behind (for input the current hardware position that is safe to do IO.
    #[doc(alias = "kAudioDevicePropertySafetyOffset")]
    pub const DEVICE_SAFETY_OFFSET: Self = Self(u32::from_be_bytes(*b"saft"));

    /// A f64 that indicates the current nominal sample rate of the AudioDevice.
    #[doc(alias = "kAudioDevicePropertyNominalSampleRate")]
    pub const DEVICE_NOMINAL_SAMPLE_RATE: Self = Self(u32::from_be_bytes(*b"nsrt"));

    /// An array of AudioValueRange structs that indicates the valid ranges for the
    /// nominal sample rate of the AudioDevice.
    #[doc(alias = "kAudioDevicePropertyAvailableNominalSampleRates")]
    pub const DEVICE_AVAILABLE_SAMPLE_RATES: Self = Self(u32::from_be_bytes(*b"nsr#"));

    /// A cf::Url that indicates an image file that can be used to represent the
    /// device visually. The caller is responsible for releasing the returned
    /// cf::Obj.
    #[doc(alias = "kAudioDevicePropertyIcon")]
    pub const DEVICE_ICON: Self = Self(u32::from_be_bytes(*b"icon"));

    /// A u32 where a non-zero value indicates that the device is not included
    /// in the normal list of devices provided by kAudioHardwarePropertyDevices nor
    /// can it be the default device. Hidden devices can only be discovered by
    /// knowing their UID and using kAudioHardwarePropertyDeviceForUID.
    #[doc(alias = "kAudioDevicePropertyIsHidden")]
    pub const DEVICE_IS_HIDDEN: Self = Self(u32::from_be_bytes(*b"hidn"));

    /// An array of two u32s, the first for the left channel, the second for the
    /// right channel, that indicate the channel numbers to use for stereo IO on the
    /// device. The value of this property can be different for input and output and
    /// there are no restrictions on the channel numbers that can be used.
    #[doc(alias = "kAudioDevicePropertyPreferredChannelsForStereo")]
    pub const DEVICE_PREFERRED_CHANNELS_FOR_STEREO: Self = Self(u32::from_be_bytes(*b"dch2"));

    /// An AudioChannelLayout that indicates how each channel of the AudioDevice
    /// should be used.
    #[doc(alias = "kAudioDevicePropertyPreferredChannelLayout")]
    pub const DEVICE_PREFERRED_CHANNEL_LAYOUT: Self = Self(u32::from_be_bytes(*b"srnd"));
}
