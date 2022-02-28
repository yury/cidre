use crate::objc;

pub use objc::ns::Integer;
pub use objc::ns::UInteger;
pub use objc::Class;
pub use objc::Id;
pub use objc::Sel;

#[link(name = "Foundation", kind = "framework")]
extern "C" {}

#[cfg(test)]
mod tests {

    #[test]
    fn it_works() {}
}
