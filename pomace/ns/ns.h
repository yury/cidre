//
//  ns.h
//  ns
//
//  Created by Yury Korolev on 07.07.2022.
//

#import <Foundation/Foundation.h>
#include "../macro.h"

NS_ASSUME_NONNULL_BEGIN

#pragma mark - Common

wsel2(, id, scheduleInRunLoop, NSRunLoop *, forMode, NSRunLoopMode)
wsel2(, id, removeFromRunLoop, NSRunLoop *, forMode, NSRunLoopMode)

#pragma mark - NSPort

csel0(, NSPort, port, NSPort *)
rsel0(, id, machPort, uint32_t)

@interface CidreMachPortDelegate : NSObject<NSMachPortDelegate> {
  @public void * _vtable[2];
}
@end

NS_RETURNS_RETAINED
CidreMachPortDelegate * make_mach_port_delegate(void * _Nonnull vtable[_Nonnull 2]) {
  CidreMachPortDelegate * result = [CidreMachPortDelegate new];
  memcpy(result->_vtable, vtable, 2 * sizeof(void *));
  return result;
}

#pragma mark - NSProcessInfo

NS_RETURNS_NOT_RETAINED
csel0(, NSProcessInfo, processInfo, NSProcessInfo *)

rsel0(, id, isLowPowerModeEnabled, BOOL)
rsel0(, id, processorCount, NSUInteger)
rsel0(, id, activeProcessorCount, NSUInteger)

rsel0(, id, isMacCatalystApp, BOOL)
rsel0(, id, isiOSAppOnMac, BOOL)

void cidre_raise_exception(NSString *message) {
  [NSException raise:NSGenericException format:@"%@", message];
}

void cidre_throw_exception(NSString *message) {
  @throw message;
}

id _Nullable cidre_try_catch(void (*during)(void *), void * context ) {
  @try {
    during(context);
    return nil;
  } @catch (id e) {
    return e;
  }
}

#pragma mark - NSURLSession

NS_RETURNS_NOT_RETAINED
csel0(, NSURLSession, sharedSession, NSURLSession *)

rsel1(, id, dataTaskWithURL, NSURL *, NSURLSessionDataTask *)
rsel1(, id, dataTaskWithRequest, NSURLRequest *, NSURLSessionDataTask *)

#pragma mark - NSURLSessionTask

wsel0(NSURLSessionTask_, NSURLSessionTask *, resume)
wsel0(NSURLSessionTask_, NSURLSessionTask *, cancel)
wsel0(NSURLSessionTask_, NSURLSessionTask *, suspend)
rsel0(NSURLSessionTask_, NSURLSessionTask *, state, NSURLSessionTaskState)
rsel0(NSURLSessionTask_, NSURLSessionTask *, error, NSError *)

rsel0(NSURLSessionTask_, NSURLSessionTask *, taskIdentifier, NSUInteger)
rsel0(NSURLSessionTask_, NSURLSessionTask *, originalRequest, NSURLRequest * _Nullable)
rsel0(NSURLSessionTask_, NSURLSessionTask *, currentRequest, NSURLRequest * _Nullable)
rsel0(NSURLSessionTask_, NSURLSessionTask *, response, NSURLResponse * _Nullable)

rwsel(NSURLSessionTask_, NSURLSessionTask *, priority, setPriority, float)

#pragma mark - NSURLRequest

NS_RETURNS_RETAINED
csel1(, NSURLRequest, requestWithURL, NSURL *, NSURLRequest *)

NS_RETURNS_RETAINED
csel3(, NSURLRequest, requestWithURL, NSURL *, cachePolicy, NSURLRequestCachePolicy, timeoutInterval, NSTimeInterval, NSURLRequest *)

rsel0(NSURLRequest_, NSURLRequest *, cachePolicy, NSURLRequestCachePolicy)
rsel0(NSURLRequest_, NSURLRequest *, timeoutInterval, NSTimeInterval)
rsel0(NSURLRequest_, NSURLRequest *, networkServiceType, NSURLRequestNetworkServiceType)
rsel0(NSURLRequest_, NSURLRequest *, allowsCellularAccess, BOOL)
rsel0(NSURLRequest_, NSURLRequest *, allowsExpensiveNetworkAccess, BOOL)
rsel0(NSURLRequest_, NSURLRequest *, allowsConstrainedNetworkAccess, BOOL)
rsel0(NSURLRequest_, NSURLRequest *, assumesHTTP3Capable, BOOL)
rsel0(NSURLRequest_, NSURLRequest *, attribution, NSURLRequestAttribution)
rsel0(NSURLRequest_, NSURLRequest *, requiresDNSSECValidation, BOOL)

rsel0(NSURLRequest_, NSURLRequest *, HTTPMethod, NSString *)
rsel0(NSURLRequest_, NSURLRequest *, allHTTPHeaderFields, NSDictionary * _Nullable)

rsel1(NSURLRequest_, NSURLRequest *, valueForHTTPHeaderField, NSString *, NSString * _Nullable)
rsel0(NSURLRequest_, NSURLRequest *, HTTPBody, NSData * _Nullable)

#pragma mark - NSMutableURLRequest

NS_RETURNS_RETAINED
csel1(, NSMutableURLRequest, requestWithURL, NSURL *, NSURLRequest *)

NS_RETURNS_RETAINED
csel3(, NSMutableURLRequest, requestWithURL, NSURL *, cachePolicy, NSURLRequestCachePolicy, timeoutInterval, NSTimeInterval, NSURLRequest *)

rsel0(NSMutableURLRequest_, NSMutableURLRequest *, cachePolicy, NSURLRequestCachePolicy)

wsel1(NSMutableURLRequest_, NSMutableURLRequest *, setCachePolicy, NSURLRequestCachePolicy)
wsel1(NSMutableURLRequest_, NSMutableURLRequest *, setTimeoutInterval, NSTimeInterval)
wsel1(NSMutableURLRequest_, NSMutableURLRequest *, setNetworkServiceType, NSURLRequestNetworkServiceType)
wsel1(NSMutableURLRequest_, NSMutableURLRequest *, setAllowsCellularAccess, BOOL)
wsel1(NSMutableURLRequest_, NSMutableURLRequest *, setAllowsExpensiveNetworkAccess, BOOL)
wsel1(NSMutableURLRequest_, NSMutableURLRequest *, setAllowsConstrainedNetworkAccess, BOOL)
wsel1(NSMutableURLRequest_, NSMutableURLRequest *, setAssumesHTTP3Capable, BOOL)
wsel1(NSMutableURLRequest_, NSMutableURLRequest *, setAttribution, NSURLRequestAttribution)
wsel1(NSMutableURLRequest_, NSMutableURLRequest *, setRequiresDNSSECValidation, BOOL)

wsel1(NSMutableURLRequest_, NSMutableURLRequest *, setHTTPMethod, NSString * _Nullable)
wsel1(NSMutableURLRequest_, NSMutableURLRequest *, setAllHTTPHeaderFields, NSDictionary * _Nullable)
wsel1(NSMutableURLRequest_, NSMutableURLRequest *, setHTTPBody, NSData * _Nullable)


#pragma mark NSURLResponse

asel4(, NSURLResponse, initWithURL, NSURL *, MIMEType, NSString *, expectedContentLength, NSInteger, textEncodingName, NSString *)

#pragma mark NSURLSessionWebSocketMessage

asel1(, NSURLSessionWebSocketMessage, initWithData, NSData *)
asel1(, NSURLSessionWebSocketMessage, initWithString, NSString *)

rsel0(NSURLSessionWebSocketMessage_, NSURLSessionWebSocketMessage *, type, NSURLSessionWebSocketMessageType)
rsel0(NSURLSessionWebSocketMessage_, NSURLSessionWebSocketMessage *, data, NSData * _Nullable)
rsel0(NSURLSessionWebSocketMessage_, NSURLSessionWebSocketMessage *, string, NSString * _Nullable)

#pragma mark NSURLCache

csel0(, NSURLCache, sharedURLCache, NSURLCache *)

asel3(, NSURLCache, initWithMemoryCapacity, NSUInteger, diskCapacity, NSUInteger, directoryURL, NSURL * _Nullable)

#pragma mark NSData

NS_RETURNS_RETAINED
csel3(, NSData, dataWithContentsOfFile, NSString *, options, NSDataReadingOptions, error, NSError **, NSData *)

NS_RETURNS_RETAINED
csel3(, NSData, dataWithContentsOfURL, NSURL *, options, NSDataReadingOptions, error, NSError **, NSData *)

rsel2(, id, writeToFile,NSString *,atomically, BOOL, BOOL)

#pragma mark - NSNumber




NS_RETURNS_RETAINED
csel1(, NSNumber, numberWithChar, char, NSNumber *)

NS_RETURNS_RETAINED
csel1(, NSNumber, numberWithUnsignedChar, unsigned char, NSNumber *)

NS_RETURNS_RETAINED
csel1(, NSNumber, numberWithShort, short, NSNumber *)

NS_RETURNS_RETAINED
csel1(, NSNumber, numberWithUnsignedShort, unsigned short, NSNumber *)


NS_RETURNS_RETAINED
csel1(, NSNumber, numberWithInt, int, NSNumber *)

NS_RETURNS_RETAINED
csel1(, NSNumber, numberWithUnsignedInt, unsigned int, NSNumber *)

NS_RETURNS_RETAINED
csel1(, NSNumber, numberWithLongLong, long long, NSNumber *)

NS_RETURNS_RETAINED
csel1(, NSNumber, numberWithUnsignedLongLong, unsigned long long, NSNumber *)

NS_RETURNS_RETAINED
csel1(, NSNumber, numberWithFloat, float, NSNumber *)

NS_RETURNS_RETAINED
csel1(, NSNumber, numberWithDouble, double, NSNumber *)

NS_RETURNS_RETAINED
csel1(, NSNumber, numberWithBool, BOOL, NSNumber *)

NS_RETURNS_RETAINED
csel1(, NSNumber, numberWithInteger, NSInteger, NSNumber *)

NS_RETURNS_RETAINED
csel1(, NSNumber, numberWithUnsignedInteger, NSUInteger, NSNumber *)

rsel1(, id, isEqualToNumber, NSNumber *, BOOL)

#pragma mark - NSArray

NS_RETURNS_RETAINED
NSArray * NSArray_withObjs(id _Nullable * _Nonnull objects, NSUInteger count) {
  return [NSArray arrayWithObjects:objects count: count];
}

NS_RETURNS_RETAINED
csel0(, NSArray, array, NSArray *)

#pragma mark - NSSet

NS_RETURNS_RETAINED
NSSet * NSSet_withObjs(id _Nullable * _Nonnull objects, NSUInteger count) {
  return [NSSet setWithObjects:objects count: count];
}

NS_RETURNS_RETAINED
csel0(, NSSet, set, NSSet *)


#pragma mark - NSRegularExpression

NS_RETURNS_RETAINED
csel3(, NSRegularExpression, regularExpressionWithPattern, NSString *, options, NSRegularExpressionOptions, error, NSError **, NSRegularExpression *)

#pragma mark - NSString

NS_RETURNS_RETAINED
asel3(, NSString, initWithBytes, const void * _Nonnull, length, NSUInteger, encoding, NSStringEncoding)

NS_RETURNS_RETAINED
asel4(, NSString, initWithBytesNoCopy, void * _Nonnull, length, NSUInteger, encoding, NSStringEncoding, freeWhenDone, BOOL)

#pragma mark - NSURL

NS_RETURNS_RETAINED
csel3(, NSURL, fileURLWithPath, NSString *, isDirectory, BOOL, relativeToURL, NSURL *, NSURL *)

NS_RETURNS_RETAINED
csel2(, NSURL, URLWithString, NSString *, relativeToURL, NSURL *, NSURL *)

rsel0(, id, absoluteString, NSString *)

#pragma mark - NSDictionary

csel0(, NSDictionary, dictionary, NSDictionary *)

#pragma mark - SELECTORS

SEL ns_bytes;


SEL ns_isEqual;

SEL ns_resultType;
SEL ns_range;

SEL ns_code;
SEL ns_domain;

SEL ns_respondsToSelector;
SEL ns_lengthOfBytesUsingEncoding;
//SEL ns_retainCount;


__attribute__((constructor))
static void common_initializer()
{
  static int initialized = 0;
  if (!initialized) {
    ns_bytes = @selector(bytes);
    
    ns_isEqual = @selector(isEqual:);

    ns_resultType = @selector(resultType);
    ns_range = @selector(range);
    
    ns_code = @selector(code);
    ns_domain = @selector(domain);
    
    ns_respondsToSelector = @selector(respondsToSelector:);
    ns_lengthOfBytesUsingEncoding = @selector(lengthOfBytesUsingEncoding:);
    
//    ns_retainCount = @selector(retainCount);

  }
}
NS_ASSUME_NONNULL_END
