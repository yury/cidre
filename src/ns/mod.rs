use crate::objc;

pub use objc::ns::Integer;
pub use objc::ns::UInteger;
pub use objc::Class;
pub use objc::Id;
pub use objc::Sel;

// typedef struct _NSRange {
//     NSUInteger location;
//     NSUInteger length;
// } NSRange;

#[derive(PartialEq, Eq)]
#[repr(C)]
pub struct Range {
    location: UInteger,
    length: UInteger,
}

impl Range {
    #[inline]
    pub fn make(location: UInteger, length: UInteger) -> Self {
        Self { location, length }
    }

    #[inline]
    pub fn max(&self) -> UInteger {
        self.location + self.length
    }
}

#[link(name = "Foundation", kind = "framework")]
extern "C" {}

#[cfg(test)]
mod tests {

    #[test]
    fn it_works() {}
}
