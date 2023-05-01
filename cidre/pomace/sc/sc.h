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

Class SC_STREAM_CONFIGURATION;
Class SC_CONTENT_FILTER;
Class SC_STREAM;
Class SC_SHAREABLE_CONTENT;

__attribute__((constructor))
static void mtl_initializer(void)
{
  static int initialized = 0;
  if (!initialized) {
    SC_STREAM_CONFIGURATION = [SCStreamConfiguration class];
    SC_CONTENT_FILTER = [SCContentFilter class];
    SC_STREAM = [SCStream class];
    SC_SHAREABLE_CONTENT = [SCShareableContent class];
    
    initialized = 1;
  }
}

NS_ASSUME_NONNULL_END
