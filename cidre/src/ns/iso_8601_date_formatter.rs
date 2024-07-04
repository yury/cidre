use crate::{arc, cf, define_obj_type, define_opts, ns, objc};

define_opts!(
    #[doc(alias = "NSISO8601DateFormatOptions")]
    pub Iso8601DateFormatOpts(usize)
);

impl Iso8601DateFormatOpts {
    pub const WITH_YEAR: Self = Self(cf::Iso8601DateFormatOpts::WITH_YEAR.0);
    pub const WITH_MONTH: Self = Self(cf::Iso8601DateFormatOpts::WITH_MONTH.0);
    pub const WITH_WEEK_OF_YEAR: Self = Self(cf::Iso8601DateFormatOpts::WITH_WEEK_OF_YEAR.0);
    pub const WITH_DAY: Self = Self(cf::Iso8601DateFormatOpts::WITH_DAY.0);
    pub const WITH_TIME: Self = Self(cf::Iso8601DateFormatOpts::WITH_TIME.0);
    pub const WITH_TIME_ZONE: Self = Self(cf::Iso8601DateFormatOpts::WITH_TIME_ZONE.0);
    pub const WITH_SPACE_BETWEEN_DATE_AND_TIME: Self =
        Self(cf::Iso8601DateFormatOpts::WITH_SPACE_BETWEEN_DATE_AND_TIME.0);

    pub const WITH_DASH_SEPARATOR_IN_DATE: Self =
        Self(cf::Iso8601DateFormatOpts::WITH_DASH_SEPARATOR_IN_DATE.0);

    pub const WITH_COLON_SEPARATOR_IN_TIME: Self =
        Self(cf::Iso8601DateFormatOpts::WITH_COLON_SEPARATOR_IN_TIME.0);

    pub const WITH_COLON_SEPARATOR_IN_TIME_ZONE: Self =
        Self(cf::Iso8601DateFormatOpts::WITH_COLON_SEPARATOR_IN_TIME_ZONE.0);

    pub const WITH_FRACTIONAL_SECONDS: Self =
        Self(cf::Iso8601DateFormatOpts::WITH_FRACTIONAL_SECONDS.0);

    pub const WITH_FULL_DATE: Self = Self(cf::Iso8601DateFormatOpts::WITH_FULL_DATE.0);
    pub const WITH_FULL_TIME: Self = Self(cf::Iso8601DateFormatOpts::WITH_FULL_TIME.0);
    pub const WITH_INTERNET_DATE_TIME: Self =
        Self(cf::Iso8601DateFormatOpts::WITH_INTERNET_DATE_TIME.0);
}

define_obj_type!(
    #[doc(alias = "NSISO8601DateFormatter")]
    pub Iso8601DateFormatter(ns::Formatter),
    NS_ISO_8601_DATE_FORMATTER
);

impl Iso8601DateFormatter {
    #[objc::msg_send(stringFromDate:)]
    pub fn string_from_date(&self, date: &ns::Date) -> arc::R<ns::String>;

    #[objc::msg_send(formatOptions)]
    pub fn format_opts(&self) -> ns::Iso8601DateFormatOpts;

    #[objc::msg_send(setFormatOptions:)]
    pub fn set_format_opts(&mut self, val: ns::Iso8601DateFormatOpts);
}

#[link(name = "ns", kind = "static")]
extern "C" {
    static NS_ISO_8601_DATE_FORMATTER: &'static objc::Class<Iso8601DateFormatter>;
}

#[cfg(test)]
mod tests {
    use crate::{cf, ns};

    #[test]
    fn basics() {
        let date = ns::Date::new();
        let formatter = ns::Iso8601DateFormatter::new();
        formatter.as_type_ref().show();
        let str = formatter.string_from_date(&date);
        assert!(!str.is_empty());

        assert_eq!(
            formatter.format_opts().0,
            cf::Iso8601DateFormatOpts::WITH_INTERNET_DATE_TIME.0
        );
    }
}
