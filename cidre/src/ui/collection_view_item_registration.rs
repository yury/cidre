use crate::{define_obj_type, ns};

define_obj_type!(
    #[doc(alias = "UICollectionViewCellRegistration")]
    pub CollectionViewCellRegistration(ns::Id)
);

define_obj_type!(
    #[doc(alias = "UICollectionViewSupplementaryRegistration")]
    pub CollectionViewSupplementaryRegistration(ns::Id)
);
