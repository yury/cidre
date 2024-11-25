use crate::{
    arc, cat, cf,
    core_audio::{AudioObjId, TapDesc},
    os,
};

use super::{AudioObjPropAddr, AudioObjPropElement, AudioObjPropScope, AudioObjPropSelector};

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

impl std::ops::DerefMut for Tap {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl Tap {
    pub fn uid(&self) -> os::Result<arc::R<cf::String>> {
        self.cf_prop(&AudioObjPropAddr {
            selector: AudioObjPropSelector::TAP_UID,
            scope: AudioObjPropScope::GLOBAL,
            element: AudioObjPropElement::MAIN,
        })
    }

    pub fn desc(&self) -> os::Result<arc::R<TapDesc>> {
        self.cf_prop(&AudioObjPropAddr {
            selector: AudioObjPropSelector::TAP_DESCRIPTION,
            scope: AudioObjPropScope::GLOBAL,
            element: AudioObjPropElement::MAIN,
        })
    }

    pub fn asbd(&self) -> os::Result<cat::AudioBasicStreamDesc> {
        self.prop(&AudioObjPropAddr {
            selector: AudioObjPropSelector::TAP_FORMAT,
            scope: AudioObjPropScope::GLOBAL,
            element: AudioObjPropElement(0),
        })
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
    pub use crate::{core_audio as ca, ns};

    #[test]
    fn basics() {
        let desc = {
            let tap_desc =
                ca::TapDesc::with_stereo_global_tap_excluding_processes(&ns::Array::new());
            println!("{tap_desc:?}");
            let tap = tap_desc.create_process_tap().unwrap();
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
