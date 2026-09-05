use crate::{arc, define_obj_type, objc, ui};

define_obj_type!(
    #[doc(alias = "UICollectionViewCompositionalLayout")]
    pub CollectionViewCompositionalLayout(ui::CollectionViewLayout),
    UI_COLLECTION_VIEW_COMPOSITIONAL_LAYOUT
);

unsafe extern "C" {
    static UI_COLLECTION_VIEW_COMPOSITIONAL_LAYOUT:
        &'static objc::Class<CollectionViewCompositionalLayout>;
}
