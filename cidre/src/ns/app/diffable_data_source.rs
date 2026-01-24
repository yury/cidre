use crate::{define_obj_type, ns};

define_obj_type!(
    #[doc(alias = "NSCollectionViewDiffableDataSource")]
    pub CollectionViewDiffableDataSrc(ns::Id)
);

impl ns::CollectionViewDataSrc for CollectionViewDiffableDataSrc {}
