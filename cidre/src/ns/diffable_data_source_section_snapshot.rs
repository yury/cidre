use std::marker::PhantomData;

use crate::{
    arc, define_cls,
    ns::{self, Copying},
    objc,
};

#[doc(alias = "NSDiffableDataSourceSectionSnapshot")]
#[repr(transparent)]
pub struct DiffableDataSrcSectionSnapshot<I>(ns::Id, PhantomData<I>);

unsafe impl<I> Send for DiffableDataSrcSectionSnapshot<I> where I: objc::Obj {}

impl<I: objc::Obj> arc::A<DiffableDataSrcSectionSnapshot<I>> {
    #[objc::msg_send(init)]
    pub fn init(self) -> arc::R<DiffableDataSrcSectionSnapshot<I>>;
}

impl<I: objc::Obj> objc::Obj for DiffableDataSrcSectionSnapshot<I> {}

impl<I: objc::Obj> DiffableDataSrcSectionSnapshot<I> {
    define_cls!(NS_DIFFABLE_DATA_SOURCE_SECTION_SNAPSHOT);

    #[inline]
    pub fn new() -> arc::R<Self> {
        Self::cls().alloc().init()
    }

    #[objc::msg_send(numberOfItems)]
    pub fn items_n(&self) -> ns::Integer;

    #[objc::msg_send(itemIdentifiers)]
    pub fn items(&self) -> arc::R<ns::Array<I>>;

    #[objc::msg_send(rootItems)]
    pub fn root_items(&self) -> arc::R<ns::Array<I>>;

    #[objc::msg_send(indexOfItemIdentifier:)]
    pub fn index_of_item_id(&self, item_id: &I) -> ns::Integer;

    #[objc::msg_send(childrenOfItemIdentifier:)]
    pub fn children_of_item_id(&self, item_id: &I) -> arc::R<ns::Array<I>>;

    #[objc::msg_send(parentForItemIdentifier:)]
    pub fn parent_of_item_id(&self, item_id: &I) -> Option<arc::R<I>>;

    #[objc::msg_send(appendItemsWithIdentifiers:)]
    pub unsafe fn append_items_throws(&mut self, items: &ns::Array<I>);

    #[inline]
    pub fn append_items<'ear>(&mut self, items: &ns::Array<I>) -> ns::ExResult<'ear> {
        ns::try_catch(|| unsafe { self.append_items_throws(items) })
    }

    #[objc::msg_send(appendItemsWithIdentifiers:intoItemWithIdentifier:)]
    pub unsafe fn append_items_into_throws(&mut self, items: &ns::Array<I>, parent: &I);

    #[inline]
    pub fn append_items_into<'ear>(
        &mut self,
        items: &ns::Array<I>,
        parent: impl AsRef<I>,
    ) -> ns::ExResult<'ear> {
        ns::try_catch(|| unsafe { self.append_items_into_throws(items, parent.as_ref()) })
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
    #[objc::available(ios = 15.0, tvos = 15.0, macos = 12.0, watchos = 8.0, visionos = 1.0)]
    pub unsafe fn reconfigure_items_throws(&mut self, items: &ns::Array<I>);

    #[objc::msg_send(expandItemsWithIdentifiers:)]
    pub unsafe fn expand_items_throws(&mut self, items: &ns::Array<I>);

    #[inline]
    pub fn expand_items<'ear>(&mut self, items: &ns::Array<I>) -> ns::ExResult<'ear> {
        ns::try_catch(|| unsafe { self.expand_items_throws(items) })
    }

    #[objc::msg_send(collapseItemsWithIdentifiers:)]
    pub unsafe fn collapse_items_throws(&mut self, items: &ns::Array<I>);

    #[inline]
    pub fn collapse_items<'ear>(&mut self, items: &ns::Array<I>) -> ns::ExResult<'ear> {
        ns::try_catch(|| unsafe { self.collapse_items_throws(items) })
    }

    #[objc::msg_send(collapseItemsWithIdentifiers:mode:)]
    pub unsafe fn collapse_items_mode_throws(&mut self, items: &ns::Array<I>, mode: ns::Integer);

    #[inline]
    pub fn collapse_items_mode<'ear>(
        &mut self,
        items: &ns::Array<I>,
        mode: ns::Integer,
    ) -> ns::ExResult<'ear> {
        ns::try_catch(|| unsafe { self.collapse_items_mode_throws(items, mode) })
    }

    #[objc::msg_send(isItemExpanded:)]
    pub fn is_item_expanded(&self, item: &I) -> bool;

    #[objc::msg_send(visualDescription)]
    pub fn visual_description(&self) -> arc::R<ns::String>;
}

impl<I: objc::Obj> Clone for DiffableDataSrcSectionSnapshot<I> {
    fn clone(&self) -> Self {
        unsafe { std::mem::transmute(self.copy_with_zone(std::ptr::null_mut())) }
    }
}

impl<I: objc::Obj> ns::Copying for DiffableDataSrcSectionSnapshot<I> {}

#[link(name = "ui", kind = "static")]
unsafe extern "C" {
    static NS_DIFFABLE_DATA_SOURCE_SECTION_SNAPSHOT:
        &'static objc::Class<DiffableDataSrcSectionSnapshot<ns::Id>>;
}
