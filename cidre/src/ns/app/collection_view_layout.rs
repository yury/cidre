use crate::{define_obj_type, ns};

define_obj_type!(
    #[doc(alias = "NSCollectionViewLayout")]
    pub CollectionViewLayout(ns::Id)
);

define_obj_type!(
    #[doc(alias = "NSCollectionViewLayoutAttributes")]
    pub CollectionViewLayoutAttrs(ns::Id)
);
