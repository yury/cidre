use std::ffi::c_void;

use crate::{arc, core_audio, os};

use super::{AudioObjId, AudioObjPropAddr};

impl core_audio::AudioObjId {
    #[doc(alias = "kAudioObjectSystemObject")]
    pub const SYS_OBJECT: Self = Self(1);

    #[doc(alias = "AudioObjectSetPropertyData")]
    pub fn set_prop<T: Sized>(&self, address: &AudioObjPropAddr, val: T) -> os::Result {
        let data_size = std::mem::size_of_val(&val) as u32;
        unsafe {
            AudioObjectSetPropertyData(
                *self,
                address,
                0,
                std::ptr::null(),
                data_size,
                &val as *const _ as _,
            )
            .result()
        }
    }

    #[doc(alias = "AudioObjectGetPropertyData")]
    pub fn prop<T: Sized>(&self, address: &AudioObjPropAddr) -> os::Result<T> {
        let mut data_size = std::mem::size_of::<T>() as u32;
        let mut val = std::mem::MaybeUninit::<T>::uninit();
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
        let mut data_size = 0;
        unsafe {
            AudioObjectGetPropertyDataSize(*self, address, 0, std::ptr::null(), &mut data_size)
                .result()?;
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

    pub fn show(&self) {
        unsafe { AudioObjectShow(*self) }
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
}

// CF_ENUM(AudioObjectPropertySelector)
// {
//     kAudioHardwarePropertyTranslateUIDToDevice                  = 'uidd',
//     kAudioHardwarePropertyMixStereoToMono                       = 'stmo',
//     kAudioHardwarePropertyPlugInList                            = 'plg#',
//     kAudioHardwarePropertyTranslateBundleIDToPlugIn             = 'bidp',
//     kAudioHardwarePropertyTransportManagerList                  = 'tmg#',
//     kAudioHardwarePropertyTranslateBundleIDToTransportManager   = 'tmbi',
//     kAudioHardwarePropertyBoxList                               = 'box#',
//     kAudioHardwarePropertyTranslateUIDToBox                     = 'uidb',
//     kAudioHardwarePropertyClockDeviceList                       = 'clk#',
//     kAudioHardwarePropertyTranslateUIDToClockDevice             = 'uidc',
//     kAudioHardwarePropertyProcessIsMain							= 'main',
//     kAudioHardwarePropertyIsInitingOrExiting                    = 'inot',
//     kAudioHardwarePropertyUserIDChanged                         = 'euid',
//     kAudioHardwarePropertyProcessIsAudible                      = 'pmut',
//     kAudioHardwarePropertySleepingIsAllowed                     = 'slep',
//     kAudioHardwarePropertyUnloadingIsAllowed                    = 'unld',
//     kAudioHardwarePropertyHogModeIsAllowed                      = 'hogr',
//     kAudioHardwarePropertyUserSessionIsActiveOrHeadless         = 'user',
//     kAudioHardwarePropertyServiceRestarted                      = 'srst',
//     kAudioHardwarePropertyPowerHint                             = 'powh',
//     kAudioHardwarePropertyProcessObjectList                     = 'prs#',
//     kAudioHardwarePropertyTranslatePIDToProcessObject           = 'id2p',
//     kAudioHardwarePropertyTapList                               = 'tps#',
//     kAudioHardwarePropertyTranslateUIDToTap                     = 'uidt',
// };

#[link(name = "CoreAudio", kind = "framework")]
extern "C" {

    fn AudioObjectShow(objectId: AudioObjId);

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
        address: &AudioObjPropAddr,
        qualifier_data_size: u32,
        qualifier_data: *const c_void,
        data_size: u32,
        data: *const c_void,
    ) -> os::Status;

    fn AudioObjectGetPropertyDataSize(
        objectId: AudioObjId,
        address: &AudioObjPropAddr,
        qualifier_data_size: u32,
        qualifier_data: *const c_void,
        data_size: *mut u32,
    ) -> os::Status;

}

#[cfg(test)]
mod tests {
    use crate::{
        arc, cf,
        core_audio::{
            AudioObjId, AudioObjPropAddr, AudioObjPropElement, AudioObjPropScope,
            AudioObjPropSelector,
        },
    };

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
}
