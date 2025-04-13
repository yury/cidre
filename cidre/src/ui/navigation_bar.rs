use crate::{define_obj_type, ui};

define_obj_type!(
    #[doc(alias = "UINavigationBar")]
    pub NavBar(ui::View)
);
