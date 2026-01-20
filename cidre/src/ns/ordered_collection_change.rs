use std::{marker::PhantomData, ops::Deref};

use crate::{arc, define_cls, ns, objc};

#[doc(alias = "NSOrderedCollectionChange")]
#[derive(Debug)]
#[repr(transparent)]
pub struct OrderedCollectionChange<T: objc::Obj>(ns::Id, PhantomData<T>);

unsafe impl<T> Send for OrderedCollectionChange<T> where T: objc::Obj + Send {}

impl<T: objc::Obj> arc::A<OrderedCollectionChange<T>> {
    #[objc::msg_send(initWithObject:type:index:)]
    pub fn init_with_object_type_index(
        self,
        object: Option<&T>,
        type_: CollectionChangeType,
        index: ns::UInteger,
    ) -> arc::R<OrderedCollectionChange<T>>;

    #[objc::msg_send(initWithObject:type:index:associatedIndex:)]
    pub fn init_with_object_type_index_associated(
        self,
        object: Option<&T>,
        type_: CollectionChangeType,
        index: ns::UInteger,
        associated_index: ns::UInteger,
    ) -> arc::R<OrderedCollectionChange<T>>;
}

impl<T: objc::Obj> objc::Obj for OrderedCollectionChange<T> {}

impl<T: objc::Obj> Deref for OrderedCollectionChange<T> {
    type Target = ns::Id;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T: objc::Obj> OrderedCollectionChange<T> {
    define_cls!(NS_ORDERED_COLLECTION_CHANGE);

    #[inline]
    pub fn new(
        object: Option<&T>,
        type_: CollectionChangeType,
        index: ns::UInteger,
    ) -> arc::R<Self> {
        Self::alloc().init_with_object_type_index(object, type_, index)
    }

    #[inline]
    pub fn with_associated_index(
        object: Option<&T>,
        type_: CollectionChangeType,
        index: ns::UInteger,
        associated_index: ns::UInteger,
    ) -> arc::R<Self> {
        Self::alloc().init_with_object_type_index_associated(object, type_, index, associated_index)
    }

    #[inline]
    pub fn with_obj(
        object: Option<&T>,
        type_: CollectionChangeType,
        index: ns::UInteger,
    ) -> arc::R<Self> {
        Self::alloc().init_with_object_type_index(object, type_, index)
    }

    #[inline]
    pub fn with_obj_and_index(
        object: Option<&T>,
        type_: CollectionChangeType,
        index: ns::UInteger,
        associated_index: ns::UInteger,
    ) -> arc::R<Self> {
        Self::alloc().init_with_object_type_index_associated(object, type_, index, associated_index)
    }

    #[objc::msg_send(object)]
    pub fn obj(&self) -> Option<arc::R<T>>;

    #[objc::msg_send(changeType)]
    pub fn change_type(&self) -> CollectionChangeType;

    #[objc::msg_send(index)]
    pub fn index(&self) -> ns::UInteger;

    #[objc::msg_send(associatedIndex)]
    pub fn associated_index(&self) -> ns::UInteger;
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[doc(alias = "NSCollectionChangeType")]
#[repr(i64)]
pub enum CollectionChangeType {
    Insert = 0,
    Remove = 1,
}

#[link(name = "ns", kind = "static")]
unsafe extern "C" {
    static NS_ORDERED_COLLECTION_CHANGE: &'static objc::Class<OrderedCollectionChange<ns::Id>>;
}

#[cfg(test)]
mod tests {
    use crate::ns;

    #[test]
    fn basics() {
        let change = ns::OrderedCollectionChange::<ns::String>::with_obj(
            Some(ns::str!(c"foo")),
            ns::CollectionChangeType::Insert,
            0,
        );
        assert_eq!(change.index(), 0);
        assert_eq!(change.change_type(), ns::CollectionChangeType::Insert);
        assert!(change.obj().is_some());

        let change = ns::OrderedCollectionChange::<ns::String>::with_obj_and_index(
            Some(ns::str!(c"bar")),
            ns::CollectionChangeType::Remove,
            5,
            ns::NOT_FOUND as ns::UInteger,
        );
        assert_eq!(change.index(), 5);
        assert_eq!(change.change_type(), ns::CollectionChangeType::Remove);
    }
}
