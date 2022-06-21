//
//  ca.h
//  ca
//
//  Created by Yury Korolev on 22.05.2022.
//

#import <Foundation/Foundation.h>
#include "../macro.h"

#import <QuartzCore/QuartzCore.h>

NS_ASSUME_NONNULL_BEGIN

API_AVAILABLE(ios(3.1), watchos(2.0), tvos(9.0)) API_UNAVAILABLE(macos)
@interface CidreDisplayLinkDelegate : NSObject {
  @public void * _vtable[2];
}

- (void)onDisplayLink:(CADisplayLink *)link;

@end

API_AVAILABLE(ios(3.1), watchos(2.0), tvos(9.0)) API_UNAVAILABLE(macos)
NS_RETURNS_RETAINED
CidreDisplayLinkDelegate * make_display_link_delegate(void * _Nonnull vtable[_Nonnull 2]) {
  CidreDisplayLinkDelegate * result = [CidreDisplayLinkDelegate new];
  memcpy(result->_vtable, vtable, 2 * sizeof(void *));
  return result;
}

API_AVAILABLE(ios(3.1), watchos(2.0), tvos(9.0)) API_UNAVAILABLE(macos)
NS_RETURNS_RETAINED
CADisplayLink * cidre_CADisplayLinkWithDelegate(CidreDisplayLinkDelegate * delegate) {
  return [CADisplayLink displayLinkWithTarget:delegate selector:@selector(onDisplayLink:)];
}
//NS_RETURNS_RETAINED
//csel(, CADisplayLink, new, CADisplayLink *)
wsel_ab(, CADisplayLink *, addToRunLoop, NSRunLoop *, forMode, NSRunLoopMode)
wsel_ab(, CADisplayLink *, removeFromRunLoop, NSRunLoop *, forMode, NSRunLoopMode)

//@property(readonly, nonatomic) CFTimeInterval timestamp;
rsel(, CADisplayLink *, timestamp, CFTimeInterval)
rsel(, CADisplayLink *, duration, CFTimeInterval)
rsel(, CADisplayLink *, targetTimestamp, CFTimeInterval)

rwsel(, CADisplayLink *, preferredFrameRateRange, setPreferredFrameRateRange, CAFrameRateRange)

NS_ASSUME_NONNULL_END

