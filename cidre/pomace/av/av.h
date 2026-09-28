//
//  av.h
//  av
//
//  Created by Yury Korolev on 02.05.2022.
//

#import <AVFoundation/AVFoundation.h>

NS_ASSUME_NONNULL_BEGIN

Class AV_CAPTURE_MULTI_CAM_SESSION;
Class AV_CAPTURE_METADATA_OUTPUT;
Class AV_CAPTURE_DEVICE_DISCOVERY_SESSION;
Class AV_CAPTURE_DEVICE_ROTATION_COORDINATOR = nil;
Class AV_CAPTURE_PHOTO_OUTPUT;
Class AV_CAPTURE_PHOTO_SETTINGS;

Class AV_PLAYER_ITEM_SAMPLE_BUFFER_OUTPUT = nil;
Class AV_PLAYER_ITEM_SAMPLE_BUFFER_OUTPUT_CONFIGURATION = nil;
Class AV_PLAYER_ITEM_SAMPLE_BUFFER_OUTPUT_AUDIO_CONFIGURATION = nil;

Class AV_SAMPLE_BUFFER_VIDEO_RENDERER;

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
#if TARGET_OS_VISION
#else
        AV_CAPTURE_METADATA_OUTPUT = NSClassFromString(@"AVCaptureMetadataOutput");
        AV_CAPTURE_DEVICE_DISCOVERY_SESSION = [AVCaptureDeviceDiscoverySession class];
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
#else
        AV_CAPTURE_MULTI_CAM_SESSION = [AVCaptureMultiCamSession class];
#endif
#endif

        AV_PLAYER_ITEM_SAMPLE_BUFFER_OUTPUT = NSClassFromString(@"AVPlayerItemSampleBufferOutput");
        AV_PLAYER_ITEM_SAMPLE_BUFFER_OUTPUT_CONFIGURATION = NSClassFromString(@"AVPlayerItemSampleBufferOutputConfiguration");
        AV_PLAYER_ITEM_SAMPLE_BUFFER_OUTPUT_AUDIO_CONFIGURATION = NSClassFromString(@"AVPlayerItemSampleBufferOutputAudioConfiguration");

        AV_CAPTURE_SYSTEM_ZOOM_SLIDER = NSClassFromString(@"AVCaptureSystemZoomSlider");
        AV_CAPTURE_SYSTEM_EXPOSURE_BIAS_SLIDER =  NSClassFromString(@"AVCaptureSystemExposureBiasSlider");
        AV_CAPTURE_SLIDER = NSClassFromString(@"AVCaptureSlider");
        AV_CAPTURE_INDEX_PICKER = NSClassFromString(@"AVCaptureIndexPicker");

        AV_EXTERNAL_STORAGE_DEVICE = NSClassFromString(@"AVExternalStorageDevice");
        AV_EXTERNAL_STORAGE_DEVICE_DISCOVERY_SESSION = NSClassFromString(@"AVExternalStorageDeviceDiscoverySession");
        
        AV_CAPTURE_PHOTO_SETTINGS = NSClassFromString(@"AVCapturePhotoSettings");

    }
}

NS_ASSUME_NONNULL_END
