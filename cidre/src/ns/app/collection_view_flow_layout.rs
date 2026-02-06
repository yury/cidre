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

    #[link(name = "AppKit", kind = "framework")]
    unsafe extern "C" {
        static NSCollectionElementKindSectionHeader: &'static CollectionViewElementKind;
        static NSCollectionElementKindSectionFooter: &'static CollectionViewElementKind;
    }
}
