pub mod animation;
pub use animation::LayerVideoGravity;

pub mod media_format;
pub use media_format::FileType;
pub use media_format::FileTypeProfile;
pub use media_format::MediaCharacteristic;
pub use media_format::MediaType;
pub use media_format::VideoRange;

pub mod capture;
pub use capture::AudioChannel as CaptureAudioChannel;
pub use capture::AuthorizationStatus;
pub use capture::AutoFocusSys as CaptureAutoFocusSys;
pub use capture::CenterStageControlMode as CaptureCenterStageControlMode;
pub use capture::ColorSpace as CaptureColorSpace;
pub use capture::Connection as CaptureConnection;
pub use capture::Control as CaptureControl;
pub use capture::DataDroppedReason as CaptureDataDroppedReason;
pub use capture::Device as CaptureDevice;
pub use capture::DeviceConfigurationLockGuard as CaptureDeviceConfigurationLockGuard;
pub use capture::DeviceFormat as CaptureDeviceFormat;
pub use capture::DeviceInput as CaptureDeviceInput;
pub use capture::DevicePos as CaptureDevicePos;
pub use capture::DeviceRotationCoordinator as CaptureDeviceRotationCoordinator;
pub use capture::DeviceType as CaptureDeviceType;
pub use capture::DiscoverySession as CaptureDeviceDiscoverySession;
pub use capture::ExposureMode as CaptureExposureMode;
pub use capture::FlashMode as CaptureFlashMode;
pub use capture::FocusMode as CaptureFocusMode;
pub use capture::FrameRateRange;
pub use capture::IndexPicker as CaptureIndexPicker;
pub use capture::Input as CaptureInput;
pub use capture::InputPort as CaptureInputPort;
#[cfg(not(any(target_os = "macos", target_os = "watchos")))]
pub use capture::InterruptionReason as CaptureInterruptionReason;
pub use capture::MetadataOutput as CaptureMetadataOutput;
pub use capture::MicMode as CaptureMicMode;
pub use capture::MultiCamSession as CaptureMultiCamSession;
pub use capture::Output as CaptureOutput;
pub use capture::PhotoOutput as CapturePhotoOutput;
pub use capture::PhotoQualityPrioritization as CapturePhotoQualityPrioritization;
pub use capture::ReactionEffectState as CaptureReactionEffectState;
pub use capture::ReactionType as CaptureReactionType;
pub use capture::Session as CaptureSession;
pub use capture::SessionPreset as CaptureSessionPreset;
pub use capture::Slider as CaptureSlider;
pub use capture::SysExposureBiasSlider as CaptureSysExposureBiasSlider;
pub use capture::SysZoomSlider as CaptureSysZoomSlider;
pub use capture::TorchMode as CaptureTouchMode;
pub use capture::VideoDataOutput as CaptureVideoDataOutput;
pub use capture::VideoOrienation as CaptureVideoOrienation;
pub use capture::VideoPreviewLayer as CaptureVideoPreviewLayer;
pub use capture::VideoStabilizationMode as CaptureVideoStabilizationMode;
pub use capture::WbChromaticityValues as CaptureWbChromaticityValues;
pub use capture::WbGains as CaptureWbGains;
pub use capture::WbMode as CaptureWbMode;
pub use capture::WbTempTintValues as CaptureWbTempTintValues;
pub use capture::device_notifications as capture_device_notifications;
pub use capture::input_port_notifications as capture_input_port_notifications;
pub use capture::session_err_key as capture_session_err_key;
pub use capture::session_notifications as capture_session_notifications;

#[cfg(not(target_os = "macos"))]
pub use capture::SysPressureFactors as CaptureSysPressureFactors;
#[cfg(not(target_os = "macos"))]
pub use capture::SysPressureLevel as CaptureSysPressureLevel;
#[cfg(not(target_os = "macos"))]
pub use capture::SysPressureState as CaptureSysPressureState;

#[cfg(any(target_os = "ios", target_os = "tvos"))]
pub use capture::MetadataInput as CaptureMetadataInput;

pub mod external_storage_device;
pub use external_storage_device::Device as ExternalStorageDevice;
pub use external_storage_device::DiscoverySession as ExternalStorageDeviceDiscoverySession;

pub mod metadata_object;
pub use metadata_object::CatBodyObj as MetadataCatBodyObj;
pub use metadata_object::DogBodyObj as MetadataDogBodyObj;
pub use metadata_object::FaceObj as MetadataFaceObj;
pub use metadata_object::HumanBodyObj as MetadataHumanBodyObj;
pub use metadata_object::HumanFullBodyObj as MetadataHumanFullBodyObj;
pub use metadata_object::Obj as MetadataObj;
pub use metadata_object::SalientObj as MetadataSalientObj;
pub use metadata_object::Type as MetadataObjType;

pub mod player;
pub use player::ActionAtItemEnd as PlayerActionAtItemEnd;
pub use player::AudiovisualBackgroundPlaybackPolicy as PlayerAudiovisualBackgroundPlaybackPolicy;
pub use player::HdrMode as PlayerHdrMode;
pub use player::ItemStatus as PlayerItemStatus;
pub use player::Player;
pub use player::QueuePlayer;
pub use player::Status as PlayerStatus;
pub use player::TimeControlStatus as PlayerTimeControlStatus;

#[cfg(feature = "ca")]
mod player_layer;
#[cfg(feature = "ca")]
pub use player_layer::PlayerLayer;

pub mod asset;
pub use asset::Asset;
pub use asset::AssetCache;
pub use asset::AssetWriter;
pub use asset::AssetWriterDelegate;
pub use asset::AssetWriterDelegateImpl;
pub use asset::FragmentedAsset;
pub use asset::FragmentedAssetMinder;
pub use asset::FragmentedTrack as FragmentedAssetTrack;
pub use asset::Reader as AssetReader;
pub use asset::ReaderOutput as AssetReaderOutput;
pub use asset::ReaderOutput as AssetReaderOuput;
pub use asset::ReaderStatus as AssetReaderStatus;
pub use asset::ReaderTrackOutput as AssetReaderTrackOutput;
pub use asset::UrlAsset;
pub use asset::WriterInput as AssetWriterInput;
pub use asset::WriterStatus as AssetWriterStatus;

pub use asset::SegmentReport as AssetSegmentReport;
pub use asset::SegmentReportSampleInfo as AssetSegmentReportSampleInfo;
pub use asset::SegmentTrackReport as AssetSegmentTrackReport;
pub use asset::SegmentType as AssetSegmentType;

pub use asset::AssetImageGenerator;
#[cfg(feature = "blocks")]
pub use asset::AssetImageGeneratorCh;
pub use asset::AssetImageGeneratorResult;

pub mod audio;
pub use audio::Buf as AudioBuf;
pub use audio::ChannelCount as AudioChannelCount;
pub use audio::ChannelLayout as AudioChannelLayout;
pub use audio::CommonFormat as AudioCommonFormat;
pub use audio::CompressedBuf as AudioCompressedBuf;
pub use audio::Converter as AudioConverter;
#[cfg(feature = "blocks")]
pub use audio::ConverterInputBlock as AudioConverterInputBlock;
pub use audio::ConverterInputStatus as AudioConverterInputStatus;
pub use audio::Engine as AudioEngine;
pub use audio::File as AudioFile;
pub use audio::Format as AudioFormat;
pub use audio::FrameCount as AudioFrameCount;
pub use audio::FramePos as AudioFramePos;
pub use audio::InputNode as AudioInputNode;
pub use audio::IoNode as AudioIoNode;
#[cfg(feature = "blocks")]
pub use audio::IoNodeInputBlock as AudioIoNodeInputBlock;
pub use audio::MixerNode as AudioMixerNode;
pub use audio::Node as AudioNode;
pub use audio::NodeBus as AudioNodeBus;
#[cfg(feature = "blocks")]
pub use audio::NodeCh as AudioNodeCompletionHandler;
pub use audio::OutputNode as AudioOutputNode;
pub use audio::PacketCount as AudioPacketCount;
pub use audio::PcmBuf as AudioPcmBuf;
pub use audio::Player as AudioPlayer;
pub use audio::PlayerDelegate as AudioPlayerDelegate;
pub use audio::PlayerNode as AudioPlayerNode;
pub use audio::Time as AudioTime;
pub use audio::VpOtherAudioDuckingCfg as AudioVpOtherAudioDuckingCfg;
pub use audio::VpOtherAudioDuckingLevel as AudioVpOtherAudioDuckingLevel;
pub use audio::VpSpeechActivityEvent as AudioVpSpeechActivityEvent;

#[cfg(any(target_os = "ios", target_os = "watchos", target_os = "tvos"))]
pub use audio::Session as AudioSession;
#[cfg(any(target_os = "ios", target_os = "watchos", target_os = "tvos"))]
pub use audio::SessionActivationOpts as AudioSessionActivationOpts;
#[cfg(any(target_os = "ios", target_os = "watchos", target_os = "tvos"))]
pub use audio::SessionCategory as AudioSessionCategory;
#[cfg(any(target_os = "ios", target_os = "watchos", target_os = "tvos"))]
pub use audio::SessionCategoryOpts as AudioSessionCategoryOpts;
#[cfg(any(target_os = "ios", target_os = "watchos", target_os = "tvos"))]
pub use audio::SessionInterruptionOpts as AudioSessionInterruptionOpts;
#[cfg(any(target_os = "ios", target_os = "watchos", target_os = "tvos"))]
pub use audio::SessionInterruptionReason as AudioSessionInterruptionReason;
#[cfg(any(target_os = "ios", target_os = "watchos", target_os = "tvos"))]
pub use audio::SessionInterruptionType as AudioSessionInterruptionType;
#[cfg(any(target_os = "ios", target_os = "watchos", target_os = "tvos"))]
pub use audio::SessionIoType as AudioSessionIoType;
#[cfg(any(target_os = "ios", target_os = "watchos", target_os = "tvos"))]
pub use audio::SessionMode as AudioSessionMode;
#[cfg(any(target_os = "ios", target_os = "watchos", target_os = "tvos"))]
pub use audio::SessionPort as AudioSessionPort;
#[cfg(any(target_os = "ios", target_os = "watchos", target_os = "tvos"))]
pub use audio::SessionPortOverride as AudioSessionPortOverride;
#[cfg(any(target_os = "ios", target_os = "watchos", target_os = "tvos"))]
pub use audio::SessionPromptStyle as AudioSessionPromptStyle;
#[cfg(any(target_os = "ios", target_os = "watchos", target_os = "tvos"))]
pub use audio::SessionRecordPermission as AudioSessionRecordPermission;
#[cfg(any(target_os = "ios", target_os = "watchos", target_os = "tvos"))]
pub use audio::SessionRouteChangeReason as AudioSessionRouteChangeReason;
#[cfg(any(target_os = "ios", target_os = "watchos", target_os = "tvos"))]
pub use audio::SessionRouteDesc as AudioSessionRouteDesc;
#[cfg(any(target_os = "ios", target_os = "watchos", target_os = "tvos"))]
pub use audio::SessionRouteSharingPolicy as AudioSessionRouteSharingPolicy;
#[cfg(any(target_os = "ios", target_os = "watchos", target_os = "tvos"))]
pub use audio::SessionSetActiveOpts as AudioSessionSetActiveOpts;
#[cfg(any(target_os = "ios", target_os = "watchos", target_os = "tvos"))]
pub use audio::SessionSilenceSecondaryAudioHintType as AudioSessionSilenceSecondaryAudioHintType;
#[cfg(any(target_os = "ios", target_os = "watchos", target_os = "tvos"))]
pub use audio::session_keys as audio_session_keys;

pub use audio::App as AudioApp;
pub use audio::AppRecordPermission as AudioAppRecordPermission;
pub use audio::SpeechBoundery;
pub use audio::SpeechSynthesisMarker;
pub use audio::SpeechSynthesisMarkerMark;
pub use audio::SpeechSynthesisVoice;
pub use audio::SpeechSynthesisVoiceGender;
pub use audio::SpeechSynthesisVoiceQuality;
pub use audio::SpeechSynthesizer;
pub use audio::SpeechSynthesizerDelegate;
pub use audio::SpeechSynthesizerDelegateImpl;
pub use audio::SpeechUtterance;

pub mod video;
pub use video::Codec as VideoCodec;
pub use video::settings_keys as video_settings_keys;

pub mod sample_buffer;
pub use sample_buffer::DisplayLayer as SampleBufDisplayLayer;
#[cfg(all(feature = "blocks", feature = "dispatch"))]
pub use sample_buffer::QueuedSampleBufRendering;
pub use sample_buffer::QueuedSampleBufRenderingStatus;
pub use sample_buffer::VideoRenderer as SampleBufVideoRenderer;

pub mod output_settings_assistant;
pub use output_settings_assistant::OutputSettingsAssistant;
pub use output_settings_assistant::OutputSettingsPreset;

pub mod geometry;

mod time;

#[cfg(feature = "av_kit")]
mod kit;
#[cfg(feature = "av_kit")]
pub use kit::AudioSessionRouteSelection;
#[cfg(feature = "av_kit")]
pub use kit::ErrorCode as KitErrorCode;
#[cfg(feature = "av_kit")]
pub use kit::PipController;
#[cfg(feature = "av_kit")]
pub use kit::PipControllerContentSrc;
#[cfg(feature = "av_kit")]
pub use kit::PipControllerDelegate;
#[cfg(feature = "av_kit")]
pub use kit::PipControllerDelegateImpl;
#[cfg(feature = "av_kit")]
pub use kit::PipSampleBufPlayerDelegate;
#[cfg(feature = "av_kit")]
pub use kit::PipSampleBufPlayerDelegateImpl;
#[cfg(all(feature = "av_kit", any(target_os = "ios", target_os = "visionos")))]
pub use kit::PipViewCallController;
#[cfg(feature = "av_kit")]
pub use kit::PlaybackSpeed;
#[cfg(all(
    feature = "av_kit",
    any(target_os = "ios", target_os = "visionos", target_os = "tvos")
))]
pub use kit::PlayerViewController;
#[cfg(all(
    feature = "av_kit",
    any(target_os = "ios", target_os = "visionos", target_os = "tvos")
))]
pub use kit::PlayerViewControllerDelegate;
#[cfg(all(
    feature = "av_kit",
    any(target_os = "ios", target_os = "visionos", target_os = "tvos")
))]
pub use kit::PlayerViewControllerDelegateImpl;
#[cfg(all(
    feature = "av_kit",
    any(target_os = "tvos")
    // any(target_os = "ios", target_os = "visionos", target_os = "tvos")
))]
pub use kit::PlayerViewControllerSkippingBehavior;
#[cfg(feature = "av_kit")]
pub use kit::VideoFrameAnalysisType;
