//
//  ca.m
//  ca
//
//  Created by Yury Korolev on 22.05.2022.
//

#import "ca.h"

API_AVAILABLE(ios(3.1), watchos(2.0), tvos(9.0)) API_UNAVAILABLE(macos)
@implementation CidreDisplayLinkDelegate

- (void)onDisplayLink:(CADisplayLink *)link {
  void(*cb)(void *, CADisplayLink *) = _vtable[1];
  cb( _vtable[0], link);
}

@end

