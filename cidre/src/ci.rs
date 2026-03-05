mod context;
pub use context::Context;

mod image;
pub use image::Format;
pub use image::Image;
pub use image::ImageOpt;

mod vector;
pub use vector::Vec;

mod barcode_descriptor;
pub use barcode_descriptor::BarcodeDesc;

#[link(name = "CoreImage", kind = "framework")]
unsafe extern "C" {}

#[link(name = "ci", kind = "static")]
unsafe extern "C" {}
