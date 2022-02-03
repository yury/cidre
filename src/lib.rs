pub type FourCharCode = u32;
pub type ResType = FourCharCode; // ??

pub mod os {
    use super::FourCharCode;

    pub type Err = i16;
    pub type Status = i32;
    pub type Type = FourCharCode;

    pub const NO_ERR: Status = 0;
}

pub mod cf;
pub mod cm;
pub mod io;
pub mod vt;

#[cfg(test)]
mod tests {
    use crate::cf;
    #[test]
    fn it_works() {
        let null = cf::Null::value();
        let num = cf::Number::from_i16(0).unwrap();
        let arr = cf::Array::from_type_refs(&[null, &num]).unwrap();

        println!("len {:?}", arr.len());
        arr.show();
    }
}
