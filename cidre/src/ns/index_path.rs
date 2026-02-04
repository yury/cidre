use crate::{arc, define_obj_type, ns, objc};

define_obj_type!(
    #[doc(alias = "NSIndexPath")]
    pub IndexPath(ns::Id),
    NS_INDEX_PATH
);

impl ns::Copying for IndexPath {}
unsafe impl Send for IndexPath {}
unsafe impl Sync for IndexPath {}

impl arc::A<IndexPath> {
    #[objc::msg_send(initWithIndex:)]
    pub fn init_with_index(self, index: usize) -> arc::R<IndexPath>;

    #[objc::msg_send(initWithIndexes:length:)]
    pub fn init_with_indexes(self, indexes: *const usize, lenght: usize) -> arc::R<IndexPath>;
}

impl IndexPath {
    pub fn with_index(index: usize) -> arc::R<Self> {
        Self::alloc().init_with_index(index)
    }
    pub fn with_indexes(indexes: &[usize]) -> arc::R<Self> {
        Self::alloc().init_with_indexes(indexes.as_ptr(), indexes.len())
    }

    #[objc::msg_send(length)]
    pub fn len(&self) -> usize;

    #[objc::msg_send(indexPathByAddingIndex:)]
    pub fn push(&self, index: usize) -> arc::R<Self>;

    #[objc::msg_send(indexPathByRemovingLastIndex)]
    pub fn pop(&self) -> arc::R<Self>;

    pub fn copy(&self) -> arc::R<Self> {
        unsafe { std::mem::transmute(ns::Copying::copy_with_zone(self, std::ptr::null_mut())) }
    }

    /// Provides the value at a particular node in the index path.
    #[objc::msg_send(indexAtPosition:)]
    pub fn index_at(&self, pos: usize) -> usize;

    #[objc::msg_send(isEqual:)]
    pub fn is_equal(&self, other: &Self) -> bool;

    #[objc::msg_send(hash)]
    pub fn hash(&self) -> ns::UInteger;
}

impl From<&[usize]> for arc::R<IndexPath> {
    fn from(value: &[usize]) -> Self {
        IndexPath::with_indexes(&value)
    }
}

impl From<[usize; 2]> for arc::R<IndexPath> {
    fn from(value: [usize; 2]) -> Self {
        IndexPath::with_indexes(&value)
    }
}

impl std::borrow::ToOwned for ns::IndexPath {
    type Owned = arc::R<IndexPath>;

    fn to_owned(&self) -> Self::Owned {
        self.retained()
    }
}

impl<'a> From<&'a ns::IndexPath> for std::borrow::Cow<'a, ns::IndexPath> {
    fn from(value: &'a ns::IndexPath) -> Self {
        std::borrow::Cow::Borrowed(value)
    }
}

impl Eq for IndexPath {}
impl std::hash::Hash for IndexPath {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.hash().hash(state);
    }
}

impl Eq for arc::R<IndexPath> {}
impl std::hash::Hash for arc::R<IndexPath> {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.0.hash().hash(state);
    }
}

unsafe extern "C" {
    static NS_INDEX_PATH: &'static objc::Class<IndexPath>;
}

#[cfg(test)]
mod tests {
    use crate::{arc, ns};

    #[test]
    fn basics() {
        let ip = ns::IndexPath::new();
        assert_eq!(ip.len(), 0);
        let ip = ip.push(10);
        assert_eq!(ip.len(), 1);

        let ip = ip.copy();
        assert_eq!(ip.len(), 1);

        let ip = ns::IndexPath::with_indexes(&[1, 2]);
        assert_eq!(ip.len(), 2);
        let ip = ip.pop();
        assert_eq!(ip.len(), 1);

        let ip: arc::R<ns::IndexPath> = [1usize, 2, 3][..].into();
        assert_eq!(ip.len(), 3);

        let ip: arc::R<ns::IndexPath> = [0, 1].into();
        assert_eq!(ip.len(), 2);
    }
}
