//
//  ns.h
//  ns
//
//  Created by Yury Korolev on 07.07.2022.
//

#import <Foundation/Foundation.h>
#include "../macro.h"

NS_ASSUME_NONNULL_BEGIN

void cidre_raise_exception(NSString *message) {
  [NSException raise:NSGenericException format:@"%@", message];
}

void cidre_throw(id ex) {
  @throw ex;
}

id _Nullable cidre_try_catch(void (*during)(void *), void * context ) {
  @try {
    during(context);
    return nil;
  } @catch (id e) {
    return e;
  }
}

#pragma mark - NSNumber

NS_RETURNS_RETAINED
csel1(, NSNumber, numberWithInteger, NSInteger, NSNumber *)

Class NS_ARRAY;
Class NS_DATA;
Class NS_DATE;
Class NS_DICTIONARY;
Class NS_MACH_PORT;
Class NS_MUTABLE_ARRAY;
Class NS_MUTABLE_DATA;
Class NS_MUTABLE_DICTIONARY;
Class NS_MUTABLE_SET;
Class NS_MUTABLE_STRING;
Class NS_MUTABLE_URL_REQUEST;
Class NS_NUMBER;
Class NS_OBJECT;
Class NS_PORT;
Class NS_PROCESS_INFO;
Class NS_REGULAR_EXPRESSION;
Class NS_RUN_LOOP;
Class NS_SET;
Class NS_STRING;
Class NS_TIMER;
Class NS_URL;
Class NS_URL_CACHE;
Class NS_URL_REQUEST;
Class NS_URL_RESPONSE;
Class NS_URL_SESSION;
Class NS_URL_SESSION_WEB_SOCKET_MESSAGE;
Class NS_UUID;
Class NS_VALUE;

Class NS_FILE_MANAGER;

__attribute__((constructor))
static void common_initializer(void)
{
  static int initialized = 0;
  if (!initialized) {

    NS_ARRAY = [NSArray class];
    NS_DATA = [NSData class];
    NS_DATE = [NSDate class];
    NS_DICTIONARY = [NSDictionary class];
    NS_MACH_PORT = [NSMachPort class];
    NS_MUTABLE_ARRAY = [NSMutableArray class];
    NS_MUTABLE_DATA = [NSMutableData class];
    NS_MUTABLE_DICTIONARY= [NSMutableDictionary class];
    NS_MUTABLE_SET = [NSMutableSet class];
    NS_MUTABLE_STRING = [NSMutableString class];
    NS_MUTABLE_URL_REQUEST = [NSMutableURLRequest class];
    NS_NUMBER = [NSNumber class];
    NS_OBJECT = [NSObject class];
    NS_PORT = [NSPort class];
    NS_PROCESS_INFO = [NSProcessInfo class];
    NS_REGULAR_EXPRESSION = [NSRegularExpression class];
    NS_RUN_LOOP = [NSRunLoop class];
    NS_SET = [NSSet class];
    NS_STRING = [NSString class];
    NS_TIMER = [NSTimer class];
    NS_URL = [NSURL class];
    NS_URL_CACHE = [NSURLCache class];
    NS_URL_REQUEST = [NSURLRequest class];
    NS_URL_RESPONSE = [NSURLResponse class];
    NS_URL_SESSION = [NSURLSession class];
    NS_URL_SESSION_WEB_SOCKET_MESSAGE = [NSURLSessionWebSocketMessage class];
    NS_UUID = [NSUUID class];
    NS_VALUE = [NSValue class];
    
    NS_FILE_MANAGER = [NSFileManager class];
  }
}
NS_ASSUME_NONNULL_END
