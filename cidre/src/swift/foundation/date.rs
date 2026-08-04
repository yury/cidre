use crate::swift::{
    SwiftMetadata, abi,
    value::{Storage, Value},
};

#[link(name = "Foundation", kind = "framework")]
unsafe extern "C" {
    #[link_name = "$s10Foundation4DateVMa"]
    fn date_metadata();

    #[link_name = "$s10Foundation4DateV026timeIntervalSinceReferenceB0Sdvg"]
    fn date_time_interval_since_reference_date();
}

crate::define_swift_marker!(pub(crate) DateValue = accessor date_metadata);

/// Swift `Foundation.Date`.
pub struct Date(Value<DateValue>);

unsafe impl Send for Date {}
unsafe impl Sync for Date {}

impl Date {
    pub(crate) unsafe fn copy_from_ptr(value: *const ()) -> Self {
        unsafe {
            let mut storage = Storage::<DateValue>::new();
            abi::initialize_with_copy(storage.as_mut_ptr(), value, DateValue::metadata());
            Self(storage.assume_init())
        }
    }

    #[allow(dead_code)]
    pub(crate) unsafe fn from_value(value: Value<DateValue>) -> Self {
        Self(value)
    }

    pub fn time_interval_since_reference_date(&self) -> f64 {
        unsafe {
            abi::call_value_to_double(
                date_time_interval_since_reference_date as *const (),
                self.0.as_ptr(),
            )
        }
    }
}

impl Clone for Date {
    fn clone(&self) -> Self {
        unsafe { Self::copy_from_ptr(self.0.as_ptr()) }
    }
}
