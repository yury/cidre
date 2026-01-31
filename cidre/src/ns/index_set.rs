use crate::{arc, define_obj_type, ns, objc};

impl arc::A<IndexSet> {
    #[objc::msg_send(initWithIndex:)]
    pub fn init_with_index(self, val: usize) -> arc::R<IndexSet>;

    #[objc::msg_send(initWithIndexSet:)]
    pub fn init_with_index_set(self, val: &ns::IndexSet) -> arc::R<IndexSet>;

    #[objc::msg_send(initWithIndexesInRange:)]
    pub fn init_with_range(self, val: ns::Range) -> arc::R<IndexSet>;
}

impl arc::A<IndexSetMut> {
    #[objc::msg_send(initWithIndex:)]
    pub fn init_with_index(self, val: usize) -> arc::R<IndexSetMut>;

    #[objc::msg_send(initWithIndexSet:)]
    pub fn init_with_index_set(self, val: &ns::IndexSet) -> arc::R<IndexSetMut>;

    #[objc::msg_send(initWithIndexesInRange:)]
    pub fn init_with_range(self, val: ns::Range) -> arc::R<IndexSetMut>;
}

define_obj_type!(
    #[doc(alias = "NSIndexSet")]
    pub IndexSet(ns::Id),
    NS_INDEX_SET
);

define_obj_type!(
    #[doc(alias = "NSMutableIndexSet")]
    pub IndexSetMut(ns::IndexSet),
    NS_MUTABLE_INDEX_SET
);

impl IndexSet {
    #[objc::msg_send(count)]
    pub fn len(&self) -> usize;

    #[inline]
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    #[inline]
    pub fn with_index(val: usize) -> arc::R<Self> {
        Self::alloc().init_with_index(val)
    }

    #[inline]
    pub fn with_index_set(val: &ns::IndexSet) -> arc::R<Self> {
        Self::alloc().init_with_index_set(val)
    }

    #[inline]
    pub fn with_range(val: ns::Range) -> arc::R<Self> {
        Self::alloc().init_with_range(val)
    }

    #[objc::msg_send(containsIndex:)]
    pub fn contains_index(&self, val: usize) -> bool;

    #[objc::msg_send(containsIndexesInRange:)]
    pub fn contains_index_in(&self, val: ns::Range) -> bool;

    #[objc::msg_send(containsIndexes:)]
    pub fn contains_indexes(&self, val: &ns::IndexSet) -> bool;

    #[objc::msg_send(mutableCopy)]
    pub fn copy_mut(&self) -> arc::Retained<IndexSetMut>;

    #[objc::msg_send(copy)]
    pub fn copy(&self) -> arc::Retained<IndexSet>;
}

impl IndexSetMut {
    #[inline]
    pub fn with_index(val: usize) -> arc::R<Self> {
        Self::alloc().init_with_index(val)
    }

    #[inline]
    pub fn with_index_set(val: &ns::IndexSet) -> arc::R<Self> {
        Self::alloc().init_with_index_set(val)
    }

    #[inline]
    pub fn with_range(val: ns::Range) -> arc::R<Self> {
        Self::alloc().init_with_range(val)
    }

    #[objc::msg_send(addIndexes:)]
    pub fn add_indexes(&mut self, val: &ns::IndexSet);

    #[objc::msg_send(removeIndexes:)]
    pub fn remove_indexes(&mut self, val: &ns::IndexSet);

    #[objc::msg_send(removeAllIndexes)]
    pub fn clear(&mut self);

    #[objc::msg_send(addIndex:)]
    pub fn add(&mut self, val: usize);

    #[objc::msg_send(removeIndex:)]
    pub fn remove(&mut self, val: usize);

    #[objc::msg_send(addIndexesInRange:)]
    pub fn add_range(&mut self, val: ns::Range);

    #[objc::msg_send(removeIndexesInRange:)]
    pub fn remove_range(&mut self, val: ns::Range);
}

impl Default for arc::R<IndexSetMut> {
    fn default() -> Self {
        IndexSetMut::with_range(Default::default())
    }
}

#[link(name = "ns", kind = "static")]
unsafe extern "C" {
    static NS_INDEX_SET: &'static objc::Class<IndexSet>;
    static NS_MUTABLE_INDEX_SET: &'static objc::Class<IndexSetMut>;
}

#[cfg(test)]
mod tests {
    use crate::ns;

    #[test]
    fn basics() {
        let set_a = ns::IndexSet::with_index(10);
        assert_eq!(set_a.len(), 1);

        assert!(set_a.contains_index(10));
        assert!(!set_a.contains_index(0));

        let set_b = ns::IndexSet::with_index_set(&set_a);
        assert_eq!(set_b.len(), 1);

        assert!(set_a.contains_indexes(&set_b));

        let range_set = ns::IndexSet::with_range(ns::Range::new(0, 100));
        assert_eq!(range_set.len(), 100);

        let mut copy = range_set.copy_mut();
        assert!(copy.contains_index(10));
        copy.remove(10);
        assert!(!copy.contains_index(10));

        copy.remove_range(ns::Range::new(0, 100));
        assert!(copy.is_empty());

        let _copy = copy.copy();
    }
}
