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

#pragma mark - AVCaptureDevice

NS_RETURNS_RETAINED
csel3(, AVCaptureDevice, defaultDeviceWithDeviceType, AVCaptureDeviceType, mediaType, AVMediaType _Nullable, position, AVCaptureDevicePosition, AVCaptureDevice * _Nullable)

NS_RETURNS_RETAINED
csel1(, AVCaptureDevice, deviceWithUniqueID, NSString *, AVCaptureDevice * _Nullable)

NS_RETURNS_NOT_RETAINED
rsel0(, AVCaptureDevice *, uniqueID, NSString *)

//- (BOOL)lockForConfiguration:(NSError * _Nullable * _Nullable)outError;
rsel1(, id, lockForConfiguration, NSError * _Nullable * _Nullable, BOOL)
//- (void)unlockForConfiguration;
wsel0(, id, unlockForConfiguration)

//- (BOOL)supportsAVCaptureSessionPreset:(AVCaptureSessionPreset)preset;
rsel1(, id, supportsAVCaptureSessionPreset, AVCaptureSessionPreset, BOOL)

//@property(nonatomic, readonly) NSArray<AVCaptureDeviceFormat *> *formats
NS_RETURNS_NOT_RETAINED
rsel0(, id, formats, NSArray<AVCaptureDeviceFormat *> *)

//@property(nonatomic, retain) AVCaptureDeviceFormat *activeFormat
NS_RETURNS_NOT_RETAINED
rsel0(, id, activeFormat, AVCaptureDeviceFormat *)
wsel1(, id, setActiveFormat, AVCaptureDeviceFormat* )

//@property(nonatomic) CMTime activeVideoMinFrameDuration API_AVAILABLE(ios(7.0), macCatalyst(14.0)) API_UNAVAILABLE(tvos);
rsel0(, id, activeVideoMinFrameDuration, CMTime)
wsel1(, id, setActiveVideoMinFrameDuration, CMTime)
rsel0(, id, activeVideoMaxFrameDuration, CMTime)
wsel1(, id, setActiveVideoMaxFrameDuration, CMTime)

// @property(nonatomic, readonly) BOOL hasTorch;
rsel0(, id, hasTorch, BOOL)

//@property(nonatomic, readonly, getter=isVideoBinned) BOOL videoBinned API_UNAVAILABLE(macos);
#if TARGET_OS_IPHONE
rsel0(, id, isVideoBinned, BOOL)
#endif

//@property(nonatomic, readonly) NSArray<AVFrameRateRange *> *videoSupportedFrameRateRanges;
rsel0(, id, videoSupportedFrameRateRanges, NSArray<AVFrameRateRange *> *)

//@property(nonatomic, readonly) CMFormatDescriptionRef formatDescription;
rsel0(, id, formatDescription, CMFormatDescriptionRef)

//@property(nonatomic, readonly) AVCaptureAutoFocusSystem autoFocusSystem API_AVAILABLE(macos(10.15), ios(8.0), macCatalyst(14.0)) API_UNAVAILABLE(tvos);
rsel0(, id, autoFocusSystem, AVCaptureAutoFocusSystem)

//@property(nonatomic, readonly, getter=isMultiCamSupported) BOOL multiCamSupported API_AVAILABLE(ios(13.0), macCatalyst(14.0)) API_UNAVAILABLE(macos, tvos) API_UNAVAILABLE(watchos);
#if TARGET_OS_IPHONE
rsel0(, id, isMultiCamSupported, BOOL)
#endif

//@property(nonatomic, readonly, getter=isCenterStageSupported) BOOL centerStageSupported API_AVAILABLE(macos(12.3), ios(14.5), macCatalyst(14.5)) API_UNAVAILABLE(tvos) API_UNAVAILABLE(watchos);

rsel0(, id, isCenterStageSupported, BOOL)

//@property(nonatomic, readonly, nullable) AVFrameRateRange *videoFrameRateRangeForCenterStage API_AVAILABLE(macos(12.3), ios(14.5), macCatalyst(14.5)) API_UNAVAILABLE(tvos) API_UNAVAILABLE(watchos);
NS_RETURNS_NOT_RETAINED
rsel0(, id, videoFrameRateRangeForCenterStage, AVFrameRateRange* _Nullable)

//+ (nullable AVCaptureDevice *)deviceWithUniqueID:(NSString *)deviceUniqueID;
// + (nullable AVCaptureDevice *)defaultDeviceWithDeviceType:(AVCaptureDeviceType)deviceType mediaType:(nullable AVMediaType)mediaType position:(AVCaptureDevicePosition)position API_AVAILABLE(macos(10.15), ios(10.0), macCatalyst(14.0)) API_UNAVAILABLE(tvos);

#pragma mark - AVFrameRateRange

rsel0(,id, minFrameRate, Float64)
rsel0(,id, maxFrameRate, Float64)

rsel0(,id,  minFrameDuration, CMTime)
rsel0(,id,  maxFrameDuration, CMTime)

#pragma mark - AVCaptureInput

//@property(nonatomic, readonly) NSArray<AVCaptureInputPort *> *ports;
NS_RETURNS_NOT_RETAINED
rsel0(, id, ports, NSArray<AVCaptureInputPort *> *);


//@property(nonatomic, readonly) AVCaptureInput *input;
NS_RETURNS_NOT_RETAINED
rsel0(, id, input, AVCaptureInput *)

#pragma mark - AVCaptureOutput

// @property(nonatomic, readonly) NSArray<AVCaptureConnection *> *connections;
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

#if TARGET_OS_OSX

#else
//@property(nonatomic, readonly) float hardwareCost;
rsel0(, id, hardwareCost, float)
rsel0(, id, systemPressureCost, float)
#endif

#pragma mark - AVCaptureSession

NS_RETURNS_RETAINED
csel0(, AVCaptureSession, new, AVCaptureSession *)
//- (BOOL)canSetSessionPreset:(AVCaptureSessionPreset)preset;
rsel1(, id, canSetSessionPreset, AVCaptureSessionPreset, BOOL)

NS_RETURNS_NOT_RETAINED
rsel0(, id, sessionPreset, AVCaptureSessionPreset)

wsel1(, id, setSessionPreset, AVCaptureSessionPreset)

NS_RETURNS_NOT_RETAINED
rsel0(, AVCaptureSession *, inputs, NSArray<__kindof AVCaptureInput *> *)

//- (BOOL)canAddInput:(AVCaptureInput *)input;
rsel1(, id, canAddInput, AVCaptureInput *, BOOL)
//- (void)addInput:(AVCaptureInput *)input;
wsel1(, id, addInput, AVCaptureInput *)
//- (void)removeInput:(AVCaptureInput *)input;
wsel1(, id, removeInput, AVCaptureInput *)

//@property(nonatomic, readonly) NSArray<__kindof AVCaptureOutput *> *outputs;
NS_RETURNS_NOT_RETAINED
rsel0(, AVCaptureSession *, outputs, NSArray<__kindof AVCaptureOutput *> *)

rsel1(, id, canAddOutput, AVCaptureOutput *, BOOL)
wsel1(, id, addOutput, AVCaptureOutput *)
wsel1(, id, removeOutput, AVCaptureOutput *)

wsel1(, id, addInputWithNoConnections, AVCaptureInput *)
wsel1(, id, addOutputWithNoConnections, AVCaptureOutput *)

rsel1(, id, canAddConnection, AVCaptureConnection *, BOOL)
wsel1(, id, addConnection, AVCaptureConnection *)
wsel1(, id, removeConnection, AVCaptureConnection *)

//- (void)beginConfiguration;
wsel0(, id, beginConfiguration)
//- (void)commitConfiguration;
wsel0(, id, commitConfiguration)

wsel0(, id, startRunning)
wsel0(, id, stopRunning)

rwsel(, id, usesApplicationAudioSession, setUsesApplicationAudioSession, BOOL)

#pragma mark - AVCaptureMultiCamSession

#if TARGET_OS_OSX

#else

NS_RETURNS_RETAINED
csel0(, AVCaptureMultiCamSession, new, AVCaptureMultiCamSession *)
#endif

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


#pragma mark - AVAudioNode

//@property (nonatomic, readonly) NSUInteger numberOfInputs;
rsel0(, id, numberOfInputs, NSUInteger)
//@property (nonatomic, readonly) NSUInteger numberOfOutputs;
rsel0(, id, numberOfOutputs, NSUInteger)


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

//- (void)prepare;
//- (void)connect:(AVAudioNode *)node1 to:(AVAudioNode *)node2 format:(AVAudioFormat * __nullable)format;

#pragma mark - AVAudioTime

NS_RETURNS_RETAINED
csel1(, AVAudioTime, timeWithHostTime, uint64_t, AVAudioTime *)
NS_RETURNS_RETAINED
csel2(, AVAudioTime, timeWithAudioTimeStamp, const AudioTimeStamp *, sampleRate, double, AVAudioTime *)
NS_RETURNS_RETAINED
csel2(, AVAudioTime, timeWithSampleTime, AVAudioFramePosition, atRate, double, AVAudioTime *)

NS_RETURNS_RETAINED
csel3(, AVAudioTime, timeWithHostTime, uint64_t, sampleTime, AVAudioFramePosition, atRate, double, AVAudioTime *)


rsel0(, id, hostTime, uint64_t)
rsel0(, id, audioTimeStamp, AudioTimeStamp)
rsel0(, id, sampleRate, double)
rsel0(, id, isSampleTimeValid, BOOL)
rsel0(, id, isHostTimeValid, BOOL)
rsel0(, id, sampleTime, AVAudioFramePosition)

NS_RETURNS_RETAINED
rsel1(, id, extrapolateTimeFromAnchor, AVAudioTime *, AVAudioTime * _Nullable)

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

rwsel(, id, shouldOptimizeForNetworkUse, setShouldOptimizeForNetworkUse, BOOL)
//- (BOOL)canAddInput:(AVAssetWriterInput *)input;
rsel1(AVAssetWriter_, AVAssetWriter *, canAddInput, AVAssetWriterInput *, BOOL)
wsel1(AVAssetWriter_, AVAssetWriter *, addInput, AVAssetWriterInput *)

wsel0(, id, startWriting)
wsel1(, id, startSessionAtSourceTime, CMTime);
wsel1(, id, endSessionAtSourceTime, CMTime);
wsel0(, id, cancelWriting)
//- (void)endSessionAtSourceTime:(CMTime)endTime;
wsel0(, id, finishWriting)

rsel0(, id, error, NSError * _Nullable)
//@property (nonatomic, readonly) NSArray<AVAssetWriterInput *> *inputs;
rsel0(AVAssetWriter_, id, inputs, NSArray<AVAssetWriterInput *> *)

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

wsel1(AVAssetReader_, id, addOutput, AVAssetReaderOutput *);
rsel0(, id, startReading, BOOL)
wsel0(, id, cancelReading)

//@property (nonatomic, readonly) NSArray<AVAssetReaderOutput *> *outputs;
NS_RETURNS_NOT_RETAINED
rsel0(AVAssetReader_, AVAssetReader *, outputs, NSArray *)

rsel1(AVAssetReader_, id, canAddOutput, AVAssetReaderOutput *, BOOL);
//- (BOOL)canAddOutput:(AVAssetReaderOutput *)output;

#pragma mark - AVAssetReaderOutput

//+ (instancetype)assetReaderTrackOutputWithTrack:(AVAssetTrack *)track outputSettings:(nullable NSDictionary<NSString *, id> *)outputSettings;

NS_RETURNS_RETAINED
csel2(, AVAssetReaderTrackOutput, assetReaderTrackOutputWithTrack, AVAssetTrack *, outputSettings, NSDictionary * _Nullable, AVAssetReaderTrackOutput *)

//@property (nonatomic) BOOL supportsRandomAccess API_AVAILABLE(macos(10.10), ios(8.0), tvos(9.0)) API_UNAVAILABLE(watchos);
rsel0(, id, supportsRandomAccess, BOOL)
wsel1(, id, resetForReadingTimeRanges, NSArray *)
//- (void)resetForReadingTimeRanges:(NSArray<NSValue *> *)timeRanges API_AVAILABLE(macos(10.10), ios(8.0), tvos(9.0)) API_UNAVAILABLE(watchos);
//AVAssetReaderTrackOutput

rwsel(, id, alwaysCopiesSampleData, setAlwaysCopiesSampleData, BOOL)

#pragma mark - AVAudioConverter

rsel0(, id, availableEncodeChannelLayoutTags, NSArray<NSNumber *> * _Nullable)
rsel0(, id, applicableEncodeSampleRates, NSArray<NSNumber *> * _Nullable)
rsel0(, id, availableEncodeSampleRates, NSArray<NSNumber *> * _Nullable)
rsel0(, id, applicableEncodeBitRates, NSArray<NSNumber *> * _Nullable)
rsel0(, id, availableEncodeBitRates, NSArray<NSNumber *> * _Nullable)
rsel0(, id, maximumOutputPacketSize, NSInteger)

rsel0(, id, bitRateStrategy, NSString * _Nullable)
wsel1(, id, setBitRateStrategy, NSString * _Nullable)

//- (BOOL)convertToBuffer:(AVAudioPCMBuffer *)outputBuffer fromBuffer:(const AVAudioPCMBuffer *)inputBuffer error:(NSError **)outError;
rsel3(, id, convertToBuffer, AVAudioPCMBuffer *, fromBuffer, const AVAudioPCMBuffer *, error, NSError **, BOOL)
rsel3(, id, convertToBuffer, AVAudioBuffer *, error, NSError **, withInputFromBlock, id, AVAudioConverterOutputStatus)

#pragma mark - AVMetadataObject

rsel0(AVMetadataObject_, AVMetadataObject *, time, CMTime)
rsel0(AVMetadataObject_, AVMetadataObject *, duration, CMTime)
rsel0(AVMetadataObject_, AVMetadataObject *, type, AVMetadataObjectType)

#pragma mark - AVMetadataBodyObject

rsel0(AVMetadataBodyObject_, AVMetadataBodyObject *, bodyID, NSInteger)

#pragma mark - AVMetadataSalientObject

rsel0(AVMetadataSalientObject_, AVMetadataSalientObject *, objectID, NSInteger)

#pragma mark - AVMetadataFaceObject

rsel0(AVMetadataFaceObject_, AVMetadataFaceObject *, faceID, NSInteger)
rsel0(AVMetadataFaceObject_, AVMetadataFaceObject *, hasRollAngle, BOOL)
rsel0(AVMetadataFaceObject_, AVMetadataFaceObject *, hasYawAngle, BOOL)
rsel0(AVMetadataFaceObject_, AVMetadataFaceObject *, yawAngle, CGFloat)
rsel0(AVMetadataFaceObject_, AVMetadataFaceObject *, rollAngle, CGFloat)

#pragma mark - AVCaptureMetadataOutput

//@property(nonatomic, readonly) NSArray<AVMetadataObjectType> *availableMetadataObjectTypes;
rsel0(, id, availableMetadataObjectTypes, NSArray *)
rwsel(, id, rectOfInterest, setRectOfInterest, CGRect)
csel0(, AVCaptureMetadataOutput, new, AVCaptureMetadataOutput *)

#pragma mark - AVCaptureVideoDataOutput


NS_RETURNS_RETAINED
csel0(, AVCaptureVideoDataOutput, new, AVCaptureVideoDataOutput *)

rwsel(, AVCaptureVideoDataOutput *, alwaysDiscardsLateVideoFrames, setAlwaysDiscardsLateVideoFrames, BOOL)

rsel0(, id, availableVideoCVPixelFormatTypes, NSArray *)
rsel0(, id, availableVideoCodecTypes, NSArray *)

rsel0(, id, sampleBufferCallbackQueue, dispatch_queue_t _Nullable)
rsel0(, id, videoSettings, NSDictionary * _Nullable)
rsel1(, id, recommendedVideoSettingsForAssetWriterWithOutputFileType, AVFileType, NSDictionary * _Nullable)

rsel2(, id, recommendedVideoSettingsForVideoCodecType, AVVideoCodecType, assetWriterOutputFileType, AVFileType, NSDictionary * _Nullable)
//- (nullable NSDictionary<NSString *, id> *)recommendedVideoSettingsForVideoCodecType:(AVVideoCodecType)videoCodecType assetWriterOutputFileType:(AVFileType)outputFileType API_AVAILABLE(macos(10.15), ios(11.0), macCatalyst(14.0)) API_UNAVAILABLE(tvos);

#pragma mark - AVCaptureDeviceInput

// + (nullable instancetype)deviceInputWithDevice:(AVCaptureDevice *)device error:(NSError * _Nullable * _Nullable)outError;

csel2(, AVCaptureDeviceInput, deviceInputWithDevice, AVCaptureDevice *, error,  NSError * _Nullable * _Nullable, AVCaptureDeviceInput * _Nullable)

#pragma mark NSValue

NS_RETURNS_RETAINED
csel1(, NSValue, valueWithCMTimeRange, CMTimeRange, NSValue *)
//+ (NSValue *)valueWithCMTimeRange:(CMTimeRange)timeRange API_AVAILABLE(macos(10.7), ios(4.0), tvos(9.0), watchos(1.0));


SEL sel_copyNextSampleBuffer;
SEL sel_status;
SEL sel_timeRange;
SEL sel_setTimeRange;
SEL sel_convertToBuffer_fromBuffer_error;
SEL sel_convertToBuffer_error_withInputFromBlock;

__attribute__((constructor))
static void common_initializer()
{
  static int initialized = 0;
  if (!initialized) {
    sel_copyNextSampleBuffer = @selector(copyNextSampleBuffer);
    sel_status = @selector(status);
    sel_timeRange = @selector(timeRange);
    sel_setTimeRange = @selector(setTimeRange:);
    sel_convertToBuffer_fromBuffer_error = @selector(convertToBuffer:fromBuffer:error:);
    sel_convertToBuffer_error_withInputFromBlock = @selector(convertToBuffer:error:withInputFromBlock:);
  }
}

NS_ASSUME_NONNULL_END
