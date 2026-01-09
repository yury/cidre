use std::marker::PhantomData;

use crate::{arc, define_obj_type, ns, objc};

define_obj_type!(
    pub _DiffableDataSrcSnapshot(ns::Id),
    NS_DIFFABLE_DATA_SOURCE_SNAPSHOT
);

impl _DiffableDataSrcSnapshot {
    #[objc::msg_send(numberOfItems)]
    pub fn items_n(&self) -> ns::Integer;

    #[objc::msg_send(numberOfSections)]
    pub fn sections_n(&self) -> ns::Integer;

    #[objc::msg_send(sectionIdentifiers)]
    pub fn section_ids(&self) -> arc::R<ns::Array<ns::Id>>;

    #[objc::msg_send(itemIdentifiers)]
    pub fn item_ids(&self) -> arc::R<ns::Array<ns::Id>>;

    #[objc::msg_send(numberOfItemsInSection:)]
    pub unsafe fn items_n_in_section_throws(&self, section_id: &ns::Id) -> ns::Integer;

    #[objc::msg_send(itemIdentifiersInSectionWithIdentifier:)]
    pub unsafe fn item_ids_in_section_throws(
        &self,
        section_id: &ns::Id,
    ) -> arc::R<ns::Array<ns::Id>>;

    #[objc::msg_send(sectionIdentifierForSectionContainingItemIdentifier:)]
    pub fn section_id_containing_item_id(&self, item_id: &ns::Id) -> Option<arc::R<ns::Id>>;

    #[objc::msg_send(indexOfItemIdentifier:)]
    pub fn index_of_item_id(&self, item_id: &ns::Id) -> ns::Integer;

    #[objc::msg_send(indexOfSectionIdentifier:)]
    pub fn index_of_section_id(&self, section_id: &ns::Id) -> ns::Integer;
}

/// Items operations
impl _DiffableDataSrcSnapshot {
    #[objc::msg_send(appendItemsWithIdentifiers:)]
    pub unsafe fn append_item_ids_throws(&mut self, item_ids: &ns::Array<ns::Id>);

    #[objc::msg_send(appendItemsWithIdentifiers:intoSectionWithIdentifier:)]
    pub unsafe fn append_item_ids_to_section_id_throws(
        &mut self,
        item_ids: &ns::Array<ns::Id>,
        section_id: &ns::Id,
    );

    #[objc::msg_send(insertItemsWithIdentifiers:beforeItemWithIdentifier:)]
    pub unsafe fn insert_item_ids_before_item_id_throws(
        &mut self,
        item_ids: &ns::Array<ns::Id>,
        item_id: &ns::Id,
    );

    #[objc::msg_send(insertItemsWithIdentifiers:afterItemWithIdentifier:)]
    pub unsafe fn insert_item_ids_after_item_id_throws(
        &mut self,
        item_ids: &ns::Array<ns::Id>,
        item_id: &ns::Id,
    );

    #[objc::msg_send(deleteAllItems)]
    pub fn delete_all_items(&mut self);
}

/// Section operations
impl _DiffableDataSrcSnapshot {
    #[objc::msg_send(appendSectionsWithIdentifiers:)]
    pub unsafe fn append_section_ids_throws(&mut self, section_ids: &ns::Array<ns::Id>);
}

impl ns::Copying for _DiffableDataSrcSnapshot {}

pub struct DiffableDataSrcSnapshot<S, I>(
    arc::R<_DiffableDataSrcSnapshot>,
    PhantomData<S>,
    PhantomData<I>,
);

impl<S: objc::Obj, I: objc::Obj> DiffableDataSrcSnapshot<S, I> {
    #[inline]
    pub fn new() -> Self {
        Self(_DiffableDataSrcSnapshot::new(), PhantomData, PhantomData)
    }

    #[inline]
    pub fn items_n(&self) -> ns::Integer {
        self.0.items_n()
    }

    #[inline]
    pub fn sections_n(&self) -> ns::Integer {
        self.0.sections_n()
    }

    #[inline]
    pub fn section_ids(&self) -> arc::R<ns::Array<S>> {
        unsafe { std::mem::transmute(self.0.section_ids()) }
    }

    #[inline]
    pub fn item_ids(&self) -> arc::R<ns::Array<I>> {
        unsafe { std::mem::transmute(self.0.item_ids()) }
    }

    #[inline]
    pub unsafe fn items_n_in_section_throws(&self, section_id: impl AsRef<S>) -> ns::Integer {
        unsafe {
            self.0
                .items_n_in_section_throws(section_id.as_ref().as_id_ref())
        }
    }

    #[inline]
    pub fn items_n_in_section<'ear>(
        &self,
        section_id: impl AsRef<S>,
    ) -> ns::ExResult<'ear, ns::Integer> {
        ns::try_catch(|| unsafe {
            self.0
                .items_n_in_section_throws(section_id.as_ref().as_id_ref())
        })
    }

    #[inline]
    pub unsafe fn item_ids_in_section_throws(
        &self,
        section_id: impl AsRef<S>,
    ) -> arc::R<ns::Array<I>> {
        unsafe {
            std::mem::transmute(
                self.0
                    .item_ids_in_section_throws(section_id.as_ref().as_id_ref()),
            )
        }
    }

    #[inline]
    pub fn item_ids_in_section<'ear>(
        &self,
        section_id: impl AsRef<S>,
    ) -> ns::ExResult<'ear, arc::R<ns::Array<I>>> {
        ns::try_catch(|| unsafe { self.item_ids_in_section_throws(section_id) })
    }

    #[inline]
    pub fn section_id_containing_item_id(&self, item_id: impl AsRef<I>) -> Option<arc::R<S>> {
        unsafe {
            std::mem::transmute(
                self.0
                    .section_id_containing_item_id(item_id.as_ref().as_id_ref()),
            )
        }
    }

    #[inline]
    pub fn index_of_item_id(&self, item_id: impl AsRef<I>) -> ns::Integer {
        self.0.index_of_section_id(item_id.as_ref().as_id_ref())
    }

    #[inline]
    pub fn index_of_section_id(&self, section_id: impl AsRef<S>) -> ns::Integer {
        self.0.index_of_section_id(section_id.as_ref().as_id_ref())
    }
}

/// Items operations
impl<S: objc::Obj, I: objc::Obj> DiffableDataSrcSnapshot<S, I> {
    #[inline]
    pub unsafe fn append_item_ids_throws(&mut self, item_ids: &ns::Array<I>) {
        unsafe {
            self.0.append_item_ids_throws(std::mem::transmute(item_ids));
        }
    }

    /// Appends into last section
    #[inline]
    pub fn append_item_ids<'ear>(&mut self, item_ids: &ns::Array<I>) -> ns::ExResult<'ear> {
        ns::try_catch(|| unsafe {
            self.append_item_ids_throws(item_ids);
        })
    }

    #[inline]
    pub unsafe fn append_item_ids_to_section_id_throws(
        &mut self,
        item_ids: &ns::Array<I>,
        section_id: impl AsRef<S>,
    ) {
        unsafe {
            self.0.append_item_ids_to_section_id_throws(
                std::mem::transmute(item_ids),
                section_id.as_ref().as_id_ref(),
            );
        }
    }

    #[inline]
    pub fn append_item_ids_to_section_id<'ear>(
        &mut self,
        item_ids: &ns::Array<I>,
        section_id: impl AsRef<S>,
    ) -> ns::ExResult<'ear> {
        ns::try_catch(|| unsafe {
            self.append_item_ids_to_section_id_throws(item_ids, section_id);
        })
    }
}

/// Section operations
impl<S: objc::Obj, I: objc::Obj> DiffableDataSrcSnapshot<S, I> {
    pub unsafe fn append_section_ids_throws(&mut self, section_ids: &ns::Array<S>) {
        unsafe {
            self.0
                .append_section_ids_throws(std::mem::transmute(section_ids));
        }
    }

    pub fn append_section_ids<'ear>(&mut self, section_ids: &ns::Array<S>) -> ns::ExResult<'ear> {
        ns::try_catch(|| unsafe { self.append_section_ids_throws(section_ids) })
    }
}

unsafe extern "C" {
    static NS_DIFFABLE_DATA_SOURCE_SNAPSHOT: &'static objc::Class<_DiffableDataSrcSnapshot>;
}

#[cfg(test)]
mod tests {
    use crate::ns;

    #[test]
    fn basics() {
        let mut snapshot = ns::DiffableDataSrcSnapshot::<ns::Number, ns::String>::new();
        assert_eq!(0, snapshot.items_n());
        assert!(snapshot.item_ids().is_empty());
        let index = snapshot.index_of_item_id(ns::str!(c"foo"));
        assert_eq!(index, ns::NOT_FOUND);

        snapshot
            .append_item_ids(&ns::arr!(ns::str!(c"foo")))
            .expect_err("we need to add section first");

        snapshot
            .append_section_ids(&ns::arr![0])
            .expect("Failed to add section");

        snapshot
            .append_section_ids(&ns::arr![0])
            .expect_err("we already have section with id 0");

        snapshot
            .append_item_ids(&ns::arr!(ns::str!(c"foo")))
            .expect("Failed to append item in section");

        assert_eq!(snapshot.sections_n(), 1);
        assert_eq!(snapshot.items_n(), 1);
        assert_eq!(snapshot.items_n_in_section(0u8), Ok(1));
        snapshot
            .items_n_in_section(1u8)
            .expect_err("No section with id 1");
    }
}
