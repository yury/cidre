use crate::{arc, define_obj_type, objc, ui};

#[objc::protocol(UIApplication)]
pub trait AppDelegate {
    #[objc::optional]
    #[objc::msg_send(applicationDidFinishLaunching:)]
    fn app_did_finish_launching(&mut self, app: &App);

    #[objc::optional]
    #[objc::msg_send(application:configurationForConnectingSceneSession:options:)]
    fn app_cfg_for_connecting_scene_session<'ar>(
        &mut self,
        app: &App,
        session: &ui::SceneSession,
        options: &ui::SceneConnectionOpts,
    ) -> &'ar mut ui::SceneCfg;

    #[objc::optional]
    #[objc::msg_send(window)]
    fn window(&self) -> Option<&ui::Window>;

    #[objc::optional]
    #[objc::msg_send(setWindow:)]
    fn set_window(&mut self, val: Option<&ui::Window>);
}

define_obj_type!(
    pub App(ui::Responder),
    UI_APPLICATION
);

impl App {}

extern "C" {
    static UI_APPLICATION: &'static objc::Class<App>;
}
