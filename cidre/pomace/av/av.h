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

wsel1(, id, setActiveFormat, AVCaptureDeviceFormat* )

wsel1(, id, setActiveVideoMinFrameDuration, CMTime)
wsel1(, id, setActiveVideoMaxFrameDuration, CMTime)

#if TARGET_OS_IPHONE
rsel0(, id, isVideoBinned, BOOL)
#endif

//@property(nonatomic, readonly) NSArray<AVFrameRateRange *> *videoSupportedFrameRateRanges;
rsel0(, id, videoSupportedFrameRateRanges, NSArray<AVFrameRateRange *> *)

rsel0(, id, formatDescription, CMFormatDescriptionRef)
rsel0(, id, autoFocusSystem, AVCaptureAutoFocusSystem)

#pragma mark - AVAssetWriter

typedef void (^ VoidBlock)(void);
wsel1(, id, finishWritingWithCompletionHandler, VoidBlock)
//- (void)finishWritingWithCompletionHandler:(void (^)(void))handler API_AVAILABLE(macos(10.9), ios(6.0), tvos(9.0))

#pragma mark - AVURLAsset

wsel2(, id, loadTracksWithMediaType, AVMediaType, completionHandler, id)

#pragma mark - AVAssetReaderOutput

NS_RETURNS_RETAINED
csel2(, AVAssetReaderTrackOutput, assetReaderTrackOutputWithTrack, AVAssetTrack *, outputSettings, NSDictionary * _Nullable, AVAssetReaderTrackOutput *)

#pragma mark - AVCaptureDeviceInput

csel2(, AVCaptureDeviceInput, deviceInputWithDevice, AVCaptureDevice *, error,  NSError * _Nullable * _Nullable, AVCaptureDeviceInput * _Nullable)

Class AV_CAPTURE_DEVICE;
Class AV_CAPTURE_SESSION;
Class AV_CAPTURE_MULTI_CAM_SESSION;
Class AV_CAPTURE_METADATA_OUTPUT;
Class AV_CAPTURE_DEVICE_DISCOVERY_SESSION;
Class AV_CAPTURE_VIDEO_DATA_OUTPUT;
Class AV_CAPTURE_DEVICE_INPUT;

Class AV_AUDIO_ENGINE;

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



__attribute__((constructor))
static void mtl_initializer(void)
{
  static int initialized = 0;
  if (!initialized) {
    AV_CAPTURE_DEVICE = [AVCaptureDevice class];
    AV_CAPTURE_METADATA_OUTPUT = [AVCaptureMetadataOutput class];
    AV_CAPTURE_SESSION = [AVCaptureSession class];
    AV_CAPTURE_DEVICE_DISCOVERY_SESSION = [AVCaptureDeviceDiscoverySession class];
    AV_CAPTURE_VIDEO_DATA_OUTPUT = [AVCaptureVideoDataOutput class];
    AV_CAPTURE_DEVICE_INPUT = [AVCaptureDeviceInput class];
#if TARGET_OS_OSX
#else
    AV_CAPTURE_MULTI_CAM_SESSION = [AVCaptureMultiCamSession class];
#endif
    AV_AUDIO_ENGINE = [AVAudioEngine class];
    AV_AUDIO_TIME = [AVAudioTime class];
    AV_AUDIO_UNIT_EFFECT = [AVAudioUnitEffect class];
    AV_AUDIO_UNIT_EQ = [AVAudioUnitEQ class];
    
    AV_AUDIO_UNIT_TIME_EFFECT = [AVAudioUnitTimeEffect class];
    
    
    AV_URL_ASSET = [AVURLAsset class];
    AV_ASSET_WRITER = [AVAssetWriter class];
    AV_ASSET_WRITER_INPUT = [AVAssetWriterInput class];
    AV_ASSET_READER_TRACK_OUTPUT = [AVAssetReaderTrackOutput class];
    AV_ASSET_READER = [AVAssetReader class];
    
    AV_AUDIO_FORMAT = [AVAudioFormat class];
    
    AV_AUDIO_PCM_BUFFER = [AVAudioPCMBuffer class];
    AV_AUDIO_COMPRESSED_BUFFER = [AVAudioCompressedBuffer class];
  }
}

NS_ASSUME_NONNULL_END
