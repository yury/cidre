use crate::{define_obj_type, ui};

define_obj_type!(
    #[doc(alias = "UICollectionReusableView")]
    pub CollectionReusableView(ui::View)
);

define_obj_type!(
    #[doc(alias = "UICollectionViewCell")]
    pub CollectionViewCell(ui::CollectionReusableView)
);
