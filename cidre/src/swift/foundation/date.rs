use crate::swift;

crate::define_swift!(
    #[swift::struct("Foundation.Date", size(8), align(8), trivial, sendable)]
    /// Swift `Foundation.Date`.
    /// One `Double` since the type shipped: held inline, copied and dropped
    /// without touching Swift at all.
    pub Date
);

impl Date {
    /// `Date()`, which is the current instant.
    #[swift::call("Foundation.Date(struct).init()")]
    pub fn now() -> Self;

    #[swift::call("Foundation.Date(struct).timeIntervalSinceReferenceDate: Double { get }")]
    pub fn time_interval_since_reference_date(&self) -> f64;
}

impl Default for Date {
    fn default() -> Self {
        Self::now()
    }
}
