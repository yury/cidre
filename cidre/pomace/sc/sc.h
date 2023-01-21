//
//  sc.h
//  sc
//
//  Created by Yury Korolev on 17.04.2022.
//

#import <ScreenCaptureKit/ScreenCaptureKit.h>
#include "../macro.h"
#include "Block.h"

NS_ASSUME_NONNULL_BEGIN

@interface SidreStreamDelegate : NSObject<SCStreamDelegate> {
  @public void * _vtable[2];
}

@end

NS_RETURNS_RETAINED
SidreStreamDelegate * make_stream_delegate(void * _Nonnull vtable[_Nonnull 2]) {
  SidreStreamDelegate * result = [SidreStreamDelegate new];
  memcpy(result->_vtable, vtable, 2 * sizeof(void *));
  return result;
}


@interface SidreStreamOutDelegate : NSObject<SCStreamOutput> {
  @public void * _vtable[2];
}

@end


NS_RETURNS_RETAINED
SidreStreamOutDelegate * make_stream_out(void * _Nonnull vtable) {
  SidreStreamOutDelegate * result = [SidreStreamOutDelegate new];
  memcpy(result->_vtable, vtable, 2 * sizeof(void *));
  return result;
}


Class SC_STREAM_CONFIGURATION;
Class SC_CONTENT_FILTER;
Class SC_STREAM;
Class SC_SHAREABLE_CONTENT;

__attribute__((constructor))
static void common_initializer()
{
  static int initialized = 0;
  if (!initialized) {
    SC_STREAM_CONFIGURATION = [SCStreamConfiguration class];
    SC_CONTENT_FILTER = [SCContentFilter class];
    SC_STREAM = [SCStream class];
    SC_SHAREABLE_CONTENT = [SCShareableContent class];
  }
}

NS_ASSUME_NONNULL_END
