#[cfg(feature = "blocks")]
use crate::blocks;
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

    #[objc::msg_send(firstIndex)]
    pub fn first_index(&self) -> usize;

    #[objc::msg_send(lastIndex)]
    pub fn last_index(&self) -> usize;

    #[objc::msg_send(indexGreaterThanIndex:)]
    pub fn index_greater_than(&self, val: usize) -> usize;

    #[objc::msg_send(indexLessThanIndex:)]
    pub fn index_less_than(&self, val: usize) -> usize;

    #[objc::msg_send(indexGreaterThanOrEqualToIndex:)]
    pub fn index_greater_than_or_equal(&self, val: usize) -> usize;

    #[objc::msg_send(indexLessThanOrEqualToIndex:)]
    pub fn index_less_than_or_equal(&self, val: usize) -> usize;

    #[objc::msg_send(containsIndex:)]
    pub fn contains_index(&self, val: usize) -> bool;

    #[objc::msg_send(containsIndexesInRange:)]
    pub fn contains_index_in(&self, val: ns::Range) -> bool;

    #[objc::msg_send(containsIndexes:)]
    pub fn contains_indexes(&self, val: &ns::IndexSet) -> bool;

    #[objc::msg_send(intersectsIndexesInRange:)]
    pub fn intersects_range(&self, val: ns::Range) -> bool;

    #[objc::msg_send(countOfIndexesInRange:)]
    pub fn count_in_range(&self, val: ns::Range) -> usize;

    #[objc::msg_send(isEqualToIndexSet:)]
    pub fn is_equal_to_index_set(&self, val: &ns::IndexSet) -> bool;

    #[objc::msg_send(mutableCopy)]
    pub fn copy_mut(&self) -> arc::Retained<IndexSetMut>;

    #[objc::msg_send(copy)]
    pub fn copy(&self) -> arc::Retained<IndexSet>;
}

/// NSExtendedIndexSet
#[cfg(feature = "blocks")]
impl IndexSet {
    #[objc::msg_send(enumerateIndexesUsingBlock:)]
    pub fn enum_indexes_block(&self, block: &mut blocks::NoEscBlock<fn(usize, &mut bool)>);

    #[objc::msg_send(enumerateRangesUsingBlock:)]
    pub fn enum_ranges_block(&self, block: &mut blocks::NoEscBlock<fn(ns::Range, &mut bool)>);

    #[objc::msg_send(enumerateIndexesWithOptions:usingBlock:)]
    pub fn enum_indexes_with_opts_block(
        &self,
        opts: ns::EnumerationOpts,
        block: &mut blocks::NoEscBlock<fn(usize, &mut bool)>,
    );

    #[objc::msg_send(indexPassingTest:)]
    pub fn index_passing_test_block(
        &self,
        predicate: &mut blocks::NoEscBlock<fn(usize, &mut bool) -> bool>,
    ) -> usize;

    #[inline]
    pub fn enum_indexes(&self, mut block: impl FnMut(usize, &mut bool)) {
        unsafe {
            let mut block = blocks::NoEscBlock::stack2(&mut block);
            self.enum_indexes_block(&mut block)
        }
    }

    #[inline]
    pub fn enum_ranges(&self, mut block: impl FnMut(ns::Range, &mut bool)) {
        unsafe {
            let mut block = blocks::NoEscBlock::stack2(&mut block);
            self.enum_ranges_block(&mut block)
        }
    }

    #[inline]
    pub fn enum_indexes_with_opts(
        &self,
        opts: ns::EnumerationOpts,
        mut block: impl FnMut(usize, &mut bool),
    ) {
        unsafe {
            let mut block = blocks::NoEscBlock::stack2(&mut block);
            self.enum_indexes_with_opts_block(opts, &mut block)
        }
    }

    #[inline]
    pub fn index_passing_test(&self, mut predicate: impl FnMut(usize, &mut bool) -> bool) -> usize {
        unsafe {
            let mut predicate = blocks::NoEscBlock::stack2(&mut predicate);
            self.index_passing_test_block(&mut predicate)
        }
    }
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

    #[objc::msg_send(shiftIndexesStartingAtIndex:by:)]
    pub fn shift_starting_at(&mut self, index: usize, delta: ns::Integer);
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
        assert_eq!(range_set.first_index(), 0);
        assert_eq!(range_set.last_index(), 99);
        assert_eq!(range_set.index_greater_than(0), 1);
        assert_eq!(range_set.index_less_than(50), 49);
        assert_eq!(range_set.index_greater_than_or_equal(0), 0);
        assert_eq!(range_set.index_less_than_or_equal(50), 50);
        assert!(range_set.intersects_range(ns::Range::new(50, 10)));
        assert_eq!(range_set.count_in_range(ns::Range::new(10, 10)), 10);
        assert!(range_set.is_equal_to_index_set(&ns::IndexSet::with_range(ns::Range::new(0, 100))));

        let mut shifted = range_set.copy_mut();
        shifted.shift_starting_at(50, 2);
        assert!(!shifted.contains_index(50));
        assert!(shifted.contains_index(52));

        let mut copy = range_set.copy_mut();
        assert!(copy.contains_index(10));
        copy.remove(10);
        assert!(!copy.contains_index(10));

        copy.remove_range(ns::Range::new(0, 100));
        assert!(copy.is_empty());

        let _copy = copy.copy();
    }

    #[cfg(feature = "blocks")]
    #[test]
    fn enum_indexes() {
        let range_set = ns::IndexSet::with_range(ns::Range::new(0, 5));
        let mut seen = Vec::new();
        range_set.enum_indexes(|idx, stop| {
            seen.push(idx);
            if idx == 2 {
                *stop = true;
            }
        });
        assert_eq!(seen, vec![0, 1, 2]);
    }

    #[cfg(feature = "blocks")]
    #[test]
    fn enum_ranges() {
        let mut set = ns::IndexSetMut::with_range(ns::Range::new(0, 3));
        set.add_range(ns::Range::new(5, 2));
        let mut ranges = Vec::new();
        set.enum_ranges(|range, _| ranges.push(range));
        assert_eq!(&ranges, &[ns::Range::new(0, 3), ns::Range::new(5, 2)]);
    }
}
