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

#pragma mark - VNImageRequestHandler

NS_RETURNS_RETAINED
cinit_ab(, VNImageRequestHandler, initWithCVPixelBuffer, CVPixelBufferRef, options, NSDictionary *);


NS_RETURNS_RETAINED
cinit_abc(, VNImageRequestHandler, initWithCVPixelBuffer, CVPixelBufferRef, orientation, CGImagePropertyOrientation, options, NSDictionary *);

rsel_ab(, id, performRequests, NSArray<VNRequest *> *, error, NSError **, BOOL)


#pragma mark - VNSequenceRequestHandler

csel(, VNSequenceRequestHandler, new, VNSequenceRequestHandler *)


//csel(, VNRequest, new, VNRequest *)

//NS_RETURNS_RETAINED
//csel_ab(, CIImage, imageWithMTLTexture, id<MTLTexture>, options, NSDictionary * _Nullable, CIImage *)
//
//
//#pragma mark - CIContext
//
//NS_RETURNS_RETAINED
//csel_a(, CIContext, contextWithOptions, NSDictionary* _Nullable, CIContext *)
//
//rsel_abcd(, id, PNGRepresentationOfImage, CIImage *, format, CIFormat, colorSpace, CGColorSpaceRef, options, NSDictionary *, BOOL)
//
//rsel_abcdef(, id, writePNGRepresentationOfImage, CIImage *, toURL, NSURL *, format, CIFormat, colorSpace, CGColorSpaceRef, options, NSDictionary *, error, NSError **, BOOL)

NS_ASSUME_NONNULL_END

