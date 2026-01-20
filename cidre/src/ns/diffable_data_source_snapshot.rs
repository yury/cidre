use std::marker::PhantomData;

use crate::{
    api, arc, define_cls,
    ns::{self, Copying},
    objc,
};

#[doc(alias = "NSDiffableDataSourceSnapshot")]
#[repr(transparent)]
pub struct DiffableDataSrcSnapshot<S, I>(ns::Id, PhantomData<S>, PhantomData<I>);

unsafe impl<S, I> Send for DiffableDataSrcSnapshot<S, I>
where
    S: objc::Obj,
    I: objc::Obj,
{
}

impl<S: objc::Obj, I: objc::Obj> arc::A<DiffableDataSrcSnapshot<S, I>> {
    #[objc::msg_send(init)]
    pub fn init(self) -> arc::R<DiffableDataSrcSnapshot<S, I>>;
}

impl<S: objc::Obj, I: objc::Obj> objc::Obj for DiffableDataSrcSnapshot<S, I> {}

impl<S: objc::Obj, I: objc::Obj> DiffableDataSrcSnapshot<S, I> {
    define_cls!(NS_DIFFABLE_DATA_SOURCE_SNAPSHOT);

    #[inline]
    pub fn new() -> arc::R<Self> {
        Self::alloc().init()
    }

    #[objc::msg_send(numberOfItems)]
    pub fn items_n(&self) -> ns::Integer;

    #[objc::msg_send(numberOfSections)]
    pub fn sections_n(&self) -> ns::Integer;

    #[objc::msg_send(sectionIdentifiers)]
    pub fn section_ids(&self) -> arc::R<ns::Array<S>>;

    #[objc::msg_send(itemIdentifiers)]
    pub fn item_ids(&self) -> arc::R<ns::Array<I>>;

    #[objc::msg_send(numberOfItemsInSection:)]
    pub unsafe fn items_n_in_section_throws(&self, section_id: &S) -> ns::Integer;

    #[inline]
    pub fn items_n_in_section<'ear>(
        &self,
        section_id: impl AsRef<S>,
    ) -> ns::ExResult<'ear, ns::Integer> {
        ns::try_catch(|| unsafe { self.items_n_in_section_throws(section_id.as_ref()) })
    }

    #[objc::msg_send(itemIdentifiersInSectionWithIdentifier:)]
    pub unsafe fn item_ids_in_section_throws(&self, section_id: &S) -> arc::R<ns::Array<I>>;

    #[inline]
    pub fn item_ids_in_section<'ear>(
        &self,
        section_id: impl AsRef<S>,
    ) -> ns::ExResult<'ear, arc::R<ns::Array<I>>> {
        ns::try_catch(|| unsafe { self.item_ids_in_section_throws(section_id.as_ref()) })
    }

    #[objc::msg_send(sectionIdentifierForSectionContainingItemIdentifier:)]
    pub fn section_id_containing_item_id(&self, item_id: &I) -> Option<arc::R<S>>;

    #[objc::msg_send(indexOfItemIdentifier:)]
    pub fn index_of_item_id(&self, item_id: &I) -> ns::Integer;

    #[objc::msg_send(indexOfSectionIdentifier:)]
    pub fn index_of_section_id(&self, section_id: &S) -> ns::Integer;

    #[objc::msg_send(appendItemsWithIdentifiers:)]
    pub unsafe fn append_items_throws(&mut self, items: &ns::Array<I>);

    #[inline]
    pub fn append_items<'ear>(&mut self, items: &ns::Array<I>) -> ns::ExResult<'ear> {
        ns::try_catch(|| unsafe { self.append_items_throws(items) })
    }

    #[objc::msg_send(appendItemsWithIdentifiers:intoSectionWithIdentifier:)]
    pub unsafe fn append_items_to_section_throws(&mut self, items: &ns::Array<I>, section_id: &S);

    #[inline]
    pub fn append_items_to_section<'ear>(
        &mut self,
        items: &ns::Array<I>,
        section_id: impl AsRef<S>,
    ) -> ns::ExResult<'ear> {
        ns::try_catch(|| unsafe { self.append_items_to_section_throws(items, section_id.as_ref()) })
    }

    #[objc::msg_send(insertItemsWithIdentifiers:beforeItemWithIdentifier:)]
    pub unsafe fn insert_items_before_item_throws(&mut self, items: &ns::Array<I>, item: &I);

    #[inline]
    pub fn insert_items_before_item<'ear>(
        &mut self,
        items: &ns::Array<I>,
        item: impl AsRef<I>,
    ) -> ns::ExResult<'ear> {
        ns::try_catch(|| unsafe { self.insert_items_before_item_throws(items, item.as_ref()) })
    }

    #[objc::msg_send(insertItemsWithIdentifiers:afterItemWithIdentifier:)]
    pub unsafe fn insert_items_after_item_throws(&mut self, items: &ns::Array<I>, item: &I);

    #[inline]
    pub fn insert_items_after_item<'ear>(
        &mut self,
        items: &ns::Array<I>,
        item: impl AsRef<I>,
    ) -> ns::ExResult<'ear> {
        ns::try_catch(|| unsafe { self.insert_items_after_item_throws(items, item.as_ref()) })
    }

    #[objc::msg_send(deleteItemsWithIdentifiers:)]
    pub unsafe fn delete_items_throws(&mut self, items: &ns::Array<I>);

    #[inline]
    pub fn delete_items<'ear>(&mut self, items: &ns::Array<I>) -> ns::ExResult<'ear> {
        ns::try_catch(|| unsafe { self.delete_items_throws(items) })
    }

    #[objc::msg_send(deleteAllItems)]
    pub fn delete_all_items(&mut self);

    #[objc::msg_send(moveItemWithIdentifier:beforeItemWithIdentifier:)]
    pub unsafe fn move_item_before_item_throws(&mut self, item: &I, before: &I);

    #[inline]
    pub fn move_item_before_item<'ear>(
        &mut self,
        item: impl AsRef<I>,
        before: impl AsRef<I>,
    ) -> ns::ExResult<'ear> {
        ns::try_catch(|| unsafe {
            self.move_item_before_item_throws(item.as_ref(), before.as_ref())
        })
    }

    #[objc::msg_send(moveItemWithIdentifier:afterItemWithIdentifier:)]
    pub unsafe fn move_item_after_item_throws(&mut self, item: &I, after: &I);

    #[inline]
    pub fn move_item_after_item<'ear>(
        &mut self,
        item: impl AsRef<I>,
        after: impl AsRef<I>,
    ) -> ns::ExResult<'ear> {
        ns::try_catch(|| unsafe { self.move_item_after_item_throws(item.as_ref(), after.as_ref()) })
    }

    #[objc::msg_send(reloadItemsWithIdentifiers:)]
    pub unsafe fn reload_items_throws(&mut self, items: &ns::Array<I>);

    #[inline]
    pub fn reload_items<'ear>(&mut self, items: &ns::Array<I>) -> ns::ExResult<'ear> {
        ns::try_catch(|| unsafe { self.reload_items_throws(items) })
    }

    #[objc::msg_send(reconfigureItemsWithIdentifiers:)]
    #[api::available(ios = 15.0, tvos = 15.0)]
    pub unsafe fn reconfigure_items_throws(&mut self, items: &ns::Array<I>);

    #[inline]
    #[api::available(ios = 15.0, tvos = 15.0)]
    pub fn reconfigure_items<'ear>(&mut self, items: &ns::Array<I>) -> ns::ExResult<'ear> {
        ns::try_catch(|| unsafe { self.reconfigure_items_throws(items) })
    }

    #[objc::msg_send(appendSectionsWithIdentifiers:)]
    pub unsafe fn append_sections_throws(&mut self, sections: &ns::Array<S>);

    #[inline]
    pub fn append_sections<'ear>(&mut self, sections: &ns::Array<S>) -> ns::ExResult<'ear> {
        ns::try_catch(|| unsafe { self.append_sections_throws(sections) })
    }

    #[objc::msg_send(insertSectionsWithIdentifiers:beforeSectionWithIdentifier:)]
    pub unsafe fn insert_sections_before_section_throws(
        &mut self,
        sections: &ns::Array<S>,
        before: &S,
    );

    #[inline]
    pub fn insert_sections_before_section<'ear>(
        &mut self,
        sections: &ns::Array<S>,
        before: impl AsRef<S>,
    ) -> ns::ExResult<'ear> {
        ns::try_catch(|| unsafe {
            self.insert_sections_before_section_throws(sections, before.as_ref())
        })
    }

    #[objc::msg_send(insertSectionsWithIdentifiers:afterSectionWithIdentifier:)]
    pub unsafe fn insert_sections_after_section_throws(
        &mut self,
        sections: &ns::Array<S>,
        after: &S,
    );

    #[inline]
    pub fn insert_sections_after_section<'ear>(
        &mut self,
        sections: &ns::Array<S>,
        after: impl AsRef<S>,
    ) -> ns::ExResult<'ear> {
        ns::try_catch(|| unsafe {
            self.insert_sections_after_section_throws(sections, after.as_ref())
        })
    }

    #[objc::msg_send(deleteSectionsWithIdentifiers:)]
    pub unsafe fn delete_sections_throws(&mut self, sections: &ns::Array<S>);

    #[inline]
    pub fn delete_sections<'ear>(&mut self, sections: &ns::Array<S>) -> ns::ExResult<'ear> {
        ns::try_catch(|| unsafe { self.delete_sections_throws(sections) })
    }

    #[objc::msg_send(moveSectionWithIdentifier:beforeSectionWithIdentifier:)]
    pub unsafe fn move_section_before_section_throws(&mut self, section: &S, before: &S);

    #[inline]
    pub fn move_section_before_section<'ear>(
        &mut self,
        section: impl AsRef<S>,
        before: impl AsRef<S>,
    ) -> ns::ExResult<'ear> {
        ns::try_catch(|| unsafe {
            self.move_section_before_section_throws(section.as_ref(), before.as_ref())
        })
    }

    #[objc::msg_send(moveSectionWithIdentifier:afterSectionWithIdentifier:)]
    pub unsafe fn move_section_after_section_throws(&mut self, section: &S, after: &S);

    #[inline]
    pub fn move_section_after_section<'ear>(
        &mut self,
        section: impl AsRef<S>,
        after: impl AsRef<S>,
    ) -> ns::ExResult<'ear> {
        ns::try_catch(|| unsafe {
            self.move_section_after_section_throws(section.as_ref(), after.as_ref())
        })
    }

    #[objc::msg_send(reloadSectionsWithIdentifiers:)]
    pub unsafe fn reload_sections_throws(&mut self, sections: &ns::Array<S>);

    #[inline]
    pub fn reload_sections<'ear>(&mut self, sections: &ns::Array<S>) -> ns::ExResult<'ear> {
        ns::try_catch(|| unsafe { self.reload_sections_throws(sections) })
    }
}

impl<S: objc::Obj, I: objc::Obj> Clone for DiffableDataSrcSnapshot<S, I> {
    fn clone(&self) -> Self {
        unsafe { std::mem::transmute(self.copy_with_zone(std::ptr::null_mut())) }
    }
}

impl<S: objc::Obj, I: objc::Obj> ns::Copying for DiffableDataSrcSnapshot<S, I> {}

#[cfg(all(feature = "app", target_os = "macos"))]
#[link(name = "app", kind = "static")]
unsafe extern "C" {
    static NS_DIFFABLE_DATA_SOURCE_SNAPSHOT:
        &'static objc::Class<DiffableDataSrcSnapshot<ns::Id, ns::Id>>;
}

#[cfg(all(
    feature = "ui",
    any(target_os = "ios", target_os = "tvos", target_os = "visionos")
))]
#[link(name = "ui", kind = "static")]
unsafe extern "C" {
    static NS_DIFFABLE_DATA_SOURCE_SNAPSHOT:
        &'static objc::Class<DiffableDataSrcSnapshot<ns::Id, ns::Id>>;
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
            .append_items(&ns::arr![ns::str!(c"foo")])
            .expect_err("we need to add section first");

        snapshot
            .append_sections(&ns::arr![0])
            .expect("Failed to add section");

        snapshot
            .append_sections(&ns::arr![0])
            .expect_err("we already have section with id 0");

        snapshot
            .append_items(&ns::arr![ns::str!(c"foo")])
            .expect("Failed to append item in section");

        assert_eq!(snapshot.sections_n(), 1);
        assert_eq!(snapshot.items_n(), 1);
        assert_eq!(snapshot.items_n_in_section(0u8), Ok(1));
        snapshot
            .items_n_in_section(1u8)
            .expect_err("No section with id 1");

        snapshot
            .delete_items(&ns::arr![ns::str!(c"b"), ns::str!(c"!"), ns::str!(c"!")])
            .expect_err("Not unique id err");

        snapshot
            .reload_items(&ns::arr![ns::str!(c"b"), ns::str!(c"!")])
            .expect_err("b doesnt exists");

        snapshot.reload_sections(&ns::arr![10u8]).expect_err(
            "Attempted to reload section identifier that does not exist in the snapshot: 10",
        );
        snapshot
            .reload_sections(&ns::arr![0u8])
            .expect("Failed to reload section with id 0");

        let clone = snapshot.clone();
        assert_eq!(clone.items_n(), snapshot.items_n());
    }

    #[cfg(target_os = "ios")]
    #[test]
    fn reconfigure_items() {
        let mut snapshot = ns::DiffableDataSrcSnapshot::<ns::Number, ns::String>::new();
        snapshot
            .append_sections(&ns::arr![0])
            .expect("Failed to add section");
        snapshot
            .append_items(&ns::arr![ns::str!(c"foo"), ns::str!(c"bar")])
            .expect("Failed to append items");

        assert_eq!(snapshot.items_n(), 2);

        snapshot
            .reconfigure_items(&ns::arr![ns::str!(c"foo")])
            .expect("Failed to reconfigure item");

        assert_eq!(snapshot.items_n(), 2);
    }
}
