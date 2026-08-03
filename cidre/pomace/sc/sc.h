//
//  sc.h
//  sc
//
//  Created by Yury Korolev on 17.04.2022.
//

#if __has_include(<ScreenCaptureKit/ScreenCaptureKit.h>)
#import <ScreenCaptureKit/ScreenCaptureKit.h>
#else
#import <Foundation/Foundation.h>
#endif

NS_ASSUME_NONNULL_BEGIN

Class SC_STREAM_CONFIGURATION;
Class SC_CONTENT_FILTER;
Class SC_STREAM;
Class SC_SHAREABLE_CONTENT;

Class SC_RECORDING_OUTPUT_CONFIGURATION;
Class SC_RECORDING_OUTPUT;
Class SC_CLIP_BUFFERING_OUTPUT;
Class SC_CONTENT_SHARING_PICKER_CONFIGURATION;
Class SC_CONTENT_SHARING_PICKER;
Class SC_RECORDING_EDITOR;
Class SC_SCREENSHOT_MANAGER;
Class SC_SCREENSHOT_CONFIGURATION;
Class SC_SCREENSHOT_OUTPUT;
Class SC_VIDEO_EFFECT_OUTPUT;

__attribute__((constructor))
static void sc_initializer(void)
{
    static int initialized = 0;
    if (!initialized) {
        initialized = 1;
        
        SC_STREAM_CONFIGURATION = NSClassFromString(@"SCStreamConfiguration");
        SC_CONTENT_FILTER = NSClassFromString(@"SCContentFilter");
        SC_STREAM = NSClassFromString(@"SCStream");
        SC_SHAREABLE_CONTENT = NSClassFromString(@"SCShareableContent");
        
        SC_RECORDING_OUTPUT_CONFIGURATION = NSClassFromString(@"SCRecordingOutputConfiguration");
        SC_RECORDING_OUTPUT = NSClassFromString(@"SCRecordingOutput");
        SC_CLIP_BUFFERING_OUTPUT = NSClassFromString(@"SCClipBufferingOutput");
        SC_CONTENT_SHARING_PICKER_CONFIGURATION = NSClassFromString(@"SCContentSharingPickerConfiguration");
        SC_CONTENT_SHARING_PICKER = NSClassFromString(@"SCContentSharingPicker");
        SC_RECORDING_EDITOR = NSClassFromString(@"SCRecordingEditor");
        SC_SCREENSHOT_MANAGER = NSClassFromString(@"SCScreenshotManager");
        SC_SCREENSHOT_CONFIGURATION = NSClassFromString(@"SCScreenshotConfiguration");
        SC_SCREENSHOT_OUTPUT = NSClassFromString(@"SCScreenshotOutput");
        SC_VIDEO_EFFECT_OUTPUT = NSClassFromString(@"SCVideoEffectOutput");
        
    }
}

NS_ASSUME_NONNULL_END
