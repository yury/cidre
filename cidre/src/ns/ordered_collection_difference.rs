use std::marker::PhantomData;

use crate::{arc, define_cls, ns, objc};

#[doc(alias = "NSOrderedCollectionDifference")]
#[repr(transparent)]
pub struct OrderedCollectionDiff<T: objc::Obj>(ns::Id, PhantomData<T>);

unsafe impl<T> Send for OrderedCollectionDiff<T> where T: objc::Obj + Send {}

impl<T: objc::Obj> objc::Obj for OrderedCollectionDiff<T> {}

impl<T: objc::Obj> arc::A<OrderedCollectionDiff<T>> {
    #[objc::msg_send(initWithChanges:)]
    pub fn init_with_changes(
        self,
        changes: &ns::Array<ns::OrderedCollectionChange<T>>,
    ) -> arc::R<OrderedCollectionDiff<T>>;
}

impl<T: objc::Obj> OrderedCollectionDiff<T> {
    define_cls!(NS_ORDERED_COLLECTION_DIFFERENCE);

    #[inline]
    pub fn with_changes(changes: &ns::Array<ns::OrderedCollectionChange<T>>) -> arc::R<Self> {
        Self::alloc().init_with_changes(changes)
    }

    #[objc::msg_send(insertions)]
    pub fn insertions(&self) -> arc::R<ns::Array<ns::OrderedCollectionChange<T>>>;

    #[objc::msg_send(removals)]
    pub fn removals(&self) -> arc::R<ns::Array<ns::OrderedCollectionChange<T>>>;

    #[objc::msg_send(hasChanges)]
    pub fn has_changes(&self) -> bool;

    #[objc::msg_send(inverseDifference)]
    pub fn inverse(&self) -> arc::R<Self>;
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[doc(alias = "NSOrderedCollectionDifferenceInverseDiffingOptions")]
#[repr(i64)]
pub enum OrderedCollectionDiffInverseOpts {
    InsertedObjsFirst = 0,
    RemovedObjsFirst = 1,
}

#[link(name = "ns", kind = "static")]
unsafe extern "C" {
    static NS_ORDERED_COLLECTION_DIFFERENCE: &'static objc::Class<OrderedCollectionDiff<ns::Id>>;
}

#[cfg(test)]
mod tests {
    use crate::ns;

    #[test]
    fn basics() {
        let insert_change = ns::OrderedCollectionChange::<ns::String>::with_obj(
            Some(ns::str!(c"c")),
            ns::CollectionChangeType::Insert,
            0,
        );
        let changes = ns::arr![insert_change];

        let diff = ns::OrderedCollectionDiff::<ns::String>::with_changes(&changes);
        assert!(diff.has_changes());
        assert!(diff.insertions().len() > 0);

        let inverse = diff.inverse();
        assert!(inverse.has_changes());
    }
}
