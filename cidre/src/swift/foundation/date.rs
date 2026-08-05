use crate::swift::{abi, value::define_swift_value};

#[link(name = "Foundation", kind = "framework")]
unsafe extern "C" {
    #[link_name = "$s10Foundation4DateVMa"]
    fn date_metadata();

    #[link_name = "$s10Foundation4DateV026timeIntervalSinceReferenceB0Sdvg"]
    fn date_time_interval_since_reference_date();
}

define_swift_value!(
    /// Swift `Foundation.Date`.
    pub Date, DateValue = accessor date_metadata
);

unsafe impl Send for Date {}
unsafe impl Sync for Date {}

impl Date {
    pub fn time_interval_since_reference_date(&self) -> f64 {
        unsafe {
            abi::call_value_to_double(
                date_time_interval_since_reference_date as *const (),
                self.as_ptr(),
            )
        }
    }
}
