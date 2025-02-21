use crate::{
    arc, cat, cf,
    core_audio::{Obj, PropSelector, TapDesc},
    os,
};

#[derive(Debug)]
#[repr(transparent)]
pub struct Tap(Obj);

pub struct TapGuard(Tap);

impl std::ops::Deref for TapGuard {
    type Target = Tap;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl std::ops::DerefMut for TapGuard {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl Drop for TapGuard {
    fn drop(&mut self) {
        let res = unsafe { AudioHardwareDestroyProcessTap(self.0.0) };
        debug_assert!(res.is_ok(), "Failed to destroy process tap");
    }
}

impl std::ops::Deref for Tap {
    type Target = Obj;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl std::ops::DerefMut for Tap {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl Tap {
    pub fn uid(&self) -> os::Result<arc::R<cf::String>> {
        self.cf_prop(&PropSelector::TAP_UID.global_addr())
    }

    pub fn desc(&self) -> os::Result<arc::R<TapDesc>> {
        self.cf_prop(&PropSelector::TAP_DESCRIPTION.global_addr())
    }

    pub fn set_desc(&mut self, val: arc::R<TapDesc>) -> os::Result {
        self.set_prop(&PropSelector::TAP_DESCRIPTION.global_addr(), &val)
    }

    /// An cat::AudioStreamBasicDesc that describes the current data format for
    /// the tap. This is the format of that data that will be accessible in any aggregate
    /// device that contains the tap.
    #[doc(alias = "kAudioTapPropertyFormat")]
    pub fn asbd(&self) -> os::Result<cat::AudioBasicStreamDesc> {
        self.prop(&PropSelector::TAP_FORMAT.global_addr())
    }
}

impl TapDesc {
    pub fn create_process_tap(&self) -> os::Result<TapGuard> {
        os::result_init(|res| unsafe { AudioHardwareCreateProcessTap(self, res) })
            .map(|tap| TapGuard(Tap(tap)))
    }
}

#[link(name = "CoreAudio", kind = "framework")]
unsafe extern "C-unwind" {
    pub fn AudioHardwareCreateProcessTap(desc: &TapDesc, out_tap_id: *mut Obj) -> os::Status;
    pub fn AudioHardwareDestroyProcessTap(tap_id: Obj) -> os::Status;
}

#[cfg(test)]
pub mod tests {
    pub use crate::{core_audio::TapDesc, ns};

    #[test]
    fn basics() {
        let desc = {
            let tap_desc = TapDesc::with_stereo_global_tap_excluding_processes(&ns::Array::new());
            println!("{tap_desc:?}");
            let mut tap = tap_desc.create_process_tap().unwrap();
            tap.set_desc(tap_desc).unwrap();
            let uid = tap.uid().unwrap();
            println!("{uid:?}");
            let asbd = tap.asbd();
            println!("{asbd:?}");
            let desc = tap.desc().unwrap();
            desc
        };
        println!("{desc:?}");
        println!("{}", desc.as_type_ref().retain_count());
    }
}
