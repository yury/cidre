#[cfg(target_os = "macos")]
use crate::{arc, cf, cm, os};

#[cfg(feature = "core_audio")]
use crate::core_audio;

#[cfg(target_os = "macos")]
impl cm::Clock {
    /// Unified api with iOS
    pub fn with_default_audio_output() -> os::Result<arc::R<Self>> {
        Self::with_audio_device_uid(None)
    }

    #[doc(alias = "CMAudioDeviceClockCreate")]
    pub fn with_audio_device_uid_in(
        device_uid: Option<&cf::String>,
        allocator: Option<&cf::Allocator>,
    ) -> os::Result<Option<arc::R<Self>>> {
        let mut res = None;
        unsafe {
            CMAudioDeviceClockCreate(allocator, device_uid, &mut res).result()?;
        }

        Ok(res)
    }

    #[doc(alias = "CMAudioDeviceClockCreate")]
    pub fn with_audio_device_uid(device_uid: Option<&cf::String>) -> os::Result<arc::R<Self>> {
        let mut res = None;
        unsafe {
            CMAudioDeviceClockCreate(None, device_uid, &mut res).result()?;
        }

        Ok(unsafe { std::mem::transmute(res) })
    }

    #[cfg(feature = "core_audio")]
    #[doc(alias = "CMAudioDeviceClockCreateFromAudioDeviceID")]
    pub fn with_audio_device_in(
        audio_device: core_audio::Device,
        allocator: Option<&cf::Allocator>,
    ) -> os::Result<Option<arc::R<Self>>> {
        let mut res = None;
        unsafe {
            CMAudioDeviceClockCreateFromAudioDeviceID(allocator, audio_device, &mut res)
                .result()?;
        }
        Ok(res)
    }

    #[cfg(feature = "core_audio")]
    #[doc(alias = "CMAudioDeviceClockCreateFromAudioDeviceID")]
    pub fn with_audio_device(audio_device: core_audio::Device) -> os::Result<arc::R<Self>> {
        let mut res = None;
        unsafe {
            CMAudioDeviceClockCreateFromAudioDeviceID(None, audio_device, &mut res).result()?;
        }
        Ok(unsafe { std::mem::transmute(res) })
    }

    #[doc(alias = "CMAudioDeviceClockSetAudioDeviceUID")]
    pub fn set_audio_device_uid(&mut self, device_uid: Option<&cf::String>) -> os::Result {
        unsafe { CMAudioDeviceClockSetAudioDeviceUID(self, device_uid).result() }
    }

    #[cfg(feature = "core_audio")]
    #[doc(alias = "CMAudioDeviceClockSetAudioDeviceID")]
    pub fn set_audio_device(&mut self, audio_device: core_audio::Device) -> os::Result {
        unsafe { CMAudioDeviceClockSetAudioDeviceID(self, audio_device).result() }
    }

    #[cfg(feature = "core_audio")]
    pub fn audio_device_uid(&self) -> os::Result<Option<&cf::String>> {
        let mut device_uid_out = None;
        unsafe {
            CMAudioDeviceClockGetAudioDevice(
                self,
                &mut device_uid_out,
                std::ptr::null_mut(),
                std::ptr::null_mut(),
            )
            .result()?
        }
        Ok(device_uid_out)
    }

    #[cfg(feature = "core_audio")]
    pub fn audio_device(&self) -> os::Result<core_audio::Device> {
        let mut device_out = core_audio::Device(core_audio::Obj::UNKNOWN);
        unsafe {
            CMAudioDeviceClockGetAudioDevice(
                self,
                std::ptr::null_mut(),
                &mut device_out,
                std::ptr::null_mut(),
            )
            .result()?
        }
        Ok(device_out)
    }

    #[cfg(feature = "core_audio")]
    pub fn is_tracking_default_device(&self) -> os::Result<bool> {
        let mut tracking_default_device_out = false;
        unsafe {
            CMAudioDeviceClockGetAudioDevice(
                self,
                std::ptr::null_mut(),
                std::ptr::null_mut(),
                &mut tracking_default_device_out,
            )
            .result()?
        }
        Ok(tracking_default_device_out)
    }
}

#[cfg(target_os = "macos")]
unsafe extern "C" {
    fn CMAudioDeviceClockCreate(
        allocator: Option<&cf::Allocator>,
        device_uid: Option<&cf::String>,
        clock_out: *mut Option<arc::R<cm::Clock>>,
    ) -> os::Status;

    #[cfg(feature = "core_audio")]
    fn CMAudioDeviceClockCreateFromAudioDeviceID(
        allocator: Option<&cf::Allocator>,
        audio_device: core_audio::Device,
        clock_out: *mut Option<arc::R<cm::Clock>>,
    ) -> os::Status;

    fn CMAudioDeviceClockSetAudioDeviceUID(
        clock: &mut cm::Clock,
        device_uid: Option<&cf::String>,
    ) -> os::Status;

    #[cfg(feature = "core_audio")]
    fn CMAudioDeviceClockSetAudioDeviceID(
        clock: &mut cm::Clock,
        audio_device: core_audio::Device,
    ) -> os::Status;

    #[cfg(feature = "core_audio")]
    fn CMAudioDeviceClockGetAudioDevice(
        clock: &cm::Clock,
        device_uid_out: *mut Option<&cf::String>,
        device_out: *mut core_audio::Device,
        tracking_default_device_out: *mut bool,
    ) -> os::Status;
}

#[cfg(target_os = "macos")]
#[cfg(test)]
mod tests {
    use crate::{cm, core_audio};

    #[test]
    fn basics() {
        let clock = cm::Clock::with_audio_device_uid(None).unwrap();
        assert!(clock.audio_device_uid().unwrap().is_none());
        let device = clock.audio_device().unwrap();
        assert!(!device.is_unknown());

        assert_eq!(device, core_audio::System::default_output_device().unwrap());

        assert!(clock.is_tracking_default_device().unwrap());

        let _time = clock.time();

        let clock =
            cm::Clock::with_audio_device(core_audio::System::default_output_device().unwrap())
                .unwrap();
        assert!(clock.audio_device_uid().unwrap().is_none());
        let device = clock.audio_device().unwrap();
        assert!(!device.is_unknown());

        assert_eq!(device, core_audio::System::default_output_device().unwrap());

        assert!(!clock.is_tracking_default_device().unwrap());
        let _time = clock.time();
    }
}
