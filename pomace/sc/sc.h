//
//  sc.h
//  sc
//
//  Created by Yury Korolev on 17.04.2022.
//

#import <ScreenCaptureKit/ScreenCaptureKit.h>
#include "macro.h"

NS_ASSUME_NONNULL_BEGIN

NS_RETURNS_RETAINED csel(, SCStreamConfiguration, new, SCStreamConfiguration *)

rwsel(sc_, SCStreamConfiguration *, width, setWidth, size_t)
rwsel(sc_, SCStreamConfiguration *, height, setHeight, size_t)
rwsel(, id, minimumFrameInterval, setMinimumFrameInterval, CMTime)

__attribute__((constructor))
static void cs_initializer()
{
}

NS_ASSUME_NONNULL_END
