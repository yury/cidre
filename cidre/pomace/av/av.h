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

#pragma mark - AVCaptureSystemPressureState

#if TARGET_OS_IPHONE
//@property(atomic, readonly) AVCaptureSystemPressureLevel level;
rsel0(, AVCaptureSystemPressureState *, level, AVCaptureSystemPressureLevel)
rsel0(, AVCaptureSystemPressureState *, factors, AVCaptureSystemPressureFactors)

#endif


NS_RETURNS_RETAINED
csel1(, AVCaptureDevice, deviceWithUniqueID, NSString *, AVCaptureDevice * _Nullable)

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

#if TARGET_OS_IPHONE
rsel0(, id, isMultiCamSupported, BOOL)
#endif

#pragma mark - AVCaptureInput

NS_RETURNS_NOT_RETAINED
rsel0(, id, ports, NSArray<AVCaptureInputPort *> *);


NS_RETURNS_NOT_RETAINED
rsel0(, id, input, AVCaptureInput *)

#pragma mark - AVCaptureOutput

NS_RETURNS_NOT_RETAINED
rsel0(, id, connections, NSArray<AVCaptureConnection *> *)


NS_RETURNS_NOT_RETAINED
rsel1(, id, connectionWithMediaType, AVMediaType, AVCaptureConnection * _Nullable)

bool is_mutlicam_supported(void) {
#if TARGET_OS_OSX
  return NO;
#else
  return [AVCaptureMultiCamSession isMultiCamSupported];
#endif
}

#pragma mark - AVCaptureDeviceDiscoverySession

NS_RETURNS_RETAINED
csel3(, AVCaptureDeviceDiscoverySession, discoverySessionWithDeviceTypes, NSArray<AVCaptureDeviceType> *, mediaType, AVMediaType _Nullable, position, AVCaptureDevicePosition, AVCaptureDeviceDiscoverySession *)

NS_RETURNS_NOT_RETAINED
rsel0(, id, devices, NSArray<AVCaptureDevice *> *)

#if TARGET_OS_OSX

#else

NS_RETURNS_NOT_RETAINED
rsel0(, id, supportedMultiCamDeviceSets, NSArray<NSSet<AVCaptureDevice *> *> *)

#endif

wsel0(av_, id, reset)
NS_RETURNS_NOT_RETAINED
rsel0(, id, engine, AVAudioEngine * _Nullable)

#pragma mark - AVAudioEngine

NS_RETURNS_RETAINED
csel0(, AVAudioEngine, new, AVAudioEngine *)

//- (void)attachNode:(AVAudioNode *)node;
wsel1(, id, attachNode, AVAudioNode *)
wsel1(, id, detachNode, AVAudioNode *)

wsel5(, id, connect, AVAudioNode *, to, AVAudioNode *, fromBus, AVAudioNodeBus, toBus, AVAudioNodeBus, format, AVAudioFormat * _Nullable)
wsel3(, id, connect, AVAudioNode *, to, AVAudioNode *, format, AVAudioFormat * _Nullable)

wsel0(, id, prepare)

rsel1(, id, startAndReturnError, NSError **, BOOL);
wsel0(, id, pause);
wsel0(, id, stop);
rsel0(, id, isRunning, BOOL)
wsel4(, id, connect, AVAudioNode *, toConnectionPoints, NSArray<AVAudioConnectionPoint *> *, fromBus, AVAudioNodeBus, format, AVAudioFormat * _Nullable)
wsel2(, id, disconnectNodeInput, AVAudioNode *, bus, AVAudioNodeBus)
wsel1(, id, disconnectNodeInput, AVAudioNode *)

wsel2(, id, disconnectNodeOutput, AVAudioNode *, bus, AVAudioNodeBus)
wsel1(, id, disconnectNodeOutput, AVAudioNode *)

NS_RETURNS_NOT_RETAINED
rsel0(, id, inputNode, AVAudioInputNode *)

NS_RETURNS_NOT_RETAINED
rsel0(, id, outputNode, AVAudioOutputNode *)

NS_RETURNS_NOT_RETAINED
rsel0(, id, mainMixerNode, AVAudioMixerNode *)

#pragma mark AVAudioUnitEffect

NS_RETURNS_RETAINED
asel1(, AVAudioUnitEffect, initWithAudioComponentDescription, AudioComponentDescription)
rwsel(, id, bypass, setBypass, BOOL)

#pragma mark AVAudioUnitEQFilterParameters

rwsel(, id, filterType, setFilterType, AVAudioUnitEQFilterType)
rwsel(, id, frequency, setFrequency, float)
rwsel(, id, bandwidth, setBandwidth, float)
rwsel(, id, gain, setGain, float)

#pragma mark AVAudioUnitEQ

asel1(, AVAudioUnitEQ, initWithNumberOfBands, NSUInteger)
rsel0(, id, bands, NSArray *)

rwsel(, id, globalGain, setGlobalGain, float)

#pragma mark AVAudioUnitTimeEffect

NS_RETURNS_RETAINED
asel1(, AVAudioUnitTimeEffect, initWithAudioComponentDescription, AudioComponentDescription)

#pragma mark - AVAudioCommonFormat

NS_RETURNS_RETAINED
asel1(, AVAudioFormat, initWithStreamDescription, const AudioStreamBasicDescription *)

NS_RETURNS_RETAINED
asel2(, AVAudioFormat, initWithStreamDescription, const AudioStreamBasicDescription *, channelLayout, AVAudioChannelLayout * _Nullable)

NS_RETURNS_RETAINED
asel2(, AVAudioFormat, initStandardFormatWithSampleRate, double, channels, AVAudioChannelCount)

NS_RETURNS_RETAINED
asel2(, AVAudioFormat, initStandardFormatWithSampleRate, double, channelLayout, AVAudioChannelLayout *)

NS_RETURNS_RETAINED
asel4(, AVAudioFormat, initWithCommonFormat, AVAudioCommonFormat, sampleRate, double, interleaved, BOOL, channelLayout, AVAudioChannelLayout *)

NS_RETURNS_RETAINED
asel1(, AVAudioFormat, initWithSettings, NSDictionary *);

NS_RETURNS_NOT_RETAINED
rsel0(, id, settings, NSDictionary *)
rsel0(, id, isInterleaved, BOOL)
rsel0(, id, commonFormat, AVAudioCommonFormat)
rsel0(av_format_, AVAudioFormat *, channelCount, AVAudioChannelCount)
rsel0(, id, streamDescription, const AudioStreamBasicDescription *)
NS_RETURNS_NOT_RETAINED
rsel0(, id, channelLayout, AVAudioChannelLayout * _Nullable)

NS_RETURNS_NOT_RETAINED
rsel0(, id, magicCookie, NSData * _Nullable)

#pragma mark - AVAudioBuffer

rsel0(, id, format, AVAudioFormat *)
rsel0(, id, audioBufferList, const AudioBufferList *)
rsel0(, id, mutableAudioBufferList, AudioBufferList *)

#pragma mark - AVAudioPCMBuffer

//- (nullable instancetype)initWithPCMFormat:(AVAudioFormat *)format frameCapacity:(AVAudioFrameCount)frameCapacity NS_DESIGNATED_INITIALIZER;
asel2(, AVAudioPCMBuffer, initWithPCMFormat,AVAudioFormat *, frameCapacity, AVAudioFrameCount)

rsel0(, AVAudioPCMBuffer *, frameLength, AVAudioFrameCount)
wsel1(, AVAudioPCMBuffer *, setFrameLength, AVAudioFrameCount)
rsel0(, AVAudioPCMBuffer *, frameCapacity, AVAudioFrameCount)
//@property (nonatomic, readonly) AVAudioFrameCount frameCapacity;
rsel0(, id, stride, NSUInteger)

#pragma mark - AVAudioCompressedBuffer

asel2(, AVAudioCompressedBuffer, initWithFormat, AVAudioFormat *, packetCapacity, AVAudioPacketCount)

asel3(, AVAudioCompressedBuffer, initWithFormat, AVAudioFormat *, packetCapacity, AVAudioPacketCount, maximumPacketSize, NSInteger)


rsel0(, id, packetCapacity, AVAudioPacketCount)
rsel0(, id, packetCount, AVAudioPacketCount)
wsel1(, id, setPacketCount, AVAudioPacketCount)

rsel0(, id, maximumPacketSize, NSInteger)
rsel0(, id, byteCapacity, uint32_t)

rwsel(, id, byteLength, setByteLength, uint32_t)

//@property (nonatomic, readonly, nullable) AudioStreamPacketDescription *packetDescriptions;
rsel0(, id, packetDescriptions, AudioStreamPacketDescription * _Nullable)

rsel0(, AVAudioCompressedBuffer *, data, void *)
//@property (nonatomic) uint32_t byteLength API_AVAILABLE(macosx(10.13), ios(11.0), watchos(4.0), tvos(11.0));

//- (nullable instancetype)initWithPCMFormat:(AVAudioFormat *)format frameCapacity:(AVAudioFrameCount)frameCapacity

#pragma mark - AVAssetWriterInput

csel2(, AVAssetWriterInput, assetWriterInputWithMediaType, AVMediaType, outputSettings, NSDictionary * _Nullable, AVAssetWriterInput *)

csel3(, AVAssetWriterInput, assetWriterInputWithMediaType, AVMediaType, outputSettings, NSDictionary * _Nullable, sourceFormatHint, CMFormatDescriptionRef _Nullable, AVAssetWriterInput *)


rsel0(, id, mediaType, AVMediaType)
rsel0(, id, outputSettings, NSDictionary * _Nullable)

rsel0(, id, isReadyForMoreMediaData, BOOL)

rsel0(, id, expectsMediaDataInRealTime, BOOL)
wsel1(, id, setExpectsMediaDataInRealTime, BOOL)

wsel0(, id, markAsFinished)

//- (BOOL)appendSampleBuffer:(CMSampleBufferRef)sampleBuffer;
rsel1(, id, appendSampleBuffer, CMSampleBufferRef, BOOL)

#pragma mark - AVAssetWriter

//+ (nullable instancetype)assetWriterWithURL:(NSURL *)outputURL fileType:(AVFileType)outputFileType error:(NSError * _Nullable * _Nullable)outError;
NS_RETURNS_RETAINED
csel3(, AVAssetWriter, assetWriterWithURL, NSURL *, fileType, AVFileType, error, NSError * _Nullable * _Nullable, AVAssetWriter *)

typedef void (^ VoidBlock)(void);
wsel1(, id, finishWritingWithCompletionHandler, VoidBlock)
//- (void)finishWritingWithCompletionHandler:(void (^)(void))handler API_AVAILABLE(macos(10.9), ios(6.0), tvos(9.0))

#pragma mark - AVURLAsset

wsel2(, id, loadTracksWithMediaType, AVMediaType, completionHandler, id)

NS_RETURNS_RETAINED
csel2(, AVURLAsset, URLAssetWithURL, NSURL *, options, NSDictionary * _Nullable, AVURLAsset *)
//+ (instancetype)URLAssetWithURL:(NSURL *)URL options:(nullable NSDictionary<NSString *, id> *)options;

#pragma mark - AVAssetReader

NS_RETURNS_RETAINED
csel2(, AVAssetReader, assetReaderWithAsset, AVAsset *, error, NSError **, AVAssetReader *)

#pragma mark - AVAssetReaderOutput

//+ (instancetype)assetReaderTrackOutputWithTrack:(AVAssetTrack *)track outputSettings:(nullable NSDictionary<NSString *, id> *)outputSettings;

NS_RETURNS_RETAINED
csel2(, AVAssetReaderTrackOutput, assetReaderTrackOutputWithTrack, AVAssetTrack *, outputSettings, NSDictionary * _Nullable, AVAssetReaderTrackOutput *)

//@property (nonatomic) BOOL supportsRandomAccess API_AVAILABLE(macos(10.10), ios(8.0), tvos(9.0)) API_UNAVAILABLE(watchos);
rsel0(, id, supportsRandomAccess, BOOL)
wsel1(, id, resetForReadingTimeRanges, NSArray *)

rwsel(, id, alwaysCopiesSampleData, setAlwaysCopiesSampleData, BOOL)


#pragma mark - AVCaptureDeviceInput

csel2(, AVCaptureDeviceInput, deviceInputWithDevice, AVCaptureDevice *, error,  NSError * _Nullable * _Nullable, AVCaptureDeviceInput * _Nullable)

Class AV_CAPTURE_DEVICE;
Class AV_CAPTURE_SESSION;
Class AV_CAPTURE_MULTI_CAM_SESSION;
Class AV_CAPTURE_METADATA_OUTPUT;
Class AV_CAPTURE_DEVICE_DISCOVERY_SESSION;
Class AV_CAPTURE_VIDEO_DATA_OUTPUT;

Class AV_AUDIO_ENGINE;

Class AV_ASSET_READER;
Class AV_ASSET_READER_TRACK_OUTPUT;

Class AV_AUDIO_TIME;


__attribute__((constructor))
static void common_initializer()
{
  static int initialized = 0;
  if (!initialized) {
    AV_CAPTURE_DEVICE = [AVCaptureDevice class];
    AV_CAPTURE_METADATA_OUTPUT = [AVCaptureMetadataOutput class];
    AV_CAPTURE_SESSION = [AVCaptureSession class];
    AV_CAPTURE_DEVICE_DISCOVERY_SESSION = [AVCaptureDeviceDiscoverySession class];
    AV_CAPTURE_VIDEO_DATA_OUTPUT = [AVCaptureVideoDataOutput class];
#if TARGET_OS_OSX
#else
    AV_CAPTURE_MULTI_CAM_SESSION = [AVCaptureMultiCamSession class];
#endif
    AV_AUDIO_ENGINE = [AVAudioEngine class];
    AV_AUDIO_TIME = [AVAudioTime class];
    AV_ASSET_READER_TRACK_OUTPUT = [AVAssetReaderTrackOutput class];
    AV_ASSET_READER = [AVAssetReader class];
  }
}

NS_ASSUME_NONNULL_END
