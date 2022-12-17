//
//  vn.h
//  vn
//
//  Created by Yury Korolev on 13.10.2022.
//


#import <Foundation/Foundation.h>
#import <Vision/Vision.h>
#import "../macro.h"

NS_ASSUME_NONNULL_BEGIN

#pragma mark - VNRequest

rwsel(, id, revision, setRevision, NSUInteger)
rwsel(, id, usesCPUOnly, setUsesCPUOnly, BOOL)

#pragma mark - VNImageBasedRequest

//@property (readwrite, nonatomic, assign) CGRect regionOfInterest;
rwsel(, id, regionOfInterest, setRegionOfInterest, CGRect)

#pragma mark - VNImageRequestHandler

NS_RETURNS_RETAINED
cinit2(, VNImageRequestHandler, initWithURL, NSURL *, options, NSDictionary *);

NS_RETURNS_RETAINED
cinit3(, VNImageRequestHandler, initWithURL, NSURL *, orientation, CGImagePropertyOrientation, options, NSDictionary *);

NS_RETURNS_RETAINED
cinit2(, VNImageRequestHandler, initWithCVPixelBuffer, CVPixelBufferRef, options, NSDictionary *);


NS_RETURNS_RETAINED
cinit3(, VNImageRequestHandler, initWithCVPixelBuffer, CVPixelBufferRef, orientation, CGImagePropertyOrientation, options, NSDictionary *);

rsel2(, id, performRequests, NSArray<VNRequest *> *, error, NSError **, BOOL)

#pragma mark - VNSequenceRequestHandler

csel0(, VNSequenceRequestHandler, new, VNSequenceRequestHandler *)

rsel3(, id, performRequests, NSArray<VNRequest *> *, onCVPixelBuffer, CVPixelBufferRef, error, NSError **, BOOL)

rsel3(, id, performRequests, NSArray<VNRequest *> *, onCMSampleBuffer, CMSampleBufferRef, error, NSError **, BOOL)

#pragma mark - VNObservation

rsel0(, id, uuid, NSUUID *)
rsel0(, id, confidence, VNConfidence)
rsel0(, id, timeRange, CMTimeRange)


#pragma mark - VNDetectedObjectObservation

//@property (readonly, nonatomic, assign) CGRect boundingBox;
rsel0(, id, boundingBox, CGRect)

rsel0(, id, globalSegmentationMask, VNPixelBufferObservation *)

#pragma mark -VNHorizon

NS_RETURNS_RETAINED
csel0(, VNDetectHorizonRequest, new, VNDetectHorizonRequest *)


#pragma mark - VNHorizonObservation

//@property (readonly, nonatomic, assign) CGAffineTransform transform;
rsel0(, id, transform, CGAffineTransform)

rsel0(, id, angle, CGFloat)
//@property (readonly, nonatomic, assign) CGFloat angle;

rsel2(, id, transformForImageWidth, size_t, height, size_t, CGAffineTransform)

#pragma mark - VNFaceObservation

rsel0(, id, landmarks, VNFaceLandmarks2D *)
rsel0(, id, faceCaptureQuality, NSNumber *)

rsel0(, id, roll, NSNumber *)
rsel0(, id, yaw, NSNumber *)
rsel0(, id, pitch, NSNumber *)

#pragma mark - VNClassificationObservation

rsel0(, id, identifier, NSString *)

rsel0(, id, hasPrecisionRecallCurve, BOOL)


//- (BOOL) hasMinimumRecall:(float)minimumRecall forPrecision:(float)precision;
rsel2(, id, hasMinimumRecall, float, forPrecision, float, BOOL)
//- (BOOL) hasMinimumPrecision:(float)minimumPrecision forRecall:(float)recall;
rsel2(, id, hasMinimumPrecision, float, forRecall, float, BOOL)

#pragma mark - VNHumanObservation

rsel0(, id, upperBodyOnly, BOOL)

#pragma mark - VNRecognizedObjectObservation

rsel0(, id, labels, NSArray *)

#pragma mark - VNPixelBufferObservation

CF_RETURNS_NOT_RETAINED
rsel0(, id, pixelBuffer, CVPixelBufferRef)

rsel0(, id, featureName, NSString *)

#pragma mark - VNTextObservation

rsel0(, id, characterBoxes, NSArray *)// NSArray<VNRectangleObservation *> *characterBoxes;


#pragma mark - VNGeneratePersonSegmentationRequest

rwsel(, id, qualityLevel, setQualityLevel, VNGeneratePersonSegmentationRequestQualityLevel)

//@property (readwrite, nonatomic) OSType outputPixelFormat;
rwsel(, id, outputPixelFormat, setOutputPixelFormat, OSType)

NS_RETURNS_RETAINED
csel0(, VNGeneratePersonSegmentationRequest, new, VNGeneratePersonSegmentationRequest *)


#pragma mark - VNDetectDocumentSegmentationRequest

NS_RETURNS_RETAINED
csel0(, VNDetectDocumentSegmentationRequest, new, VNDetectDocumentSegmentationRequest *)

#pragma mark - VNSaliencyImageObservation

rsel0(, id, salientObjects, NSArray *)

#pragma mark - VNGenerateAttentionBasedSaliencyImageRequest

NS_RETURNS_RETAINED
csel0(, VNGenerateAttentionBasedSaliencyImageRequest, new, VNGenerateAttentionBasedSaliencyImageRequest *)

#pragma mark - VNGenerateObjectnessBasedSaliencyImageRequest

NS_RETURNS_RETAINED
csel0(, VNGenerateObjectnessBasedSaliencyImageRequest, new, VNGenerateObjectnessBasedSaliencyImageRequest *)


#pragma mark - VNFaceLandmarkRegion

rsel0(, id, pointCount, NSUInteger)

#pragma mark - VNClassifyImageRequest

//rsel_a(, id, supportedIdentifiersAndReturnError, NSError **, NSArray *)

NS_RETURNS_RETAINED
csel0(, VNClassifyImageRequest, new, VNClassifyImageRequest *)

#pragma mark - VNRecognizeAnimalsRequest

NS_RETURNS_RETAINED
csel0(, VNRecognizeAnimalsRequest, new, VNRecognizeAnimalsRequest *)

#pragma mark - VNFeaturePrintObservation

rsel0(vn_, VNFeaturePrintObservation *, elementType, VNElementType)
rsel0(vn_, VNFeaturePrintObservation *, elementCount, NSUInteger)
rsel0(vn_, VNFeaturePrintObservation *, data, NSData *)

rsel3(, id, computeDistance, float *, toFeaturePrintObservation, VNFeaturePrintObservation *, error, NSError **, BOOL)

#pragma mark - VNGenerateImageFeaturePrintRequest

rwsel(, id, imageCropAndScaleOption, setImageCropAndScaleOption, VNImageCropAndScaleOption)

NS_RETURNS_RETAINED
csel0(, VNGenerateImageFeaturePrintRequest, new, VNGenerateImageFeaturePrintRequest *)

#pragma mark - VNDetectFaceRectanglesRequest

NS_RETURNS_RETAINED
csel0(, VNDetectFaceRectanglesRequest, new, VNDetectFaceRectanglesRequest *)

#pragma mark - VNTrackingRequest

rwsel(, id, trackingLevel, setTrackingLevel, VNRequestTrackingLevel)

#pragma mark - VNDetectBarcodesRequest
NS_RETURNS_RETAINED
csel0(, VNDetectBarcodesRequest, new, VNDetectBarcodesRequest *)

//@property (readwrite, nonatomic, copy) NSArray<VNBarcodeSymbology> *symbologies;
rwsel(, id, symbologies, setSymbologies, NSArray *)

rsel1(, id, supportedSymbologiesAndReturnError, NSError **, NSArray *)

#pragma mark - VNRecognizeTextRequest

csel0(, VNRecognizeTextRequest, new, VNRecognizeTextRequest *)

#pragma mark - VNRecognizedTextObservation

//- (NSArray<VNRecognizedText*>*) topCandidates:(NSUInteger)maxCandidateCount;
rsel1(, id, topCandidates, NSUInteger, NSArray *)

#pragma mark - VNRecognizedText

rsel0(, id, string, NSString *)

#pragma mark - SELECTORS

SEL sel_results;
SEL sel_supportedIdentifiersAndReturnError;

__attribute__((constructor))
static void common_initializer()
{
  static int initialized = 0;
  if (!initialized) {
    sel_results = @selector(results);
    sel_supportedIdentifiersAndReturnError = @selector(supportedIdentifiersAndReturnError:);
  }
}



NS_ASSUME_NONNULL_END

