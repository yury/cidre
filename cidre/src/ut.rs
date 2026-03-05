mod _type;
pub use _type::Type;
pub mod core_types;

#[link(name = "UniformTypeIdentifiers", kind = "framework")]
unsafe extern "C" {}

#[link(name = "ut", kind = "static")]
unsafe extern "C" {}
