use crate::objc;

pub use objc::{
    ns::{Integer, UInteger},
    Class, Id, Sel,
};

pub mod range;
pub use range::Range;

pub mod process_info;
pub use process_info::ProcessInfo;
pub use process_info::ThermalState as ProcessInfoThermalState;

pub mod objc_runtime;
pub use objc_runtime::ExceptionName;

pub mod exception;
pub use exception::get_uncaought_exception_handler;
pub use exception::set_uncaught_exception_handler;
pub use exception::Exception;
pub use exception::UncaughtExceptionHandler;
pub use exception::try_catch;
