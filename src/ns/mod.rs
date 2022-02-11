
use crate::objc;

pub use objc::Class;
pub use objc::Id;
pub use objc::Sel;
pub use objc::ns::Object;


#[link(name = "Foundation", kind = "framework")]
extern "C" {
}

#[cfg(test)]
mod tests {
    use crate::ns;
    
    #[test]
    fn it_works() {
      let foo = ns::Object::new();
      foo.as_type_ref().show();
    }
}