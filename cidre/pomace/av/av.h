//
//  av.h
//  av
//
//  Created by Yury Korolev on 02.05.2022.
//

#import <Foundation/Foundation.h>
#import <AVFoundation/AVFoundation.h>

#include "../macro.h"

NS_ASSUME_NONNULL_BEGIN

Class AV_CAPTURE_DEVICE;
Class AV_CAPTURE_SESSION;
Class AV_CAPTURE_MULTI_CAM_SESSION;
Class AV_CAPTURE_METADATA_OUTPUT;
Class AV_CAPTURE_DEVICE_DISCOVERY_SESSION;
Class AV_CAPTURE_VIDEO_DATA_OUTPUT;
Class AV_CAPTURE_AUDIO_DATA_OUTPUT;
Class AV_CAPTURE_DEVICE_INPUT;

Class AV_AUDIO_PLAYER_NODE;

Class AV_AUDIO_ENGINE;

Class AV_AUDIO_CONNECTION_POINT;

Class AV_URL_ASSET;
Class AV_ASSET_WRITER;
Class AV_ASSET_READER;
Class AV_ASSET_WRITER_INPUT;
Class AV_ASSET_READER_TRACK_OUTPUT;

Class AV_AUDIO_TIME;

Class AV_AUDIO_UNIT_EQ;
Class AV_AUDIO_UNIT_EFFECT;
Class AV_AUDIO_UNIT_TIME_EFFECT;

Class AV_AUDIO_PCM_BUFFER;
Class AV_AUDIO_COMPRESSED_BUFFER;
Class AV_AUDIO_FORMAT;

Class AV_PLAYER;

__attribute__((constructor))
static void av_initializer(void)
{
  static int initialized = 0;
  if (!initialized) {
    AV_CAPTURE_DEVICE = [AVCaptureDevice class];
    AV_CAPTURE_METADATA_OUTPUT = [AVCaptureMetadataOutput class];
    AV_CAPTURE_SESSION = [AVCaptureSession class];
    AV_CAPTURE_DEVICE_DISCOVERY_SESSION = [AVCaptureDeviceDiscoverySession class];
    AV_CAPTURE_VIDEO_DATA_OUTPUT = [AVCaptureVideoDataOutput class];
    AV_CAPTURE_AUDIO_DATA_OUTPUT = [AVCaptureAudioDataOutput class];
    AV_CAPTURE_DEVICE_INPUT = [AVCaptureDeviceInput class];
#if TARGET_OS_OSX
#else
    AV_CAPTURE_MULTI_CAM_SESSION = [AVCaptureMultiCamSession class];
#endif
    AV_AUDIO_PLAYER_NODE = [AVAudioPlayerNode class];
    
    AV_AUDIO_ENGINE = [AVAudioEngine class];
    AV_AUDIO_TIME = [AVAudioTime class];
    AV_AUDIO_UNIT_EFFECT = [AVAudioUnitEffect class];
    AV_AUDIO_UNIT_EQ = [AVAudioUnitEQ class];
    AV_AUDIO_CONNECTION_POINT = [AVAudioConnectionPoint class];
    
    AV_AUDIO_UNIT_TIME_EFFECT = [AVAudioUnitTimeEffect class];
    
    
    AV_URL_ASSET = [AVURLAsset class];
    AV_ASSET_WRITER = [AVAssetWriter class];
    AV_ASSET_WRITER_INPUT = [AVAssetWriterInput class];
    AV_ASSET_READER_TRACK_OUTPUT = [AVAssetReaderTrackOutput class];
    AV_ASSET_READER = [AVAssetReader class];
    
    AV_AUDIO_FORMAT = [AVAudioFormat class];
    
    AV_AUDIO_PCM_BUFFER = [AVAudioPCMBuffer class];
    AV_AUDIO_COMPRESSED_BUFFER = [AVAudioCompressedBuffer class];
    
    AV_PLAYER = [AVPlayer class];
    
    initialized = 1;
  }
}

NS_ASSUME_NONNULL_END
