pub mod mac_types;
pub use mac_types::FourCharCode;
pub use mac_types::ResType;
pub use mac_types::UniChar;

pub mod cf;
pub mod cg;
pub mod cm;
pub mod cv;
pub mod io;
pub mod mach;
pub mod os;
pub mod sys;
pub mod vt;

pub fn test_asm() {
    unsafe {
        let null = cf::Null::value();
        let num = cf::Number::from_i16(0);
        let arr = cf::Array::from_type_refs(&[null, &num]).unwrap_unchecked();

        arr.show();
    }
}

#[cfg(test)]
mod tests {
    use crate::cf;
    #[test]
    fn it_works() {
        let f = {
            let null = cf::Null::value();
            null.show();
            let num = cf::Number::from_i16(0);
            let arr = cf::Array::from_type_refs(&[null, &num]).unwrap();

            let v = b"he".to_vec();
            let s = cf::String::create_with_bytes_no_copy(
                None,
                &v,
                2,
                cf::StringEncoding::UTF8,
                false,
                Some(cf::Allocator::null()),
            )
            .unwrap();
            // s
            let f = num;

            arr.show();
            // s.show();

            // println!("len {:?}", f.get_length());
            // f.show();
            arr
            // s.retained()
        };

        f.show()
    }
}
