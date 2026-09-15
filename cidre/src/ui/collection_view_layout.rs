use crate::{cg, define_obj_type, ns, objc};

#[doc(alias = "UICollectionViewScrollDirection")]
#[derive(Debug, Eq, PartialEq, Copy, Clone, Hash)]
#[repr(isize)]
pub enum CollectionViewScrollDir {
    Vectical,
    Horizontal,
}

pub mod element_kind {
    use crate::ns;

    #[doc(alias = "UICollectionElementKindSectionHeader")]
    pub fn section_header() -> &'static ns::String {
        unsafe { UICollectionElementKindSectionHeader }
    }

    #[doc(alias = "UICollectionElementKindSectionFooter")]
    pub fn section_footer() -> &'static ns::String {
        unsafe { UICollectionElementKindSectionFooter }
    }

    unsafe extern "C" {
        static UICollectionElementKindSectionHeader: &'static ns::String;
        static UICollectionElementKindSectionFooter: &'static ns::String;
    }
}

define_obj_type!(
    #[doc(alias = "UICollectionViewLayout")]
    pub CollectionViewLayout(ns::Id)
);

impl CollectionViewLayout {
    /// The size of the whole content, once laid out.
    #[objc::msg_send(collectionViewContentSize)]
    pub fn collection_view_content_size(&self) -> cg::Size;
}

define_obj_type!(
    #[doc(alias = "UICollectionViewLayoutAttributes")]
    pub CollectionViewLayoutAttrs(ns::Id)
);
