use crate::{arc, define_obj_type, ns, objc};

define_obj_type!(
    #[doc(alias = "NSISO8601DateFormatter")]
    pub Iso8601DateFormatter(ns::Formatter),
    NS_ISO_8601_DATE_FORMATTER
);

impl Iso8601DateFormatter {
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
    static NS_ISO_8601_DATE_FORMATTER: &'static objc::Class<Iso8601DateFormatter>;
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
