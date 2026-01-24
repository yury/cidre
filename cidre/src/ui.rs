mod application;
pub use application::App;
pub use application::AppDelegate;
pub use application::AppDelegateImpl;

mod blur_effect;
pub use blur_effect::BlurEffect;
pub use blur_effect::BlurEffectStyle;

mod device;
pub use device::BatteryState as DeviceBatteryState;
pub use device::Device;
pub use device::Idiom;
pub use device::notifications as device_notifications;

mod font;
pub use font::Font;

mod font_descriptor;
pub use font_descriptor::FontDesc;
pub use font_descriptor::FontDescClass;
pub use font_descriptor::FontDescSymbolicTraits;
pub use font_descriptor::FontWeight;
pub use font_descriptor::FontWidth;
pub use font_descriptor::TextStyle;

mod geometry;
pub use geometry::EdgeInsets;
pub use geometry::Offset;

mod gesture_recognizer;
pub use gesture_recognizer::GestureRecognizer;

mod pan_gesture_recognizer;
pub use pan_gesture_recognizer::PanGestureRecognizer;

mod tap_gesture_recognizer;
pub use tap_gesture_recognizer::TapGestureRecognizer;

mod navigation_bar;
pub use navigation_bar::NavBar;

mod responder;
pub use responder::Responder;
pub use responder::ResponderStandardEditActions;
pub use responder::ResponderStandardEditActionsImpl;

mod touch;
pub use touch::ForceTouchCapability;
pub use touch::Touch;
pub use touch::TouchPhase;
pub use touch::TouchProps;
pub use touch::TouchType;

mod press;
pub use press::Press;

mod event;
pub use event::Event;
pub use event::EventButtonMask;
pub use event::EventSubType;

mod key;
pub use key::Key;

mod key_constants;
pub use key_constants::KeyboardHidUsage;

mod command;
pub use command::KeyModFlags;

mod data_source_translating;
pub use data_source_translating::AnyDataSourceTranslating;
pub use data_source_translating::DataSourceTranslating;

mod context_menu_configuration;
pub use context_menu_configuration::ContextMenuConfiguration;

mod context_menu_interaction;
pub use context_menu_interaction::AnyContextMenuInteractionAnimating;
pub use context_menu_interaction::AnyContextMenuInteractionCommitAnimating;
pub use context_menu_interaction::ContextMenuInteraction;
pub use context_menu_interaction::ContextMenuInteractionAnimating;
pub use context_menu_interaction::ContextMenuInteractionCommitAnimating;

mod drag_item;
pub use drag_item::DragItem;

mod drag_interaction;
pub use drag_interaction::AnyDragAnimating;
pub use drag_interaction::DragAnimating;

mod drag_session;
pub use drag_session::AnyDragSession;
pub use drag_session::DragSession;

mod drag_preview_parameters;
pub use drag_preview_parameters::DragPreviewParameters;

mod drag_preview_target;
pub use drag_preview_target::DragPreviewTarget;

mod focus;
pub use focus::FocusUpdateCtx;

mod drop_interaction;
pub use drop_interaction::DropOperation;
pub use drop_interaction::DropProposal;

mod drop_session;
pub use drop_session::AnyDropSession;
pub use drop_session::DropSession;

mod focus_animation_coordinator;
pub use focus_animation_coordinator::FocusAnimationCoordinator;

mod targeted_preview;
pub use targeted_preview::TargetedPreview;

mod view;
pub use view::AnyCoordinateSpace;
pub use view::CoordinateSpace;
pub use view::View;
pub use view::ViewAutoresizing;

mod spring_loaded_interaction_context;
pub use spring_loaded_interaction_context::AnySpringLoadedInteractionContext;
pub use spring_loaded_interaction_context::SpringLoadedInteractionContext;

mod collection_view_cell;
pub use collection_view_cell::CollectionReusableView;
pub use collection_view_cell::CollectionViewCell;

mod collection_view_item_registration;
pub use collection_view_item_registration::CollectionViewCellRegistration;
pub use collection_view_item_registration::CollectionViewSupplementaryRegistration;

mod collection_view_layout;
pub use collection_view_layout::CollectionViewLayout;
pub use collection_view_layout::CollectionViewLayoutAttrs;

mod collection_view_transition_layout;
pub use collection_view_transition_layout::CollectionViewTransitionLayout;

mod collection_view;
pub use collection_view::AnyCollectionViewDataSrc;
pub use collection_view::AnyCollectionViewDataSrcPrefetching;
pub use collection_view::AnyCollectionViewDelegate;
pub use collection_view::AnyCollectionViewDragDelegate;
pub use collection_view::AnyCollectionViewDropCoordinator;
pub use collection_view::AnyCollectionViewDropDelegate;
pub use collection_view::AnyCollectionViewDropItem;
pub use collection_view::AnyCollectionViewDropPlaceholderCtx;
pub use collection_view::CollectionView;
pub use collection_view::CollectionViewDataSrc;
pub use collection_view::CollectionViewDataSrcPrefetching;
pub use collection_view::CollectionViewDelegate;
pub use collection_view::CollectionViewDragDelegate;
pub use collection_view::CollectionViewDropCoordinator;
pub use collection_view::CollectionViewDropDelegate;
pub use collection_view::CollectionViewDropIntent;
pub use collection_view::CollectionViewDropItem;
pub use collection_view::CollectionViewDropPlaceholder;
pub use collection_view::CollectionViewDropPlaceholderCtx;
pub use collection_view::CollectionViewDropProposal;
pub use collection_view::CollectionViewFocusUpdateCtx;
pub use collection_view::CollectionViewPlaceholder;
pub use collection_view::CollectionViewReorderingCadence;
pub use collection_view::CollectionViewScrollPos;
pub use collection_view::CollectionViewSelfSizingInvalidation;

mod visual_effect;
pub use visual_effect::VisualEffect;

mod window;
pub use window::Window;

mod window_scene_placement;
pub use window_scene_placement::WindowScenePlacement;

mod window_scene_standard_placement;
pub use window_scene_standard_placement::WindowSceneStandardPlacement;

mod window_scene_prominent_placement;
pub use window_scene_prominent_placement::WindowSceneProminentPlacement;

mod view_controller;
pub use view_controller::ViewController;

pub mod index_path;

mod view_controller_transition;
pub use view_controller_transition::ViewControllerTransition;
pub use view_controller_transition::ZoomTransitionSrcViewProviderCtx;

mod zoom_transition_options;
pub use zoom_transition_options::ZoomTransitionAlignmentRectCtx;
pub use zoom_transition_options::ZoomTransitionInteractionCtx;
pub use zoom_transition_options::ZoomTransitionOpts;

mod navigation_controller;
pub use navigation_controller::NavController;
pub use navigation_controller::NavControllerDelegate;
pub use navigation_controller::NavControllerDelegateImpl;

mod toolbar;
pub use toolbar::Toolbar;

mod tab;
pub use tab::Tab;

mod tab_bar;
pub use tab_bar::TabBar;

mod tab_bar_controller;
pub use tab_bar_controller::AnyTabBarControllerDelegate;
pub use tab_bar_controller::TabBarController;
pub use tab_bar_controller::TabBarControllerDelegate;
pub use tab_bar_controller::TabBarControllerDelegateImpl;
pub use tab_bar_controller::TabBarControllerMode;

mod tab_bar_controller_sidebar;
pub use tab_bar_controller_sidebar::TabBarControllerSidebar;

mod color;
pub use color::Color;
pub use color::ColorProminence;

mod image;
pub use image::Image;

mod interaction;
pub use interaction::AnyInteraction;
pub use interaction::Interaction;
pub use interaction::InteractionImpl;

mod scene_definitions;
pub use scene_definitions::SceneSessionRole;

mod orientation;
pub use orientation::DeviceOrientation;
pub use orientation::Orientation;
pub use orientation::OrientationMask;

pub mod interface;
pub use interface::BarStyle;
pub use interface::DisplayGamut;
pub use interface::LayoutDirection;
pub use interface::SizeClass;
pub use interface::Style;
pub use interface::TraitEnvLayoutDirection;

mod update_link;
pub use update_link::UpdateLink;

mod update_info;
pub use update_info::UpdateInfo;

mod update_action_phase;
pub use update_action_phase::UpdateActionPhase;

mod layout_guide;
pub use layout_guide::LayoutGuide;

mod scene;
pub use scene::AnySceneDelegate;
pub use scene::Scene;
pub use scene::SceneDelegate;
pub use scene::SceneDelegateImpl;
pub use scene::notifications as scene_notifications;

mod scene_session;
pub use scene_session::SceneCfg;
pub use scene_session::SceneSession;

mod scene_session_activation_request;
pub use scene_session_activation_request::SceneSessionActivationRequest;

mod scene_options;
pub use scene_options::SceneActivationRequestOpts;
pub use scene_options::SceneCollectionJoinBehavior;
pub use scene_options::SceneConnectionOpts;
pub use scene_options::SceneDestructionRequestOpts;

mod scene_windowing_behaviors;
pub use scene_windowing_behaviors::SceneWindowingBehaviors;

mod screen;
pub use screen::Screen;

mod screen_mode;
pub use screen_mode::ScreenMode;

mod trait_collection;
pub use trait_collection::TraitCollection;

mod window_scene;
pub use window_scene::SceneSizeRestrictions;
pub use window_scene::WindowScene;
pub use window_scene::WindowSceneDelegate;
pub use window_scene::WindowSceneDelegateImpl;

#[cfg(target_os = "ios")]
mod window_scene_activation_request_options;
#[cfg(target_os = "ios")]
pub use window_scene_activation_request_options::WindowSceneActivationRequestOpts;

mod window_scene_activation_configuration;
pub use window_scene_activation_configuration::WindowSceneActivationConfiguration;

#[inline]
pub fn app_main(
    principal_class_name: Option<&crate::ns::String>,
    delegate_class_name: Option<&crate::ns::String>,
) -> std::ffi::c_int {
    // may be make them public
    unsafe extern "C" {
        fn _NSGetArgc() -> *mut std::ffi::c_int;
        fn _NSGetArgv() -> *mut *mut *mut std::ffi::c_char;
        fn UIApplicationMain(
            argc: std::ffi::c_int,
            argv: *mut *mut std::ffi::c_char,
            principal_class_name: Option<&crate::ns::String>,
            delegate_class_name: Option<&crate::ns::String>,
        ) -> std::ffi::c_int;
    }

    unsafe {
        let argc = *_NSGetArgc();
        let argv = *_NSGetArgv();
        UIApplicationMain(argc, argv, principal_class_name, delegate_class_name)
    }
}
