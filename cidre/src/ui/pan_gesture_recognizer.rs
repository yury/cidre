use crate::{define_obj_type, ui};

define_obj_type!(
    #[doc(alias = "UIPanGestureRecognizer")]
    pub PanGestureRecognizer(ui::GestureRecognizer)
);
