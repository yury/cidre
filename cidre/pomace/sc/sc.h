//
//  sc.h
//  sc
//
//  Created by Yury Korolev on 17.04.2022.
//

#import <ScreenCaptureKit/ScreenCaptureKit.h>

NS_ASSUME_NONNULL_BEGIN

Class SC_STREAM_CONFIGURATION;
Class SC_CONTENT_FILTER;
Class SC_STREAM;
Class SC_SHAREABLE_CONTENT;

Class SC_RECORDING_OUTPUT_CONFIGURATION = nil;
Class SC_RECORDING_OUTPUT = nil;
Class SC_CONTENT_SHARING_PICKER_CONFIGURATION = nil;
Class SC_CONTENT_SHARING_PICKER = nil;

__attribute__((constructor))
static void sc_initializer(void)
{
    static int initialized = 0;
    if (!initialized) {
        initialized = 1;
        
        SC_STREAM_CONFIGURATION = [SCStreamConfiguration class];
        SC_CONTENT_FILTER = [SCContentFilter class];
        SC_STREAM = [SCStream class];
        SC_SHAREABLE_CONTENT = [SCShareableContent class];
        
        SC_RECORDING_OUTPUT_CONFIGURATION = NSClassFromString(@"SCRecordingOutputConfiguration");
        SC_RECORDING_OUTPUT = NSClassFromString(@"SCRecordingOutput");
        SC_CONTENT_SHARING_PICKER_CONFIGURATION = NSClassFromString(@"SCContentSharingPickerConfiguration");
        SC_CONTENT_SHARING_PICKER = NSClassFromString(@"SCContentSharingPicker");
        
        
    }
}

NS_ASSUME_NONNULL_END
