pub mod audio_processing_tap;

pub use audio_processing_tap::Cbs as AudioProcessingTapCbs;
pub use audio_processing_tap::CbsVersion as AudioProcessingTapCbsVersion;
pub use audio_processing_tap::CreationFlags as AudioProcessingTapCreationFlags;
pub use audio_processing_tap::FinalizeCb as AudioProcessingTapFinalizeCb;
pub use audio_processing_tap::Flags as AudioProcessingTapFlags;
pub use audio_processing_tap::InitCb as AudioProcessingTapInitCb;
pub use audio_processing_tap::PrepareCb as AudioProcessingTapPrepareCb;
pub use audio_processing_tap::ProcessCb as AudioProcessingTapProcessCb;
pub use audio_processing_tap::Tap as AudioProcessingTap;
pub use audio_processing_tap::UnprepareCb as AudioProcessingTapUnprepareCb;

pub mod format_names;

/// Allows the client to use media format readers appropriate for professional video workflows.
#[doc(alias = "MTRegisterProfessionalVideoWorkflowFormatReaders")]
#[cfg(target_os = "macos")]
pub fn register_professional_video_workflow_format_readers() {
    unsafe { MTRegisterProfessionalVideoWorkflowFormatReaders() }
}

#[cfg(target_os = "macos")]
#[link(name = "MediaToolkit", kind = "framework")]
unsafe extern "C" {
    fn MTRegisterProfessionalVideoWorkflowFormatReaders();
}
