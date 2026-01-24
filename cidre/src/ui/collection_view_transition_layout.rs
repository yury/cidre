use crate::{define_obj_type, ui};

define_obj_type!(
    #[doc(alias = "UICollectionViewTransitionLayout")]
    pub CollectionViewTransitionLayout(ui::CollectionViewLayout)
);
