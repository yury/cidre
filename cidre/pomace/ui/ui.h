//
//  ui.h
//  ui
//
//  Created by Yury Korolev on 25.05.2022.
//

#import <UIKit/UIKit.h>

NS_ASSUME_NONNULL_BEGIN

Class UI_DEVICE;
Class UI_APPLICATION;
Class UI_VIEW;
Class UI_WINDOW;
Class UI_SCENE;
Class UI_SCENE_CONFIGURATION;
Class UI_SCREEN;
Class UI_COLOR;
Class UI_RESPONDER;
Class UI_VIEW_CONTROLLER;
Class UI_TAB_BAR_CONTROLLER;
Class UI_IMAGE;
Class UI_TRAIT_COLLECTION;
Class NS_TEXT_ATTACHMENT;
Class UI_LAYOUT_GUIDE;
Class UI_FONT;
Class UI_FONT_DESCRIPTOR;
Class UI_UPDATE_LINK;
Class UI_UPDATE_ACTION_PHASE;
Class UI_UPDATE_INFO;

Class UI_WINDOW_SCENE_STANDARD_PLACEMENT;
Class UI_WINDOW_SCENE_PROMINENT_PLACEMENT;
Class UI_WINDOW_SCENE_ACTIVATION_REQUEST_OPTIONS;

__attribute__((constructor))
static void ui_initializer(void)
{
    static int initialized = 0;
    if (!initialized) {
        initialized = 1;
        
        UI_DEVICE = NSClassFromString(@"UIDevice");
        UI_SCENE = NSClassFromString(@"UIScene");
        UI_VIEW = NSClassFromString(@"UIView");
        UI_SCENE_CONFIGURATION = NSClassFromString(@"UISceneConfiguration");
        UI_SCREEN = [UIScreen class];
        UI_COLOR = [UIColor class];
        UI_RESPONDER = NSClassFromString(@"UIResponder");
        UI_VIEW_CONTROLLER = NSClassFromString(@"UIViewController");
        UI_TAB_BAR_CONTROLLER = NSClassFromString(@"UITabBarController");
        UI_APPLICATION = NSClassFromString(@"UIApplication");
        UI_WINDOW = NSClassFromString(@"UIWindow");
        UI_IMAGE = [UIImage class];
        UI_TRAIT_COLLECTION = [UITraitCollection class];
        
        NS_TEXT_ATTACHMENT = [NSTextAttachment class];
        
        UI_LAYOUT_GUIDE = [UILayoutGuide class];
        UI_FONT = [UIFont class];
        UI_FONT_DESCRIPTOR = [UIFontDescriptor class];
        UI_UPDATE_LINK = NSClassFromString(@"UIUpdateLink");
        UI_UPDATE_ACTION_PHASE = NSClassFromString(@"UIUpdateActionPhase");
        UI_UPDATE_INFO = NSClassFromString(@"UIUpdateInfo");
        
        UI_WINDOW_SCENE_STANDARD_PLACEMENT = NSClassFromString(@"UIWindowSceneStandardPlacement");
        UI_WINDOW_SCENE_PROMINENT_PLACEMENT = NSClassFromString(@"UIWindowSceneProminentPlacement");
        UI_WINDOW_SCENE_ACTIVATION_REQUEST_OPTIONS = NSClassFromString(@"UIWindowSceneActivationRequestOptions");
    }
}

NS_ASSUME_NONNULL_END
