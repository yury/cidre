use crate::{
    core_audio::{AudioObjId, TapDesc},
    os,
};

#[repr(transparent)]
pub struct Tap(AudioObjId);

impl Drop for Tap {
    fn drop(&mut self) {
        let res = unsafe { AudioHardwareDestroyProcessTap(self.0) };
        debug_assert!(res.is_ok(), "Failed to destroy process tap");
    }
}

impl std::ops::Deref for Tap {
    type Target = AudioObjId;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl TapDesc {
    pub fn create_process_tap(&self) -> os::Result<Tap> {
        let mut res = std::mem::MaybeUninit::uninit();
        unsafe {
            AudioHardwareCreateProcessTap(self, res.as_mut_ptr()).result()?;
            Ok(Tap(res.assume_init()))
        }
    }
}

#[link(name = "CoreAudio", kind = "framework")]
extern "C-unwind" {
    pub fn AudioHardwareCreateProcessTap(desc: &TapDesc, out_tap_id: *mut AudioObjId)
        -> os::Status;
    pub fn AudioHardwareDestroyProcessTap(tap_id: AudioObjId) -> os::Status;
}

#[cfg(test)]
pub mod tests {
    pub use crate::core_audio as ca;
    use crate::ns;

    #[test]
    fn basics() {
        let tap_desc = ca::TapDesc::with_stereo_global_tap_excluding_processes(&ns::Array::new());
        let _tap = tap_desc.create_process_tap().unwrap();
    }
}
