use crate::{define_obj_type, ui};

define_obj_type!(
    #[doc(alias = "UITapGestureRecognizer")]
    pub TapGestureRecognizer(ui::GestureRecognizer)
);
