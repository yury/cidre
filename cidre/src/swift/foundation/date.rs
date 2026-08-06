use crate::swift;

crate::define_swift!(
    #[swift::struct("Foundation.Date")]
    /// Swift `Foundation.Date`.
    pub Date, DateValue
);

crate::impl_swift_sendable!(DateValue);

impl Date {
    /// `Date()`, which is the current instant.
    #[allow(clippy::new_without_default)]
    #[swift::call("Foundation.Date(struct).init()")]
    pub fn now() -> Self;

    #[swift::call("Foundation.Date(struct).timeIntervalSinceReferenceDate: Double { get }")]
    pub fn time_interval_since_reference_date(&self) -> f64;
}
