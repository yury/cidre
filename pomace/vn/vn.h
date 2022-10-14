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

rsel(, id, results, NSArray *)
rwsel(, id, revision, setRevision, NSUInteger)
rwsel(, id, usesCPUOnly, setUsesCPUOnly, BOOL)

#pragma mark - VNImageBasedRequest

//@property (readwrite, nonatomic, assign) CGRect regionOfInterest;
rwsel(, id, regionOfInterest, setRegionOfInterest, CGRect)

#pragma mark - VNImageRequestHandler

NS_RETURNS_RETAINED
cinit_ab(, VNImageRequestHandler, initWithCVPixelBuffer, CVPixelBufferRef, options, NSDictionary *);


NS_RETURNS_RETAINED
cinit_abc(, VNImageRequestHandler, initWithCVPixelBuffer, CVPixelBufferRef, orientation, CGImagePropertyOrientation, options, NSDictionary *);

rsel_ab(, id, performRequests, NSArray<VNRequest *> *, error, NSError **, BOOL)

#pragma mark - VNSequenceRequestHandler

csel(, VNSequenceRequestHandler, new, VNSequenceRequestHandler *)

rsel_abc(, id, performRequests, NSArray<VNRequest *> *, onCVPixelBuffer, CVPixelBufferRef, error, NSError **, BOOL)

rsel_abc(, id, performRequests, NSArray<VNRequest *> *, onCMSampleBuffer, CMSampleBufferRef, error, NSError **, BOOL)



NS_ASSUME_NONNULL_END

