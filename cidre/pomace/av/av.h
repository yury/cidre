//
//  av.h
//  av
//
//  Created by Yury Korolev on 02.05.2022.
//

#import <AVFoundation/AVFoundation.h>

NS_ASSUME_NONNULL_BEGIN

Class AV_CAPTURE_DEVICE;
Class AV_CAPTURE_SESSION;
Class AV_CAPTURE_MULTI_CAM_SESSION;
Class AV_CAPTURE_METADATA_OUTPUT;
Class AV_CAPTURE_DEVICE_DISCOVERY_SESSION;
Class AV_CAPTURE_VIDEO_DATA_OUTPUT;
Class AV_CAPTURE_AUDIO_DATA_OUTPUT;
Class AV_CAPTURE_DEVICE_INPUT;
Class AV_CAPTURE_CONNECTION;
Class AV_CAPTURE_METADATA_INPUT;
Class AV_CAPTURE_DEVICE_ROTATION_COORDINATOR = nil;
Class AV_CAPTURE_PHOTO_OUTPUT;
Class AV_CAPTURE_PHOTO_SETTINGS;
Class AV_CAPTURE_VIDEO_PREVIEW_LAYER;

Class AV_AUDIO_PLAYER_NODE;
Class AV_AUDIO_PLAYER;

Class AV_AUDIO_ENGINE;
Class AV_AUDIO_SESSION;
Class AV_AUDIO_SESSION_ROUTE_DESCRIPTION;

Class AV_AUDIO_CONNECTION_POINT;

Class AV_URL_ASSET;
Class AV_ASSET_WRITER;
Class AV_ASSET_READER;
Class AV_ASSET_WRITER_INPUT;
Class AV_ASSET_WRITER_INPUT_PIXEL_BUFFER_ADAPTOR;
Class AV_ASSET_READER_TRACK_OUTPUT;
Class AV_ASSET_IMAGE_GENERATOR;

Class AV_OUTPUT_SETTINGS_ASSISTANT;

Class AV_AUDIO_TIME;
Class AV_AUDIO_MIXER_NODE;
Class AV_AUDIO_MIX;
Class AV_MUTABLE_AUDIO_MIX;
Class AV_AUDIO_MIX_INPUT_PARAMETERS;
Class AV_MUTABLE_AUDIO_MIX_INPUT_PARAMETERS;

Class AV_AUDIO_UNIT_EQ;
Class AV_AUDIO_UNIT_EFFECT;
Class AV_AUDIO_UNIT_TIME_EFFECT;

Class AV_AUDIO_PCM_BUFFER;
Class AV_AUDIO_COMPRESSED_BUFFER;
Class AV_AUDIO_FORMAT;
Class AV_AUDIO_CONVERTER;
Class AV_AUDIO_FILE;
Class AV_AUDIO_CHANNEL_LAYOUT;

Class AV_PLAYER;
Class AV_QUEUE_PLAYER;
Class AV_PLAYER_ITEM;
Class AV_PLAYER_LOOPER;

Class AV_SAMPLE_BUFFER_DISPLAY_LAYER;
Class AV_SAMPLE_BUFFER_VIDEO_RENDERER;
Class AV_PLAYER_LAYER;

Class AV_SPEECH_SYNTHESIS_VOICE;
Class AV_SPEECH_SYNTHESIZER;
Class AV_SPEECH_UTTERANCE;

Class AV_AUDIO_APPLICATION;

Class AV_CAPTURE_SYSTEM_ZOOM_SLIDER;
Class AV_CAPTURE_SYSTEM_EXPOSURE_BIAS_SLIDER;
Class AV_CAPTURE_SLIDER;
Class AV_CAPTURE_INDEX_PICKER;


Class AV_EXTERNAL_STORAGE_DEVICE;
Class AV_EXTERNAL_STORAGE_DEVICE_DISCOVERY_SESSION;


__attribute__((constructor))
static void av_initializer(void)
{
    static int initialized = 0;
    if (!initialized) {
        initialized = 1;
        
#if TARGET_OS_WATCH
#else
        AV_CAPTURE_DEVICE = [AVCaptureDevice class];
        
        AV_CAPTURE_SESSION = [AVCaptureSession class];
        AV_CAPTURE_CONNECTION = [AVCaptureConnection class];
        
        AV_CAPTURE_VIDEO_DATA_OUTPUT = [AVCaptureVideoDataOutput class];
        
        AV_CAPTURE_DEVICE_INPUT = [AVCaptureDeviceInput class];
        
#if TARGET_OS_VISION
#else
        AV_CAPTURE_METADATA_OUTPUT = NSClassFromString(@"AVCaptureMetadataOutput");
        AV_CAPTURE_DEVICE_DISCOVERY_SESSION = [AVCaptureDeviceDiscoverySession class];
        AV_CAPTURE_AUDIO_DATA_OUTPUT = [AVCaptureAudioDataOutput class];
        AV_CAPTURE_VIDEO_PREVIEW_LAYER = [AVCaptureVideoPreviewLayer class];
        AV_CAPTURE_PHOTO_OUTPUT = [AVCapturePhotoOutput class];
#endif
#endif
        if (@available(iOS 17.0, *)) {
#if TARGET_OS_WATCH
#else
#if TARGET_OS_VISION
#else
    AV_CAPTURE_DEVICE_ROTATION_COORDINATOR = NSClassFromString(@"AVCaptureDeviceRotationCoordinator");
#endif
    
    AV_SAMPLE_BUFFER_VIDEO_RENDERER = NSClassFromString(@"AVSampleBufferVideoRenderer");
    AV_AUDIO_APPLICATION = NSClassFromString(@"AVAudioApplication");
#endif
        } else {
#if TARGET_OS_WATCH
#else
    AV_CAPTURE_DEVICE_ROTATION_COORDINATOR = nil;
    
    AV_SAMPLE_BUFFER_VIDEO_RENDERER = nil;
    AV_AUDIO_APPLICATION = nil;
#endif

        }
#if TARGET_OS_OSX
#else
        
#if TARGET_OS_WATCH
#elif TARGET_OS_VISION
        AV_CAPTURE_METADATA_INPUT = nil;
#else
        AV_CAPTURE_MULTI_CAM_SESSION = [AVCaptureMultiCamSession class];
        AV_CAPTURE_METADATA_INPUT = [AVCaptureMetadataInput class];
#endif
        AV_AUDIO_SESSION = [AVAudioSession class];
#endif
        
        AV_AUDIO_MIX = [AVAudioMix class];
        AV_MUTABLE_AUDIO_MIX = [AVMutableAudioMix class];
        AV_AUDIO_MIX_INPUT_PARAMETERS = [AVAudioMixInputParameters class];
        AV_MUTABLE_AUDIO_MIX_INPUT_PARAMETERS = [AVMutableAudioMixInputParameters class];

#if TARGET_OS_WATCH
#else
        AV_OUTPUT_SETTINGS_ASSISTANT = [AVOutputSettingsAssistant class];
#endif
        
        AV_AUDIO_PLAYER_NODE = [AVAudioPlayerNode class];
        AV_AUDIO_PLAYER = [AVAudioPlayer class];
        
        AV_AUDIO_ENGINE = [AVAudioEngine class];
        AV_AUDIO_TIME = [AVAudioTime class];
        AV_AUDIO_MIXER_NODE = [AVAudioMixerNode class];
        
#if TARGET_OS_WATCH
#else
        AV_AUDIO_UNIT_EFFECT = [AVAudioUnitEffect class];
        AV_AUDIO_UNIT_EQ = [AVAudioUnitEQ class];
        AV_AUDIO_UNIT_TIME_EFFECT = [AVAudioUnitTimeEffect class];
#endif
        AV_AUDIO_CONNECTION_POINT = [AVAudioConnectionPoint class];
        
        
        AV_URL_ASSET = [AVURLAsset class];
        
        AV_PLAYER_ITEM = [AVPlayerItem class];
        
#if TARGET_OS_WATCH
#else
        AV_ASSET_WRITER = [AVAssetWriter class];
        AV_ASSET_WRITER_INPUT = [AVAssetWriterInput class];
        AV_ASSET_WRITER_INPUT_PIXEL_BUFFER_ADAPTOR = [AVAssetWriterInputPixelBufferAdaptor class];
        AV_ASSET_READER_TRACK_OUTPUT = [AVAssetReaderTrackOutput class];
        AV_ASSET_READER = [AVAssetReader class];
        
        AV_SAMPLE_BUFFER_DISPLAY_LAYER = [AVSampleBufferDisplayLayer class];
        
        AV_PLAYER_LAYER = [AVPlayerLayer class];
#endif
        
        AV_AUDIO_FORMAT = [AVAudioFormat class];
        
        AV_AUDIO_PCM_BUFFER = [AVAudioPCMBuffer class];
        AV_AUDIO_COMPRESSED_BUFFER = [AVAudioCompressedBuffer class];
        AV_AUDIO_CONVERTER = [AVAudioConverter class];
        AV_AUDIO_FILE = [AVAudioFile class];
        AV_AUDIO_CHANNEL_LAYOUT = [AVAudioChannelLayout class];
        
        AV_PLAYER = [AVPlayer class];
        AV_QUEUE_PLAYER = [AVQueuePlayer class];
        AV_PLAYER_LOOPER = [AVPlayerLooper class];

        AV_SPEECH_SYNTHESIS_VOICE = [AVSpeechSynthesisVoice class];
        AV_SPEECH_SYNTHESIZER = [AVSpeechSynthesizer class];
        AV_SPEECH_UTTERANCE = [AVSpeechUtterance class];
        
        AV_CAPTURE_SYSTEM_ZOOM_SLIDER = NSClassFromString(@"AVCaptureSystemZoomSlider");
        AV_CAPTURE_SYSTEM_EXPOSURE_BIAS_SLIDER =  NSClassFromString(@"AVCaptureSystemExposureBiasSlider");
        AV_CAPTURE_SLIDER = NSClassFromString(@"AVCaptureSlider");
        AV_CAPTURE_INDEX_PICKER = NSClassFromString(@"AVCaptureIndexPicker");
        
        AV_ASSET_IMAGE_GENERATOR =  NSClassFromString(@"AVAssetImageGenerator");
        
        AV_EXTERNAL_STORAGE_DEVICE = NSClassFromString(@"AVExternalStorageDevice");
        AV_EXTERNAL_STORAGE_DEVICE_DISCOVERY_SESSION = NSClassFromString(@" AVExternalStorageDeviceDiscoverySession");
        
        AV_CAPTURE_PHOTO_SETTINGS = NSClassFromString(@"AVCapturePhotoSettings");
        
        AV_AUDIO_SESSION_ROUTE_DESCRIPTION = NSClassFromString(@"AVAudioSessionRouteDescription");

    }
}

NS_ASSUME_NONNULL_END
