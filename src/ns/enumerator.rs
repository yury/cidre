use std::{ffi::c_void, mem::transmute, ptr::slice_from_raw_parts};

use crate::{msg_send, objc};

#[derive(Debug)]
#[repr(C)]
pub struct FastEnumerationState {
    pub state: u32,
    pub items_ptr: *mut *mut c_void,
    pub mutations_ptr: *mut u32,
    pub extra: [u32; 5],
}

impl FastEnumerationState {
    pub fn new() -> Self {
        Self {
            state: 0,
            items_ptr: std::ptr::null_mut(),
            mutations_ptr: std::ptr::null_mut(),
            extra: [0; 5],
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
        objects: *mut *const T,
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

#[derive(Debug)]
pub struct FEIterator<'a, E, T, const N: usize = 16>
where
    E: objc::Obj + FastEnumeration<T>,
    T: objc::Obj + 'a,
{
    pub obj: &'a E,
    state: FastEnumerationState,
    objects: [*const T; N],
    slice: &'a [*const T],
    mutation: *const u32,
    slice_index: usize,
    pub index: usize,
}

impl<'a, E, T, const N: usize> FEIterator<'a, E, T, N>
where
    E: objc::Obj + FastEnumeration<T>,
    T: objc::Obj + 'a,
{
    pub fn new(obj: &'a E) -> Self {
        let mut objects = [std::ptr::null(); N];
        let mut state = Default::default();
        let len = obj.count_by_enumerating(&mut state, objects.as_mut_ptr(), N);
        let slice = unsafe { &*slice_from_raw_parts(state.items_ptr as _, len) };
        let mutation = state.mutations_ptr;

        Self {
            obj,
            state,
            objects,
            slice,
            mutation,
            slice_index: 0,
            index: 0,
        }
    }
}

impl<'a, E, T, const N: usize> Iterator for FEIterator<'a, E, T, N>
where
    E: objc::Obj + FastEnumeration<T>,
    T: objc::Obj + 'a,
{
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        let len = self.slice.len();
        if len > self.slice_index {
            let item = unsafe { transmute(self.slice[self.slice_index]) };
            self.slice_index += 1;
            self.index += 1;
            return Some(item);
        }

        if len < N || len == 0 {
            return None;
        }
        let len = self
            .obj
            .count_by_enumerating(&mut self.state, self.objects.as_mut_ptr(), N);
        self.slice = unsafe { &*slice_from_raw_parts(self.state.items_ptr as _, len) };
        self.mutation = self.state.mutations_ptr;
        self.slice_index = 0;

        self.next()
    }
}

#[cfg(test)]
mod tests {
    use crate::ns;

    #[test]
    fn basics() {
        let one = ns::Number::with_i32(1);

        let nums: &[&ns::Number] = &[&one, &one];
        let arr = ns::Array::from_slice(nums);
        let sum = arr.iter().map(|v| v.as_i32()).sum();
        // let mut sum = 0;
        // for i in arr.iter() {
        //     sum += i.as_i32();
        // }

        assert_eq!(2, sum);
    }

    #[test]
    fn basics2() {
        let one = ns::Number::with_i64(1);

        let arr: &[&ns::Number] = &[&one];
        let array = ns::Array::from_slice(arr);

        let mut k = 0;
        for i in array.iter() {
            k += 1;
            //            println!("{:?}", i);
        }

        assert_eq!(1, k);
    }
}
