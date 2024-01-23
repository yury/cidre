use crate::{arc, cf, define_cf_type, define_opts};

#[cfg(feature = "ns")]
use crate::ns;

use super::{runtime::Type, String};
use std::{borrow::Cow, cmp::Ordering, ffi::c_void, fmt::Debug, intrinsics::transmute};

pub type Index = isize;
pub type TypeId = usize;
pub type HashCode = usize;

define_opts!(pub OptionFlags(usize));

impl OptionFlags {
    pub const NONE: Self = Self(0);
}

pub type ComparatorFn = extern "C" fn(
    val1: *const c_void,
    val2: *const c_void,
    context: *mut c_void,
) -> ComparisonResult;

pub const NOT_FOUND: Index = -1;

#[doc(alias = "CFRange")]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(C)]
pub struct Range {
    pub loc: Index,
    pub len: Index,
}

impl Range {
    #[inline]
    pub fn new(loc: Index, len: Index) -> Self {
        Self { loc, len }
    }
}

#[derive(Debug, PartialEq, Eq)]
#[repr(isize)]
pub enum ComparisonResult {
    LessThen = -1,
    EqualTo = 0,
    GreaterThen = 1,
}

/// ```
/// use cidre::cf;
///
/// let s = cf::Number::from_i32(10);//from_static_string("hello").unwrap();
/// let t = s.get_type_id();
/// unsafe {
///     let desc = cf::copy_type_id_desc(t).unwrap();
///     assert_eq!("CFNumber", desc.to_string());
/// }
/// ```
pub unsafe fn copy_type_id_desc(type_id: TypeId) -> Option<arc::R<String>> {
    CFCopyTypeIDDescription(type_id)
}

impl From<ComparisonResult> for Ordering {
    /// ```
    /// use cidre::cf::ComparisonResult as CR;
    /// use core::cmp::Ordering as O;
    ///
    /// let less: O = CR::LessThen.into();
    /// let equal: O = CR::EqualTo.into();
    /// let greater: O = CR::GreaterThen.into();
    ///
    /// assert_eq!(less, O::Less);
    /// assert_eq!(equal, O::Equal);
    /// assert_eq!(greater, O::Greater);
    /// ```
    #[inline]
    fn from(o: ComparisonResult) -> Self {
        unsafe { transmute(o as i8) }
    }
}

define_cf_type!(Null(Type));

impl Type {
    #[inline]
    pub fn show(&self) {
        unsafe { CFShow(Some(self)) }
    }

    #[inline]
    pub fn allocator(&self) -> Option<&Allocator> {
        unsafe { CFGetAllocator(self) }
    }

    #[inline]
    pub fn retain_count(&self) -> isize {
        unsafe { CFGetRetainCount(self) }
    }

    /// ```
    /// use cidre::cf;
    ///
    /// let n1 = cf::Number::from_i8(4);
    /// let n2 = cf::Number::from_i32(4);
    /// let n3 = cf::Number::from_f64(3.0);
    ///
    /// assert!(n1.equal(&n2));
    /// assert_eq!(false, n1.equal(&n3));
    /// ```
    #[inline]
    pub fn equal(&self, other: &Type) -> bool {
        unsafe { CFEqual(self, other) }
    }

    #[inline]
    pub fn hash(&self) -> usize {
        unsafe { CFHash(self) }
    }

    #[inline]
    pub fn desc(&self) -> arc::R<String> {
        unsafe { CFCopyDescription(Some(self)).unwrap_unchecked() }
    }
}

impl Debug for Type {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let desc = self.desc();
        f.write_str(&Cow::from(desc.as_ref()))

        // f.debug_tuple("cf::Type")
        //     .field(&Cow::from(desc.as_ref()))
        //     .finish()
    }
}

impl Null {
    #[inline]
    pub fn value() -> &'static Null {
        unsafe { kCFNull }
    }

    #[cfg(feature = "ns")]
    #[inline]
    pub fn as_ns(&self) -> &ns::Null {
        unsafe { std::mem::transmute(self) }
    }
}

define_cf_type!(Allocator(Type));

pub type AllocatorRetainCb<T = c_void> = extern "C" fn(info: *const T) -> *const T;
pub type AllocatorReleaseCb<T = c_void> = extern "C" fn(info: *const T);
pub type AllocatorCopyDescCb<T = c_void> =
    extern "C" fn(info: *const T) -> Option<arc::R<cf::String>>;
pub type AllocatorAllocateCb<T = c_void> =
    extern "C" fn(alloc_size: Index, hint: cf::OptionFlags, info: *mut T) -> *mut c_void;
pub type AllocatorRealloacteCb<T = c_void> = extern "C" fn(
    ptr: *mut c_void,
    new_size: cf::Index,
    hint: cf::OptionFlags,
    info: *mut T,
) -> *mut c_void;
pub type AllocatorDealloacteCb<T = c_void> = extern "C" fn(ptr: *mut c_void, info: *mut T);
pub type AllocatorPreferredSizeCb<T = c_void> =
    extern "C" fn(size: cf::Index, hint: cf::OptionFlags, info: *mut T) -> Index;

/// A structure that defines the context or operating environment for an allocator
/// (cf::Allocator) object. Every Core Foundation allocator object must have a context
/// defined for it.
#[repr(C)]
pub struct AllocatorContext<T = c_void> {
    pub version: Index,

    /// An untyped pointer to program-defined data. Allocate memory for this data and assign a pointer to it.
    /// This data is often control information for the allocator. You may assign NULL.
    pub info: *const T,

    /// A prototype for a function callback that retains the data pointed to by the info field.
    /// In implementing this function, retain the data you have defined for the allocator context
    /// in this field. (This might make sense only if the data is a Core Foundation object.)
    /// You may set this function pointer to None.
    pub retain: Option<AllocatorRetainCb<T>>,

    /// A prototype for a function callback that releases the data pointed to by the info field.
    /// In implementing this function, release (or free) the data you have defined for the allocator
    /// context. You may set this function pointer to None, but doing so might result in memory leaks.
    pub release: Option<AllocatorReleaseCb<T>>,

    /// A prototype for a function callback that provides a description of the data pointed to
    /// by the info field. In implementing this function, return a reference to a cf::String object
    /// that describes your allocator, particularly some characteristics of your program-defined data.
    /// You may set this function pointer to None, in which case Core Foundation will provide a rudimentary
    /// description.
    pub copy_description: Option<AllocatorCopyDescCb<T>>,

    /// A prototype for a function callback that allocates memory of a requested size.
    /// In implementing this function, allocate a block of memory of at least size
    /// bytes and return a pointer to the start of the block. The hint argument is
    /// a bitfield that you should currently not use (that is, assign 0). The size parameter
    /// should always be greater than 0. If it is not, or if problems in allocation occur,
    /// return NULL. This function pointer may not be assigned NULL.
    pub allocate: AllocatorAllocateCb<T>,

    /// A prototype for a function callback that reallocates memory of a requested size
    /// for an existing block of memory.
    pub reallocate: AllocatorRealloacteCb<T>,

    /// A prototype for a function callback that deallocates a given block of memory.
    /// In implementing this function, make the block of memory pointed to by ptr available
    /// for subsequent reuse by the allocator but unavailable for continued use by the program.
    /// The ptr parameter cannot be NULL and if the ptr parameter is not a block of memory that
    /// has been previously allocated by the allocator, the results are undefined; abnormal
    /// program termination can occur. You can set this callback to None, in which case the
    /// CFAllocatorDeallocate function has no effect.
    pub deallocate: Option<AllocatorDealloacteCb<T>>,

    /// A prototype for a function callback that determines whether there is enough free memory
    /// to satisfy a request. In implementing this function, return the actual size the allocator
    /// is likely to allocate given a request for a block of memory of size size. The hint argument
    /// is a bitfield that you should currently not use.
    pub preferred_size: AllocatorPreferredSizeCb<T>,
}

/// Most of the time when specifying an allocator to Create functions, the None
/// argument indicates "use the default"; this is the same as using Allocator::default()
/// or the return value from cf::Allocator::default().  This assures that you will use
/// the allocator in effect at that time.
impl Allocator {
    #[doc(alias = "CFAllocatorCreate")]
    #[inline]
    pub fn new<T>(context: &mut AllocatorContext<T>) -> Option<arc::R<Allocator>> {
        Self::new_in(context, None)
    }

    #[doc(alias = "CFAllocatorCreate")]
    #[inline]
    pub fn new_in<T>(
        context: &mut AllocatorContext<T>,
        allocator: Option<&Allocator>,
    ) -> Option<arc::R<Allocator>> {
        unsafe { CFAllocatorCreate(allocator, std::mem::transmute(context)) }
    }

    /// This is a synonym for NULL, if you'd rather use a named constant.
    #[doc(alias = "kCFAllocatorDefault")]
    #[inline]
    pub fn default() -> Option<&'static Allocator> {
        unsafe { kCFAllocatorDefault }
    }

    /// Default system allocator; you rarely need to use this.
    #[doc(alias = "kCFAllocatorSystemDefault")]
    #[inline]
    pub fn system_default() -> Option<&'static Allocator> {
        unsafe { kCFAllocatorSystemDefault }
    }

    /// Null allocator which does nothing and allocates no memory. This allocator
    /// is useful as the `bytes_deallocator` in [`cf::Data`] or `contents_deallocator`
    /// in cf::String where the memory should not be freed.
    #[doc(alias = "kCFAllocatorNull")]
    #[inline]
    pub fn null() -> Option<&'static Allocator> {
        unsafe { kCFAllocatorNull }
    }

    /// Special allocator argument to cf::Allocator::create() which means
    /// "use the functions given in the context to allocate the allocator
    /// itself as well
    #[doc(alias = "kCFAllocatorUseContext")]
    #[inline]
    pub fn use_context() -> Option<&'static Allocator> {
        unsafe { kCFAllocatorUseContext }
    }

    #[doc(alias = "CFAllocatorAllocate")]
    #[inline]
    pub unsafe fn allocate(&self, size: Index, hint: OptionFlags) -> *mut c_void {
        CFAllocatorAllocate(self, size, hint)
    }

    #[doc(alias = "CFAllocatorReallocate")]
    #[inline]
    pub unsafe fn reallocate(
        &self,
        ptr: *mut c_void,
        new_size: Index,
        hint: OptionFlags,
    ) -> *mut c_void {
        CFAllocatorReallocate(self, ptr, new_size, hint)
    }

    #[doc(alias = "CFAllocatorDeallocate")]
    #[inline]
    pub unsafe fn deallocate(&self, ptr: *mut c_void) {
        CFAllocatorDeallocate(self, ptr)
    }

    #[doc(alias = "CFAllocatorGetPreferredSizeForSize")]
    #[inline]
    pub fn preferred_size(&self, size: Index, hint: cf::OptionFlags) -> Index {
        unsafe { CFAllocatorGetPreferredSizeForSize(self, size, hint) }
    }
}

#[link(name = "CoreFoundation", kind = "framework")]
extern "C" {
    fn CFCopyTypeIDDescription(type_id: TypeId) -> Option<arc::R<String>>;

    static kCFNull: &'static Null;

    static kCFAllocatorDefault: Option<&'static Allocator>;
    static kCFAllocatorSystemDefault: Option<&'static Allocator>;
    static kCFAllocatorNull: Option<&'static Allocator>;
    static kCFAllocatorUseContext: Option<&'static Allocator>;

    fn CFGetAllocator<'a>(cf: &Type) -> Option<&'a Allocator>;
    fn CFShow(cf: Option<&Type>);
    fn CFGetRetainCount(cf: &Type) -> Index;

    fn CFHash(cf: &Type) -> usize;

    fn CFEqual(cf1: &Type, cf2: &Type) -> bool;
    fn CFCopyDescription(cf: Option<&Type>) -> Option<arc::R<String>>;

    fn CFAllocatorAllocate(allocator: &Allocator, size: Index, hint: OptionFlags) -> *mut c_void;
    fn CFAllocatorReallocate(
        allocator: &Allocator,
        ptr: *mut c_void,
        new_size: Index,
        hint: OptionFlags,
    ) -> *mut c_void;
    fn CFAllocatorDeallocate(allocator: &Allocator, ptr: *mut c_void);
    fn CFAllocatorCreate(
        allocator: Option<&Allocator>,
        context: &mut AllocatorContext,
    ) -> Option<arc::R<Allocator>>;

    fn CFAllocatorGetPreferredSizeForSize(
        allocator: &Allocator,
        size: Index,
        hint: cf::OptionFlags,
    ) -> Index;
}

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
#[repr(isize)]
pub enum PlistFormat {
    #[doc(alias = "kCFPropertyListOpenStepFormat")]
    OpenStep = 1,

    #[doc(alias = "kCFPropertyListXMLFormat_v1_0")]
    XmlV1_0 = 100,

    #[doc(alias = "kCFPropertyListBinaryFormat_v1_0")]
    BinaryV1_0 = 200,
}

define_opts!(
    #[doc(alias = "CFPropertyListMutabilityOptions")]
    pub PlistMutabilityOpts(cf::OptionFlags)
);

impl PlistMutabilityOpts {
    pub const IMMUTABLE: Self = Self(cf::OptionFlags(0));
    pub const MUTABLE_CONTAINERS: Self = Self(cf::OptionFlags(1 << 0));
    pub const MUTABLE_CONTAINERS_AND_LEAVES: Self = Self(cf::OptionFlags(1 << 1));
}

define_cf_type!(
    #[doc(alias = "CFProperyListRef")]
    Plist(Type)
);

#[cfg(test)]
mod tests {
    use std::ffi::c_void;

    use crate::{arc, cf};

    #[test]
    fn custom_allocator_with_padding() {
        let system = cf::Allocator::system_default().unwrap();
        const PADDING: isize = 10isize;

        extern "C" fn retain(info: *const cf::Allocator) -> *const cf::Allocator {
            eprintln!("retain");
            unsafe {
                let rf = info.as_ref().unwrap().retained();
                let res = rf.as_ref() as *const cf::Allocator;
                std::mem::forget(rf);
                res
            }
        }
        extern "C" fn release(info: *const cf::Allocator) {
            eprintln!("release");
            let retained: arc::R<cf::Allocator> = unsafe { std::mem::transmute(info) };
            std::mem::drop(retained);
        }

        extern "C" fn copy_desc(info: *const cf::Allocator) -> Option<arc::R<cf::String>> {
            unsafe { info.as_ref().map(|a| a.desc()) }
        }

        extern "C" fn allocate(
            size: cf::Index,
            hint: cf::OptionFlags,
            info: *mut cf::Allocator,
        ) -> *mut c_void {
            unsafe {
                info.as_ref()
                    .map(|a| a.allocate(size + PADDING, hint).offset(PADDING))
                    .unwrap_or(std::ptr::null_mut())
            }
        }

        extern "C" fn reallocate(
            ptr: *mut c_void,
            new_size: cf::Index,
            hint: cf::OptionFlags,
            info: *mut cf::Allocator,
        ) -> *mut c_void {
            unsafe {
                info.as_ref()
                    .map(|a| {
                        a.reallocate(ptr.offset(-PADDING), new_size + PADDING, hint)
                            .offset(PADDING)
                    })
                    .unwrap_or(std::ptr::null_mut())
            }
        }

        extern "C" fn deallocate(ptr: *mut c_void, info: *mut cf::Allocator) {
            unsafe { info.as_ref().unwrap().deallocate(ptr.offset(-PADDING)) }
        }

        extern "C" fn preferred_size(
            size: cf::Index,
            hint: cf::OptionFlags,
            info: *mut cf::Allocator,
        ) -> cf::Index {
            unsafe { info.as_ref().unwrap().preferred_size(size + PADDING, hint) - PADDING }
        }

        let mut context = cf::AllocatorContext::<cf::Allocator> {
            version: 0,
            info: system as *const _,
            retain: Some(retain),
            release: Some(release),
            copy_description: Some(copy_desc),
            allocate,
            reallocate,
            deallocate: Some(deallocate),
            preferred_size,
        };

        let alloc = cf::Allocator::new(&mut context).unwrap();

        assert_eq!(alloc.preferred_size(10, cf::OptionFlags::NONE), 10);

        let mem = unsafe { alloc.allocate(10, cf::OptionFlags::NONE) };
        assert!(!mem.is_null());
        let buf: *mut u8 = mem.cast();
        let slice = unsafe { std::slice::from_raw_parts_mut(buf, 10) };
        slice[0] = 10;
        let bigger =
            unsafe { std::slice::from_raw_parts_mut(buf.offset(-PADDING), 10 + PADDING as usize) };

        bigger[0] = 1;
        assert_eq!(bigger[PADDING as usize], 10);
        unsafe { alloc.deallocate(mem) };
    }
}
