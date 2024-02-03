use std::ffi::c_void;

use crate::{core_audio, os};

use super::{AudioObjectId, AudioObjectPropertyAddress};

impl core_audio::AudioObjectId {
    #[doc(alias = "kAudioObjectSystemObject")]
    pub const SYSTEM_OBJECT: Self = Self(1);

    #[doc(alias = "AudioObjectSetPropertyData")]
    pub fn set_prop<T: Sized>(
        &self,
        address: &AudioObjectPropertyAddress,
        val: T,
    ) -> Result<(), os::Status> {
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
}

impl core_audio::AudioObjectPropertySelector {
    #[doc(alias = "kAudioHardwarePropertyProcessInputMute")]
    pub const HARDWARE_PROCESS_INPUT_MUTE: Self = Self(u32::from_be_bytes(*b"pmin"));
}

// CF_ENUM(AudioObjectPropertySelector)
// {
//     kAudioHardwarePropertyDevices                               = 'dev#',
//     kAudioHardwarePropertyDefaultInputDevice                    = 'dIn ',
//     kAudioHardwarePropertyDefaultOutputDevice                   = 'dOut',
//     kAudioHardwarePropertyDefaultSystemOutputDevice             = 'sOut',
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
    fn AudioObjectSetPropertyData(
        objectID: AudioObjectId,
        address: &AudioObjectPropertyAddress,
        qualifier_data_size: u32,
        qualifier_data: *const c_void,
        data_size: u32,
        data: *const c_void,
    ) -> os::Status;

}
