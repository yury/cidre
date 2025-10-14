use crate::{arc, cat, cf, cm, define_cf_type, define_opts, mt, os};

use std::ptr::NonNull;

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
    tap: &mut mt::AudioProcessingTap,
    client_info: *mut T,
    tap_storage_out: NonNull<*mut std::ffi::c_void>,
);

#[doc(alias = "MTAudioProcessingTapFinalizeCallback")]
pub type FinalizeCb = extern "C-unwind" fn(tap: &mut mt::AudioProcessingTap);

#[doc(alias = "MTAudioProcessingTapPrepareCallback")]
pub type PrepareCb = extern "C-unwind" fn(
    tap: &mut mt::AudioProcessingTap,
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
pub type UnprepareCb = extern "C-unwind" fn(tap: &mut mt::AudioProcessingTap);

/// A function called when an audio track has data to be processed by its tap.
///
/// A processing callback is invoked when the audio track has data that can be processed by a
/// given tap.
///
/// The processing callback will be called when there is sufficient input data to provide for
/// processing. The callback should then go and request as much source data as it needs in order
/// to produce the requested number of processed samples. When the callback requests source data,
/// it may receive less data than it requests.
///
/// The tap must provide the same number of samples that are being requested. Under normal circumstances,
/// the source data it requests should be satisfied (as the client running the audio queue is also
/// providing the queue with the audio source material). If there is insufficient source data available
/// (this is indicated by the numberFramesOut from the GetSourceAudio call), then the processing tap
/// should cope as best as it can; it can either return less data than was requested, insert silence,
/// insert noise, etc.
/// If less data is returned than requested, the remainder will be filled with silence.
///
/// A processing tap is a real-time operation, so the general Core Audio limitations for real-time
/// processing apply.  For example, care should be taken not to allocate memory or call into
/// blocking system calls, as this will interfere with the real-time nature of audio playback.
///
/// Under normal operation, the source data will be continuous from the last time the callback was
/// called, and the processed samples should be continuous from the previous samples returned. If
/// there is any discontinuity between the last samples provided for processing, the audio queue will
/// set the kMTAudioProcessingTapFlag_StartOfStream bit in the flags. After a discontinuity, the
/// first sample that the processing tap outputs should correspond to the first sample that was
/// provided in the source samples (so a reset + consequent process serves to re-anchor a
/// relationship between the processing tap's source and processed samples). In this case, the
/// processing tap will typically discard any previous state (for example, if a processing tap was
/// adding a reverb to a signal, then the discontinuity flag would act the same as AudioUnitReset;
/// any previous source information in the processing tap should be discarded).
///
/// The caller is responsible for absorbing any processing delays. For example, if the
/// processing is to be done by an audio unit that reports a processing latency, then the caller
/// should remove those latency samples from the audio unit's rendering and not return them to
/// the tap.
///
/// The processing tap may operate on the provided source data in place ("in-place processing")
/// and return pointers to that buffer, rather than its own. This is similar to audio
/// unit render operations. The processing tap will be provided with a bufferList on input
/// where the mData pointers are NULL.
///
/// When the output audio is stopped asynchronously, the processing tap will see the
/// kMTAudioProcessingTapFlag_EndOfStream bit set on return from GetSourceAudio, and is responsible
/// for propagating this bit from the callback when its processing has reached this point.
///
/// A processing tap will NEVER see the same source data again, so, it should keep its own copy,
/// if it needs to keep it for further reference past the duration of this call. It also cannot
/// assume that the pointers to the source data that it retrieves will remain valid AFTER the
/// processing tap has executed.
///
/// Should the processing tap provide custom buffers in bufferListInOut, it should ensure that the
/// data pointers remain valid until the tap is executed again.
#[doc(alias = "MTAudioProcessingTapProcessCallback")]
pub type ProcessCb<const N: usize = 1> = extern "C-unwind" fn(
    tap: &mut mt::AudioProcessingTap,
    frames_n: cm::ItemCount,
    flags: mt::AudioProcessingTapFlags,
    buf_list_in_out: &mut cat::AudioBufList<N>,
    frames_n_out: &mut cm::ItemCount,
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
#[repr(C, packed(4))]
pub struct Cbs<T, const N: usize = 1> {
    pub version: CbsVersion,
    pub client_info: *mut T,
    pub init: Option<InitCb<T>>,
    pub finalize: Option<FinalizeCb>,
    pub prepare: Option<PrepareCb>,
    pub unprepare: Option<UnprepareCb>,
    pub process: ProcessCb,
}

impl<T, const N: usize> Default for Cbs<T, N> {
    fn default() -> Self {
        Self {
            version: CbsVersion::V0,
            client_info: std::ptr::null_mut(),
            init: Default::default(),
            finalize: Default::default(),
            prepare: Default::default(),
            unprepare: Default::default(),
            process: default_process,
        }
    }
}

extern "C-unwind" fn default_process(
    tap: &mut mt::AudioProcessingTap,
    frames_n: cm::ItemCount,
    _flags: mt::AudioProcessingTapFlags,
    buf_list_in_out: &mut cat::AudioBufList,
    frames_n_out: &mut cm::ItemCount,
    flags_out: &mut mt::AudioProcessingTapFlags,
) {
    tap.src_audio_unchecked(
        frames_n,
        buf_list_in_out,
        flags_out,
        std::ptr::null_mut(),
        frames_n_out,
    );
}

#[cfg(not(target_os = "watchos"))]
impl Tap {
    #[inline]
    pub fn type_id() -> cf::TypeId {
        unsafe { MTAudioProcessingTapGetTypeID() }
    }

    #[doc(alias = "MTAudioProcessingTapCreate")]
    pub fn with_callbacks_in<T, const N: usize>(
        callbacks: &Cbs<T, N>,
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
    pub fn with_callbacks<T, const N: usize>(
        callbacks: &Cbs<T, N>,
        flags: CreationFlags,
    ) -> os::Result<arc::R<Tap>> {
        unsafe { std::mem::transmute(Self::with_callbacks_in(callbacks, flags, None)) }
    }

    #[doc(alias = "MTAudioProcessingTapGetSourceAudio")]
    #[inline]
    pub fn src_audio<const N: usize>(
        &mut self,
        frames_n: cm::ItemCount,
        buf_list_in_out: &mut cat::AudioBufList<N>,
        flags_out: *mut Flags,
        time_range_out: *mut cm::TimeRange,
        frames_n_out: *mut cm::ItemCount,
    ) -> os::Result {
        self.src_audio_unchecked(
            frames_n,
            buf_list_in_out,
            flags_out,
            time_range_out,
            frames_n_out,
        )
        .result()
    }

    #[doc(alias = "MTAudioProcessingTapGetSourceAudio")]
    #[inline]
    pub fn src_audio_unchecked<const N: usize>(
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

    #[doc(alias = "MTAudioProcessingTapGetStorage")]
    #[inline]
    pub fn storage(&self) -> *const std::ffi::c_void {
        unsafe { MTAudioProcessingTapGetStorage(self) }
    }

    #[doc(alias = "MTAudioProcessingTapGetStorage")]
    #[inline]
    pub fn storage_mut(&mut self) -> *mut std::ffi::c_void {
        unsafe { MTAudioProcessingTapGetStorage(self) }
    }
}

#[cfg(not(target_os = "watchos"))]
#[link(name = "MediaToolbox", kind = "framework")]
unsafe extern "C-unwind" {
    fn MTAudioProcessingTapGetTypeID() -> cf::TypeId;

    fn MTAudioProcessingTapCreate(
        allocator: Option<&cf::Allocator>,
        callbacks: *const Cbs<std::ffi::c_void>,
        flags: CreationFlags,
        tap_out: *mut Option<arc::R<Tap>>,
    ) -> os::Status;

    fn MTAudioProcessingTapGetSourceAudio(
        tap: &mut Tap,
        frames_n: cm::ItemCount,
        buf_list_in_out: &mut cat::AudioBufList,
        flags: *mut Flags,
        time_range_out: *mut cm::TimeRange,
        frames_n_out: *mut cm::ItemCount,
    ) -> os::Status;

    fn MTAudioProcessingTapGetStorage(tap: &Tap) -> *mut std::ffi::c_void;
}

#[cfg(test)]
mod tests {
    use crate::mt;

    #[test]
    fn basics() {
        let _type_id = mt::AudioProcessingTap::type_id();
    }
}
