//
//  ui.h
//  ui
//
//  Created by Yury Korolev on 25.05.2022.
//

#import <UIKit/UIKit.h>

NS_ASSUME_NONNULL_BEGIN

Class UI_DEFERRED_MENU_ELEMENT_PROVIDER;
Class UI_TAB_ACCESSORY;
Class UI_GLASS_EFFECT;
Class UI_CORNER_RADIUS;
Class UI_CORNER_CONFIGURATION;
Class UI_VIEW_LAYOUT_REGION;
Class UI_VIEW_CONTROLLER_TRANSITION;
Class UI_TAB;
Class UI_TAB_GROUP;
Class UI_BAR_MINIMIZATION;
Class UI_CONTEXT_MENU_CONFIGURATION;
Class UI_TRAIT_HORIZONTAL_SIZE_CLASS;
Class UI_TRAIT_VERTICAL_SIZE_CLASS;
Class UI_TRAIT_USER_INTERFACE_STYLE;
Class UI_TRAIT_LAYOUT_DIRECTION;
Class UI_TRAIT_DISPLAY_SCALE;
Class UI_TRAIT_TAB_ACCESSORY_ENVIRONMENT;
Class UI_UPDATE_LINK;
Class UI_UPDATE_ACTION_PHASE;
Class UI_UPDATE_INFO;

Class UI_WINDOW_SCENE_STANDARD_PLACEMENT;
Class UI_WINDOW_SCENE_PROMINENT_PLACEMENT;
Class UI_SCENE_SESSION_ACTIVATION_REQUEST;

Class UI_BACKGROUND_EXTENSION_VIEW;
Class UI_ZOOM_TRANSITION_OPTIONS;

Class UI_SCENE_ACCESSORY;
Class UI_ARRANGEMENT_VIEW_CONTROLLER;
Class UI_SPLIT_ARRANGEMENT_DIMENSION;
Class UI_SPLIT_ARRANGEMENT_DIMENSION_RANGE;
Class UI_SPLIT_ARRANGEMENT_VIEW_PROPERTIES;
Class UI_SPLIT_ARRANGEMENT;
Class UI_OVERLAY_ARRANGEMENT_VIEW_PROPERTIES;
Class UI_OVERLAY_ARRANGEMENT;

Class UI_HINGE_INTERACTION;

__attribute__((constructor))
static void ui_initializer(void)
{
    static int initialized = 0;
    if (!initialized) {
        initialized = 1;
        UI_HINGE_INTERACTION = NSClassFromString(@"UIHingeInteraction");
        UI_SCENE_ACCESSORY = NSClassFromString(@"UISceneAccessory");
        UI_ARRANGEMENT_VIEW_CONTROLLER = NSClassFromString(@"UIArrangementViewController");
        UI_SPLIT_ARRANGEMENT_DIMENSION = NSClassFromString(@"UISplitArrangementDimension");
        UI_SPLIT_ARRANGEMENT_DIMENSION_RANGE = NSClassFromString(@"UISplitArrangementDimensionRange");
        UI_SPLIT_ARRANGEMENT_VIEW_PROPERTIES = NSClassFromString(@"UISplitArrangementViewProperties");
        UI_SPLIT_ARRANGEMENT = NSClassFromString(@"UISplitArrangement");
        UI_OVERLAY_ARRANGEMENT_VIEW_PROPERTIES = NSClassFromString(@"UIOverlayArrangementViewProperties");
        UI_OVERLAY_ARRANGEMENT = NSClassFromString(@"UIOverlayArrangement");

        UI_DEFERRED_MENU_ELEMENT_PROVIDER = NSClassFromString(@"UIDeferredMenuElementProvider");
        UI_TAB_ACCESSORY = NSClassFromString(@"UITabAccessory");
        UI_GLASS_EFFECT = NSClassFromString(@"UIGlassEffect");
        UI_CORNER_RADIUS = NSClassFromString(@"UICornerRadius");
        UI_CORNER_CONFIGURATION = NSClassFromString(@"UICornerConfiguration");
        UI_VIEW_LAYOUT_REGION = NSClassFromString(@"UIViewLayoutRegion");

        UI_VIEW_CONTROLLER_TRANSITION = NSClassFromString(@"UIViewControllerTransition");
        UI_TAB = NSClassFromString(@"UITab");
        UI_TAB_GROUP = NSClassFromString(@"UITabGroup");
        UI_BAR_MINIMIZATION = NSClassFromString(@"UIBarMinimization");
        UI_CONTEXT_MENU_CONFIGURATION = NSClassFromString(@"UIContextMenuConfiguration");
        UI_TRAIT_HORIZONTAL_SIZE_CLASS = NSClassFromString(@"UITraitHorizontalSizeClass");
        UI_TRAIT_VERTICAL_SIZE_CLASS = NSClassFromString(@"UITraitVerticalSizeClass");
        UI_TRAIT_USER_INTERFACE_STYLE = NSClassFromString(@"UITraitUserInterfaceStyle");
        UI_TRAIT_LAYOUT_DIRECTION = NSClassFromString(@"UITraitLayoutDirection");
        UI_TRAIT_DISPLAY_SCALE = NSClassFromString(@"UITraitDisplayScale");
        UI_TRAIT_TAB_ACCESSORY_ENVIRONMENT = NSClassFromString(@"UITraitTabAccessoryEnvironment");

        UI_UPDATE_LINK = NSClassFromString(@"UIUpdateLink");
        UI_UPDATE_ACTION_PHASE = NSClassFromString(@"UIUpdateActionPhase");
        UI_UPDATE_INFO = NSClassFromString(@"UIUpdateInfo");
        
        UI_WINDOW_SCENE_STANDARD_PLACEMENT = NSClassFromString(@"UIWindowSceneStandardPlacement");
        UI_WINDOW_SCENE_PROMINENT_PLACEMENT = NSClassFromString(@"UIWindowSceneProminentPlacement");
        UI_SCENE_SESSION_ACTIVATION_REQUEST = NSClassFromString(@"UISceneSessionActivationRequest");
        
        UI_BACKGROUND_EXTENSION_VIEW = NSClassFromString(@"UIBackgroundExtensionView");
        UI_ZOOM_TRANSITION_OPTIONS = NSClassFromString(@"UIZoomTransitionOptions");

    }
}

NS_ASSUME_NONNULL_END
