use crate::{arc, define_obj_type, ns, objc};

define_obj_type!(
    #[doc(alias = "NSCollectionViewFlowLayout")]
    pub CollectionViewFlowLayout(ns::CollectionViewLayout),
    sym NSCollectionViewFlowLayout
);

impl CollectionViewFlowLayout {
    #[objc::msg_send(setItemSize:)]
    pub fn set_item_size(&mut self, val: ns::Size);

    #[objc::msg_send(setSectionInset:)]
    pub fn set_section_inset(&mut self, val: ns::EdgeInsets);
}

#[doc(alias = "NSCollectionViewScrollDirection")]
#[derive(Debug, Eq, PartialEq, Copy, Clone, Hash)]
#[repr(isize)]
pub enum CollectionViewScrollDir {
    Vectical,
    Horizontal,
}

pub mod element_kind {
    use crate::ns::CollectionViewElementKind;

    #[doc(alias = "NSCollectionElementKindSectionHeader")]
    pub fn section_header() -> &'static CollectionViewElementKind {
        unsafe { NSCollectionElementKindSectionHeader }
    }

    #[doc(alias = "NSCollectionElementKindSectionFooter")]
    pub fn section_footer() -> &'static CollectionViewElementKind {
        unsafe { NSCollectionElementKindSectionFooter }
    }

    unsafe extern "C" {
        static NSCollectionElementKindSectionHeader: &'static CollectionViewElementKind;
        static NSCollectionElementKindSectionFooter: &'static CollectionViewElementKind;
    }
}
