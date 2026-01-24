use crate::{define_obj_type, ns};

define_obj_type!(
    #[doc(alias = "UICollectionViewLayout")]
    pub CollectionViewLayout(ns::Id)
);

define_obj_type!(
    #[doc(alias = "UICollectionViewLayoutAttributes")]
    pub CollectionViewLayoutAttrs(ns::Id)
);
