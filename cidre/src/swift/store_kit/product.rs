use crate::{api, arc, ns, swift, swift::abi};

use crate::swift::{
    FromSwift, SwiftMetadata,
    concurrency::{self, AsyncCallArgs},
    value::Storage,
};

crate::define_swift!(#[swift::struct("StoreKit.Product")] pub(crate) ProductValue);

// `Product` is `Sendable` in Swift, so the storage holding one may cross
// threads, and with it the wrapper and any array of them.
crate::impl_swift_sendable!(ProductValue);

#[link(name = "StoreKit", kind = "framework")]
unsafe extern "C" {
    /// `static func products<Identifiers: Collection>(for: Identifiers) async
    /// throws -> [Product] where Identifiers.Element == String`
    #[link_name = "$s8StoreKit7ProductV8products3forSayACGx_tYaKSlRzSS7ElementRtzlFZ"]
    fn product_products_for();

    #[link_name = "$s8StoreKit7ProductV8products3forSayACGx_tYaKSlRzSS7ElementRtzlFZTu"]
    static PRODUCT_PRODUCTS_FOR_ASYNC_FN: u8;
}

#[link(name = "swiftCore")]
unsafe extern "C" {
    /// The conformance descriptor of `Array<Element>: Collection`, which the
    /// generic call above is instantiated through for `[String]`.
    #[link_name = "$sSayxGSlsMc"]
    static ARRAY_COLLECTION_CONFORMANCE: u8;
}

/// A StoreKit 2 `Product`.
///
/// The struct is resilient, so its layout is the runtime's to know: the value
/// lives in storage sized from its metadata and every access goes through the
/// framework's own getters.
#[doc(alias = "StoreKit.Product")]
pub struct Product {
    value: Storage<ProductValue>,
}

/// The value's own address is what a getter takes as `self`.
unsafe impl swift::SwiftSelf for Product {
    #[inline]
    fn swift_self_ptr(&self) -> *const () {
        self.value.as_ptr()
    }
}

unsafe impl SwiftMetadata for Product {
    #[inline]
    fn metadata() -> *const abi::TypeMetadata {
        ProductValue::metadata()
    }
}

/// What lets a `Product` be an element of a [`swift::Array`]: the array's
/// subscript writes an owned value into runtime-sized scratch storage, and this
/// moves it into storage of its own.
unsafe impl FromSwift for Product {
    #[inline]
    unsafe fn copy_swift(value: *const ()) -> Self {
        let mut storage = Storage::<ProductValue>::new();
        unsafe {
            abi::initialize_with_copy(storage.as_mut_ptr(), value, ProductValue::metadata());
        }
        Self { value: storage }
    }

    #[inline]
    unsafe fn take_swift(value: *mut ()) -> Self {
        let mut storage = Storage::<ProductValue>::new();
        unsafe {
            abi::initialize_with_take(storage.as_mut_ptr(), value, ProductValue::metadata());
        }
        Self { value: storage }
    }
}

impl Clone for Product {
    #[inline]
    fn clone(&self) -> Self {
        unsafe { Self::copy_swift(self.value.as_ptr()) }
    }
}

impl Drop for Product {
    #[inline]
    fn drop(&mut self) {
        unsafe { self.value.destroy() }
    }
}

impl Product {
    #[doc(alias = "Product.id")]
    #[swift::call(sym = "$s8StoreKit7ProductV2idSSvg")]
    pub fn id(&self) -> swift::String;

    #[doc(alias = "Product.displayName")]
    #[swift::call(sym = "$s8StoreKit7ProductV11displayNameSSvg")]
    pub fn display_name(&self) -> swift::String;

    #[doc(alias = "Product.displayPrice")]
    #[swift::call(sym = "$s8StoreKit7ProductV12displayPriceSSvg")]
    pub fn display_price(&self) -> swift::String;

    /// The `[String]: Collection` witness table the generic `products(for:)`
    /// is instantiated with, built once from the standard library's
    /// conformance descriptor.
    fn string_array_collection_witness() -> *const () {
        static CACHE: abi::WitnessCache = abi::WitnessCache::new();
        CACHE.get(|| unsafe {
            let witness = abi::witness_table(
                (&raw const ARRAY_COLLECTION_CONFORMANCE).cast(),
                <swift::Array<swift::String> as SwiftMetadata>::metadata(),
            );
            assert!(
                !witness.is_null(),
                "[String]: Collection witness table must exist"
            );
            witness
        })
    }

    /// The registers `products(for:)` takes, instantiated for `[String]`.
    ///
    /// The identifiers are a generic `@in_guaranteed` argument, so what goes in
    /// `x0` is the address of the array word rather than the word itself, with
    /// the type's metadata and its `Collection` conformance following it. The
    /// static method's `self` is the `Product` metadata.
    fn products_args(ids: &mut swift::Array<swift::String>) -> AsyncCallArgs {
        AsyncCallArgs::new()
            .swift_self(ProductValue::metadata().cast_mut().cast())
            .arg(0, core::ptr::from_mut(ids).cast())
            .arg(
                1,
                <swift::Array<swift::String> as SwiftMetadata>::metadata()
                    .cast_mut()
                    .cast(),
            )
            .arg(2, Self::string_array_collection_witness().cast_mut())
    }

    /// The array the call returns is one owned word, which is what the wrapper
    /// holds.
    fn products_output(_ids: swift::Array<swift::String>, result: *mut ()) -> swift::Array<Self> {
        unsafe { swift::Array::from_raw(result) }
    }

    fn ids_array(ids: &[&str]) -> swift::Array<swift::String> {
        swift::Array::from_iter(ids.iter().map(|id| swift::String::from(*id)))
    }

    /// Requests the App Store products for the given identifiers, reporting
    /// the result to `callback`.
    #[doc(alias = "Product.products(for:)")]
    #[api::available(
        macos = 12.0,
        ios = 15.0,
        maccatalyst = 15.0,
        tvos = 15.0,
        visionos = 1.0
    )]
    pub fn products_handler<F>(ids: &[&str], callback: F)
    where
        F: FnOnce(Result<swift::Array<Self>, arc::R<ns::Error>>) + Send + 'static,
    {
        unsafe {
            concurrency::call_async_result(
                product_products_for as *const (),
                &raw const PRODUCT_PRODUCTS_FOR_ASYNC_FN,
                Self::ids_array(ids),
                Self::products_args,
                Self::products_output,
                callback,
            );
        }
    }

    /// Requests the App Store products for the given identifiers.
    #[cfg(feature = "async")]
    #[doc(alias = "Product.products(for:)")]
    #[api::available(
        macos = 12.0,
        ios = 15.0,
        maccatalyst = 15.0,
        tvos = 15.0,
        visionos = 1.0
    )]
    pub fn products(
        ids: &[&str],
    ) -> impl std::future::Future<Output = Result<swift::Array<Self>, arc::R<ns::Error>>> {
        unsafe {
            concurrency::call_async_future(
                product_products_for as *const (),
                &raw const PRODUCT_PRODUCTS_FOR_ASYNC_FN,
                Self::ids_array(ids),
                Self::products_args,
                Self::products_output,
            )
        }
    }
}

impl std::fmt::Debug for Product {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Product")
            .field("id", &self.id().to_string())
            .field("display_name", &self.display_name().to_string())
            .field("display_price", &self.display_price().to_string())
            .finish()
    }
}
