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

//@property(nonatomic, assign) OSType pixelFormat;
rwsel(sc_, SCStreamConfiguration *, pixelFormat, setPixelFormat, OSType)
//@property(nonatomic, assign) BOOL scalesToFit;
rwsel(, id, scalesToFit, setScalesToFit, BOOL)
// @property(nonatomic, assign) BOOL showsCursor;
rwsel(, id, showsCursor, setShowsCursor, BOOL)
// @property(nonatomic, assign) CGColorRef backgroundColor;
rwsel(sc_, SCStreamConfiguration *, backgroundColor, setBackgroundColor, CGColorRef)
//@property(nonatomic, assign) CGRect sourceRect;
rwsel(_, id, sourceRect, setSourceRect, CGRect)
//@property(nonatomic, assign) CGRect destinationRect;
rwsel(_, id, destinationRect, setDestinationRect, CGRect)

__attribute__((constructor))
static void cs_initializer()
{
}

NS_ASSUME_NONNULL_END
