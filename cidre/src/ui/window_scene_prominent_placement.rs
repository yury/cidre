use crate::{arc, define_obj_type, objc, ui};

define_obj_type!(
    #[doc(alias = "UIWindowSceneProminentPlacement")]
    pub WindowSceneProminentPlacement(ui::WindowScenePlacement)
);

impl WindowSceneProminentPlacement {
    #[objc::available(ios = 17.0)]
    crate::define_cls!(UI_WINDOW_SCENE_PROMINENT_PLACEMENT);

    #[objc::msg_send(prominentPlacement)]
    #[objc::available(ios = 17.0)]
    pub fn new() -> arc::R<Self>;
}

#[cfg(target_os = "ios")]
#[link(name = "ui", kind = "static")]
unsafe extern "C" {
    static UI_WINDOW_SCENE_PROMINENT_PLACEMENT: &'static objc::Class<WindowSceneProminentPlacement>;
}
