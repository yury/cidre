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

__attribute__((constructor))
static void mtl_initializer(void)
{
    
    static int initialized = 0;
    if (!initialized) {
        NS_APPLICATION = [NSApplication class];
        NS_VIEW = [NSView class];
        NS_COLOR = [NSColor class];
        initialized = 1;
    }
}

NS_ASSUME_NONNULL_END
