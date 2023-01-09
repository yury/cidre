use std::{ffi::c_void, mem::transmute, ptr::slice_from_raw_parts};

use crate::{msg_send, objc};

// static MUTATIONS_TARGET: u32 = 0;
// static MUTATIONS_PTR: &u32 = &MUTATIONS_TARGET;

#[derive(Debug, Copy, Clone)]
#[repr(C)]
pub struct FastEnumerationState {
    pub state: u32,
    pub items_ptr: *const *const c_void,
    pub mutations_ptr: *mut u32,
    pub extra: [u32; 5],
}

impl FastEnumerationState {
    pub fn new() -> Self {
        Self {
            state: 0,
            items_ptr: std::ptr::null(),
            mutations_ptr: std::ptr::null_mut(),
            extra: Default::default(),
        }
    }
}

impl Default for FastEnumerationState {
    fn default() -> Self {
        Self::new()
    }
}

pub trait FastEnumeration<T>: objc::Obj
where
    T: objc::Obj,
{
    #[inline]
    fn count_by_enumerating(
        &self,
        state: &mut FastEnumerationState,
        objects: &mut [*const T],
        count: usize,
    ) -> usize {
        msg_send!(
            "ns",
            self,
            ns_countByEnumeratingWithState_objects_count,
            state,
            objects,
            count
        )
    }

    #[inline]
    fn iter(&self) -> FEIterator<Self, T> {
        FEIterator::new(self)
    }
}

// https://github.com/apple/swift-corelibs-foundation/blob/main/Darwin/Foundation-swiftoverlay/NSFastEnumeration.swift
#[derive(Debug)]
pub struct FEIterator<'a, E, T, const N: usize = 16>
where
    E: objc::Obj + FastEnumeration<T>,
    T: objc::Obj + 'a,
{
    pub obj: &'a E,
    objects: [*const T; N],
    items: &'a [*const T],
    state: FastEnumerationState,
    index: usize,
    len: usize,
    total_index: usize,
}

impl<'a, E, T, const N: usize> FEIterator<'a, E, T, N>
where
    E: objc::Obj + FastEnumeration<T>,
    T: objc::Obj + 'a,
{
    pub fn new(obj: &'a E) -> Self {
        Self {
            obj,
            state: Default::default(),
            objects: [std::ptr::null(); N],
            items: &[],
            index: 0,
            len: 0,
            total_index: 0,
        }
    }
}

// impl<'a, E, T, const N: usize> ExactSizeIterator for FEIterator<'a, E, T, N>
// where
//     E: objc::Obj + FastEnumeration<T>,
//     T: objc::Obj + 'a,
// {
//     fn len(&self) -> usize {
//         self.total_len - self.total_index
//     }
// }

impl<'a, E, T, const N: usize> Iterator for FEIterator<'a, E, T, N>
where
    E: objc::Obj + FastEnumeration<T>,
    T: objc::Obj + 'a,
{
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index + 1 > self.len {
            if self.len > 0 && self.len < N {
                // we have processed last batch
                return None;
            }
            self.index = 0;

            self.len = self
                .obj
                .count_by_enumerating(&mut self.state, &mut self.objects, N);

            if self.len == 0 {
                return None;
            }

            // if self.state.items_ptr == self.objects.as_ptr() as _ {
            //     // this is the common case for things like NSArray
            //     println!("im here1");
            // } else {
            //     // Most cocoa classes will emit their own inner pointer buffers instead of traversing this path. Notable exceptions include NSDictionary and NSSet
            //     println!("im here");
            // }

            self.items = unsafe {
                &*slice_from_raw_parts(self.state.items_ptr as *const *const T, self.len)
            };
        }

        let item = unsafe { transmute(self.items[self.index]) };
        self.index += 1;
        self.total_index += 1;
        Some(item)
    }
}

#[cfg(test)]
mod tests {
    use crate::ns;

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
        let one = ns::String::with_str("1");

        let arr: &[&ns::String] = &[&one];
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
        let one = ns::String::with_str("1");

        let arr: &[&ns::String] = &[&one, &one];
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
}
