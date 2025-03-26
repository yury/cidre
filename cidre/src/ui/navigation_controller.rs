use crate::{define_obj_type, ui};

define_obj_type!(
    #[doc(alias = "UINavigationController")]
    pub NavController(ui::ViewController)
);
