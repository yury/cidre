use crate::{arc, cf, define_obj_type, ns, objc};

define_obj_type!(
    #[doc(alias = "NSDateFormatter")]
    pub DateFormatter(ns::Formatter),
    NS_DATE_FORMATTER
);

unsafe impl Send for DateFormatter {}
unsafe impl Sync for DateFormatter {}

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
#[repr(usize)]
pub enum DateFormatterStyle {
    No = cf::DateFormatterStyle::No as usize,
    Short = cf::DateFormatterStyle::Short as usize,
    Medium = cf::DateFormatterStyle::Medium as usize,
    Long = cf::DateFormatterStyle::Long as usize,
    Full = cf::DateFormatterStyle::Full as usize,
}

impl DateFormatter {
    #[objc::msg_send(stringFromDate:)]
    pub fn string_from_date_ar(&self, date: &ns::Date) -> arc::Rar<ns::String>;

    #[objc::rar_retain]
    pub fn string_from_date(&self, date: &ns::Date) -> arc::R<ns::String>;

    #[objc::msg_send(dateStyle)]
    pub fn date_style(&self) -> ns::DateFormatterStyle;

    #[objc::msg_send(setDateStyle:)]
    pub fn set_date_style(&mut self, val: ns::DateFormatterStyle);

    #[objc::msg_send(timeStyle)]
    pub fn time_style(&self) -> ns::DateFormatterStyle;

    #[objc::msg_send(setTimeStyle:)]
    pub fn set_time_style(&mut self, val: ns::DateFormatterStyle);

    #[objc::msg_send(locale)]
    pub fn locale_ar(&self) -> arc::Rar<ns::Locale>;

    #[objc::rar_retain]
    pub fn locale(&self) -> arc::R<ns::Locale>;

    #[objc::msg_send(setLocale:)]
    pub fn set_locale(&mut self, val: Option<&ns::Locale>);

    #[objc::msg_send(dateFormat)]
    pub fn date_format_ar(&self) -> arc::Rar<ns::String>;

    #[objc::rar_retain]
    pub fn date_format(&self) -> arc::R<ns::String>;

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
        let mut formatter = ns::DateFormatter::new();
        let str = formatter.string_from_date(&date);
        assert!(str.is_empty());

        let format = formatter.date_format();
        assert!(format.is_empty());

        let locale = ns::Locale::with_locale_id(ns::str!(c"en_GB"));
        formatter.set_locale(Some(&locale));
        formatter.set_time_style(ns::DateFormatterStyle::Short);

        let format_str = formatter.date_format();
        assert_eq!(format_str.to_string(), "HH:mm");
    }
}
