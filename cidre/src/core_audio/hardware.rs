use std::ffi::c_void;

use crate::{arc, at::AudioBufListN, cat, cf, core_audio, os, sys};

#[cfg(all(feature = "blocks", feature = "dispatch"))]
use crate::{blocks, dispatch};

use super::{
    hardware_tapping::AudioHardwareCreateProcessTap, AudioObjId, AudioObjPropAddr,
    AudioObjPropElement, AudioObjPropScope, AudioObjPropSelector,
};

#[doc(alias = "AudioObjectPropertyListenerProc")]
pub type AudioObjPropListenerFn<T = c_void> = extern "C-unwind" fn(
    obj_id: core_audio::AudioObjId,
    number_addresses: u32,
    addresses: *const AudioObjPropAddr,
    client_data: *mut T,
) -> os::Status;

#[doc(alias = "AudioObjectPropertyListenerBlock")]
#[cfg(all(feature = "blocks", feature = "dispatch"))]
pub type AudioObjPropListenerBlock =
    blocks::EscBlock<fn(number_addresses: u32, addresses: *const AudioObjPropAddr)>;

impl core_audio::AudioObjId {
    #[doc(alias = "kAudioObjectSystemObject")]
    pub const SYS_OBJECT: Self = Self(1);

    pub fn hardware_devices() -> os::Result<Vec<Self>> {
        Self::SYS_OBJECT.prop_vec::<Self>(&AudioObjPropAddr {
            selector: AudioObjPropSelector::HARDWARE_DEVICES,
            scope: AudioObjPropScope::GLOBAL,
            element: AudioObjPropElement::WILDCARD,
        })
    }

    pub fn hardware_default_input_device() -> os::Result<Self> {
        Self::SYS_OBJECT.prop(&AudioObjPropAddr {
            selector: AudioObjPropSelector::HARDWARE_DEFAULT_INPUT_DEVICE,
            scope: AudioObjPropScope::GLOBAL,
            element: AudioObjPropElement::MAIN,
        })
    }

    pub fn hardware_default_output_device() -> os::Result<Self> {
        Self::SYS_OBJECT.prop(&AudioObjPropAddr {
            selector: AudioObjPropSelector::HARDWARE_DEFAULT_OUTPUT_DEVICE,
            scope: AudioObjPropScope::GLOBAL,
            element: AudioObjPropElement::MAIN,
        })
    }

    pub fn nominal_sample_rate(&self) -> os::Result<f64> {
        self.prop(&AudioObjPropAddr {
            selector: AudioObjPropSelector::NOMINAL_SAMPLE_RATE,
            scope: AudioObjPropScope::GLOBAL,
            element: AudioObjPropElement::MAIN,
        })
    }

    #[doc(alias = "AudioObjectSetPropertyData")]
    pub fn set_prop<T: Sized>(&self, address: &AudioObjPropAddr, val: &T) -> os::Result {
        unsafe {
            AudioObjectSetPropertyData(
                *self,
                address,
                0,
                std::ptr::null(),
                std::mem::size_of_val(val) as u32,
                val as *const _ as _,
            )
            .result()
        }
    }
    pub fn device_uid(&self) -> os::Result<arc::R<cf::String>> {
        self.cf_prop(&AudioObjPropAddr {
            selector: AudioObjPropSelector::DEVICE_UID,
            scope: AudioObjPropScope::GLOBAL,
            element: AudioObjPropElement::MAIN,
        })
    }

    pub fn stream_cfg(&self, scope: AudioObjPropScope) -> os::Result<AudioBufListN> {
        let addr = AudioObjPropAddr {
            selector: AudioObjPropSelector::STREAM_CFG,
            scope,
            element: AudioObjPropElement::WILDCARD,
        };
        let mut size = self.prop_size(&addr)?;
        let mut res = AudioBufListN::new(size as _);
        unsafe {
            AudioObjectGetPropertyData(
                *self,
                &addr,
                0,
                std::ptr::null(),
                &mut size,
                res.as_mut_ptr() as _,
            )
        }
        .result()?;
        Ok(res)
    }

    pub fn input_stream_cfg(&self) -> os::Result<AudioBufListN> {
        self.stream_cfg(AudioObjPropScope::INPUT)
    }

    pub fn output_stream_cfg(&self) -> os::Result<AudioBufListN> {
        self.stream_cfg(AudioObjPropScope::OUTPUT)
    }

    pub fn name(&self) -> os::Result<arc::R<cf::String>> {
        self.cf_prop(&AudioObjPropAddr {
            selector: AudioObjPropSelector::NAME,
            scope: AudioObjPropScope::WILDCARD,
            element: AudioObjPropElement::WILDCARD,
        })
    }

    #[doc(alias = "AudioObjectHasProperty")]
    pub fn has_prop(&self, address: &AudioObjPropAddr) -> bool {
        unsafe { AudioObjectHasProperty(*self, address) }
    }

    #[doc(alias = "AudioObjectIsPropertySettable")]
    pub fn is_prop_settable(&self, address: &AudioObjPropAddr) -> os::Result<bool> {
        os::result_init(|res| unsafe { AudioObjectIsPropertySettable(*self, address, res) })
    }

    #[doc(alias = "AudioObjectGetPropertyDataSize")]
    pub fn prop_size(&self, address: &AudioObjPropAddr) -> os::Result<u32> {
        let mut val = std::mem::MaybeUninit::uninit();
        unsafe {
            AudioObjectGetPropertyDataSize(*self, address, 0, std::ptr::null(), val.as_mut_ptr())
                .result()?;
            Ok(val.assume_init())
        }
    }

    #[doc(alias = "AudioObjectGetPropertyData")]
    pub fn prop<T: Sized>(&self, address: &AudioObjPropAddr) -> os::Result<T> {
        let mut data_size = std::mem::size_of::<T>() as u32;
        let mut val = std::mem::MaybeUninit::uninit();
        unsafe {
            AudioObjectGetPropertyData(
                *self,
                address,
                0,
                std::ptr::null(),
                &mut data_size,
                val.as_mut_ptr() as _,
            )
            .result()?;
            Ok(val.assume_init())
        }
    }

    #[doc(alias = "AudioObjectGetPropertyData")]
    pub fn prop_with_qualifier<T: Sized, Q: Sized>(
        &self,
        address: &AudioObjPropAddr,
        qualifier: &Q,
    ) -> os::Result<T> {
        let mut data_size = std::mem::size_of::<T>() as u32;
        let qualifier_size = std::mem::size_of::<Q>() as u32;
        let mut val = std::mem::MaybeUninit::uninit();
        unsafe {
            let res = AudioObjectGetPropertyData(
                *self,
                address,
                qualifier_size,
                qualifier as *const Q as *const _,
                &mut data_size,
                val.as_mut_ptr() as _,
            )
            .result()?;
            Ok(val.assume_init())
        }
    }

    pub fn cf_prop<T: arc::Release>(&self, address: &AudioObjPropAddr) -> os::Result<arc::R<T>> {
        os::result_init(|res| unsafe {
            let mut data_size = std::mem::size_of::<arc::R<T>>() as u32;
            AudioObjectGetPropertyData(
                *self,
                address,
                0,
                std::ptr::null(),
                &mut data_size,
                res as _,
            )
        })
    }

    #[doc(alias = "AudioObjectGetPropertyData")]
    pub fn prop_vec<T: Sized>(&self, address: &AudioObjPropAddr) -> os::Result<Vec<T>> {
        let mut data_size = self.prop_size(address)?;
        unsafe {
            let len = (data_size as usize) / std::mem::size_of::<T>();
            if len == 0 {
                return Ok(vec![]);
            }
            let mut out = Vec::<T>::with_capacity(len);
            AudioObjectGetPropertyData(
                *self,
                address,
                0,
                std::ptr::null(),
                &mut data_size,
                out.as_mut_ptr().cast(),
            )
            .result()?;
            out.set_len(len);
            Ok(out)
        }
    }

    #[doc(alias = "AudioObjectShow")]
    pub fn show(&self) {
        unsafe { AudioObjectShow(*self) }
    }

    #[doc(alias = "AudioObjectAddPropertyListener")]
    pub fn add_prop_listener<T>(
        &self,
        address: &AudioObjPropAddr,
        listener: AudioObjPropListenerFn<T>,
        client_data: *mut T,
    ) -> os::Result {
        unsafe {
            AudioObjectAddPropertyListener(
                *self,
                address,
                std::mem::transmute(listener),
                client_data.cast(),
            )
            .result()
        }
    }

    #[doc(alias = "AudioObjectRemovePropertyListener")]
    pub fn remove_prop_listener<T>(
        &self,
        address: &AudioObjPropAddr,
        listener: AudioObjPropListenerFn<T>,
        client_data: *mut T,
    ) -> os::Result {
        unsafe {
            AudioObjectRemovePropertyListener(
                *self,
                address,
                std::mem::transmute(listener),
                client_data.cast(),
            )
            .result()
        }
    }

    #[doc(alias = "AudioObjectAddPropertyListenerBlock")]
    #[cfg(all(feature = "blocks", feature = "dispatch"))]
    pub fn add_prop_listener_block(
        &self,
        address: &AudioObjPropAddr,
        dispatch_queue: Option<&dispatch::Queue>,
        listener: &mut AudioObjPropListenerBlock,
    ) -> os::Result {
        unsafe {
            AudioObjectAddPropertyListenerBlock(*self, address, dispatch_queue, listener).result()
        }
    }

    #[doc(alias = "AudioObjectRemovePropertyListenerBlock")]
    #[cfg(all(feature = "blocks", feature = "dispatch"))]
    pub fn remove_prop_listener_block(
        &self,
        address: &AudioObjPropAddr,
        dispatch_queue: Option<&dispatch::Queue>,
        listener: &mut AudioObjPropListenerBlock,
    ) -> os::Result {
        unsafe {
            AudioObjectRemovePropertyListenerBlock(*self, address, dispatch_queue, listener)
                .result()
        }
    }
}

#[derive(Debug, Eq, PartialEq)]
#[repr(transparent)]
pub struct ProcessObj(pub core_audio::AudioObjId);

impl std::ops::Deref for ProcessObj {
    type Target = core_audio::AudioObjId;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl std::ops::DerefMut for ProcessObj {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl ProcessObj {
    pub fn pid(&self) -> os::Result<sys::Pid> {
        self.prop(&AudioObjPropAddr {
            selector: core_audio::AudioObjPropSelector::PROCESS_PID,
            scope: Default::default(),
            element: Default::default(),
        })
    }

    pub fn bundle_id(&self) -> os::Result<arc::R<cf::String>> {
        self.cf_prop(&AudioObjPropAddr {
            selector: core_audio::AudioObjPropSelector::PROCESS_BUNDLE_ID,
            scope: Default::default(),
            element: Default::default(),
        })
    }

    pub fn is_running(&self) -> os::Result<bool> {
        let res: u32 = self.prop(&AudioObjPropAddr {
            selector: core_audio::AudioObjPropSelector::PROCESS_IS_RUNNING,
            scope: Default::default(),
            element: Default::default(),
        })?;
        Ok(res == 1)
    }

    pub fn is_running_input(&self) -> os::Result<bool> {
        let res: u32 = self.prop(&AudioObjPropAddr {
            selector: core_audio::AudioObjPropSelector::PROCESS_IS_RUNNING_INPUT,
            scope: Default::default(),
            element: Default::default(),
        })?;
        Ok(res == 1)
    }

    pub fn is_running_output(&self) -> os::Result<bool> {
        let res: u32 = self.prop(&AudioObjPropAddr {
            selector: core_audio::AudioObjPropSelector::PROCESS_IS_RUNNING_OUTPUT,
            scope: Default::default(),
            element: Default::default(),
        })?;
        Ok(res == 1)
    }

    pub fn devices(&self) -> os::Result<Vec<core_audio::AudioObjId>> {
        self.prop_vec(&AudioObjPropAddr {
            selector: core_audio::AudioObjPropSelector::PROCESS_DEVICES,
            scope: Default::default(),
            element: Default::default(),
        })
    }

    pub fn with_pid(pid: sys::Pid) -> os::Result<Self> {
        core_audio::AudioObjId::SYS_OBJECT.prop_with_qualifier(
            &AudioObjPropAddr {
                selector: AudioObjPropSelector::HARDWARE_TRANSLATE_PID_TO_PROCESS_OBJ,
                scope: Default::default(),
                element: Default::default(),
            },
            &pid,
        )
    }

    pub fn list() -> os::Result<Vec<Self>> {
        core_audio::AudioObjId::SYS_OBJECT.prop_vec(&AudioObjPropAddr {
            selector: AudioObjPropSelector::HARDWARE_PROCESS_OBJ_LIST,
            scope: Default::default(),
            element: Default::default(),
        })
    }
}

/// Processes AudioObjectPropertySelector values provided by the Process class.
impl core_audio::AudioObjPropSelector {
    /// A pid_t indicating the process ID associated with the process.
    #[doc(alias = "kAudioProcessPropertyPID")]
    pub const PROCESS_PID: Self = Self(u32::from_be_bytes(*b"ppid"));

    /// A cf::String that contains the bundle ID of the process. The caller is
    /// responsible for releasing the returned cf::Object.
    #[doc(alias = "kAudioProcessPropertyBundleID")]
    pub const PROCESS_BUNDLE_ID: Self = Self(u32::from_be_bytes(*b"pbid"));

    /// An array of AudioObjectIds that represent the devices currently used by the
    /// process for input or used by the process for output. The scope will select
    /// the input or output device list.
    pub const PROCESS_DEVICES: Self = Self(u32::from_be_bytes(*b"pdv#"));

    /// A u32 where a value of 0 indicates that there is not audio IO in progress
    /// in the process, and a value of 1 indicates that there is audio IO in progress
    /// in the process. Note that audio IO may in progress even if no input or output
    /// streams are active.
    pub const PROCESS_IS_RUNNING: Self = Self(u32::from_be_bytes(*b"pir?"));

    /// A u32 where a value of 0 indicates that the process is not running any
    /// IO or there is not any active input streams, and a value of 1 indicates that
    /// the process is running IO and there is at least one active input stream.
    pub const PROCESS_IS_RUNNING_INPUT: Self = Self(u32::from_be_bytes(*b"piri"));

    /// A u32 where a value of 0 indicates that the process is not running any
    /// IO or there is not any active output streams, and a value of 1 indicates that
    /// the process is running IO and there is at least one active output stream.
    pub const PROCESS_IS_RUNNING_OUTPUT: Self = Self(u32::from_be_bytes(*b"piro"));
}

impl core_audio::AudioObjId {
    pub fn create_io_proc_id<const IN: usize, const ON: usize, T>(
        &self,
        proc: AudioDeviceIoProc<IN, ON, T>,
        client_data: Option<&mut T>,
    ) -> os::Result<AudioDeviceIoProcId> {
        let mut res = None;
        unsafe {
            AudioDeviceCreateIOProcID(
                *self,
                std::mem::transmute(proc),
                std::mem::transmute(client_data),
                &mut res,
            )
            .result()?;
            Ok(res.unwrap_unchecked())
        }
    }
}

impl core_audio::AudioObjPropSelector {
    #[doc(alias = "kAudioHardwarePropertyProcessInputMute")]
    pub const HARDWARE_PROCESS_INPUT_MUTE: Self = Self(u32::from_be_bytes(*b"pmin"));

    /// An array of the AudioObjectIds that represent all the devices currently
    /// available to the system.
    #[doc(alias = "kAudioHardwarePropertyDevices")]
    pub const HARDWARE_DEVICES: Self = Self(u32::from_be_bytes(*b"dev#"));

    /// The AudioObjectId of the default input AudioDevice.
    #[doc(alias = "kAudioHardwarePropertyDefaultInputDevice")]
    pub const HARDWARE_DEFAULT_INPUT_DEVICE: Self = Self(u32::from_be_bytes(*b"dIn "));

    /// The AudioObjectId of the default output AudioDevice.
    #[doc(alias = "kAudioHardwarePropertyDefaultOutputDevice")]
    pub const HARDWARE_DEFAULT_OUTPUT_DEVICE: Self = Self(u32::from_be_bytes(*b"dOut"));

    /// The AudioObjectId of the output AudioDevice to use for system related sound
    /// from the alert sound to digital call progress.
    #[doc(alias = "kAudioHardwarePropertyDefaultSystemOutputDevice")]
    pub const HARDWARE_DEFAULT_SYS_OUTPUT_DEVICE: Self = Self(u32::from_be_bytes(*b"sOut"));

    /// This property fetches the AudioObjectId that corresponds to the AudioDevice
    /// that has the given UID. The UID is passed in via the qualifier as a cf::String
    /// while the AudioObjectId for the AudioDevice is returned to the caller as the
    /// property's data. Note that an error is not returned if the UID doesn't refer
    /// to any AudioDevices. Rather, this property will return kAudioObjectUnknown
    /// as the value of the property.
    #[doc(alias = "kAudioHardwarePropertyTranslateUIDToDevice")]
    pub const HARDWARE_TRANSLATE_UID_TO_DEVICE: Self = Self(u32::from_be_bytes(*b"uidd"));

    /// An array of AudioObjectIds that represent the Process objects for all client processes
    /// currently connected to the system.
    #[doc(alias = "kAudioHardwarePropertyProcessObjectList")]
    pub const HARDWARE_PROCESS_OBJ_LIST: Self = Self(u32::from_be_bytes(*b"prs#"));

    /// This property fetches the AudioObjectID that corresponds to the Process object
    /// that has the given PID. The PID is passed in via the qualifier as a pid_t
    /// while the AudioObjectID for the Process is returned to the caller as the
    /// property's data. Note that an error is not returned if the PID doesn't refer
    /// to any Process. Rather, this property will return kAudioObjectUnknown
    /// as the value of the property.
    #[doc(alias = "kAudioHardwarePropertyTranslatePIDToProcessObject")]
    pub const HARDWARE_TRANSLATE_PID_TO_PROCESS_OBJ: Self = Self(u32::from_be_bytes(*b"id2p"));
}

///  AudioDevice Properties
impl core_audio::AudioObjPropSelector {
    /// An os::Status that contains any error codes generated by loading the IOAudio
    /// driver plug-in for the AudioDevice or kAudioHardwareNoError if the plug-in
    /// loaded successfully. This property only exists for IOAudio-based
    /// AudioDevices whose driver has specified a plug-in to load.
    #[doc(alias = "kAudioDevicePropertyPlugIn")]
    pub const PLUG_IN: Self = Self(u32::from_be_bytes(*b"plug"));

    /// The type of this property is a UInt32, but its value has no meaning. This
    /// property exists so that clients can listen to it and be told when the
    /// configuration of the AudioDevice has changed in ways that cannot otherwise
    /// be conveyed through other notifications. In response to this notification,
    /// clients should re-evaluate everything they need to know about the device,
    /// particularly the layout and values of the controls.
    #[doc(alias = "kAudioDevicePropertyDeviceHasChanged")]
    pub const DEVICE_HAS_CHANGED: Self = Self(u32::from_be_bytes(*b"diff"));

    /// u32 where 1 means that the AudioDevice is running in at least one
    /// process on the system and 0 means that it isn't running at all.
    #[doc(alias = "kAudioDevicePropertyDeviceIsRunningSomewhere")]
    pub const DEVICE_IS_RUNNING_SOMEWHERE: Self = Self(u32::from_be_bytes(*b"gone"));

    /// A u32 where the value has no meaning. This property exists so that
    /// clients can be notified when the AudioDevice detects that an IO cycle has
    /// run past its deadline. Note that the notification for this property is
    /// usually sent from the AudioDevice's IO thread.
    #[doc(alias = "kAudioDeviceProcessorOverload")]
    pub const PROCESSOR_OVERLOAD: Self = Self(u32::from_be_bytes(*b"over"));

    /// A u32 where the value has no meaning. This property exists so that
    /// clients can be notified when IO on the device has stopped outside of the
    /// normal mechanisms. This typically comes up when IO is stopped after
    /// AudioDeviceStart has returned successfully but prior to the notification for
    /// kAudioDevicePropertyIsRunning being sent.
    #[doc(alias = "kAudioDevicePropertyIOStoppedAbnormally")]
    pub const IO_STOPPED_ABNORMALLY: Self = Self(u32::from_be_bytes(*b"stpd"));

    /// A pid_t indicating the process that currently owns exclusive access to the
    /// AudioDevice or a value of -1 indicating that the device is currently
    /// available to all processes.

    /// If the AudioDevice is in a non-mixable mode,
    /// the HAL will automatically take hog mode on behalf of the first process to
    /// start an IOProc.
    ///
    /// Note that when setting this property, the value passed in is ignored. If
    /// another process owns exclusive access, that remains unchanged. If the
    /// current process owns exclusive access, it is released and made available to
    /// all processes again. If no process has exclusive access (meaning the current
    /// value is -1), this process gains ownership of exclusive access.  On return,
    /// the pid_t pointed to by inPropertyData will contain the new value of the
    /// property.
    #[doc(alias = "kAudioDevicePropertyHogMode")]
    pub const HOG_MODE: Self = Self(u32::from_be_bytes(*b"oink"));

    ///  A u32 whose value indicates the number of frames in the IO buffers.
    #[doc(alias = "kAudioDevicePropertyBufferFrameSize")]
    pub const BUF_FRAME_SIZE: Self = Self(u32::from_be_bytes(*b"fsiz"));

    /// An AudioValueRange indicating the minimum and maximum values, inclusive, for
    /// kAudioDevicePropertyBufferFrameSize.
    #[doc(alias = "kAudioDevicePropertyBufferFrameSizeRange")]
    pub const BUF_FRAME_SIZE_RANGE: Self = Self(u32::from_be_bytes(*b"fsz#"));

    /// A u32 that, if implemented by a device, indicates that the sizes of the
    /// buffers passed to an IOProc will vary by a small amount. The value of this
    /// property will indicate the largest buffer that will be passed and
    /// kAudioDevicePropertyBufferFrameSize will indicate the smallest buffer that
    /// will get passed to the IOProc. The usage of this property is narrowed to
    /// only allow for devices whose buffer sizes vary by small amounts greater than
    /// kAudioDevicePropertyBufferFrameSize. It is not intended to be a license for
    /// devices to be able to send buffers however they please. Rather, it is
    /// intended to allow for hardware whose natural rhythms lead to this necessity.
    #[doc(alias = "kAudioDevicePropertyUsesVariableBufferFrameSizes")]
    pub const USES_VARIABLE_BUF_FRAME_SIZES: Self = Self(u32::from_be_bytes(*b"vfsz"));

    /// A f32 whose range is from 0 to 1. This value indicates how much of the
    /// client portion of the IO cycle the process will use. The client portion of
    /// the IO cycle is the portion of the cycle in which the device calls the
    /// IOProcs so this property does not the apply to the duration of the entire
    /// cycle.
    #[doc(alias = "kAudioDevicePropertyIOCycleUsage")]
    pub const IO_CYCLE_USAGE: Self = Self(u32::from_be_bytes(*b"ncyc"));

    /// This property returns the stream configuration of the device in an
    /// AudioBufListN (with the buffer pointers set to NULL) which describes the
    /// list of streams and the number of channels in each stream. This corresponds
    /// to what will be passed into the IOProc.
    #[doc(alias = "kAudioDevicePropertyStreamConfiguration")]
    pub const STREAM_CFG: Self = Self(u32::from_be_bytes(*b"slay"));

    /// An AudioHardwareIOProcStreamUsage structure which details the stream usage
    /// of a given IO proc. If a stream is marked as not being used, the given
    /// IOProc will see a corresponding NULL buffer pointer in the AudioBufferList
    /// passed to its IO proc. Note that the number of streams detailed in the
    /// AudioHardwareIOProcStreamUsage must include all the streams of that
    /// direction on the device. Also, when getting the value of the property, one
    /// must fill out the mIOProc field of the AudioHardwareIOProcStreamUsage with
    /// the address of the of the IOProc whose stream usage is to be retrieved.
    #[doc(alias = "kAudioDevicePropertyIOProcStreamUsage")]
    pub const IO_PROC_STREAM_USAGE: Self = Self(u32::from_be_bytes(*b"suse"));

    /// A f64 that indicates the current actual sample rate of the AudioDevice
    /// as measured by its time stamps.
    #[doc(alias = "kAudioDevicePropertyActualSampleRate")]
    pub const ACTUAL_SAMPLE_RATE: Self = Self(u32::from_be_bytes(*b"asrt"));

    /// A cf::String that contains the UID for the AudioClockDevice that is currently
    /// serving as the main time base of the device. The caller is responsible
    /// for releasing the returned cf::String.
    #[doc(alias = "kAudioDevicePropertyClockDevice")]
    pub const CLOCK_DEVICE: Self = Self(u32::from_be_bytes(*b"apcd"));

    /// An os_workgroup_t that represents the thread workgroup the AudioDevice's
    /// IO thread belongs to. The caller is responsible for releasing the returned
    /// object.
    #[doc(alias = "kAudioDevicePropertyIOThreadOSWorkgroup")]
    pub const IO_THREAD_OS_WORKGROUP: Self = Self(u32::from_be_bytes(*b"oswg"));

    /// A u32 where a non-zero value indicates that the current process's audio
    /// will be zeroed out by the system. Note that this property does not apply to
    /// aggregate devices, just real, physical devices.
    #[doc(alias = "kAudioDevicePropertyProcessMute")]
    pub const PROCESS_MUTE: Self = Self(u32::from_be_bytes(*b"appm"));
}

/// AudioDevice Properties Implemented via AudioControl objects
///
/// AudioObjectPropertySelector values for AudioDevice properties that are
/// implemented by AudioControl objects.
///
/// These properties are also accessible by locating the AudioControl object
/// attached to the AudioDevice and using that object to access the properties of
/// the control.
impl core_audio::AudioObjPropSelector {
    /// A u32 where a value of 0 means that there isn't anything plugged into the
    /// jack associated withe given element and scope. This property is implemented
    /// by an AudioJackControl, a subclass of AudioBooleanControl.
    #[doc(alias = "kAudioDevicePropertyJackIsConnected")]
    pub const JACK_IS_CONNECTED: Self = Self(u32::from_be_bytes(*b"jack"));

    /// A f32 that represents the value of the volume control. The range is
    /// between 0.0 and 1.0 (inclusive). Note that the set of all Float32 values
    /// between 0.0 and 1.0 inclusive is much larger than the set of actual values
    /// that the hardware can select. This means that the Float32 range has a many
    /// to one mapping with the underlying hardware values. As such, setting a
    /// scalar value will result in the control taking on the value nearest to what
    /// was set. This property is implemented by an AudioControl object that is a
    /// subclass of AudioVolumeControl.
    #[doc(alias = "kAudioDevicePropertyVolumeScalar")]
    pub const VOLUME_SCALAR: Self = Self(u32::from_be_bytes(*b"volm"));

    /// A f32 that represents the value of the volume control in dB. Note that
    /// the set of all f32 values in the dB range for the control is much larger
    /// than the set of actual values that the hardware can select. This means that
    /// the f32 range has a many to one mapping with the underlying hardware
    /// values. As such, setting a dB value will result in the control taking on the
    /// value nearest to what was set. This property is implemented by an
    /// AudioControl object that is a subclass of AudioVolumeControl.
    #[doc(alias = "kAudioDevicePropertyVolumeDecibels")]
    pub const VOLUME_DECIBELS: Self = Self(u32::from_be_bytes(*b"vold"));

    /// An AudioValueRange that contains the minimum and maximum dB values the
    /// control can have. This property is implemented by an AudioControl object
    /// that is a subclass of AudioVolumeControl.
    #[doc(alias = "kAudioDevicePropertyVolumeRangeDecibels")]
    pub const VOLUME_RANGE_DECIBELS: Self = Self(u32::from_be_bytes(*b"vdb#"));

    /// A f32 that on input contains a scalar volume value for the and on exit
    /// contains the equivalent dB value. This property is implemented by an
    /// AudioControl object that is a subclass of AudioVolumeControl.
    #[doc(alias = "kAudioDevicePropertyVolumeScalarToDecibels")]
    pub const VOLUME_SCALAR_TO_DECIBELS: Self = Self(u32::from_be_bytes(*b"v2db"));

    /// A f32 that on input contains a dB volume value for the and on exit
    /// contains the equivalent scalar value. This property is implemented by
    /// AudioControl object that is a subclass of AudioVolumeControl.
    #[doc(alias = "kAudioDevicePropertyVolumeDecibelsToScalar")]
    pub const VOLUME_DECIBELS_TO_SCALAR: Self = Self(u32::from_be_bytes(*b"db2v"));

    /// A f32 where 0.0 is full left, 1.0 is full right, and 0.5 is center. This
    /// property is implemented by an AudioControl object that is a subclass of
    /// AudioStereoPanControl.
    #[doc(alias = "kAudioDevicePropertyStereoPan")]
    pub const STEREO_PAN: Self = Self(u32::from_be_bytes(*b"span"));

    /// An array of two u32s that indicate which elements of the owning object
    /// the signal is being panned between. This property is implemented by an
    /// AudioControl object that is a subclass of AudioStereoPanControl.
    #[doc(alias = "kAudioDevicePropertyStereoPanChannels")]
    pub const STEREO_PAN_CHANNELS: Self = Self(u32::from_be_bytes(*b"spn#"));

    /// A u32 where a value of 1 means that mute is enabled making that element
    /// inaudible. The property is implemented by an AudioControl object that is a
    /// subclass of AudioMuteControl.
    #[doc(alias = "kAudioDevicePropertyMute")]
    pub const MUTE: Self = Self(u32::from_be_bytes(*b"mute"));

    /// A u32 where a value of 1 means that just that element is audible and the
    /// other elements are inaudible. The property is implemented by an AudioControl
    /// object that is a subclass of AudioSoloControl.
    #[doc(alias = "kAudioDevicePropertySolo")]
    pub const SOLO: Self = Self(u32::from_be_bytes(*b"solo"));

    /// A u32 where a value of 1 means that the AudioDevice has enabled phantom
    /// power for the given element. The property is implemented by an AudioControl
    /// object that is a subclass of AudioPhantomPowerControl.
    #[doc(alias = "kAudioDevicePropertyPhantomPower")]
    pub const PHANTOM_POWER: Self = Self(u32::from_be_bytes(*b"phan"));

    /// A u32 where a value of 1 means that phase of the signal for the given
    /// element has been flipped 180 degrees. The property is implemented by an
    /// AudioControl object that is a subclass of AudioPhaseInvertControl.
    #[doc(alias = "kAudioDevicePropertyPhaseInvert")]
    pub const PHASE_INVERT: Self = Self(u32::from_be_bytes(*b"phsi"));

    /// A u32 where a value of 1 means that the signal for the element has
    /// exceeded the sample range. Once a clip light is turned on, it is to stay on
    /// until either the value of the control is set to false or the current IO
    /// session stops and a new IO session starts. The property is implemented by an
    /// AudioControl object that is a subclass of AudioClipLightControl.
    #[doc(alias = "kAudioDevicePropertyClipLight")]
    pub const CLIP_LIGHT: Self = Self(u32::from_be_bytes(*b"clip"));

    /// A u32 where a value of 1 means that the talkback channel is enabled. The
    /// property is implemented by an AudioControl object that is a subclass of
    /// AudioTalkbackControl.
    #[doc(alias = "kAudioDevicePropertyTalkback")]
    pub const TALKBACK: Self = Self(u32::from_be_bytes(*b"talb"));

    /// A u32 where a value of 1 means that the listenback channel is enabled.
    /// The property is implemented by an AudioControl object that is a subclass of
    /// AudioListenbackControl.
    #[doc(alias = "kAudioDevicePropertyListenback")]
    pub const LISTENBACK: Self = Self(u32::from_be_bytes(*b"lsnb"));

    /// An array of u32s whose values are the item IDs for the currently selected
    /// data sources. This property is implemented by an AudioControl object that is
    /// a subclass of AudioDataSourceControl.
    #[doc(alias = "kAudioDevicePropertyDataSource")]
    pub const DATA_SRC: Self = Self(u32::from_be_bytes(*b"ssrc"));

    /// An array of u32s that are represent all the IDs of all the data sources
    /// currently available. This property is implemented by an AudioControl object
    /// that is a subclass of AudioDataSourceControl.
    #[doc(alias = "kAudioDevicePropertyDataSources")]
    pub const DATA_SRCS: Self = Self(u32::from_be_bytes(*b"ssc#"));

    /// This property translates the given data source item ID into a human readable
    /// name using an AudioValueTranslation structure. The input data is the u32
    /// containing the item ID to translated and the output data is a cf::String. The
    /// caller is responsible for releasing the returned cf::Object. This property is
    /// implemented by an AudioControl object that is a subclass of
    /// AudioDataSourceControl.
    #[doc(alias = "kAudioDevicePropertyDataSourceNameForIDCFString")]
    pub const DATA_SRC_NAME_FOR_IDCF_STR: Self = Self(u32::from_be_bytes(*b"lscn"));

    /// This property returns a u32 that identifies the kind of data source
    /// the item ID refers to using an AudioValueTranslation structure. The input
    /// data is the u32 containing the item ID and the output data is the u32.
    #[doc(alias = "kAudioDevicePropertyDataSourceKindForID")]
    pub const DATA_SRC_KIND_FOR_ID: Self = Self(u32::from_be_bytes(*b"ssck"));

    /// An array of u32s whose values are the item IDs for the currently selected
    /// clock sources. This property is implemented by an AudioControl object that
    /// is a subclass of AudioClockControl.
    #[doc(alias = "kAudioDevicePropertyClockSource")]
    pub const CLOCK_SRC: Self = Self(u32::from_be_bytes(*b"csrc"));

    /// An array of u32s that are represent all the IDs of all the clock sources
    /// currently available. This property is implemented by an AudioControl object
    /// that is a subclass of AudioClockControl.
    #[doc(alias = "kAudioDevicePropertyClockSources")]
    pub const CLOCK_SRCS: Self = Self(u32::from_be_bytes(*b"csc#"));

    /// This property translates the given clock source item ID into a human
    /// readable name using an AudioValueTranslation structure. The input data is
    /// the u32 containing the item ID to translated and the output data is a
    /// cf::String. The caller is responsible for releasing the returned cf::Object.
    /// This property is implemented by an AudioControl object that is a subclass of
    /// AudioClockControl.
    #[doc(alias = "kAudioDevicePropertyClockSourceNameForIDCFString")]
    pub const CLOCK_SRC_NAME_FOR_IDCF_STR: Self = Self(u32::from_be_bytes(*b"lcsn"));

    /// This property returns a u32 that identifies the kind of clock source
    /// the item ID refers to using an AudioValueTranslation structure. The input
    /// data is the u32 containing the item ID and the output data is the u32.
    #[doc(alias = "kAudioDevicePropertyClockSourceKindForID")]
    pub const CLOCK_SRC_KIND_FOR_ID: Self = Self(u32::from_be_bytes(*b"csck"));

    /// A u32 where a value of 0 means that play through is off and a value of 1
    /// means that it is on. This property is implemented by an AudioControl object
    /// that is a subclass of AudioMuteControl. Further, the control that implements
    /// this property is only available through
    /// kAudioDevicePropertyScopePlayThrough.
    #[doc(alias = "kAudioDevicePropertyPlayThru")]
    pub const PLAY_THRU: Self = Self(u32::from_be_bytes(*b"thru"));

    /// A u32 where a value of 1 means that just that play through element is
    /// audible and the other elements are inaudible. The property is implemented by
    /// an AudioControl object that is a subclass of AudioSoloControl. Further, the
    /// control that implements this property is only available through
    /// kAudioDevicePropertyScopePlayThrough.
    #[doc(alias = "kAudioDevicePropertyPlayThruSolo")]
    pub const PLAY_THRU_SOLO: Self = Self(u32::from_be_bytes(*b"thrs"));

    /// A f32 that represents the value of the volume control. The range is
    /// between 0.0 and 1.0 (inclusive). Note that the set of all f32 values
    /// between 0.0 and 1.0 inclusive is much larger than the set of actual values
    /// that the hardware can select. This means that the f32 range has a many
    /// to one mapping with the underlying hardware values. As such, setting a
    /// scalar value will result in the control taking on the value nearest to what
    /// was set. This property is implemented by an AudioControl object that is a
    /// subclass of AudioVolumeControl.Further, the control that implements this
    /// property is only available through kAudioDevicePropertyScopePlayThrough.
    #[doc(alias = "kAudioDevicePropertyPlayThruVolumeScalar")]
    pub const PLAY_THRU_VOLUME_SCALAR: Self = Self(u32::from_be_bytes(*b"mvsc"));

    /// A f32 that represents the value of the volume control in dB. Note that
    /// the set of all f32 values in the dB range for the control is much larger
    /// than the set of actual values that the hardware can select. This means that
    /// the f32 range has a many to one mapping with the underlying hardware
    /// values. As such, setting a dB value will result in the control taking on the
    /// value nearest to what was set. This property is implemented by an
    /// AudioControl object that is a subclass of AudioVolumeControl. Further, the
    /// control that implements this property is only available through
    /// kAudioDevicePropertyScopePlayThrough.
    #[doc(alias = "kAudioDevicePropertyPlayThruVolumeDecibels")]
    pub const PLAY_THRU_VOLUME_DECIBELS: Self = Self(u32::from_be_bytes(*b"mvdb"));

    /// An AudioValueRange that contains the minimum and maximum dB values the
    /// control can have. This property is implemented by an AudioControl object
    /// that is a subclass of AudioVolumeControl. Further, the control that
    /// implements this property is only available through
    /// kAudioDevicePropertyScopePlayThrough.
    #[doc(alias = "kAudioDevicePropertyPlayThruVolumeRangeDecibels")]
    pub const PLAY_THRU_VOLUME_RANGE_DECIBELS: Self = Self(u32::from_be_bytes(*b"mvd#"));

    /// A f32 that on input contains a scalar volume value for the and on exit
    /// contains the equivalent dB value. This property is implemented by an
    /// AudioControl object that is a subclass of AudioVolumeControl. Further, the
    /// control that implements this property is only available through
    /// kAudioDevicePropertyScopePlayThrough.
    #[doc(alias = "kAudioDevicePropertyPlayThruVolumeScalarToDecibels")]
    pub const PLAY_THRU_VOLUME_SCALAR_TO_DECIBELS: Self = Self(u32::from_be_bytes(*b"mv2d"));

    /// A f32 that on input contains a dB volume value for the and on exit
    /// contains the equivalent scalar value. This property is implemented by an
    /// AudioControl object that is a subclass of AudioVolumeControl. Further, the
    /// control that implements this property is only available through
    /// kAudioDevicePropertyScopePlayThrough.
    #[doc(alias = "kAudioDevicePropertyPlayThruVolumeDecibelsToScalar")]
    pub const PLAY_THRU_VOLUME_DECIBELS_TO_SCALAR: Self = Self(u32::from_be_bytes(*b"mv2s"));

    /// A f32 where 0.0 is full left, 1.0 is full right, and 0.5 is center. This
    /// property is implemented by an AudioControl object that is a subclass of
    /// AudioStereoPanControl. Further, the control that implements this property is
    /// only available through kAudioDevicePropertyScopePlayThrough.
    #[doc(alias = "kAudioDevicePropertyPlayThruStereoPan")]
    pub const PLAY_THRU_STEREO_PAN: Self = Self(u32::from_be_bytes(*b"mspn"));

    /// An array of two u32s that indicate which elements of the owning object
    /// the signal is being panned between. This property is implemented by an
    /// AudioControl object that is a subclass of AudioStereoPanControl. Further,
    /// the control that implements this property is only available through
    /// kAudioDevicePropertyScopePlayThrough.
    #[doc(alias = "kAudioDevicePropertyPlayThruStereoPanChannels")]
    pub const PLAY_THRU_STEREO_PAN_CHANNELS: Self = Self(u32::from_be_bytes(*b"msp#"));

    /// An array of u32s whose values are the item IDs for the currently selected
    /// play through data destinations. This property is implemented by an
    /// AudioControl object that is a subclass of AudioDataDestinationControl.
    /// Further, the control that implements this property is only available through
    /// kAudioDevicePropertyScopePlayThrough.
    #[doc(alias = "kAudioDevicePropertyPlayThruDestination")]
    pub const PLAY_THRU_DST: Self = Self(u32::from_be_bytes(*b"mdds"));

    /// An array of u32s that are represent all the IDs of all the play through
    /// data destinations currently available. This property is implemented by an
    /// AudioControl object that is a subclass of AudioDataDestinationControl.
    /// Further, the control that implements this property is only available through
    /// kAudioDevicePropertyScopePlayThrough.
    #[doc(alias = "kAudioDevicePropertyPlayThruDestinations")]
    pub const PLAY_THRU_DSTS: Self = Self(u32::from_be_bytes(*b"mdd#"));

    /// This property translates the given play through data destination item ID
    /// into a human readable name using an AudioValueTranslation structure. The
    /// input data is the u32 containing the item ID to translated and the output
    /// data is a cf::String. The caller is responsible for releasing the returned
    /// cf::Object. This property is implemented by an AudioControl object that is a
    /// subclass of AudioDataDestinationControl. Further, the control that
    /// implements this property is only available through
    /// kAudioDevicePropertyScopePlayThrough.
    #[doc(alias = "kAudioDevicePropertyPlayThruDestinationNameForIDCFString")]
    pub const PLAY_THRU_DST_NAME_FOR_IDCF_STR: Self = Self(u32::from_be_bytes(*b"mddc"));

    /// An array of u32s whose values are the item IDs for the currently selected
    /// nominal line levels. This property is implemented by an AudioControl object
    /// that is a subclass of AudioLineLevelControl.
    #[doc(alias = "kAudioDevicePropertyChannelNominalLineLevel")]
    pub const CHANNEL_NOMINAL_LINE_LEVEL: Self = Self(u32::from_be_bytes(*b"nlvl"));

    /// An array of u32s that represent all the IDs of all the nominal line
    /// levels currently available. This property is implemented by an AudioControl
    /// object that is a subclass of AudioLineLevelControl.
    #[doc(alias = "kAudioDevicePropertyChannelNominalLineLevels")]
    pub const CHANNEL_NOMINAL_LINE_LEVELS: Self = Self(u32::from_be_bytes(*b"nlv#"));

    /// This property translates the given nominal line level item ID into a human
    /// readable name using an AudioValueTranslation structure. The input data is
    /// the u32 containing the item ID to be translated and the output data is a
    /// cf::String. The caller is responsible for releasing the returned cf::Object.
    /// This property is implemented by an AudioControl object that is a subclass of
    /// AudioLineLevelControl.
    #[doc(alias = "kAudioDevicePropertyChannelNominalLineLevelNameForIDCFString")]
    pub const CHANNEL_NOMINAL_LINE_LEVEL_NAME_FOR_IDCF_STR: Self =
        Self(u32::from_be_bytes(*b"lcnl"));

    /// An array of u32s whose values are the item IDs for the currently selected
    /// high pass filter setting. This property is implemented by an AudioControl
    /// object that is a subclass of AudioHighPassFilterControl.
    #[doc(alias = "kAudioDevicePropertyHighPassFilterSetting")]
    pub const HIGH_PASS_FILTER_SETTING: Self = Self(u32::from_be_bytes(*b"hipf"));

    /// An array of u32s that represent all the IDs of all the high pass filter
    /// settings currently available. This property is implemented by an
    /// AudioControl object that is a subclass of AudioHighPassFilterControl.
    #[doc(alias = "kAudioDevicePropertyHighPassFilterSettings")]
    pub const HIGH_PASS_FILTER_SETTINGS: Self = Self(u32::from_be_bytes(*b"hip#"));

    /// This property translates the given high pass filter setting item ID into a
    /// human readable name using an AudioValueTranslation structure. The input data
    /// is the u32 containing the item ID to be translated and the output data is
    /// a cf::String. The caller is responsible for releasing the returned cf::Object.
    /// This property is implemented by an AudioControl object that is a subclass of
    /// AudioHighPassFilterControl.
    #[doc(alias = "kAudioDevicePropertyHighPassFilterSettingNameForIDCFString")]
    pub const HIGH_PASS_FILTER_SETTING_NAME_FOR_IDCF_STR: Self = Self(u32::from_be_bytes(*b"hipl"));

    /// A f32 that represents the value of the LFE volume control. The range is
    /// between 0.0 and 1.0 (inclusive). Note that the set of all f32 values
    /// between 0.0 and 1.0 inclusive is much larger than the set of actual values
    /// that the hardware can select. This means that the f32 range has a many
    /// to one mapping with the underlying hardware values. As such, setting a
    /// scalar value will result in the control taking on the value nearest to what
    /// was set. This property is implemented by an AudioControl object that is a
    /// subclass of AudioLFEVolumeControl.
    #[doc(alias = "kAudioDevicePropertySubVolumeScalar")]
    pub const SUB_VOLUME_SCALAR: Self = Self(u32::from_be_bytes(*b"svlm"));

    /// A f32 that represents the value of the LFE volume control in dB. Note
    /// that the set of all f32 values in the dB range for the control is much
    /// larger than the set of actual values that the hardware can select. This
    /// means that the f32 range has a many to one mapping with the underlying
    /// hardware values. As such, setting a dB value will result in the control
    /// taking on the value nearest to what was set. This property is implemented by
    /// an AudioControl object that is a subclass of AudioLFE VolumeControl.
    #[doc(alias = "kAudioDevicePropertySubVolumeDecibels")]
    pub const SUB_VOLUME_DECIBELS: Self = Self(u32::from_be_bytes(*b"svld"));

    /// An AudioValueRange that contains the minimum and maximum dB values the
    /// control can have. This property is implemented by an AudioControl object
    /// that is a subclass of AudioLFEVolumeControl.
    #[doc(alias = "kAudioDevicePropertySubVolumeRangeDecibels")]
    pub const SUB_VOLUME_RANGE_DECIBELS: Self = Self(u32::from_be_bytes(*b"svd#"));

    /// A f32 that on input contains a scalar volume value for the and on exit
    /// contains the equivalent dB value. This property is implemented by an
    /// AudioControl object that is a subclass of AudioLFEVolumeControl.
    #[doc(alias = "kAudioDevicePropertySubVolumeScalarToDecibels")]
    pub const SUB_VOLUME_SCALAR_TO_DECIBELS: Self = Self(u32::from_be_bytes(*b"sv2d"));

    /// A f32 that on input contains a dB volume value for the and on exit
    /// contains the equivalent scalar value. This property is implemented by an
    /// AudioControl object that is a subclass of AudioLFEVolumeControl.
    #[doc(alias = "kAudioDevicePropertySubVolumeDecibelsToScalar")]
    pub const SUB_VOLUME_DECIBELS_TO_SCALAR: Self = Self(u32::from_be_bytes(*b"sd2v"));

    /// A u32 where a value of 1 means that mute is enabled making the LFE on
    ///
    /// that element inaudible. The property is implemented by an AudioControl
    /// object that is a subclass of AudioLFEMuteControl.
    #[doc(alias = "kAudioDevicePropertySubMute")]
    pub const SUB_MUTE: Self = Self(u32::from_be_bytes(*b"smut"));

    /// A u32 where 0 disables voice activity detection process and non-zero enables it.
    ///
    /// Voice activity detection can be used with input audio and has echo cancellation.
    /// Detection works when a process mute is used, but not with hardware mute.
    #[doc(alias = "kAudioDevicePropertyVoiceActivityDetectionEnable")]
    pub const VOICE_ACTIVITY_DETECTION_ENABLE: Self = Self(u32::from_be_bytes(*b"vAd+"));

    /// A read-only u32 where 0 indicates no voice currently detected and 1 indicates voice.
    ///
    /// Used in conjunction with VOICE_ACTIVITY_DETECTION_ENABLE.
    /// A client would normally register to listen to this property for changes and then query
    /// the state rather than continuously poll the value.
    /// NOTE: If input audio is not active/runnning or the voice activity detection is disabled,
    /// then it is not analyzed and this will provide 0.
    #[doc(alias = "kAudioDevicePropertyVoiceActivityDetectionState")]
    pub const VOICE_ACTIVITY_DETECTION_STATE: Self = Self(u32::from_be_bytes(*b"vAdS"));
}

/// AudioObjectPropertySelector values provided by the Tap Object class.
/// The Tap class is a subclass of the AudioObject class. the class
/// has just the global scope, kAudioObjectPropertyScopeGlobal, and only a master element.
impl core_audio::AudioObjPropSelector {
    /// A cf::String that contains a persistent identifier for the Tap. A Taps UID
    /// persists until the tap is destroyed. The caller is responsible for releasing
    /// the returned cf::Object.
    #[doc(alias = "kAudioTapPropertyUID")]
    pub const TAP_UID: Self = Self(u32::from_be_bytes(*b"tuid"));

    /// The CATapDescription used to initially create this tap. This property can be used
    /// to modify and set the description of an existing tap.
    #[doc(alias = "kAudioTapPropertyDescription")]
    pub const TAP_DESCRIPTION: Self = Self(u32::from_be_bytes(*b"tdsc"));

    /// An AudioStreamBasicDescription that describes the current data format for
    /// the tap. This is the format of that data that will be accessible in any aggregate
    /// device that contains the tap.
    #[doc(alias = "kAudioTapPropertyFormat")]
    pub const TAP_FORMAT: Self = Self(u32::from_be_bytes(*b"tfmt"));
}

#[doc(alias = "AudioDeviceIOProc")]
pub type AudioDeviceIoProc<const IN: usize = 1, const ON: usize = 1, T = std::ffi::c_void> =
    extern "C" fn(
        device: AudioObjId,
        now: &cat::AudioTimeStamp,
        input_data: &cat::AudioBufList<IN>,
        input_time: &cat::AudioTimeStamp,
        output_data: &mut cat::AudioBufList<ON>,
        output_time: &cat::AudioTimeStamp,
        cliend_data: Option<&mut T>,
    ) -> os::Status;

#[doc(alias = "AudioDeviceIOBlock")]
#[cfg(all(feature = "blocks", feature = "dispatch"))]
pub type AudioDeviceIoBlock<const IN: usize = 1, const ON: usize = 1> = blocks::EscBlock<
    fn(
        now: &cat::AudioTimeStamp,
        input_data: &cat::AudioBufList<IN>,
        input_time: &cat::AudioTimeStamp,
        output_data: &mut cat::AudioBufList<ON>,
        output_time: &cat::AudioTimeStamp,
    ),
>;

pub type AudioDeviceIoProcId = AudioDeviceIoProc;

#[repr(transparent)]
pub struct AudioAggregateDevice(core_audio::AudioObjId);

impl std::ops::Deref for AudioAggregateDevice {
    type Target = core_audio::AudioObjId;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl std::ops::DerefMut for AudioAggregateDevice {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

pub mod aggregate_device_keys {
    use crate::cf;

    /// The key used in a CFDictionary that describes the composition of an
    /// AudioAggregateDevice. The value for this key is a CFString that contains the UID
    /// of the AudioAggregateDevice.
    #[doc(alias = "kAudioAggregateDeviceUIDKey")]
    pub fn device_uid() -> &'static cf::String {
        cf::str!(c"uid")
    }

    /// The key used in a CFDictionary that describes the composition of an
    /// AudioAggregateDevice. The value for this key is a CFString that contains the
    /// human readable name of the AudioAggregateDevice.
    #[doc(alias = "kAudioAggregateDeviceNameKey")]
    pub fn device_name() -> &'static cf::String {
        cf::str!(c"name")
    }

    /// The key used in a CFDictionary that describes the composition of an
    /// AudioAggregateDevice. The value for this key is a CFArray of CFDictionaries that
    /// describe each sub-device in the AudioAggregateDevice. The keys for this
    /// CFDictionary are defined in the AudioSubDevice section.
    #[doc(alias = "kAudioAggregateDeviceSubDeviceListKey")]
    pub fn sub_device_list() -> &'static cf::String {
        cf::str!(c"subdevices")
    }

    /// The key used in a CFDictionary that describes the composition of an
    /// AudioAggregateDevice. The value for this key is a CFString that contains the
    /// UID for the sub-device that is the time source for the
    /// AudioAggregateDevice.
    #[doc(alias = "kAudioAggregateDeviceMainSubDeviceKey")]
    pub fn main_sub_device() -> &'static cf::String {
        cf::str!(c"master")
    }

    /// The key used in a CFDictionary that describes the composition of an
    /// AudioAggregateDevice. The value for this key is a CFString that contains the
    /// UID for the clock device that is the time source for the
    /// AudioAggregateDevice. If the aggregate device includes both a main audio
    /// device and a clock device, the clock device will control the time base.
    #[doc(alias = "kAudioAggregateDeviceClockDeviceKey")]
    pub fn clock_device() -> &'static cf::String {
        cf::str!(c"clock")
    }

    /// The key used in a CFDictionary that describes the composition of an
    /// AudioAggregateDevice. The value for this key is a CFNumber where a value of 0
    /// means that the AudioAggregateDevice is to be published to the entire system and
    /// a value of 1 means that the AudioAggregateDevice is private to the process that
    /// created it. Note that a private AudioAggregateDevice is not persistent across
    /// launches of the process that created it. Note that if this key is not present,
    /// it implies that the AudioAggregateDevice is published to the entire system.
    #[doc(alias = "kAudioAggregateDeviceIsPrivateKey")]
    pub fn is_private() -> &'static cf::String {
        cf::str!(c"private")
    }

    /// The key used in a CFDictionary that describes the composition of an
    /// AudioAggregateDevice. The value for this key is a CFNumber where a value of 0
    /// means that the sub-devices of the AudioAggregateDevice are arranged such that
    /// the output streams are all fed the same data.
    #[doc(alias = "kAudioAggregateDeviceIsStackedKey")]
    pub fn is_stacked() -> &'static cf::String {
        cf::str!(c"stacked")
    }

    /// The key used in a CFDictionary that describes the Tap composition of an
    /// AudioAggregateDevice.  The value for this key is a CFArray of CFDictionaries
    /// that describe each tap in the AudioAggregateDevice.  The keys for this
    /// CFDictionary are defined in the AudioTap section.
    #[doc(alias = "kAudioAggregateDeviceTapListKey")]
    pub fn tap_list() -> &'static cf::String {
        cf::str!(c"taps")
    }

    /// The key used in a CFDictionary that describes the composition of an
    /// AudioAggregateDevice. The value for this key is a CFNumber where a non-zero  
    /// value indicates that this aggregate device’s start should wait for the first
    /// tap that receives audio. When this key is used, calling AudioDeviceStart with
    /// the aggregate device will wait until a tapped process begins receiving its  
    /// first audio from any tapped applications. The composition must also include
    /// the private key so that the aggregate is private to the process that created
    /// it.
    #[doc(alias = "kAudioAggregateDeviceTapAutoStartKey")]
    pub fn tap_auto_start() -> &'static cf::String {
        cf::str!(c"tapautostart")
    }
}

impl AudioAggregateDevice {
    pub fn with_desc(desc: &cf::DictionaryOf<cf::String, cf::Type>) -> os::Result<Self> {
        let mut res = std::mem::MaybeUninit::uninit();
        unsafe {
            AudioHardwareCreateAggregateDevice(desc, res.as_mut_ptr()).result()?;
            Ok(Self(res.assume_init()))
        }
    }
}

impl Drop for AudioAggregateDevice {
    fn drop(&mut self) {
        unsafe {
            let mut id = AudioObjId::UNKNOWN;
            std::mem::swap(&mut self.0, &mut id);
            let res = AudioHardwareDestroyAggregateDevice(id).result();
            debug_assert!(res.is_ok());
        }
    }
}

#[link(name = "CoreAudio", kind = "framework")]
extern "C-unwind" {

    fn AudioObjectShow(objectId: AudioObjId);

    fn AudioObjectHasProperty(objectId: AudioObjId, address: *const AudioObjPropAddr) -> bool;

    fn AudioObjectIsPropertySettable(
        objectId: AudioObjId,
        address: *const AudioObjPropAddr,
        out_is_settable: *mut bool,
    ) -> os::Status;

    fn AudioObjectGetPropertyData(
        objectId: AudioObjId,
        address: *const AudioObjPropAddr,
        qualifier_data_size: u32,
        qualifier_data: *const c_void,
        data_size: *mut u32,
        data: *mut c_void,
    ) -> os::Status;

    fn AudioObjectSetPropertyData(
        objectId: AudioObjId,
        address: *const AudioObjPropAddr,
        qualifier_data_size: u32,
        qualifier_data: *const c_void,
        data_size: u32,
        data: *const c_void,
    ) -> os::Status;

    fn AudioObjectGetPropertyDataSize(
        objectId: AudioObjId,
        address: *const AudioObjPropAddr,
        qualifier_data_size: u32,
        qualifier_data: *const c_void,
        data_size: *mut u32,
    ) -> os::Status;

    fn AudioObjectAddPropertyListener(
        objectId: AudioObjId,
        address: *const AudioObjPropAddr,
        listener: AudioObjPropListenerFn,
        client_data: *mut c_void,
    ) -> os::Status;

    fn AudioObjectRemovePropertyListener(
        objectId: AudioObjId,
        address: *const AudioObjPropAddr,
        listener: AudioObjPropListenerFn,
        client_data: *mut c_void,
    ) -> os::Status;

    #[cfg(all(feature = "blocks", feature = "dispatch"))]
    fn AudioObjectAddPropertyListenerBlock(
        objectId: AudioObjId,
        address: *const AudioObjPropAddr,
        dispatch_queue: Option<&dispatch::Queue>,
        listener: *mut AudioObjPropListenerBlock,
    ) -> os::Status;

    #[cfg(all(feature = "blocks", feature = "dispatch"))]
    fn AudioObjectRemovePropertyListenerBlock(
        objectId: AudioObjId,
        address: *const AudioObjPropAddr,
        dispatch_queue: Option<&dispatch::Queue>,
        listener: *mut AudioObjPropListenerBlock,
    ) -> os::Status;

    fn AudioDeviceCreateIOProcID(
        device: AudioObjId,
        proc: AudioDeviceIoProc,
        client_data: *mut std::ffi::c_void,
        out_proc_id: *mut Option<AudioDeviceIoProcId>,
    ) -> os::Status;

    fn AudioHardwareCreateAggregateDevice(
        desc: &cf::DictionaryOf<cf::String, cf::Type>,
        out_device_id: *mut AudioObjId,
    ) -> os::Status;

    fn AudioHardwareDestroyAggregateDevice(device_id: AudioObjId) -> os::Status;

}

#[cfg(test)]
mod tests {
    use crate::{
        arc, cat, cf,
        core_audio::{
            aggregate_device_keys as agg_keys, AudioObjId, AudioObjPropAddr, AudioObjPropElement,
            AudioObjPropScope, AudioObjPropSelector, ProcessObj, TapDesc,
        },
        ns, os,
    };

    use super::AudioAggregateDevice;

    #[test]
    fn list_devices() {
        let addr = AudioObjPropAddr {
            selector: AudioObjPropSelector::HARDWARE_DEFAULT_INPUT_DEVICE,
            scope: AudioObjPropScope::GLOBAL,
            element: AudioObjPropElement::MAIN,
        };
        let _device_id: AudioObjId = AudioObjId::SYS_OBJECT.prop(&addr).unwrap();

        let addr = AudioObjPropAddr {
            selector: AudioObjPropSelector::HARDWARE_DEVICES,
            scope: AudioObjPropScope::INPUT,
            element: AudioObjPropElement::MAIN,
        };
        let devices: Vec<AudioObjId> = AudioObjId::SYS_OBJECT.prop_vec(&addr).unwrap();

        assert!(!devices.is_empty());

        let name_addr = AudioObjPropAddr {
            selector: AudioObjPropSelector::NAME,
            scope: AudioObjPropScope::GLOBAL,
            element: AudioObjPropElement::MAIN,
        };
        let man_addr = AudioObjPropAddr {
            selector: AudioObjPropSelector::MANUFACTURER,
            scope: AudioObjPropScope::GLOBAL,
            element: AudioObjPropElement::MAIN,
        };
        for d in devices {
            let _val: arc::R<cf::String> = d.cf_prop(&name_addr).unwrap();
            let _val: arc::R<cf::String> = d.cf_prop(&man_addr).unwrap();
        }
    }

    #[test]
    fn proc_io() {
        let desc = TapDesc::with_stereo_global_tap_excluding_processes(&ns::Array::new());
        let tap = desc.create_process_tap().unwrap();

        // extern "C" fn proc(
        //     device: AudioObjId,
        //     now: &cat::AudioTimeStamp,
        //     input_data: &cat::AudioBufList<1>,
        //     input_time: &cat::AudioTimeStamp,
        //     output_data: &mut cat::AudioBufList<1>,
        //     output_time: &cat::AudioTimeStamp,
        //     cliend_data: Option<&mut std::ffi::c_void>,
        // ) -> os::Status {
        //     os::Status::NO_ERR
        // }

        // tap.create_io_proc_id(proc, None).unwrap();
    }

    #[test]
    fn aggregate_device() {
        let output_device = AudioObjId::hardware_default_output_device().unwrap();
        let output_uid = output_device.device_uid().unwrap();
        let uuid = cf::Uuid::new().to_cf_string();
        let dict = cf::DictionaryOf::with_keys_values(
            &[
                agg_keys::is_private(),
                agg_keys::is_stacked(),
                agg_keys::tap_auto_start(),
                agg_keys::device_name(),
                agg_keys::main_sub_device(),
                agg_keys::device_uid(),
            ],
            &[
                cf::Boolean::value_true().as_type_ref(),
                cf::Boolean::value_false(),
                cf::Boolean::value_true(),
                cf::str!(c"Tap"),
                &output_uid,
                &uuid,
            ],
        );
        let agg_device = AudioAggregateDevice::with_desc(&dict).unwrap();
    }

    #[test]
    fn translate_pid() {
        let obj = ProcessObj::with_pid(0).unwrap();
        assert_eq!(obj.0, AudioObjId::UNKNOWN);

        let _objs = ProcessObj::list().unwrap();

        let pid = ns::ProcessInfo::current().process_id();
        let obj = ProcessObj::with_pid(pid).unwrap();
        assert_ne!(obj.0, AudioObjId::UNKNOWN);
        let process_id = obj.pid().unwrap();
        assert_eq!(pid, process_id);
        println!("{:?}", obj.bundle_id());

        assert!(!obj.is_running().unwrap());

        let _devices = obj.devices().unwrap();
    }
}
