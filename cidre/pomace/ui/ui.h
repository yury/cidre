//
//  ui.h
//  ui
//
//  Created by Yury Korolev on 25.05.2022.
//

#import <Foundation/Foundation.h>
#include "../macro.h"

#import <UIKit/UIKit.h>

NS_ASSUME_NONNULL_BEGIN

Class UI_DEVICE;

__attribute__((constructor))
static void ui_initializer(void)
{
  static int initialized = 0;
  if (!initialized) {
    UI_DEVICE = [UIDevice class];
  }
}

NS_ASSUME_NONNULL_END
