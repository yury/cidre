pub type FourCharCode = u32;
pub type ResType = FourCharCode; // ??

pub mod os {
    use super::FourCharCode;

    pub type Err = i16;
    pub type Status = i32;
    pub type Type = FourCharCode;
    
    pub const NO_ERR: Err = 0;
}

pub mod cf;
pub mod vt;
pub mod cm;

#[cfg(test)]
mod tests {
    use crate::cf;
    #[test]
    fn it_works() {
        cf::Null::value().show();
        let str = cf::MutableString::create(None, 0).unwrap();
        str.show_str();
        assert_eq!(cf::array_get_type_id(), 19);
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
