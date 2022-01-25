pub mod cf;

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
