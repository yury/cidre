//
//  ci.h
//  ci
//
//  Created by Yury Korolev on 22.05.2022.
//

#import <Foundation/Foundation.h>
#import <CoreImage/CoreImage.h>
#import "../macro.h"

NS_ASSUME_NONNULL_BEGIN

#pragma mark - CIImage

NS_RETURNS_RETAINED
csel2(, CIImage, imageWithMTLTexture, id<MTLTexture>, options, NSDictionary * _Nullable, CIImage *)


#pragma mark - CIContext

NS_RETURNS_RETAINED
csel1(, CIContext, contextWithOptions, NSDictionary* _Nullable, CIContext *)

rsel4(, id, PNGRepresentationOfImage, CIImage *, format, CIFormat, colorSpace, CGColorSpaceRef, options, NSDictionary *, BOOL)

rsel6(, id, writePNGRepresentationOfImage, CIImage *, toURL, NSURL *, format, CIFormat, colorSpace, CGColorSpaceRef, options, NSDictionary *, error, NSError **, BOOL)

NS_ASSUME_NONNULL_END

