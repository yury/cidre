use crate::{define_obj_type, objc, ui};

define_obj_type!(
    pub WindowScene(ui::Scene)
);

#[objc::protocol(UIWindowSceneDelegate)]
pub trait WindowSceneDelegate {
    #[objc::msg_send(window)]
    fn window(&self) -> Option<&ui::Window>;

    #[objc::msg_send(setWindow:)]
    fn set_window(&mut self, val: Option<&ui::Window>);
}
