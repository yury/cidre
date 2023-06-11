//
//  ns.m
//  ns
//
//  Created by Yury Korolev on 07.07.2022.
//

#import "ns.h"

@implementation CidreObserver {
  __weak NSObject * _obj;
  NSString *_keyPath;
  void * _context;
  cidre_change * _fn_ptr;
}

- (instancetype)initWithObject: (NSObject *)obj
                       keyPath: (NSString *)keyPath
                       options: (NSKeyValueObservingOptions)options
                       context: (void *)context
                         fnPtr: (cidre_change *)fn_ptr
{
  if (self = [super init]) {
    _obj = obj;
    _keyPath = keyPath;
    _context = context;
    _fn_ptr = fn_ptr;
    [_obj addObserver:self forKeyPath:keyPath options:options context:context];
  }
  return self;
}

- (void)observeValueForKeyPath:(nullable NSString *)keyPath ofObject:(nullable id)object change:(nullable NSDictionary<NSKeyValueChangeKey,id> *)change context:(nullable void *)context {
  _fn_ptr(context, keyPath, object, change);
}

- (void)invalidate {
  [_obj removeObserver:self forKeyPath:_keyPath context:_context];
}

@end
