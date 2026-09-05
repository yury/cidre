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
Class NS_MENU_ITEM;
Class NS_PANEL;
Class NS_SAVE_PANEL;
Class NS_OPEN_PANEL;
Class NS_PASTEBOARD;
Class NS_SPLIT_VIEW_ITEM;
Class NS_TITLEBAR_ACCESSORY_VIEW_CONTROLLER;
Class NS_TOOLBAR;
Class NS_TOOLBAR_ITEM;
Class NS_BUTTON;
Class NS_TEXT_FIELD;
Class NS_TABLE_VIEW;
Class NS_OUTLINE_VIEW;
Class NS_FONT;
Class NS_FONT_MANAGER;
Class NS_GESTURE_RECOGNIZER;
Class NS_CLICK_GESTURE_RECOGNIZER;
Class NS_PAN_GESTURE_RECOGNIZER;
Class NS_MAGNIFICATION_GESTURE_RECOGNIZER;
Class NS_ROTATION_GESTURE_RECOGNIZER;
Class NS_STATUS_BAR;
Class NS_ANIMATION_CONTEXT;

Class NS_DIFFABLE_DATA_SOURCE_SNAPSHOT;
Class NS_COLLECTION_VIEW_DIFFABLE_DATA_SOURCE;

Class NS_SCROLL_VIEW;
Class NS_TABLE_COLUMN;
Class NS_VISUAL_EFFECT_VIEW;
Class NS_SECURE_TEXT_FIELD;
Class NS_SPLIT_VIEW_CONTROLLER;
Class NS_SPLIT_VIEW;

__attribute__((constructor))
static void app_initializer(void)
{
    
    static int initialized = 0;
    if (!initialized) {
        initialized = 1;
        NS_SCROLL_VIEW = NSClassFromString(@"NSScrollView");
        NS_TABLE_COLUMN = NSClassFromString(@"NSTableColumn");
        NS_VISUAL_EFFECT_VIEW = NSClassFromString(@"NSVisualEffectView");
        NS_SECURE_TEXT_FIELD = NSClassFromString(@"NSSecureTextField");
        NS_SPLIT_VIEW_CONTROLLER = NSClassFromString(@"NSSplitViewController");
        NS_SPLIT_VIEW = NSClassFromString(@"NSSplitView");

        
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
        NS_MENU_ITEM = [NSMenuItem class];
        NS_PANEL = [NSPanel class];
        NS_SAVE_PANEL = [NSSavePanel class];
        NS_OPEN_PANEL = [NSOpenPanel class];
        NS_PASTEBOARD = [NSPasteboard class];
        NS_SPLIT_VIEW_ITEM = [NSSplitViewItem class];
        NS_TITLEBAR_ACCESSORY_VIEW_CONTROLLER = [NSTitlebarAccessoryViewController class];
        NS_TOOLBAR = [NSToolbar class];
        NS_TOOLBAR_ITEM = [NSToolbarItem class];
        NS_BUTTON = [NSButton class];
        NS_TEXT_FIELD = [NSTextField class];
        NS_TABLE_VIEW = [NSTableView class];
        NS_OUTLINE_VIEW = [NSOutlineView class];
        NS_FONT = [NSFont class];
        NS_FONT_MANAGER = [NSFontManager class];
        NS_GESTURE_RECOGNIZER = [NSGestureRecognizer class];
        NS_CLICK_GESTURE_RECOGNIZER = [NSClickGestureRecognizer class];
        NS_PAN_GESTURE_RECOGNIZER = [NSPanGestureRecognizer class];
        NS_MAGNIFICATION_GESTURE_RECOGNIZER = [NSMagnificationGestureRecognizer class];
        NS_ROTATION_GESTURE_RECOGNIZER = [NSRotationGestureRecognizer class];
        NS_STATUS_BAR = [NSStatusBar class];
        NS_ANIMATION_CONTEXT = [NSAnimationContext class];
        
        NS_DIFFABLE_DATA_SOURCE_SNAPSHOT = [NSDiffableDataSourceSnapshot class];
        NS_COLLECTION_VIEW_DIFFABLE_DATA_SOURCE = [NSCollectionViewDiffableDataSource class];
    }
}

NS_ASSUME_NONNULL_END
