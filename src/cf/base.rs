use super::String;
use std::{
    cmp::Ordering,
    ffi::c_void,
    ops::{Deref, DerefMut},
    ptr::NonNull,
};

pub type Index = isize;
pub type TypeID = usize;
pub type HashCode = usize;

pub type OptionFlags = usize;

pub type ComparatorFunction = extern "C" fn(
    val1: *const c_void,
    val2: *const c_void,
    context: *mut c_void,
) -> ComparisonResult;

pub const NOT_FOUND: Index = -1;

#[repr(C)]
pub struct Range {
    location: Index,
    length: Index,
}

impl Range {
    #[inline]
    pub fn new(location: Index, length: Index) -> Self {
        Range { location, length }
    }
}

#[repr(isize)]
pub enum ComparisonResult {
    LessThen = -1,
    EqualTo = 0,
    GreaterThen = 1,
}

/// ```
/// use cidre::cf;
///
/// let s = cf::String::from_static_string("hello").unwrap();
/// let t = s.get_type_id();
/// let desc = cf::copy_type_id_description(t).unwrap();
/// desc.show_str();
/// ```

pub fn copy_type_id_description(type_id: TypeID) -> Option<String> {
    unsafe { CFCopyTypeIDDescription(type_id) }
}

#[derive(Copy, Clone)]
#[repr(transparent)]
pub struct TypeRef(NonNull<*const c_void>);

#[derive(Copy, Clone, PartialEq, Eq)]
#[repr(transparent)]
pub struct Null(TypeRef);

impl Null {
    pub fn null() -> Self {
        unsafe { kCFNull }
    }
}

impl Default for Null {
    #[inline]
    fn default() -> Self {
        Self::null()
    }
}

#[repr(transparent)]
pub struct Type(TypeRef);

impl Drop for Type {
    fn drop(&mut self) {
        self.release()
    }
}

impl Deref for Type {
    type Target = TypeRef;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Type {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

#[derive(Copy, Clone, PartialEq, Eq)]
#[repr(transparent)]
pub struct AllocatorRef(TypeRef);

impl AllocatorRef {
    #[inline]
    pub fn system_default() -> Option<AllocatorRef> {
        unsafe { kCFAllocatorSystemDefault }
    }

    #[inline]
    pub fn malloc() -> Option<AllocatorRef> {
        unsafe { kCFAllocatorMalloc }
    }

    #[inline]
    pub fn malloc_zone() -> Option<AllocatorRef> {
        unsafe { kCFAllocatorMallocZone }
    }

    #[inline]
    pub fn null() -> Option<AllocatorRef> {
        unsafe { kCFAllocatorNull }
    }

    #[inline]
    pub fn default() -> Option<AllocatorRef> {
        unsafe { kCFAllocatorDefault }
    }
}

impl TypeRef {
    #[inline]
    pub fn retained(&self) -> Type {
        unsafe { Type(self.retain()) }
    }

    #[inline]
    pub unsafe fn retain(&self) -> TypeRef {
        CFRetain(*self)
    }

    #[inline]
    pub fn release(&mut self) {
        unsafe { CFRelease(*self) }
    }

    pub fn get_retain_count(&self) -> Index {
        unsafe { CFGetRetainCount(*self) }
    }

    #[inline]
    pub fn get_type_id(&self) -> TypeID {
        unsafe { CFGetTypeID(*self) }
    }

    #[inline]
    pub fn hash(&self) -> HashCode {
        unsafe { CFHash(*self) }
    }

    #[inline]
    pub fn equal(&self, other: &TypeRef) -> bool {
        unsafe { CFEqual(*self, *other) }
    }

    #[inline]
    pub fn copy_description(&self) -> String {
        unsafe { CFCopyDescription(*self) }
    }

    #[inline]
    pub fn get_allocator(&self) -> Option<AllocatorRef> {
        unsafe { CFGetAllocator(*self) }
    }
}

impl PartialEq for TypeRef {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        self.equal(other)
    }
}

impl Eq for TypeRef {}

impl Deref for Null {
    type Target = TypeRef;

    #[inline]
    fn deref(&self) -> &TypeRef {
        &self.0
    }
}

impl Deref for AllocatorRef {
    type Target = TypeRef;

    #[inline]
    fn deref(&self) -> &TypeRef {
        &self.0
    }
}

impl From<ComparisonResult> for Ordering {
    fn from(o: ComparisonResult) -> Self {
        // TODO: transmute
        match o {
            ComparisonResult::LessThen => Ordering::Less,
            ComparisonResult::EqualTo => Ordering::Equal,
            ComparisonResult::GreaterThen => Ordering::Greater,
        }
    }
}

extern "C" {
    static kCFNull: Null;

    fn CFCopyTypeIDDescription(type_id: TypeID) -> Option<String>;

    fn CFRetain(cf: TypeRef) -> TypeRef;
    fn CFRelease(cf: TypeRef);
    fn CFGetRetainCount(cf: TypeRef) -> Index;
    fn CFHash(cf: TypeRef) -> HashCode;
    fn CFEqual(cf1: TypeRef, cf2: TypeRef) -> bool;
    fn CFGetTypeID(cf: TypeRef) -> TypeID;
    fn CFCopyDescription(cf: TypeRef) -> String;
    fn CFGetAllocator(cf: TypeRef) -> Option<AllocatorRef>;

    static kCFAllocatorDefault: Option<AllocatorRef>;
    static kCFAllocatorSystemDefault: Option<AllocatorRef>;
    static kCFAllocatorMalloc: Option<AllocatorRef>;
    static kCFAllocatorMallocZone: Option<AllocatorRef>;
    static kCFAllocatorNull: Option<AllocatorRef>;

}
