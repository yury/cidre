use crate::{arc, define_cls, define_obj_type, objc, ui};

define_obj_type!(
    #[doc(alias = "UIWindowSceneStandardPlacement")]
    pub WindowSceneStandardPlacement(ui::WindowScenePlacement)
);

impl WindowSceneStandardPlacement {
    #[objc::available(ios = 17.0, tvos = 17.0)]
    define_cls!(UI_WINDOW_SCENE_STANDARD_PLACEMENT);

    #[objc::msg_send(standardPlacement)]
    pub fn new() -> arc::R<Self>;
}

#[link(name = "ui", kind = "static")]
unsafe extern "C" {
    static UI_WINDOW_SCENE_STANDARD_PLACEMENT: &'static objc::Class<WindowSceneStandardPlacement>;
}
