use crate::objc;

static MUTATIONS_TARGET: std::ffi::c_ulong = 0;
static MUTATIONS_PTR: &std::ffi::c_ulong = &MUTATIONS_TARGET;

#[doc(alias = "NSFastEnumerationState")]
#[derive(Debug, Copy, Clone)]
#[repr(C)]
pub struct FeState<T> {
    pub state: std::ffi::c_ulong,
    pub items_ptr: *const *const T,
    pub mutations_ptr: &'static std::ffi::c_ulong,
    pub extra: [std::ffi::c_ulong; 5],
}

impl<T> FeState<T> {
    #[inline]
    pub fn new() -> Self {
        Self {
            state: 0,
            items_ptr: std::ptr::null(),
            mutations_ptr: MUTATIONS_PTR,
            extra: Default::default(),
        }
    }
}

impl<T> Default for FeState<T> {
    fn default() -> Self {
        Self::new()
    }
}

#[doc(alias = "NSFastEnumeration")]
pub trait FastEnum<T: objc::Obj>: objc::Obj {
    #[objc::msg_send(countByEnumeratingWithState:objects:count:)]
    fn count_by_enum(&self, state: &mut FeState<T>, objects: *mut *const T, count: usize) -> usize;

    #[inline]
    fn iter(&self) -> FeIterator<Self, T> {
        FeIterator::new(self)
    }
}

// https://github.com/apple/swift-corelibs-foundation/blob/main/Darwin/Foundation-swiftoverlay/NSFastEnumeration.swift
// https://www.mikeash.com/pyblog/friday-qa-2010-04-16-implementing-fast-enumeration.html
#[derive(Debug)]
pub struct FeIterator<'a, E, T, const N: usize = 16>
where
    E: objc::Obj + FastEnum<T>,
    T: objc::Obj + 'a,
{
    obj: &'a E,
    batch: [*const T; N],
    state: FeState<T>,
    index: usize,
    len: usize,
}

impl<'a, E, T, const N: usize> FeIterator<'a, E, T, N>
where
    E: objc::Obj + FastEnum<T>,
    T: objc::Obj + 'a,
{
    #[inline]
    pub fn new(obj: &'a E) -> Self {
        Self {
            obj,
            state: Default::default(),
            // on stack buffer to fill
            batch: [std::ptr::null(); N],
            index: 0,
            len: 0,
        }
    }
}

impl<'a, E, T, const N: usize> Iterator for FeIterator<'a, E, T, N>
where
    E: objc::Obj + FastEnum<T>,
    T: objc::Obj + 'a,
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
                .count_by_enum(&mut self.state, self.batch.as_mut_ptr(), N);

            if self.len == 0 {
                return None;
            }

            self.index = 0;
        }

        let item = unsafe { self.state.items_ptr.add(self.index) };
        self.index += 1;
        let item = unsafe { item.read_unaligned() };
        Some(unsafe { std::mem::transmute(item) })
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
