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
Class UI_PASTEBOARD;
Class UI_VIEW;
Class UI_SCROLL_VIEW;
Class UI_COLLECTION_VIEW;
Class UI_COLLECTION_VIEW_CELL;
Class UI_COLLECTION_VIEW_LIST_CELL;
Class UI_COLLECTION_VIEW_CELL_REGISTRATION;
Class UI_CELL_ACCESSORY_OUTLINE_DISCLOSURE;
Class UI_COLLECTION_VIEW_FLOW_LAYOUT;
Class UI_LABEL;
Class UI_CONTROL;
Class UI_BUTTON;
Class UI_ACTION;
Class UI_MENU;
Class UI_DEFERRED_MENU_ELEMENT;
Class UI_DEFERRED_MENU_ELEMENT_PROVIDER;
Class UI_COMMAND;
Class UI_KEY_COMMAND;
Class UI_TEXT_FIELD;
Class UI_ALERT_CONTROLLER;
Class UI_DOCUMENT_PICKER_VIEW_CONTROLLER;
Class UI_ALERT_ACTION;
Class UI_GESTURE_RECOGNIZER;
Class UI_PAN_GESTURE_RECOGNIZER;
Class UI_PINCH_GESTURE_RECOGNIZER;
Class UI_ROTATION_GESTURE_RECOGNIZER;
Class UI_TAP_GESTURE_RECOGNIZER;
Class UI_SWIPE_GESTURE_RECOGNIZER;
Class UI_TAB_ACCESSORY;
Class UI_GLASS_EFFECT;
Class UI_CORNER_RADIUS;
Class UI_CORNER_CONFIGURATION;
Class UI_VIEW_LAYOUT_REGION;
Class UI_IMAGE_SYMBOL_CONFIGURATION;
Class UI_TOOLBAR;
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
Class UI_TAB;
Class UI_TAB_GROUP;
Class UI_BAR_MINIMIZATION;
Class UI_CONTEXT_MENU_CONFIGURATION;
Class UI_SHEET_PRESENTATION_CONTROLLER_DETENT;
Class UI_IMAGE;
Class UI_TRAIT_COLLECTION;
Class UI_TRAIT_HORIZONTAL_SIZE_CLASS;
Class UI_TRAIT_VERTICAL_SIZE_CLASS;
Class UI_TRAIT_USER_INTERFACE_STYLE;
Class UI_TRAIT_LAYOUT_DIRECTION;
Class UI_TRAIT_DISPLAY_SCALE;
Class UI_TRAIT_TAB_ACCESSORY_ENVIRONMENT;
Class UI_BACKGROUND_CONFIGURATION;
Class UI_STACK_VIEW;
Class UI_SWITCH;
Class UI_SLIDER;
Class UI_STEPPER;
Class UI_TEXT_VIEW;
Class NS_TEXT_ATTACHMENT;
Class UI_LAYOUT_GUIDE;
Class NS_LAYOUT_CONSTRAINT;
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
Class UI_BACKGROUND_EXTENSION_VIEW;
Class UI_ZOOM_TRANSITION_OPTIONS;

Class UI_COLLECTION_VIEW_DROP_PROPOSAL;
Class UI_COLLECTION_VIEW_PLACEHOLDER;
Class UI_COLLECTION_VIEW_DROP_PLACEHOLDER;
Class UI_COLLECTION_VIEW_DIFFABLE_DATA_SOURCE;
Class UI_COLLECTION_VIEW_DIFFABLE_DATA_SOURCE_REORDERING_HANDLERS;
Class UI_COLLECTION_VIEW_DIFFABLE_DATA_SOURCE_SECTION_SNAPSHOT_HANDLERS;

Class NS_DIFFABLE_DATA_SOURCE_SNAPSHOT;
Class NS_DIFFABLE_DATA_SOURCE_SECTION_SNAPSHOT;

Class UI_BUTTON_CONFIGURATION;
Class UI_SPLIT_VIEW_CONTROLLER;
Class UI_SCENE_ACCESSORY;
Class UI_ARRANGEMENT_VIEW_CONTROLLER;
Class UI_SPLIT_ARRANGEMENT_DIMENSION;
Class UI_SPLIT_ARRANGEMENT_DIMENSION_RANGE;
Class UI_SPLIT_ARRANGEMENT_VIEW_PROPERTIES;
Class UI_SPLIT_ARRANGEMENT;
Class UI_OVERLAY_ARRANGEMENT_VIEW_PROPERTIES;
Class UI_OVERLAY_ARRANGEMENT;
Class UI_BAR_BUTTON_ITEM;
Class UI_NAVIGATION_ITEM;

Class UI_LIST_CONTENT_CONFIGURATION;

Class UI_COLLECTION_VIEW_COMPOSITIONAL_LAYOUT;
Class UI_HINGE_INTERACTION;
Class UI_COLLECTION_VIEW_SUPPLEMENTARY_REGISTRATION;
Class UI_COLLECTION_REUSABLE_VIEW;
Class NS_COLLECTION_LAYOUT_DIMENSION;
Class NS_COLLECTION_LAYOUT_SIZE;
Class NS_COLLECTION_LAYOUT_ITEM;
Class NS_COLLECTION_LAYOUT_GROUP;
Class NS_COLLECTION_LAYOUT_BOUNDARY_SUPPLEMENTARY_ITEM;
Class NS_COLLECTION_LAYOUT_SECTION;

Class UI_COLLECTION_LAYOUT_LIST_CONFIGURATION;

__attribute__((constructor))
static void ui_initializer(void)
{
    static int initialized = 0;
    if (!initialized) {
        initialized = 1;
        UI_COLLECTION_LAYOUT_LIST_CONFIGURATION = NSClassFromString(@"UICollectionLayoutListConfiguration");
        UI_COLLECTION_VIEW_COMPOSITIONAL_LAYOUT = NSClassFromString(@"UICollectionViewCompositionalLayout");
        UI_HINGE_INTERACTION = NSClassFromString(@"UIHingeInteraction");
        UI_COLLECTION_VIEW_SUPPLEMENTARY_REGISTRATION = NSClassFromString(@"UICollectionViewSupplementaryRegistration");
        UI_COLLECTION_REUSABLE_VIEW = NSClassFromString(@"UICollectionReusableView");
        NS_COLLECTION_LAYOUT_DIMENSION = NSClassFromString(@"NSCollectionLayoutDimension");
        NS_COLLECTION_LAYOUT_SIZE = NSClassFromString(@"NSCollectionLayoutSize");
        NS_COLLECTION_LAYOUT_ITEM = NSClassFromString(@"NSCollectionLayoutItem");
        NS_COLLECTION_LAYOUT_GROUP = NSClassFromString(@"NSCollectionLayoutGroup");
        NS_COLLECTION_LAYOUT_BOUNDARY_SUPPLEMENTARY_ITEM = NSClassFromString(@"NSCollectionLayoutBoundarySupplementaryItem");
        NS_COLLECTION_LAYOUT_SECTION = NSClassFromString(@"NSCollectionLayoutSection");
        UI_LIST_CONTENT_CONFIGURATION = NSClassFromString(@"UIListContentConfiguration");
        UI_BUTTON_CONFIGURATION = NSClassFromString(@"UIButtonConfiguration");
        UI_SPLIT_VIEW_CONTROLLER = NSClassFromString(@"UISplitViewController");
        UI_SCENE_ACCESSORY = NSClassFromString(@"UISceneAccessory");
        UI_ARRANGEMENT_VIEW_CONTROLLER = NSClassFromString(@"UIArrangementViewController");
        UI_SPLIT_ARRANGEMENT_DIMENSION = NSClassFromString(@"UISplitArrangementDimension");
        UI_SPLIT_ARRANGEMENT_DIMENSION_RANGE = NSClassFromString(@"UISplitArrangementDimensionRange");
        UI_SPLIT_ARRANGEMENT_VIEW_PROPERTIES = NSClassFromString(@"UISplitArrangementViewProperties");
        UI_SPLIT_ARRANGEMENT = NSClassFromString(@"UISplitArrangement");
        UI_OVERLAY_ARRANGEMENT_VIEW_PROPERTIES = NSClassFromString(@"UIOverlayArrangementViewProperties");
        UI_OVERLAY_ARRANGEMENT = NSClassFromString(@"UIOverlayArrangement");
        UI_BAR_BUTTON_ITEM = NSClassFromString(@"UIBarButtonItem");
        UI_NAVIGATION_ITEM = NSClassFromString(@"UINavigationItem");

        
        UI_DEVICE = NSClassFromString(@"UIDevice");
        UI_SCENE = NSClassFromString(@"UIScene");
        UI_VIEW = NSClassFromString(@"UIView");
        UI_SCROLL_VIEW = NSClassFromString(@"UIScrollView");
        UI_COLLECTION_VIEW = NSClassFromString(@"UICollectionView");
        UI_COLLECTION_VIEW_CELL = NSClassFromString(@"UICollectionViewCell");
        UI_COLLECTION_VIEW_LIST_CELL = NSClassFromString(@"UICollectionViewListCell");
        UI_COLLECTION_VIEW_CELL_REGISTRATION = NSClassFromString(@"UICollectionViewCellRegistration");
        UI_CELL_ACCESSORY_OUTLINE_DISCLOSURE = NSClassFromString(@"UICellAccessoryOutlineDisclosure");
        UI_COLLECTION_VIEW_FLOW_LAYOUT = NSClassFromString(@"UICollectionViewFlowLayout");
        UI_LABEL = NSClassFromString(@"UILabel");
        UI_CONTROL = NSClassFromString(@"UIControl");
        UI_ACTION = NSClassFromString(@"UIAction");
        UI_MENU = NSClassFromString(@"UIMenu");
        UI_DEFERRED_MENU_ELEMENT = NSClassFromString(@"UIDeferredMenuElement");
        UI_DEFERRED_MENU_ELEMENT_PROVIDER = NSClassFromString(@"UIDeferredMenuElementProvider");
        UI_COMMAND = NSClassFromString(@"UICommand");
        UI_KEY_COMMAND = NSClassFromString(@"UIKeyCommand");
        UI_BUTTON = NSClassFromString(@"UIButton");
        UI_TEXT_FIELD = NSClassFromString(@"UITextField");
        UI_ALERT_CONTROLLER = NSClassFromString(@"UIAlertController");
        UI_DOCUMENT_PICKER_VIEW_CONTROLLER = NSClassFromString(@"UIDocumentPickerViewController");
        UI_ALERT_ACTION = NSClassFromString(@"UIAlertAction");
        UI_GESTURE_RECOGNIZER = NSClassFromString(@"UIGestureRecognizer");
        UI_PAN_GESTURE_RECOGNIZER = NSClassFromString(@"UIPanGestureRecognizer");
        UI_PINCH_GESTURE_RECOGNIZER = NSClassFromString(@"UIPinchGestureRecognizer");
        UI_ROTATION_GESTURE_RECOGNIZER = NSClassFromString(@"UIRotationGestureRecognizer");
        UI_TAP_GESTURE_RECOGNIZER = NSClassFromString(@"UITapGestureRecognizer");
        UI_SWIPE_GESTURE_RECOGNIZER = NSClassFromString(@"UISwipeGestureRecognizer");
        UI_TAB_ACCESSORY = NSClassFromString(@"UITabAccessory");
        UI_GLASS_EFFECT = NSClassFromString(@"UIGlassEffect");
        UI_CORNER_RADIUS = NSClassFromString(@"UICornerRadius");
        UI_CORNER_CONFIGURATION = NSClassFromString(@"UICornerConfiguration");
        UI_VIEW_LAYOUT_REGION = NSClassFromString(@"UIViewLayoutRegion");
        UI_IMAGE_SYMBOL_CONFIGURATION = NSClassFromString(@"UIImageSymbolConfiguration");
        UI_TOOLBAR = NSClassFromString(@"UIToolbar");
        UI_SCENE_CONFIGURATION = NSClassFromString(@"UISceneConfiguration");
        UI_SCREEN = [UIScreen class];
        UI_COLOR = [UIColor class];
        UI_RESPONDER = NSClassFromString(@"UIResponder");
        UI_VIEW_CONTROLLER = NSClassFromString(@"UIViewController");
        UI_NAVIGATION_CONTROLLER = NSClassFromString(@"UINavigationController");

        UI_VIEW_CONTROLLER_TRANSITION = NSClassFromString(@"UIViewControllerTransition");
        UI_TAB_BAR_CONTROLLER = NSClassFromString(@"UITabBarController");
        UI_TAB = NSClassFromString(@"UITab");
        UI_TAB_GROUP = NSClassFromString(@"UITabGroup");
        UI_BAR_MINIMIZATION = NSClassFromString(@"UIBarMinimization");
        UI_CONTEXT_MENU_CONFIGURATION = NSClassFromString(@"UIContextMenuConfiguration");
        UI_SHEET_PRESENTATION_CONTROLLER_DETENT = NSClassFromString(@"UISheetPresentationControllerDetent");
        UI_APPLICATION = NSClassFromString(@"UIApplication");
        UI_PASTEBOARD = NSClassFromString(@"UIPasteboard");
        UI_WINDOW = NSClassFromString(@"UIWindow");
        UI_IMAGE = [UIImage class];
        UI_TRAIT_COLLECTION = [UITraitCollection class];
        UI_TRAIT_HORIZONTAL_SIZE_CLASS = NSClassFromString(@"UITraitHorizontalSizeClass");
        UI_TRAIT_VERTICAL_SIZE_CLASS = NSClassFromString(@"UITraitVerticalSizeClass");
        UI_TRAIT_USER_INTERFACE_STYLE = NSClassFromString(@"UITraitUserInterfaceStyle");
        UI_TRAIT_LAYOUT_DIRECTION = NSClassFromString(@"UITraitLayoutDirection");
        UI_TRAIT_DISPLAY_SCALE = NSClassFromString(@"UITraitDisplayScale");
        UI_TRAIT_TAB_ACCESSORY_ENVIRONMENT = NSClassFromString(@"UITraitTabAccessoryEnvironment");
        UI_BACKGROUND_CONFIGURATION = NSClassFromString(@"UIBackgroundConfiguration");
        UI_STACK_VIEW = [UIStackView class];
#if TARGET_OS_TV
#else
        UI_SWITCH = [UISwitch class];
        UI_SLIDER = [UISlider class];
        UI_STEPPER = [UIStepper class];
#endif
        UI_TEXT_VIEW = [UITextView class];
        
        NS_TEXT_ATTACHMENT = [NSTextAttachment class];
        
        UI_LAYOUT_GUIDE = [UILayoutGuide class];
        NS_LAYOUT_CONSTRAINT = [NSLayoutConstraint class];
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
        UI_BACKGROUND_EXTENSION_VIEW = NSClassFromString(@"UIBackgroundExtensionView");
        UI_ZOOM_TRANSITION_OPTIONS = NSClassFromString(@"UIZoomTransitionOptions");

        UI_COLLECTION_VIEW_DROP_PROPOSAL = NSClassFromString(@"UICollectionViewDropProposal");
        UI_COLLECTION_VIEW_PLACEHOLDER = NSClassFromString(@"UICollectionViewPlaceholder");
        UI_COLLECTION_VIEW_DROP_PLACEHOLDER = NSClassFromString(@"UICollectionViewDropPlaceholder");
        UI_COLLECTION_VIEW_DIFFABLE_DATA_SOURCE = NSClassFromString(@"UICollectionViewDiffableDataSource");
        UI_COLLECTION_VIEW_DIFFABLE_DATA_SOURCE_REORDERING_HANDLERS = NSClassFromString(@"UICollectionViewDiffableDataSourceReorderingHandlers");
        UI_COLLECTION_VIEW_DIFFABLE_DATA_SOURCE_SECTION_SNAPSHOT_HANDLERS = NSClassFromString(@"UICollectionViewDiffableDataSourceSectionSnapshotHandlers");
        
        NS_DIFFABLE_DATA_SOURCE_SNAPSHOT = [NSDiffableDataSourceSnapshot class];
        NS_DIFFABLE_DATA_SOURCE_SECTION_SNAPSHOT = NSClassFromString(@"NSDiffableDataSourceSectionSnapshot");
    }
}

NS_ASSUME_NONNULL_END
