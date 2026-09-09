use std::marker::PhantomData;

use crate::{
    arc, define_cls,
    ns::{self, Copying},
    objc,
};

/// A hierarchical snapshot of the items in one section: items may have
/// children, and parents can be expanded or collapsed.
#[doc(alias = "NSDiffableDataSourceSectionSnapshot")]
#[repr(transparent)]
pub struct DiffableDataSrcSectionSnapshot<I>(ns::Id, PhantomData<I>);

unsafe impl<I> Send for DiffableDataSrcSectionSnapshot<I> where I: objc::Obj {}

impl<I: objc::Obj> objc::Obj for DiffableDataSrcSectionSnapshot<I> {}

impl<I: objc::Obj> DiffableDataSrcSectionSnapshot<I> {
    #[objc::init(init)]
    pub fn init(self) -> arc::R<DiffableDataSrcSectionSnapshot<I>>;

    define_cls!(NS_DIFFABLE_DATA_SOURCE_SECTION_SNAPSHOT);

    #[inline]
    pub fn new() -> arc::R<Self> {
        Self::cls().alloc().init()
    }

    /// All items, in depth-first order.
    #[objc::msg_send(items)]
    pub fn items(&self) -> arc::R<ns::Array<I>>;

    /// The items without a parent.
    #[objc::msg_send(rootItems)]
    pub fn root_items(&self) -> arc::R<ns::Array<I>>;

    /// The items whose ancestors are all expanded.
    #[objc::msg_send(visibleItems)]
    pub fn visible_items(&self) -> arc::R<ns::Array<I>>;

    #[objc::msg_send(expandedItems)]
    pub fn expanded_items(&self) -> arc::R<ns::Array<I>>;

    /// `ns::NOT_FOUND` if the item is not in the snapshot.
    #[objc::msg_send(indexOfItem:)]
    pub fn index_of_item(&self, item: &I) -> ns::Integer;

    /// The depth of the item; `ns::NOT_FOUND` if it is not in the snapshot.
    #[objc::msg_send(levelOfItem:)]
    pub fn level_of_item(&self, item: &I) -> ns::Integer;

    #[objc::msg_send(containsItem:)]
    pub fn contains_item(&self, item: &I) -> bool;

    /// `None` for a root item.
    #[objc::msg_send(parentOfChildItem:)]
    pub fn parent_of_child_item(&self, item: &I) -> Option<arc::R<I>>;

    /// A snapshot of `parent`'s descendants, without `parent` itself.
    #[objc::msg_send(snapshotOfParentItem:)]
    pub fn snapshot_of_parent_item(&self, parent: &I) -> arc::R<Self>;

    #[objc::msg_send(snapshotOfParentItem:includingParentItem:)]
    pub fn snapshot_of_parent_item_including_parent(
        &self,
        parent: &I,
        including_parent: bool,
    ) -> arc::R<Self>;

    #[objc::msg_send(appendItems:)]
    pub unsafe fn append_items_throws(&mut self, items: &ns::Array<I>);

    /// Appends `items` at the root level.
    #[inline]
    pub fn append_items<'ear>(&mut self, items: &ns::Array<I>) -> ns::ExResult<'ear> {
        ns::try_catch(|| unsafe { self.append_items_throws(items) })
    }

    #[objc::msg_send(appendItems:intoParentItem:)]
    pub unsafe fn append_items_into_throws(&mut self, items: &ns::Array<I>, parent: Option<&I>);

    /// Appends `items` as the last children of `parent`.
    #[inline]
    pub fn append_items_into<'ear>(
        &mut self,
        items: &ns::Array<I>,
        parent: impl AsRef<I>,
    ) -> ns::ExResult<'ear> {
        ns::try_catch(|| unsafe { self.append_items_into_throws(items, Some(parent.as_ref())) })
    }

    #[objc::msg_send(insertItems:beforeItem:)]
    pub unsafe fn insert_items_before_item_throws(&mut self, items: &ns::Array<I>, item: &I);

    #[inline]
    pub fn insert_items_before_item<'ear>(
        &mut self,
        items: &ns::Array<I>,
        item: impl AsRef<I>,
    ) -> ns::ExResult<'ear> {
        ns::try_catch(|| unsafe { self.insert_items_before_item_throws(items, item.as_ref()) })
    }

    #[objc::msg_send(insertItems:afterItem:)]
    pub unsafe fn insert_items_after_item_throws(&mut self, items: &ns::Array<I>, item: &I);

    #[inline]
    pub fn insert_items_after_item<'ear>(
        &mut self,
        items: &ns::Array<I>,
        item: impl AsRef<I>,
    ) -> ns::ExResult<'ear> {
        ns::try_catch(|| unsafe { self.insert_items_after_item_throws(items, item.as_ref()) })
    }

    #[objc::msg_send(insertSnapshot:beforeItem:)]
    pub unsafe fn insert_snapshot_before_item_throws(&mut self, snapshot: &Self, item: &I);

    #[inline]
    pub fn insert_snapshot_before_item<'ear>(
        &mut self,
        snapshot: &Self,
        item: impl AsRef<I>,
    ) -> ns::ExResult<'ear> {
        ns::try_catch(|| unsafe {
            self.insert_snapshot_before_item_throws(snapshot, item.as_ref())
        })
    }

    #[objc::msg_send(insertSnapshot:afterItem:)]
    pub unsafe fn insert_snapshot_after_item_throws(&mut self, snapshot: &Self, item: &I);

    #[inline]
    pub fn insert_snapshot_after_item<'ear>(
        &mut self,
        snapshot: &Self,
        item: impl AsRef<I>,
    ) -> ns::ExResult<'ear> {
        ns::try_catch(|| unsafe { self.insert_snapshot_after_item_throws(snapshot, item.as_ref()) })
    }

    #[objc::msg_send(replaceChildrenOfParentItem:withSnapshot:)]
    pub unsafe fn replace_children_of_parent_item_throws(&mut self, parent: &I, snapshot: &Self);

    #[inline]
    pub fn replace_children_of_parent_item<'ear>(
        &mut self,
        parent: impl AsRef<I>,
        snapshot: &Self,
    ) -> ns::ExResult<'ear> {
        ns::try_catch(|| unsafe {
            self.replace_children_of_parent_item_throws(parent.as_ref(), snapshot)
        })
    }

    /// Deletes `items` and their children.
    #[objc::msg_send(deleteItems:)]
    pub unsafe fn delete_items_throws(&mut self, items: &ns::Array<I>);

    #[inline]
    pub fn delete_items<'ear>(&mut self, items: &ns::Array<I>) -> ns::ExResult<'ear> {
        ns::try_catch(|| unsafe { self.delete_items_throws(items) })
    }

    #[objc::msg_send(deleteAllItems)]
    pub fn delete_all_items(&mut self);

    /// Logs if an item is not found.
    #[objc::msg_send(expandItems:)]
    pub unsafe fn expand_items_throws(&mut self, items: &ns::Array<I>);

    #[inline]
    pub fn expand_items<'ear>(&mut self, items: &ns::Array<I>) -> ns::ExResult<'ear> {
        ns::try_catch(|| unsafe { self.expand_items_throws(items) })
    }

    /// Logs if an item is not found.
    #[objc::msg_send(collapseItems:)]
    pub unsafe fn collapse_items_throws(&mut self, items: &ns::Array<I>);

    #[inline]
    pub fn collapse_items<'ear>(&mut self, items: &ns::Array<I>) -> ns::ExResult<'ear> {
        ns::try_catch(|| unsafe { self.collapse_items_throws(items) })
    }

    #[objc::msg_send(isExpanded:)]
    pub fn is_expanded(&self, item: &I) -> bool;

    #[objc::msg_send(isVisible:)]
    pub fn is_visible(&self, item: &I) -> bool;

    #[objc::msg_send(visualDescription)]
    pub fn visual_description(&self) -> arc::R<ns::String>;
}

impl<I: objc::Obj + 'static> DiffableDataSrcSectionSnapshot<I> {
    #[inline]
    pub fn items_n(&self) -> usize {
        self.items().len()
    }

    /// The direct children of `parent`.
    #[inline]
    pub fn children_of_item(&self, parent: &I) -> arc::R<ns::Array<I>> {
        self.snapshot_of_parent_item(parent).root_items()
    }
}

impl<I: objc::Obj> Clone for DiffableDataSrcSectionSnapshot<I> {
    fn clone(&self) -> Self {
        unsafe { std::mem::transmute(self.copy_with_zone(std::ptr::null_mut())) }
    }
}

impl<I: objc::Obj> ns::Copying for DiffableDataSrcSectionSnapshot<I> {}

unsafe extern "C" {
    static NS_DIFFABLE_DATA_SOURCE_SECTION_SNAPSHOT:
        &'static objc::Class<DiffableDataSrcSectionSnapshot<ns::Id>>;
}
