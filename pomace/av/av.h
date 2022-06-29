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
rsel(, AVCaptureSystemPressureState *, level, AVCaptureSystemPressureLevel)
rsel(, AVCaptureSystemPressureState *, factors, AVCaptureSystemPressureFactors)

#endif

#pragma mark - AVCaptureDevice

NS_RETURNS_RETAINED
csel_abc(, AVCaptureDevice, defaultDeviceWithDeviceType, AVCaptureDeviceType, mediaType, AVMediaType _Nullable, position, AVCaptureDevicePosition, AVCaptureDevice * _Nullable)

NS_RETURNS_RETAINED
csel_a(, AVCaptureDevice, deviceWithUniqueID, NSString *, AVCaptureDevice * _Nullable)

NS_RETURNS_NOT_RETAINED
rsel(, AVCaptureDevice *, uniqueID, NSString *)

//- (BOOL)lockForConfiguration:(NSError * _Nullable * _Nullable)outError;
rsel_a(, id, lockForConfiguration, NSError * _Nullable * _Nullable, BOOL)
//- (void)unlockForConfiguration;
wsel(, id, unlockForConfiguration)

//- (BOOL)supportsAVCaptureSessionPreset:(AVCaptureSessionPreset)preset;
rsel_a(, id, supportsAVCaptureSessionPreset, AVCaptureSessionPreset, BOOL)

//@property(nonatomic, readonly) NSArray<AVCaptureDeviceFormat *> *formats
NS_RETURNS_NOT_RETAINED
rsel(, id, formats, NSArray<AVCaptureDeviceFormat *> *)

//@property(nonatomic, retain) AVCaptureDeviceFormat *activeFormat
NS_RETURNS_NOT_RETAINED
rsel(, id, activeFormat, AVCaptureDeviceFormat *)
wsel_a(, id, setActiveFormat, AVCaptureDeviceFormat* )

//@property(nonatomic) CMTime activeVideoMinFrameDuration API_AVAILABLE(ios(7.0), macCatalyst(14.0)) API_UNAVAILABLE(tvos);
rsel(, id, activeVideoMinFrameDuration, CMTime)
wsel_a(, id, setActiveVideoMinFrameDuration, CMTime)
rsel(, id, activeVideoMaxFrameDuration, CMTime)
wsel_a(, id, setActiveVideoMaxFrameDuration, CMTime)

// @property(nonatomic, readonly) BOOL hasTorch;
rsel(, id, hasTorch, BOOL)

//@property(nonatomic, readonly, getter=isVideoBinned) BOOL videoBinned API_UNAVAILABLE(macos);
#if TARGET_OS_IPHONE
rsel(, id, isVideoBinned, BOOL)
#endif

//@property(nonatomic, readonly) NSArray<AVFrameRateRange *> *videoSupportedFrameRateRanges;
rsel(, id, videoSupportedFrameRateRanges, NSArray<AVFrameRateRange *> *)

//@property(nonatomic, readonly) CMFormatDescriptionRef formatDescription;
rsel(, id, formatDescription, CMFormatDescriptionRef)

//@property(nonatomic, readonly) AVCaptureAutoFocusSystem autoFocusSystem API_AVAILABLE(macos(10.15), ios(8.0), macCatalyst(14.0)) API_UNAVAILABLE(tvos);
rsel(, id, autoFocusSystem, AVCaptureAutoFocusSystem)

//@property(nonatomic, readonly, getter=isMultiCamSupported) BOOL multiCamSupported API_AVAILABLE(ios(13.0), macCatalyst(14.0)) API_UNAVAILABLE(macos, tvos) API_UNAVAILABLE(watchos);
#if TARGET_OS_IPHONE
rsel(, id, isMultiCamSupported, BOOL)
#endif

//@property(nonatomic, readonly, getter=isCenterStageSupported) BOOL centerStageSupported API_AVAILABLE(macos(12.3), ios(14.5), macCatalyst(14.5)) API_UNAVAILABLE(tvos) API_UNAVAILABLE(watchos);

rsel(, id, isCenterStageSupported, BOOL)

//@property(nonatomic, readonly, nullable) AVFrameRateRange *videoFrameRateRangeForCenterStage API_AVAILABLE(macos(12.3), ios(14.5), macCatalyst(14.5)) API_UNAVAILABLE(tvos) API_UNAVAILABLE(watchos);
NS_RETURNS_NOT_RETAINED
rsel(, id, videoFrameRateRangeForCenterStage, AVFrameRateRange* _Nullable)

//+ (nullable AVCaptureDevice *)deviceWithUniqueID:(NSString *)deviceUniqueID;
// + (nullable AVCaptureDevice *)defaultDeviceWithDeviceType:(AVCaptureDeviceType)deviceType mediaType:(nullable AVMediaType)mediaType position:(AVCaptureDevicePosition)position API_AVAILABLE(macos(10.15), ios(10.0), macCatalyst(14.0)) API_UNAVAILABLE(tvos);

#pragma mark - AVFrameRateRange

rsel(,id, minFrameRate, Float64)
rsel(,id, maxFrameRate, Float64)

rsel(,id,  minFrameDuration, CMTime)
rsel(,id,  maxFrameDuration, CMTime)

#pragma mark - AVCaptureInput

//@property(nonatomic, readonly) NSArray<AVCaptureInputPort *> *ports;
NS_RETURNS_NOT_RETAINED
rsel(, id, ports, NSArray<AVCaptureInputPort *> *);


//@property(nonatomic, readonly) AVCaptureInput *input;
NS_RETURNS_NOT_RETAINED
rsel(, id, input, AVCaptureInput *)

#pragma mark - AVCaptureOutput

// @property(nonatomic, readonly) NSArray<AVCaptureConnection *> *connections;
NS_RETURNS_NOT_RETAINED
rsel(, id, connections, NSArray<AVCaptureConnection *> *)


NS_RETURNS_NOT_RETAINED
rsel_a(, id, connectionWithMediaType, AVMediaType, AVCaptureConnection * _Nullable)

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
rsel(, id, hardwareCost, float)
rsel(, id, systemPressureCost, float)
#endif

#pragma mark - AVCaptureSession

NS_RETURNS_RETAINED
csel(, AVCaptureSession, new, AVCaptureSession *)
//- (BOOL)canSetSessionPreset:(AVCaptureSessionPreset)preset;
rsel_a(, id, canSetSessionPreset, AVCaptureSessionPreset, BOOL)

NS_RETURNS_NOT_RETAINED
rsel(, id, sessionPreset, AVCaptureSessionPreset)

wsel_a(, id, setSessionPreset, AVCaptureSessionPreset)

NS_RETURNS_NOT_RETAINED
rsel(, AVCaptureSession *, inputs, NSArray<__kindof AVCaptureInput *> *)

//- (BOOL)canAddInput:(AVCaptureInput *)input;
rsel_a(, id, canAddInput, AVCaptureInput *, BOOL)
//- (void)addInput:(AVCaptureInput *)input;
wsel_a(, id, addInput, AVCaptureInput *)
//- (void)removeInput:(AVCaptureInput *)input;
wsel_a(, id, removeInput, AVCaptureInput *)

//@property(nonatomic, readonly) NSArray<__kindof AVCaptureOutput *> *outputs;
NS_RETURNS_NOT_RETAINED
rsel(, AVCaptureSession *, outputs, NSArray<__kindof AVCaptureOutput *> *)

rsel_a(, id, canAddOutput, AVCaptureOutput *, BOOL)
wsel_a(, id, addOutput, AVCaptureOutput *)
wsel_a(, id, removeOutput, AVCaptureOutput *)

wsel_a(, id, addInputWithNoConnections, AVCaptureInput *)
wsel_a(, id, addOutputWithNoConnections, AVCaptureOutput *)

rsel_a(, id, canAddConnection, AVCaptureConnection *, BOOL)
wsel_a(, id, addConnection, AVCaptureConnection *)
wsel_a(, id, removeConnection, AVCaptureConnection *)

//- (void)beginConfiguration;
wsel(, id, beginConfiguration)
//- (void)commitConfiguration;
wsel(, id, commitConfiguration)

wsel(, id, startRunning)
wsel(, id, stopRunning)

#pragma mark - AVCaptureMultiCamSession

#if TARGET_OS_OSX

#else

NS_RETURNS_RETAINED
csel(, AVCaptureMultiCamSession, new, AVCaptureMultiCamSession *)
#endif

#pragma mark - AVCaptureDeviceDiscoverySession

NS_RETURNS_RETAINED
csel_abc(, AVCaptureDeviceDiscoverySession, discoverySessionWithDeviceTypes, NSArray<AVCaptureDeviceType> *, mediaType, AVMediaType _Nullable, position, AVCaptureDevicePosition, AVCaptureDeviceDiscoverySession *)

NS_RETURNS_NOT_RETAINED
rsel(, id, devices, NSArray<AVCaptureDevice *> *)

#if TARGET_OS_OSX

#else

NS_RETURNS_NOT_RETAINED
rsel(, id, supportedMultiCamDeviceSets, NSArray<NSSet<AVCaptureDevice *> *> *)

#endif

wsel(av_, id, reset)
NS_RETURNS_NOT_RETAINED
rsel(, id, engine, AVAudioEngine * _Nullable)


#pragma mark - AVAudioNode

//@property (nonatomic, readonly) NSUInteger numberOfInputs;
rsel(, id, numberOfInputs, NSUInteger)
//@property (nonatomic, readonly) NSUInteger numberOfOutputs;
rsel(, id, numberOfOutputs, NSUInteger)


#pragma mark - AVAudioEngine

NS_RETURNS_RETAINED
csel(, AVAudioEngine, new, AVAudioEngine *)

//- (void)attachNode:(AVAudioNode *)node;
wsel_a(, id, attachNode, AVAudioNode *)
wsel_a(, id, detachNode, AVAudioNode *)

wsel_abcde(, id, connect, AVAudioNode *, to, AVAudioNode *, fromBus, AVAudioNodeBus, toBus, AVAudioNodeBus, format, AVAudioFormat * _Nullable)
wsel_abc(, id, connect, AVAudioNode *, to, AVAudioNode *, format, AVAudioFormat * _Nullable)

wsel(, id, prepare)

rsel_a(, id, startAndReturnError, NSError **, BOOL);
wsel_abcd(, id, connect, AVAudioNode *, toConnectionPoints, NSArray<AVAudioConnectionPoint *> *, fromBus, AVAudioNodeBus, format, AVAudioFormat * _Nullable)
wsel_ab(, id, disconnectNodeInput, AVAudioNode *, bus, AVAudioNodeBus)
wsel_a(, id, disconnectNodeInput, AVAudioNode *)

wsel_ab(, id, disconnectNodeOutput, AVAudioNode *, bus, AVAudioNodeBus)
wsel_a(, id, disconnectNodeOutput, AVAudioNode *)

NS_RETURNS_NOT_RETAINED
rsel(, id, inputNode, AVAudioInputNode *)

NS_RETURNS_NOT_RETAINED
rsel(, id, outputNode, AVAudioOutputNode *)

NS_RETURNS_NOT_RETAINED
rsel(, id, mainMixerNode, AVAudioMixerNode *)

//- (void)prepare;
//- (void)connect:(AVAudioNode *)node1 to:(AVAudioNode *)node2 format:(AVAudioFormat * __nullable)format;

#pragma mark - AVAudioTime

NS_RETURNS_RETAINED
csel_a(, AVAudioTime, timeWithHostTime, uint64_t, AVAudioTime *)
NS_RETURNS_RETAINED
csel_ab(, AVAudioTime, timeWithAudioTimeStamp, const AudioTimeStamp *, sampleRate, double, AVAudioTime *)
NS_RETURNS_RETAINED
csel_ab(, AVAudioTime, timeWithSampleTime, AVAudioFramePosition, atRate, double, AVAudioTime *)

NS_RETURNS_RETAINED
csel_abc(, AVAudioTime, timeWithHostTime, uint64_t, sampleTime, AVAudioFramePosition, atRate, double, AVAudioTime *)


rsel(, id, hostTime, uint64_t)
rsel(, id, audioTimeStamp, AudioTimeStamp)
rsel(, id, sampleRate, double)
rsel(, id, isSampleTimeValid, BOOL)
rsel(, id, isHostTimeValid, BOOL)
rsel(, id, sampleTime, AVAudioFramePosition)

NS_RETURNS_RETAINED
rsel_a(, id, extrapolateTimeFromAnchor, AVAudioTime *, AVAudioTime * _Nullable)

#pragma mark - AVAudioCommonFormat

NS_RETURNS_RETAINED
asel_a(, AVAudioFormat, initWithStreamDescription, const AudioStreamBasicDescription *)

NS_RETURNS_RETAINED
asel_ab(, AVAudioFormat, initWithStreamDescription, const AudioStreamBasicDescription *, channelLayout, AVAudioChannelLayout * _Nullable)

NS_RETURNS_RETAINED
asel_ab(, AVAudioFormat, initStandardFormatWithSampleRate, double, channels, AVAudioChannelCount)

NS_RETURNS_RETAINED
asel_ab(, AVAudioFormat, initStandardFormatWithSampleRate, double, channelLayout, AVAudioChannelLayout *)

NS_RETURNS_RETAINED
asel_abcd(, AVAudioFormat, initWithCommonFormat, AVAudioCommonFormat, sampleRate, double, interleaved, BOOL, channelLayout, AVAudioChannelLayout *)

NS_RETURNS_RETAINED
asel_a(, AVAudioFormat, initWithSettings, NSDictionary *);

NS_RETURNS_NOT_RETAINED
rsel(, id, settings, NSDictionary *)
rsel(, id, isInterleaved, BOOL)
rsel(, id, commonFormat, AVAudioCommonFormat)
rsel(av_format_, AVAudioFormat *, channelCount, AVAudioChannelCount)
rsel(, id, streamDescription, const AudioStreamBasicDescription *)
NS_RETURNS_NOT_RETAINED
rsel(, id, channelLayout, AVAudioChannelLayout * _Nullable)

NS_RETURNS_NOT_RETAINED
rsel(, id, magicCookie, NSData * _Nullable)

#pragma mark - AVAudioBuffer

rsel(, id, format, AVAudioFormat *)
rsel(, id, audioBufferList, const AudioBufferList *)
rsel(, id, mutableAudioBufferList, AudioBufferList *)


//- (nullable instancetype)initWithPCMFormat:(AVAudioFormat *)format frameCapacity:(AVAudioFrameCount)frameCapacity

#pragma mark - AVAssetWriterInput

//+ (instancetype)assetWriterInputWithMediaType:(AVMediaType)mediaType outputSettings:(nullable NSDictionary<NSString *, id> *)outputSettings;
csel_ab(, AVAssetWriterInput, assetWriterInputWithMediaType, AVMediaType, outputSettings, NSDictionary * _Nullable, AVAssetWriterInput *)

//@property (nonatomic, readonly) AVMediaType mediaType;
rsel(, id, mediaType, AVMediaType)
rsel(, id, outputSettings, NSDictionary * _Nullable)

rsel(, id, isReadyForMoreMediaData, BOOL)

//@property (nonatomic) BOOL expectsMediaDataInRealTime;
rsel(, id, expectsMediaDataInRealTime, BOOL)
wsel_a(, id, setExpectsMediaDataInRealTime, BOOL)

//- (void)markAsFinished;
wsel(, id, markAsFinished)

//- (BOOL)appendSampleBuffer:(CMSampleBufferRef)sampleBuffer;
rsel_a(, id, appendSampleBuffer, CMSampleBufferRef, BOOL)

#pragma mark - AVAssetWriter

rwsel(, id, shouldOptimizeForNetworkUse, setShouldOptimizeForNetworkUse, BOOL)
//- (BOOL)canAddInput:(AVAssetWriterInput *)input;
rsel_a(AVAssetWriter_, AVAssetWriter *, canAddInput, AVAssetWriterInput *, BOOL)
wsel_a(AVAssetWriter_, AVAssetWriter *, addInput, AVAssetWriterInput *)

//- (BOOL)startWriting;
wsel(, id, startWriting)
wsel_a(, id, startSessionAtSourceTime, CMTime);

NS_ASSUME_NONNULL_END
