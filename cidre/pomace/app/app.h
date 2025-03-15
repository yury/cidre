//
//  app.h
//  app
//
//  Created by Yury Korolev on 11/1/23.
//

#import <AppKit/AppKit.h>

NS_ASSUME_NONNULL_BEGIN

Class NS_APPLICATION;
Class NS_CELL;
Class NS_COLOR_SPACE;
Class NS_VIEW;
Class NS_COLOR;
Class NS_WINDOW;
Class NS_COLOR_SPACE;
Class NS_RESPONDER;
Class NS_SCREEN;
Class NS_VIEW_CONTROLLER;
Class NS_WINDOW_CONTROLLER;
Class NS_WORKSPACE;
Class NS_WORKSPACE_OPEN_CONFIGURATION;
Class NS_RUNNING_APPLICATION;
Class NS_TEXT_ATTACHMENT;
Class NS_IMAGE;
Class NS_EVENT;
Class NS_MENU;

__attribute__((constructor))
static void app_initializer(void)
{
    
    static int initialized = 0;
    if (!initialized) {
        initialized = 1;
        
        NS_APPLICATION = [NSApplication class];
        NS_CELL = [NSCell class];
        NS_COLOR_SPACE = [NSColorSpace class];
        NS_VIEW = [NSView class];
        NS_COLOR = [NSColor class];
        NS_WINDOW = [NSWindow class];
        NS_COLOR_SPACE = [NSColorSpace class];
        NS_RESPONDER = [NSResponder class];
        NS_SCREEN = [NSScreen class];
        NS_VIEW_CONTROLLER = [NSViewController class];
        NS_WINDOW_CONTROLLER = [NSWindowController class];
        NS_WORKSPACE = [NSWorkspace class];
        NS_WORKSPACE_OPEN_CONFIGURATION = [NSWorkspaceOpenConfiguration class];
        NS_RUNNING_APPLICATION = [NSRunningApplication class];
    
        
        NS_TEXT_ATTACHMENT = [NSTextAttachment class];
        NS_IMAGE = [NSImage class];
        NS_EVENT = [NSEvent class];
        NS_MENU = [NSMenu class];
    }
}

NS_ASSUME_NONNULL_END
