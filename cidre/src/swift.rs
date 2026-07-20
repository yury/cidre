//! Swift ABI interop.
//!
//! This module intentionally calls Swift ABI entry points directly from Rust
//! inline assembly. It does not use C or Objective-C wrapper functions.

pub mod abi;
mod array;
mod int;
mod object;
mod string;

pub use array::Array;
pub use int::{Int, UInt};
pub use object::Object;
pub use string::{RawString, SmallStringError, String};
