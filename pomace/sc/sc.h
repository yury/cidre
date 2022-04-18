//
//  sc.h
//  sc
//
//  Created by Yury Korolev on 17.04.2022.
//

#import <ScreenCaptureKit/ScreenCaptureKit.h>
#include "macro.h"

NS_ASSUME_NONNULL_BEGIN

#pragma mark - SCStreamConfiguration

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

#pragma mark SCRunningApplication

// @property (readonly) NSString *bundleIdentifier;
NS_RETURNS_NOT_RETAINED
rsel(, id, bundleIdentifier, NSString *)

NS_RETURNS_NOT_RETAINED
rsel(, id, applicationName, NSString *)

//@property (readonly) pid_t processID;
rsel(, id, processID, pid_t)

#pragma mark - SCWindow

//@property (readonly) CGWindowID windowID;
rsel(, id, windowID, CGWindowID)

#pragma mark - SCShareableContent

//@property (readonly) NSArray<SCWindow *> *windows;
rsel(, id, windows, NSArray<SCWindow *> *)
rsel(, id, displays, NSArray<SCDisplay *> *)
rsel(, id, applications, NSArray<SCRunningApplication *> *)

void cs_shareable_content_with_completion_handler(void * _Nonnull * _Nonnull rb) {
  [SCShareableContent getShareableContentWithCompletionHandler:^(SCShareableContent * _Nullable shareableContent, NSError * _Nullable error) {
    void(*ch)(void *, SCShareableContent * _Nullable, NSError * _Nullable error) = rb[0];
    ch(rb, shareableContent, error);
  }];
}

#pragma mark - SCContentFilter

cinit_ab(, SCContentFilter, initWithDisplay, SCDisplay *, excludingWindows, NSArray<SCWindow *>*)


__attribute__((constructor))
static void cs_initializer()
{
}

NS_ASSUME_NONNULL_END
