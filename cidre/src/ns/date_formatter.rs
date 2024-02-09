use crate::{arc, define_obj_type, ns, objc};

define_obj_type!(
    #[doc(alias = "NSDateFormatter")]
    pub DateFormatter(ns::Formatter),
    NS_DATE_FORMATTER
);

unsafe impl Send for DateFormatter {}
unsafe impl Sync for DateFormatter {}

impl DateFormatter {
    #[objc::msg_send(stringFromDate:)]
    pub fn string_from_date_ar(&self, date: &ns::Date) -> arc::Rar<ns::String>;

    #[objc::rar_retain]
    pub fn string_from_date(&self, date: &ns::Date) -> arc::R<ns::String>;

    #[objc::msg_send(dateFormat)]
    pub fn date_format(&self) -> &ns::String;

    #[objc::msg_send(setDateFormat:)]
    pub fn set_date_format(&mut self, val: Option<&ns::String>);
}

#[link(name = "ns", kind = "static")]
extern "C" {
    static NS_DATE_FORMATTER: &'static objc::Class<DateFormatter>;
}

#[cfg(test)]
mod tests {
    use crate::ns;

    #[test]
    fn basics() {
        let date = ns::Date::new();
        let formatter = ns::DateFormatter::new();
        let str = formatter.string_from_date(&date);
        assert!(str.is_empty());

        let format = formatter.date_format();
        assert!(format.is_empty());
    }
}
