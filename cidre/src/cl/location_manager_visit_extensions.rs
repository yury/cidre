use crate::cl;

#[cfg(any(target_os = "macos", target_os = "ios"))]
use crate::objc;

impl cl::LocationManager {
    #[cfg(any(target_os = "macos", target_os = "ios"))]
    #[objc::msg_send(startMonitoringVisits)]
    #[objc::available(macos = 10.15, ios = 8.0)]
    pub fn start_monitoring_visits(&mut self);

    #[cfg(any(target_os = "macos", target_os = "ios"))]
    #[objc::msg_send(stopMonitoringVisits)]
    #[objc::available(macos = 10.15, ios = 8.0)]
    pub fn stop_monitoring_visits(&mut self);
}
