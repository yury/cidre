use std::ffi::c_ulong;

use crate::objc::{self, Obj};

static MUTATIONS_TARGET: c_ulong = 0;
static MUTATIONS_PTR: &c_ulong = &MUTATIONS_TARGET;

#[derive(Debug, Copy, Clone)]
#[repr(C)]
pub struct FastEnumerationState<T> {
    pub state: c_ulong,
    pub items_ptr: *const *const T,
    pub mutations_ptr: &'static c_ulong,
    pub extra: [c_ulong; 5],
}

impl<T> FastEnumerationState<T> {
    pub fn new() -> Self {
        Self {
            state: 0,
            items_ptr: std::ptr::null(),
            mutations_ptr: MUTATIONS_PTR,
            extra: Default::default(),
        }
    }
}

impl<T> Default for FastEnumerationState<T> {
    fn default() -> Self {
        Self::new()
    }
}

pub trait FastEnumeration<T: Obj>: Obj {
    #[objc::msg_send(countByEnumeratingWithState:objects:count:)]
    fn count_by_enumerating(
        &self,
        state: &mut FastEnumerationState<T>,
        objects: *mut *const T,
        count: usize,
    ) -> usize;

    #[inline]
    fn iter(&self) -> FEIterator<Self, T> {
        FEIterator::new(self)
    }
}

// https://github.com/apple/swift-corelibs-foundation/blob/main/Darwin/Foundation-swiftoverlay/NSFastEnumeration.swift
// https://www.mikeash.com/pyblog/friday-qa-2010-04-16-implementing-fast-enumeration.html
#[derive(Debug)]
pub struct FEIterator<'a, E, T, const N: usize = 16>
where
    E: Obj + FastEnumeration<T>,
    T: Obj + 'a,
{
    pub obj: &'a E,
    objs: [*const T; N],
    state: FastEnumerationState<T>,
    index: usize,
    len: usize,
}

impl<'a, E, T, const N: usize> FEIterator<'a, E, T, N>
where
    E: Obj + FastEnumeration<T>,
    T: Obj + 'a,
{
    #[inline]
    pub fn new(obj: &'a E) -> Self {
        Self {
            obj,
            state: Default::default(),
            // on stack buffer to fill
            objs: [std::ptr::null(); N],
            index: 0,
            len: 0,
        }
    }
}

impl<'a, E, T, const N: usize> Iterator for FEIterator<'a, E, T, N>
where
    E: Obj + FastEnumeration<T>,
    T: Obj + 'a,
{
    type Item = &'a T;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        // We need first/next batch
        if self.index == self.len {
            if self.len > 0 && self.len < N {
                return None;
            }

            self.len = self
                .obj
                .count_by_enumerating(&mut self.state, self.objs.as_mut_ptr(), N);

            if self.len == 0 {
                return None;
            }

            self.index = 0;
        }

        let item = unsafe { *self.state.items_ptr.add(self.index) };
        self.index += 1;
        Some(unsafe { &*item })
    }
}

#[cfg(test)]
mod tests {
    use crate::{ns, objc::Obj};

    #[test]
    fn basics1() {
        let one = ns::Number::with_i32(1);

        let nums: &[&ns::Number] = &[&one, &one];
        let arr = ns::Array::from_slice(nums);
        let sum = arr.iter().map(|v| v.as_i32()).sum();

        assert_eq!(2, sum);
    }

    #[test]
    fn basics2() {
        let one = ns::str!(c"1");

        let arr: &[&ns::String] = &[one];
        let arr = ns::Array::from_slice(arr);
        assert_eq!(1, arr.len());

        let mut k = 0;
        for _ in arr.iter() {
            k += 1;
            //println!("{:?}", i);
        }

        assert_eq!(1, k);
    }

    #[test]
    fn basics3() {
        let one = ns::str!(c"1");

        let arr: &[&ns::String] = &[one, one];
        let arr = ns::Array::from_slice(arr);
        assert_eq!(2, arr.len());

        let mut k = 0;
        for i in arr.iter() {
            k += 1;
            println!("{:?}", i);
        }

        assert_eq!(2, k);
    }

    #[test]
    fn basics4() {
        const N: usize = 100;
        let mut vec: Vec<&ns::Number> = Vec::with_capacity(N);
        let two = ns::Number::with_i64(2);
        for _ in 0..N {
            vec.push(&two);
        }
        let arr = ns::Array::from_slice(&vec[..]);
        let sum = arr.iter().map(|v| v.as_i32()).sum();
        assert_eq!(200, sum);
    }

    #[test]
    fn basics5() {
        let two = ns::Number::with_i32(10);
        let set: &[&ns::Number] = &[&two, &two, &two];
        let set = ns::Set::from_slice(set);
        println!("{:?}", set.desc());
        //        assert_eq!(1, set.len());
        let sum = set.iter().map(|v| v.as_i32()).sum();
        assert_eq!(10, sum);
    }
}
