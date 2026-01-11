use crate::{define_obj_type, ns};

define_obj_type!(
    #[doc(alias = "NSCollectionViewDiffableDataSource")]
    pub _CollectionViewDiffableDataSrc(ns::Id)
);

impl ns::CollectionViewDataSrc for _CollectionViewDiffableDataSrc {}


impl _CollectionViewDiffableDataSrc {    
}
