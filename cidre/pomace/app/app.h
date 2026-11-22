//
//  app.h
//  app
//
//  Created by Yury Korolev on 11/1/23.
//

#import <AppKit/AppKit.h>

NS_ASSUME_NONNULL_BEGIN

Class NS_PREVIEW_REPRESENTING_ACTIVITY_ITEM;
Class NS_ITEM_BADGE;
Class NS_SCROLL_EDGE_EFFECT_STYLE;
Class NS_SPLIT_VIEW_ITEM_ACCESSORY_VIEW_CONTROLLER;
Class NS_BACKGROUND_EXTENSION_VIEW;
Class NS_GLASS_EFFECT_CONTAINER_VIEW;
Class NS_GLASS_EFFECT_VIEW;

__attribute__((constructor))
static void app_initializer(void)
{
    
    static int initialized = 0;
    if (!initialized) {
        initialized = 1;
        NS_PREVIEW_REPRESENTING_ACTIVITY_ITEM = NSClassFromString(@"NSPreviewRepresentingActivityItem");
        NS_ITEM_BADGE = NSClassFromString(@"NSItemBadge");
        NS_SCROLL_EDGE_EFFECT_STYLE = NSClassFromString(@"NSScrollEdgeEffectStyle");
        NS_SPLIT_VIEW_ITEM_ACCESSORY_VIEW_CONTROLLER = NSClassFromString(@"NSSplitViewItemAccessoryViewController");
        NS_BACKGROUND_EXTENSION_VIEW = NSClassFromString(@"NSBackgroundExtensionView");
        NS_GLASS_EFFECT_CONTAINER_VIEW = NSClassFromString(@"NSGlassEffectContainerView");
        NS_GLASS_EFFECT_VIEW = NSClassFromString(@"NSGlassEffectView");

    }
}

NS_ASSUME_NONNULL_END
