pub mod device;
pub use device::Action as DeviceAction;
pub use device::Device;
pub use device::InterfaceConnectionType as DeviceInterfaceConnectionType;
pub use device::Notification as DeviceNotification;
pub use device::QueryBuilder as DeviceQueryBuilder;
pub use device::Speed as DeviceSpeed;

pub mod service_connection;
pub use service_connection::ServiceConnection;
pub use service_connection::InvalidSocketError;