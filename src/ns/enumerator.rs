use std::{ffi::c_void, mem::transmute, ptr::slice_from_raw_parts};

use crate::{msg_send, ns, objc};

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

pub trait FastEnumeration<T>: objc::Obj {
    #[inline]
    fn count_by_enumerating(
        &self,
        state: &mut FastEnumerationState,
        objects: *const *mut T,
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

    fn iter(&self) -> FEIterator<Self, T>
    where
        T: objc::Obj,
    {
        FEIterator::new(self)
    }
}

#[derive(Debug)]
pub struct FEIterator<'a, E, T, const N: usize = 16>
where
    E: objc::Obj + FastEnumeration<T>,
    T: objc::Obj + 'a,
{
    enu: &'a E,
    state: FastEnumerationState,
    objects: [*mut T; N],
    slice: &'a [*mut T],
    mutation: *const u32,
    slice_index: usize,
    index: usize,
}

impl<'a, E, T, const N: usize> FEIterator<'a, E, T, N>
where
    E: objc::Obj + FastEnumeration<T>,
    T: objc::Obj + 'a,
{
    pub fn new(enu: &'a E) -> Self {
        let mut res = Self {
            enu,
            state: FastEnumerationState::new(),
            objects: [std::ptr::null_mut(); N],
            mutation: std::ptr::null(),
            slice: &[],
            slice_index: 0,
            index: 0,
        };
        let len = enu.count_by_enumerating(&mut res.state, res.objects.as_ptr(), N);

        let ptr = res.state.items_ptr as *mut *mut T;
        res.slice = unsafe { &*slice_from_raw_parts(ptr, len) };
        res.mutation = res.state.mutations_ptr;

        res
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
            .enu
            .count_by_enumerating(&mut self.state, self.objects.as_ptr(), N);
        let ptr = self.state.items_ptr as *mut *mut T;
        self.slice = unsafe { &*slice_from_raw_parts(ptr, len) };
        self.mutation = self.state.mutations_ptr;
        self.slice_index = 0;

        self.next()
    }
}

// impl<'a, T> ExactSizeIterator for ArrayIterator<'a, T>
// where
//     T: objc::Obj,
// {
//     fn len(&self) -> usize {
//         self.array.len() - self.index
//     }
// }

#[cfg(test)]
mod tests {
    use crate::ns;

    #[test]
    fn basics() {
        let one = ns::Number::with_i32(1);

        let nums: &[&ns::Number] = &[&one, &one];
        let arr = ns::Array::from_slice(nums);
        let sum = arr.iter().map(|v| v.as_i32()).sum();

        assert_eq!(2, sum);
    }
}
