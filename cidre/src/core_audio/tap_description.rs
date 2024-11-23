use crate::{api, arc, define_obj_type, ns, objc};

#[doc(alias = "CATapMuteBehavior")]
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
#[repr(isize)]
pub enum TapMuteBehavior {
    Unmuted = 0,
    Muted = 1,
    MuttedWhenTapped = 2,
}

define_obj_type!(
    /// This class describes a tap object that contains an input stream.
    /// The input stream is a mix of all of the specified processes output audio.
    #[doc(alias = "CATapDescription")]
    pub TapDesc(ns::Id),
    CA_TAP_DESCRIPTION,
    #[api::available(macos = 12.0)]
);

impl arc::A<TapDesc> {
    #[objc::msg_send(initStereoMixdownOfProcesses:)]
    pub fn init_stereo_mixdown_of_processes(
        self,
        processes_obj_ids_to_include: &ns::Array<ns::Number>,
    ) -> arc::R<TapDesc>;

    #[objc::msg_send(initStereoGlobalTapButExcludeProcesses:)]
    pub fn init_stereo_global_tap_but_exclude_processes(
        self,
        processes_obj_ids_to_exclude: &ns::Array<ns::Number>,
    ) -> arc::R<TapDesc>;

    #[objc::msg_send(initMonoMixdownOfProcesses:)]
    pub fn init_mono_mixdown_of_processes(
        self,
        processes_obj_ids_to_include: &ns::Array<ns::Number>,
    ) -> arc::R<TapDesc>;

    #[objc::msg_send(initMonoGlobalTapButExcludeProcesses:)]
    pub fn init_mono_global_tap_but_exclude_processes(
        self,
        processes_obj_ids_to_exclude: &ns::Array<ns::Number>,
    ) -> arc::R<TapDesc>;

    #[objc::msg_send(initWithProcesses:andDeviceUID:withStream:)]
    pub fn init_with_processes_and_device(
        self,
        processes_obj_ids_to_include: &ns::Array<ns::Number>,
        device_uid: &ns::String,
        stream: isize,
    ) -> arc::R<TapDesc>;

    #[objc::msg_send(initExcludingProcesses:andDeviceUID:withStream:)]
    pub fn init_excluding_processes_and_device(
        self,
        processes_obj_ids_to_exclude: &ns::Array<ns::Number>,
        device_uid: &ns::String,
        stream: isize,
    ) -> arc::R<TapDesc>;
}

impl TapDesc {
    /// Mix all given process audio streams down to stereo.
    ///
    /// Mono sources will be duplicated in both right and left channels.
    pub fn with_stereo_mixdown_of_processes(
        processes_obj_ids_to_include: &ns::Array<ns::Number>,
    ) -> arc::R<Self> {
        Self::alloc().init_stereo_mixdown_of_processes(processes_obj_ids_to_include)
    }

    /// Mix all processes to a stereo stream except the given processes.
    ///
    /// Mono sources will be duplicated in both right and left channels.
    pub fn with_stereo_global_tap_excluding_processes(
        processes_obj_ids_to_exclude: &ns::Array<ns::Number>,
    ) -> arc::R<Self> {
        Self::alloc().init_stereo_global_tap_but_exclude_processes(processes_obj_ids_to_exclude)
    }

    pub fn with_mono_mixdown_of_processes(
        processes_obj_ids_to_include: &ns::Array<ns::Number>,
    ) -> arc::R<Self> {
        Self::alloc().init_mono_mixdown_of_processes(processes_obj_ids_to_include)
    }

    pub fn with_mono_global_tap_excluding_processes(
        processes_obj_ids_to_exclude: &ns::Array<ns::Number>,
    ) -> arc::R<Self> {
        Self::alloc().init_mono_global_tap_but_exclude_processes(processes_obj_ids_to_exclude)
    }

    pub fn with_processes_and_device(
        processes_obj_ids_to_include: &ns::Array<ns::Number>,
        device_uid: &ns::String,
        stream: isize,
    ) -> arc::R<Self> {
        Self::alloc().init_with_processes_and_device(
            processes_obj_ids_to_include,
            device_uid,
            stream,
        )
    }

    /// Human readable name of this tap.
    #[objc::msg_send(name)]
    pub fn name(&self) -> Option<arc::R<ns::String>>;

    #[objc::msg_send(setName:)]
    pub fn set_name(&mut self, val: Option<&ns::String>);

    #[objc::msg_send(UUID)]
    pub fn uuid(&self) -> arc::R<ns::Uuid>;

    #[objc::msg_send(setUUID:)]
    pub fn set_uuid(&mut self, val: Option<&ns::Uuid>);

    #[objc::msg_send(processes)]
    pub fn processes(&self) -> arc::R<ns::Array<ns::Number>>;

    #[objc::msg_send(setProcesses:)]
    pub fn set_processes(&mut self, val: &ns::Array<ns::Number>);

    /// True if this description is a mono mixdown of channels.
    #[objc::msg_send(isMono)]
    pub fn is_mono(&self) -> bool;

    #[objc::msg_send(setMono:)]
    pub fn set_mono(&mut self, val: bool);

    /// True if this description should tap all processes except the process listed in the 'processes' property.
    #[objc::msg_send(isExclusive)]
    pub fn is_exclusive(&self) -> bool;

    #[objc::msg_send(setExclusive:)]
    pub fn set_exclusive(&mut self, val: bool);

    /// True if this description is a mono or stereo mix of the tapped device's channels.
    #[objc::msg_send(isMixdown)]
    pub fn is_mixdown(&self) -> bool;

    #[objc::msg_send(setMixdown:)]
    pub fn set_mixdown(&mut self, val: bool);

    /// True if this tap is only visible to the client process that created the tap.
    #[objc::msg_send(isPrivate)]
    pub fn is_private(&self) -> bool;

    #[objc::msg_send(setPrivate:)]
    pub fn set_private(&mut self, val: bool);

    #[objc::msg_send(isMuted)]
    pub fn mute_behavior(&self) -> TapMuteBehavior;

    #[objc::msg_send(setMuteBehavior:)]
    pub fn set_mute_behavior(&mut self, val: TapMuteBehavior);

    /// An optional deviceUID that will have a value if this tap only taps a specific hardware device
    #[objc::msg_send(deviceUID)]
    pub fn device_uid(&self) -> Option<arc::R<ns::String>>;

    #[objc::msg_send(setDeviceUID:)]
    pub fn set_device_uid(&mut self, val: Option<&ns::String>);

    /// An optional ns::Number that will have a value if this tap taps a specific device stream.
    /// The value represents the index of the hardware stream.
    #[objc::msg_send(stream)]
    pub fn stream(&self) -> Option<arc::R<ns::Number>>;

    #[objc::msg_send(setStream:)]
    pub fn set_stream(&mut self, val: Option<&ns::Number>);
}

#[link(name = "core_audio", kind = "static")]
extern "C" {
    static CA_TAP_DESCRIPTION: &'static objc::Class<TapDesc>;
}

#[cfg(test)]
mod tests {
    use crate::{core_audio as ca, ns};
    #[test]
    fn basics() {
        let _ = ca::TapDesc::new();
        let mut tap = ca::TapDesc::with_stereo_mixdown_of_processes(&ns::Array::new());
        let name = tap.name();
        println!("{name:?}");
        tap.set_name(Some(ns::str!(c"hello")));
        tap.set_name(None);
        let uuid = tap.uuid();
        println!("{uuid:?}");
    }
}
