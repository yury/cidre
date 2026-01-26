use std::{ffi::c_void, marker::PhantomData, ops::Deref};

use crate::{
    arc, define_cls,
    ns::{self, Copying, CopyingMut},
    objc::{self, Class, Obj},
};

#[cfg(feature = "cf")]
use crate::cf;

#[derive(Debug)]
#[repr(transparent)]
pub struct Set<T: Obj>(ns::Id, PhantomData<T>);

impl<T: Obj> Obj for Set<T> {}

impl<T: Obj> ns::Copying for Set<T> {}
impl<T: Obj> ns::CopyingMut for Set<T> {}

#[derive(Debug)]
#[repr(transparent)]
pub struct SetMut<T: Obj>(ns::Set<T>);

impl<T: Obj> Obj for SetMut<T> {}

impl<T: Obj> Deref for Set<T> {
    type Target = ns::Id;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T: Obj> Deref for SetMut<T> {
    type Target = Set<T>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T: Obj> AsRef<Set<T>> for SetMut<T> {
    fn as_ref(&self) -> &Set<T> {
        self
    }
}

impl<T: Obj> arc::A<Set<T>> {
    #[objc::msg_send(init)]
    pub fn init(self) -> arc::R<Set<T>>;

    #[objc::msg_send(initWithObjects:count:)]
    pub fn init_with_objs_count(self, ptr: *const c_void, count: usize) -> arc::R<Set<T>>;
}

impl<T: Obj> Set<T> {
    define_cls!(NS_SET);

    #[inline]
    pub fn new() -> arc::R<Self> {
        Self::alloc().init()
    }

    #[inline]
    pub fn from_slice(objs: &[&T]) -> arc::R<Self> {
        Self::alloc().init_with_objs_count(objs.as_ptr() as _, objs.len())
    }

    #[objc::msg_send(count)]
    pub fn len(&self) -> usize;

    #[inline]
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    #[inline]
    pub fn iter(&self) -> ns::FeIterator<'_, Self, T> {
        ns::FastEnum::iter(self)
    }

    #[objc::msg_send(member:)]
    pub fn member_obj(&self, obj: &T) -> Option<arc::R<T>>;

    #[inline]
    pub fn member(&self, obj: impl AsRef<T>) -> Option<arc::R<T>> {
        self.member_obj(obj.as_ref())
    }

    pub fn copy(&self) -> arc::R<Self> {
        unsafe { std::mem::transmute(self.copy_with_zone(std::ptr::null_mut())) }
    }

    pub fn copy_mut(&self) -> arc::R<SetMut<T>> {
        unsafe { std::mem::transmute(self.copy_with_zone_mut(std::ptr::null_mut())) }
    }

    #[cfg(feature = "cf")]
    #[inline]
    pub fn as_cf(&self) -> &cf::Set {
        unsafe { std::mem::transmute(self) }
    }
}

/// NSExtendedSet
impl<T: Obj> Set<T> {
    #[objc::msg_send(allObjects)]
    pub fn all_objs(&self) -> arc::R<ns::Array<T>>;

    #[objc::msg_send(anyObject)]
    pub fn any_obj(&self) -> Option<arc::R<T>>;

    #[objc::msg_send(containsObject:)]
    pub fn contains_obj(&self, obj: &T) -> bool;

    #[inline]
    pub fn contains(&self, obj: impl AsRef<T>) -> bool {
        self.contains_obj(obj.as_ref())
    }

    #[objc::msg_send(intersectsSet:)]
    pub fn intersects_set(&self, val: &ns::Set<T>) -> bool;

    #[inline]
    pub fn intersects(&self, val: impl AsRef<ns::Set<T>>) -> bool {
        self.intersects_set(val.as_ref())
    }

    #[objc::msg_send(isEqualToSet:)]
    pub fn is_equal_to_set(&self, val: &ns::Set<T>) -> bool;

    #[inline]
    pub fn is_equal_to(&self, val: impl AsRef<ns::Set<T>>) -> bool {
        self.is_equal_to_set(val.as_ref())
    }

    #[objc::msg_send(isSubsetOfSet:)]
    pub fn is_subset_of_set(&self, val: &ns::Set<T>) -> bool;

    #[inline]
    pub fn is_subset_of(&self, val: impl AsRef<ns::Set<T>>) -> bool {
        self.is_subset_of_set(val.as_ref())
    }
}

impl<T: Obj> arc::A<SetMut<T>> {
    #[objc::msg_send(init)]
    pub fn init(self) -> arc::R<SetMut<T>>;

    #[objc::msg_send(initWithCapacity:)]
    pub fn init_with_capacity(self, capacity: usize) -> arc::R<SetMut<T>>;
}

impl<T: Obj> SetMut<T> {
    define_cls!(NS_MUTABLE_SET);

    #[inline]
    pub fn new() -> arc::R<Self> {
        Self::alloc().init()
    }

    #[inline]
    pub fn with_capacity(capacity: usize) -> arc::R<Self> {
        Self::alloc().init_with_capacity(capacity)
    }

    #[objc::msg_send(addObject:)]
    pub fn add_obj(&mut self, val: &T);

    #[inline]
    pub fn add(&mut self, val: impl AsRef<T>) {
        self.add_obj(val.as_ref());
    }

    #[objc::msg_send(removeObject:)]
    pub fn remove_obj(&mut self, val: &T);

    #[inline]
    pub fn remove(&mut self, val: impl AsRef<T>) {
        self.remove_obj(val.as_ref());
    }

    #[cfg(feature = "cf")]
    #[inline]
    pub fn as_cf(&self) -> &cf::SetMut {
        unsafe { std::mem::transmute(self) }
    }
}

/// NSExtendedMutableSet
impl<T: Obj> SetMut<T> {
    #[objc::msg_send(addObjectsFromArray:)]
    pub fn add_objs_from_array(&mut self, val: &ns::Array<T>);

    /// Removes from the receiving set each object that isn’t a member of another given set.
    #[objc::msg_send(intersectSet:)]
    pub fn intersect_set(&mut self, val: &ns::Set<T>);

    #[inline]
    pub fn intersect(&mut self, val: impl AsRef<ns::Set<T>>) {
        self.intersect_set(val.as_ref());
    }

    /// Removes each object in another given set from the receiving set, if present.
    #[objc::msg_send(minusSet:)]
    pub fn minus_set(&mut self, val: &ns::Set<T>);

    #[inline]
    pub fn minus(&mut self, val: impl AsRef<ns::Set<T>>) {
        self.minus_set(val.as_ref());
    }

    #[objc::msg_send(removeAllObjects)]
    pub fn clear(&mut self);

    /// Adds each object in another given set to the receiving set, if not present.
    #[objc::msg_send(unionSet:)]
    pub fn union_set(&mut self, val: &ns::Set<T>);

    #[inline]
    pub fn union(&mut self, val: impl AsRef<ns::Set<T>>) {
        self.union_set(val.as_ref());
    }

    /// Empties the receiving set, then adds each object contained in another given set.
    #[objc::msg_send(setSet:)]
    pub fn set_set(&mut self, val: &ns::Set<T>);

    #[inline]
    pub fn set(&mut self, val: impl AsRef<ns::Set<T>>) {
        self.set_set(val.as_ref());
    }
}

impl<T> ns::FastEnum<T> for Set<T> where T: Obj {}
impl<T> ns::FastEnum<T> for SetMut<T> where T: Obj {}

#[link(name = "ns", kind = "static")]
unsafe extern "C" {
    static NS_SET: &'static Class<ns::Set<ns::Id>>;
    static NS_MUTABLE_SET: &'static Class<ns::SetMut<ns::Id>>;
}

#[cfg(test)]
mod tests {
    use crate::ns;

    #[test]
    fn basics() {
        let two = ns::Number::with_i32(10);
        let set: &[&ns::Number] = &[&two, &two, &two];
        let set = ns::Set::from_slice(set);
        assert_eq!(1, set.len());
        let sum = set.iter().map(|v| v.as_i32()).sum();
        assert_eq!(10, sum);

        let copy_mut = set.copy_mut();
        copy_mut.is_subset_of(&set);
    }

    #[test]
    fn basics_mut() {
        let mut s = ns::SetMut::<ns::Number>::with_capacity(10);
        s.add(0u8);
        s.add(0u8);
        s.add(10u8);
        assert_eq!(2, s.len());

        assert!(s.member(0u8).is_some());
        assert!(s.member(2u8).is_none());
        assert!(s.member(10u8).is_some());

        s.add(ns::Number::with_i8(0));
        assert_eq!(2, s.len());

        assert!(s.is_subset_of(s.as_ref()));
    }
}
