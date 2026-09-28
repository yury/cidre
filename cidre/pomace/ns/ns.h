//
//  ns.h
//  ns
//
//  Created by Yury Korolev on 07.07.2022.
//

#import <Foundation/Foundation.h>
#import "objc/objc.h"
#import "objc/objc-exception.h"
#import <dlfcn.h>

NS_ASSUME_NONNULL_BEGIN

void cidre_raise_exception(NSString *message) {
    [NSException raise:NSGenericException format:@"%@", message];
}

id _Nullable cidre_try_catch(void (*during)(void *), void * context ) {
    @try {
        during(context);
        return nil;
    } @catch (id e) {
        return e;
    }
}

Class NS_URL_SESSION_WEB_SOCKET_MESSAGE;

Class NS_ORDERED_COLLECTION_CHANGE;
Class NS_ORDERED_COLLECTION_DIFFERENCE;








typedef void cidre_change(
                          void * _Nullable,
                          NSString * _Nullable,
                          id _Nullable,
                          NSDictionary<NSKeyValueChangeKey,id> * _Nullable
                          );

@interface CidreObserver : NSObject
- (instancetype)initWithObject: (NSObject *)obj
                       keyPath: (NSString *)keyPath
                       options: (NSKeyValueObservingOptions)options
                       context: (void *)context
                         fnPtr: (cidre_change *)fn_ptr;

- (void)invalidate;
@end

NS_RETURNS_RETAINED CidreObserver *
cidre_create_observer(
                      NSObject * obj,
                      NSString * keyPath,
                      NSKeyValueObservingOptions options,
                      void * context,
                      cidre_change * fn_ptr
                      ) {
    return [[CidreObserver alloc] initWithObject:obj keyPath:keyPath options:options context:context fnPtr:fn_ptr];
}

void cidre_log(NSString * str) {
    NSLog(@"%@", str);
}


__attribute__((constructor))
static void common_initializer(void)
{
    static int initialized = 0;
    if (!initialized) {
        initialized = 1;
        
        NS_URL_SESSION_WEB_SOCKET_MESSAGE = [NSURLSessionWebSocketMessage class];
        
        

     
        
        
        NS_ORDERED_COLLECTION_DIFFERENCE = [NSOrderedCollectionDifference class];
        NS_ORDERED_COLLECTION_CHANGE = [NSOrderedCollectionChange class];
    }
}
NS_ASSUME_NONNULL_END
