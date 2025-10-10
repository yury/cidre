use crate::{arc, cat, cm, os};
use std::ptr::NonNull;

use crate::{cf, define_cf_type, define_opts};

#[cfg(not(target_os = "watchos"))]
define_cf_type!(
    #[doc(alias = "MTAudioProcessingTapRef")]
    Tap(cf::Type)
);

define_opts!(
    #[doc(alias = "MTAudioProcessingTapCreationFlags")]
    /// Flags used when creating audio processing taps.
    pub CreationFlags(u32)
);

impl CreationFlags {
    /// Signifies that the processing tap is inserted before any effects.
    #[doc(alias = "kMTAudioProcessingTapCreationFlag_PreEffects")]
    pub const PRE_EFFECTS: Self = Self(1 << 0);

    /// Signifies that the processing tap is inserted after any effects.
    #[doc(alias = "kMTAudioProcessingTapCreationFlag_PostEffects")]
    pub const POST_EFFECTS: Self = Self(1 << 1);
}

define_opts!(
    #[doc(alias = "MTAudioProcessingTapFlags")]
    pub Flags(u32)
);

impl Flags {
    /// Signifies that the source audio is the beginning of a continuous stream,
    /// i.e., following the beginning or resumption of playback.
    /// Returned from GetSourceAudio.
    #[doc(alias = "kMTAudioProcessingTapFlag_StartOfStream")]
    pub const START_OF_STREAM: Self = Self(1 << 8);

    /// Signifies that the source audio is past the end of stream. This happens when
    /// the audio queue is being stopped asynchronously and has finished playing
    /// all of its data. Returned from GetSourceAudio and should be propagated
    /// on return from the process callback.
    #[doc(alias = "kMTAudioProcessingTapFlag_EndOfStream")]
    pub const END_OF_STREAM: Self = Self(1 << 9);
}

#[doc(alias = "MTAudioProcessingTapInitCallback")]
pub type InitCb<T = std::ffi::c_void> = extern "C-unwind" fn(
    tap: &mut Tap,
    client_info: *mut T,
    tap_storage_out: *mut NonNull<std::ffi::c_void>,
);

#[doc(alias = "MTAudioProcessingTapFinalizeCallback")]
pub type FinalizeCb = extern "C-unwind" fn(tap: &mut Tap);

#[doc(alias = "MTAudioProcessingTapPrepareCallback")]
pub type PrepareCb = extern "C-unwind" fn(
    tap: &mut Tap,
    frames_max: cm::ItemCount,
    processing_format: &cat::AudioStreamBasicDesc,
);

/// Audio processing unpreparation callback.
///
/// The unpreparation callback is invoked when the underlying audio machinery stops calling the process callback.
///
/// Preparation/Unpreparation callbacks are always paired.
///
/// Process callbacks will only ever be called after the prepare callback returns, and before unprepare is called.
#[doc(alias = "MTAudioProcessingTapUnprepareCallback")]
pub type UnprepareCb = extern "C-unwind" fn(tap: &mut Tap);

#[doc(alias = "MTAudioProcessingTapProcessCallback")]
pub type ProcessCb<const N: usize = 1> = extern "C-unwind" fn(
    tap: &mut Tap,
    frames_n: cm::ItemCount,
    flags: Flags,
    buf_list_in_out: &mut cat::AudioBufList<N>,
    out_frames_n: &mut cm::ItemCount,
    flags_out: &mut Flags,
);

#[doc(alias = "MTAudioProcessingTapCallbacksVersion")]
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
#[repr(transparent)]
pub struct CbsVersion(pub std::ffi::c_int);

impl CbsVersion {
    #[doc(alias = "kMTAudioProcessingTapCallbacksVersion_0")]
    pub const V0: Self = Self(0);
}

#[doc(alias = "MTAudioProcessingTapCallbacks")]
#[derive(Debug)]
#[repr(C)]
pub struct Cbs<const N: usize, T> {
    pub version: CbsVersion,
    pub client_info: *mut T,
    pub init: Option<InitCb<T>>,
    pub finalize: Option<FinalizeCb>,
    pub prepare: Option<PrepareCb>,
    pub unprepare: Option<UnprepareCb>,
    pub process: ProcessCb,
}

#[cfg(not(target_os = "watchos"))]
impl Tap {
    #[inline]
    pub fn type_id() -> cf::TypeId {
        unsafe { MTAudioProcessingTapGetTypeID() }
    }

    #[doc(alias = "MTAudioProcessingTapCreate")]
    pub fn with_callbacks_in<const N: usize, T>(
        callbacks: &Cbs<N, T>,
        flags: CreationFlags,
        allocator: Option<&cf::Allocator>,
    ) -> os::Result<Option<arc::R<Tap>>> {
        unsafe {
            os::result_init(|t| {
                MTAudioProcessingTapCreate(allocator, std::mem::transmute(callbacks), flags, t)
            })
        }
    }

    #[doc(alias = "MTAudioProcessingTapCreate")]
    pub fn with_callbacks<const N: usize, T>(
        callbacks: &Cbs<N, T>,
        flags: CreationFlags,
    ) -> os::Result<arc::R<Tap>> {
        unsafe { std::mem::transmute(Self::with_callbacks_in(callbacks, flags, None)) }
    }

    #[doc(alias = "MTAudioProcessingTapGetSourceAudio")]
    pub fn src_audio<const N: usize>(
        &mut self,
        frames_n: cm::ItemCount,
        buf_list_in_out: &mut cat::AudioBufList<N>,
        flags: *mut Flags,
        time_range_out: *mut cm::TimeRange,
        frames_n_out: *mut cm::ItemCount,
    ) -> os::Status {
        unsafe {
            MTAudioProcessingTapGetSourceAudio(
                self,
                frames_n,
                std::mem::transmute(buf_list_in_out),
                flags,
                time_range_out,
                frames_n_out,
            )
        }
    }
}

#[cfg(not(target_os = "watchos"))]
#[link(name = "MediaToolbox", kind = "framework")]
unsafe extern "C-unwind" {
    fn MTAudioProcessingTapGetTypeID() -> cf::TypeId;

    fn MTAudioProcessingTapCreate(
        allocator: Option<&cf::Allocator>,
        callbacks: *const Cbs<1, std::ffi::c_void>,
        flags: CreationFlags,
        tap_out: *mut Option<arc::R<Tap>>,
    ) -> os::Status;

    fn MTAudioProcessingTapGetSourceAudio(
        tap: &mut Tap,
        frames_n: cm::ItemCount,
        buf_list_in_out: &mut cat::AudioBufList<1>,
        flags: *mut Flags,
        time_range_out: *mut cm::TimeRange,
        frames_n_out: *mut cm::ItemCount,
    ) -> os::Status;
}

#[cfg(test)]
mod tests {
    use crate::mt;

    #[test]
    fn basics() {
        let _type_id = mt::AudioProcessingTap::type_id();
    }
}
