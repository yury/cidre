use crate::{av, define_obj_type, ns, objc};

define_obj_type!(
    /// The location of a data source on an iOS device.
    #[doc(alias = "AVAudioSessionLocation")]
    pub Location(ns::String)
);

#[cfg(any(target_os = "ios", target_os = "watchos", target_os = "tvos"))]
impl Location {
    pub fn upper() -> &'static Self {
        unsafe { AVAudioSessionLocationUpper }
    }
    pub fn lower() -> &'static Self {
        unsafe { AVAudioSessionLocationLower }
    }
}

#[cfg(any(target_os = "ios", target_os = "watchos", target_os = "tvos"))]
extern "C" {
    static AVAudioSessionLocationUpper: &'static Location;
    static AVAudioSessionLocationLower: &'static Location;
}

define_obj_type!(
    #[doc(alias = "AVAudioSessionOrientation")]
    pub Orientation(ns::String)
);

#[cfg(any(target_os = "ios", target_os = "watchos", target_os = "tvos"))]
impl Orientation {
    pub fn top() -> &'static Self {
        unsafe { AVAudioSessionOrientationTop }
    }

    pub fn bottom() -> &'static Self {
        unsafe { AVAudioSessionOrientationBottom }
    }

    pub fn front() -> &'static Self {
        unsafe { AVAudioSessionOrientationFront }
    }

    pub fn back() -> &'static Self {
        unsafe { AVAudioSessionOrientationBack }
    }

    pub fn left() -> &'static Self {
        unsafe { AVAudioSessionOrientationLeft }
    }

    pub fn right() -> &'static Self {
        unsafe { AVAudioSessionOrientationRight }
    }
}

#[cfg(any(target_os = "ios", target_os = "watchos", target_os = "tvos"))]
extern "C" {
    static AVAudioSessionOrientationTop: &'static Orientation;
    static AVAudioSessionOrientationBottom: &'static Orientation;
    static AVAudioSessionOrientationFront: &'static Orientation;
    static AVAudioSessionOrientationBack: &'static Orientation;
    static AVAudioSessionOrientationLeft: &'static Orientation;
    static AVAudioSessionOrientationRight: &'static Orientation;
}

define_obj_type!(
    #[doc(alias = "AVAudioSessionPolarPattern")]
    pub PolarPattern(ns::String)
);

#[cfg(any(target_os = "ios", target_os = "watchos", target_os = "tvos"))]
impl PolarPattern {
    pub fn omnidirectional() -> &'static Self {
        unsafe { AVAudioSessionPolarPatternOmnidirectional }
    }

    pub fn cardioid() -> &'static Self {
        unsafe { AVAudioSessionPolarPatternCardioid }
    }

    pub fn subcardioid() -> &'static Self {
        unsafe { AVAudioSessionPolarPatternSubcardioid }
    }

    #[cfg(target_os = "ios")]
    pub fn stereo() -> &'static Self {
        unsafe { AVAudioSessionPolarPatternStereo }
    }
}

#[cfg(any(target_os = "ios", target_os = "watchos", target_os = "tvos"))]
extern "C" {
    static AVAudioSessionPolarPatternOmnidirectional: &'static PolarPattern;
    static AVAudioSessionPolarPatternCardioid: &'static PolarPattern;
    static AVAudioSessionPolarPatternSubcardioid: &'static PolarPattern;
    #[cfg(target_os = "ios")]
    static AVAudioSessionPolarPatternStereo: &'static PolarPattern;
}

define_obj_type!(
    #[doc(alias = "AVAudioSessionChannelDescription")]
    pub ChannelDesc(ns::Id)
);

impl ChannelDesc {
    /// A human-readable name for the channel.
    #[objc::msg_send(channelName)]
    pub fn channel_name(&self) -> &ns::String;

    /// The UID (unique identifier) of the port owning the channel
    #[objc::msg_send(owningPortUID)]
    pub fn owning_port_uid(&self) -> &ns::String;

    #[objc::msg_send(channelNumber)]
    pub fn channel_number(&self) -> usize;

    #[objc::msg_send(channelLabel)]
    pub fn channel_label(&self) -> crate::cat::audio::ChannelLabel;
}

define_obj_type!(
    /// Information about one of potentially multiple data sources associated with a port.
    #[doc(alias = "AVAudioSessionDataSourceDescription")]
    pub DataSrcDesc(ns::Id)
);

impl DataSrcDesc {
    // TODO: verify leak
    /// System-assigned ID for the data source.
    #[objc::msg_send(dataSourceID)]
    pub fn data_src_id(&self) -> &ns::String;

    /// Human-readable name for the data source.
    #[objc::msg_send(dataSourceName)]
    pub fn data_source_name(&self) -> &ns::String;

    /// Location and orientation can be used to distinguish between multiple data sources
    /// belonging to a single port.  For example, in the case of a port of type
    /// AVAudioSessionPortBuiltInMic, one can use these properties to differentiate between an
    /// upper/front-facing microphone and a lower/bottom-facing microphone.
    ///
    /// Will be nil for data sources for which the location is not known.
    #[objc::msg_send(location)]
    pub fn location(&self) -> Option<&Location>;

    /// Describes the orientation of a data source. Will be nil for data sources for which the
    /// orientation is not known.
    #[objc::msg_send(orientation)]
    pub fn orientation(&self) -> Option<&Orientation>;

    #[objc::msg_send(supportedPolarPatterns)]
    pub fn supported_polar_patterns(&self) -> Option<&ns::Array<PolarPattern>>;

    #[objc::msg_send(selectedPolarPattern)]
    pub fn selected_polar_pattern(&self) -> Option<&PolarPattern>;

    #[objc::msg_send(preferredPolarPattern)]
    pub fn preferred_polar_pattern(&self) -> Option<&PolarPattern>;

    // - (BOOL)setPreferredPolarPattern:(nullable AVAudioSessionPolarPattern)pattern error:(NSError **)outError

    #[objc::msg_send(setPreferredPolarPattern:error:)]
    pub unsafe fn set_preferred_polar_pattern_err<'ear>(
        &mut self,
        val: Option<&PolarPattern>,
        err: *mut Option<&'ear ns::Error>,
    ) -> bool;

    /// Select the desired polar pattern from the set of available patterns. Setting a nil value
    /// will clear the preference.
    ///
    /// If the owning port and data source are part of the active audio route, changing the polar
    /// pattern will likely result in a route reconfiguration. If the owning port and data source are
    /// not part of the active route, selecting a polar pattern will not result in an immediate route
    /// reconfiguration.  Use AVAudioSession's setPreferredInput:error: method to activate the port. Use
    /// setPreferredDataSource:error: to active the data source on the port.
    /// You must call setPreferredInputOrientation:error: on the AVAudioSession if you chose the
    /// AVAudioSessionPolarPatternStereo polar pattern.
    pub fn set_preferred_polar_pattern<'ear>(&mut self, val: Option<&PolarPattern>) -> ns::Result {
        ns::if_false(|err| unsafe { self.set_preferred_polar_pattern_err(val, err) })
    }
}

define_obj_type!(
    #[doc(alias = "AVAudioSessionPortDescription")]
    pub PortDesc(ns::Id)
);

impl PortDesc {
    #[objc::msg_send(portType)]
    pub fn port_type(&self) -> &av::AudioSessionPort;

    #[objc::msg_send(portName)]
    pub fn port_name(&self) -> &ns::String;

    #[objc::msg_send(UID)]
    pub fn uid(&self) -> &ns::String;

    /// This property's value will be true if the associated hardware port has built-in
    /// processing for two-way voice communication.
    #[objc::msg_send(hasHardwareVoiceCallProcessing)]
    pub fn has_hw_voice_call_processing(&self) -> bool;

    #[objc::msg_send(isSpatialAudioEnabled)]
    pub fn is_spatial_audio_enabled(&self) -> bool;

    #[objc::msg_send(channels)]
    pub fn channels(&self) -> Option<&ns::Array<ChannelDesc>>;

    #[objc::msg_send(dataSources)]
    pub fn data_srcs(&self) -> Option<&ns::Array<DataSrcDesc>>;

    #[objc::msg_send(selectedDataSource)]
    pub fn selected_data_src(&self) -> Option<&DataSrcDesc>;

    #[objc::msg_send(preferredDataSource)]
    pub fn preferred_data_src(&self) -> Option<&DataSrcDesc>;

    #[objc::msg_send(setPreferredDataSource:error:)]
    pub unsafe fn set_preferred_data_src_err<'ear>(
        &mut self,
        val: Option<&DataSrcDesc>,
        err: *mut Option<&'ear ns::Error>,
    ) -> bool;

    pub fn set_preferred_data_src(&mut self, val: Option<&DataSrcDesc>) -> ns::Result {
        ns::if_false(|err| unsafe { self.set_preferred_data_src_err(val, err) })
    }
}

define_obj_type!(
    #[doc(alias = "AVAudioSessionRouteDescription")]
    pub RouteDesc(ns::Id)
);

impl RouteDesc {
    #[objc::msg_send(inputs)]
    pub fn inputs(&self) -> &ns::Array<PortDesc>;

    #[objc::msg_send(outputs)]
    pub fn outputs(&self) -> &ns::Array<PortDesc>;
}
