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
Class UI_COLLECTION_VIEW;
Class UI_WINDOW;
Class UI_SCENE;
Class UI_SCENE_CONFIGURATION;
Class UI_SCREEN;
Class UI_COLOR;
Class UI_RESPONDER;
Class UI_VIEW_CONTROLLER;
Class UI_NAVIGATION_CONTROLLER;
Class UI_VIEW_CONTROLLER_TRANSITION;
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
Class UI_SCENE_SESSION_ACTIVATION_REQUEST;

Class UI_BLUR_EFFECT;
Class UI_ZOOM_TRANSITION_OPTIONS;

Class UI_COLLECTION_VIEW_DROP_PROPOSAL;
Class UI_COLLECTION_VIEW_PLACEHOLDER;
Class UI_COLLECTION_VIEW_DROP_PLACEHOLDER;
Class UI_COLLECTION_VIEW_DIFFABLE_DATA_SOURCE;
Class UI_COLLECTION_VIEW_DIFFABLE_DATA_SOURCE_REORDERING_HANDLERS;
Class UI_COLLECTION_VIEW_DIFFABLE_DATA_SOURCE_SECTION_SNAPSHOT_HANDLERS;

Class NS_DIFFABLE_DATA_SOURCE_SNAPSHOT;
Class NS_DIFFABLE_DATA_SOURCE_SECTION_SNAPSHOT;

__attribute__((constructor))
static void ui_initializer(void)
{
    static int initialized = 0;
    if (!initialized) {
        initialized = 1;
        
        UI_DEVICE = NSClassFromString(@"UIDevice");
        UI_SCENE = NSClassFromString(@"UIScene");
        UI_VIEW = NSClassFromString(@"UIView");
        UI_COLLECTION_VIEW = NSClassFromString(@"UICollectionView");
        UI_SCENE_CONFIGURATION = NSClassFromString(@"UISceneConfiguration");
        UI_SCREEN = [UIScreen class];
        UI_COLOR = [UIColor class];
        UI_RESPONDER = NSClassFromString(@"UIResponder");
        UI_VIEW_CONTROLLER = NSClassFromString(@"UIViewController");
        UI_NAVIGATION_CONTROLLER = NSClassFromString(@"UINavigationController");

        UI_VIEW_CONTROLLER_TRANSITION = NSClassFromString(@"UIViewControllerTransition");
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
        UI_SCENE_SESSION_ACTIVATION_REQUEST = NSClassFromString(@"UISceneSessionActivationRequest");
        
        UI_BLUR_EFFECT = NSClassFromString(@"UIBlurEffect");
        UI_ZOOM_TRANSITION_OPTIONS = NSClassFromString(@"UIZoomTransitionOptions");

        UI_COLLECTION_VIEW_DROP_PROPOSAL = NSClassFromString(@"UICollectionViewDropProposal");
        UI_COLLECTION_VIEW_PLACEHOLDER = NSClassFromString(@"UICollectionViewPlaceholder");
        UI_COLLECTION_VIEW_DROP_PLACEHOLDER = NSClassFromString(@"UICollectionViewDropPlaceholder");
        UI_COLLECTION_VIEW_DIFFABLE_DATA_SOURCE = NSClassFromString(@"UICollectionViewDiffableDataSource");
        UI_COLLECTION_VIEW_DIFFABLE_DATA_SOURCE_REORDERING_HANDLERS = NSClassFromString(@"UICollectionViewDiffableDataSourceReorderingHandlers");
        UI_COLLECTION_VIEW_DIFFABLE_DATA_SOURCE_SECTION_SNAPSHOT_HANDLERS = NSClassFromString(@"UICollectionViewDiffableDataSourceSectionSnapshotHandlers");
        
        NS_DIFFABLE_DATA_SOURCE_SNAPSHOT = [NSDiffableDataSourceSnapshot class];
        NS_DIFFABLE_DATA_SOURCE_SECTION_SNAPSHOT = NSClassFromString(@" NSDiffableDataSourceSectionSnapshot");
    }
}

NS_ASSUME_NONNULL_END
