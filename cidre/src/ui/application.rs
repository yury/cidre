use crate::{arc, define_obj_type, objc, ui};

#[objc::protocol(UIApplication)]
pub trait AppDelegate {
    #[objc::optional]
    #[objc::msg_send(applicationDidFinishLaunching:)]
    fn app_did_finish_launching(&mut self, app: &App);
}

define_obj_type!(
    pub App(ui::Responder),
    UI_APPLICATION
);

impl App {}

extern "C" {
    static UI_APPLICATION: &'static objc::Class<App>;
}
