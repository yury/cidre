use crate::swift::{SwiftMetadata, SwiftType, abi};

use super::{DictationTranscriber, SpeechDetector, SpeechTranscriber};

/// An owned Swift `any SpeechModule` class existential.
///
/// A class-constrained Swift existential is represented by its retained object
/// pointer and protocol witness table.
#[repr(C)]
pub struct SpeechModule {
    object: *mut (),
    witness: *const (),
}

unsafe impl Send for SpeechModule {}
unsafe impl Sync for SpeechModule {}

#[link(name = "Speech", kind = "framework")]
unsafe extern "C" {
    #[link_name = "$s6Speech0A8DetectorCAA0A6ModuleAAWP"]
    static SPEECH_DETECTOR_MODULE_WITNESS: u8;

    #[link_name = "$s6Speech0A11TranscriberCAA0A6ModuleAAWP"]
    static SPEECH_TRANSCRIBER_MODULE_WITNESS: u8;

    #[link_name = "$s6Speech20DictationTranscriberCAA0A6ModuleAAWP"]
    static DICTATION_TRANSCRIBER_MODULE_WITNESS: u8;
}

impl SpeechModule {
    unsafe fn from_object<T>(object: &T, witness: *const ()) -> Self {
        let object = unsafe { abi::object_retain((object as *const T).cast()) }.cast_mut();
        Self { object, witness }
    }

    #[inline]
    pub fn as_raw(&self) -> (*mut (), *const ()) {
        (self.object, self.witness)
    }
}

impl From<&SpeechDetector> for SpeechModule {
    #[inline]
    fn from(value: &SpeechDetector) -> Self {
        unsafe { Self::from_object(value, (&raw const SPEECH_DETECTOR_MODULE_WITNESS).cast()) }
    }
}

impl From<&SpeechTranscriber> for SpeechModule {
    #[inline]
    fn from(value: &SpeechTranscriber) -> Self {
        unsafe { Self::from_object(value, (&raw const SPEECH_TRANSCRIBER_MODULE_WITNESS).cast()) }
    }
}

impl From<&DictationTranscriber> for SpeechModule {
    #[inline]
    fn from(value: &DictationTranscriber) -> Self {
        unsafe {
            Self::from_object(
                value,
                (&raw const DICTATION_TRANSCRIBER_MODULE_WITNESS).cast(),
            )
        }
    }
}

unsafe impl SwiftMetadata for SpeechModule {
    #[inline]
    fn metadata() -> *const abi::TypeMetadata {
        static CACHE: abi::MetadataCache = abi::MetadataCache::new();
        CACHE.get(|| unsafe { abi::type_by_mangled_name("6Speech0A6Module_p") })
    }
}

unsafe impl SwiftType for SpeechModule {}

impl Clone for SpeechModule {
    #[inline]
    fn clone(&self) -> Self {
        let object = unsafe { abi::object_retain(self.object.cast_const()) }.cast_mut();
        Self {
            object,
            witness: self.witness,
        }
    }
}

impl Drop for SpeechModule {
    #[inline]
    fn drop(&mut self) {
        unsafe { abi::object_release(self.object.cast_const()) }
    }
}

impl core::fmt::Debug for SpeechModule {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("SpeechModule")
            .field("object", &self.object)
            .field("witness", &self.witness)
            .finish()
    }
}
