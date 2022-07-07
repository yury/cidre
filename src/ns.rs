use crate::objc;

pub use objc::{
    ns::{Integer, UInteger},
    Class, Id, Sel,
};

pub mod range;
pub use range::Range;

pub mod process_info;
pub use process_info::ThermalState as ProcessInfoThermalState;
