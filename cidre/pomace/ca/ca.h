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
Class CA_DISPLAY_LINK;

__attribute__((constructor))
static void ca_initializer(void)
{
  static int initialized = 0;
  if (!initialized) {
    CA_DISPLAY_LINK = [CADisplayLink class];
  }
}


NS_ASSUME_NONNULL_END

