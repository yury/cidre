//
//  app.h
//  app
//
//  Created by Yury Korolev on 11/1/23.
//

#import <AppKit/AppKit.h>
#import <Foundation/Foundation.h>

NS_ASSUME_NONNULL_BEGIN

Class NS_APPLICATION;
Class NS_VIEW;
Class NS_COLOR;
Class NS_WINDOW;
Class NS_COLOR_SPACE;
Class NS_RESPONDER;
Class NS_VIEW_CONTROLLER;

__attribute__((constructor))
static void app_initializer(void)
{
    
    static int initialized = 0;
    if (!initialized) {
        NS_APPLICATION = [NSApplication class];
        NS_VIEW = [NSView class];
        NS_COLOR = [NSColor class];
        NS_WINDOW = [NSWindow class];
        NS_COLOR_SPACE = [NSColorSpace class];
        NS_RESPONDER = [NSResponder class];
        NS_VIEW_CONTROLLER = [NSViewController class];
        initialized = 1;
    }
}

NS_ASSUME_NONNULL_END
