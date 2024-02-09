use crate::{arc, cf, define_cf_type, define_opts};

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
#[repr(isize)]
pub enum DateFormatterStyle {
    #[doc(alias = "kCFDateFormatterNoStyle")]
    No = 0,

    #[doc(alias = "kCFDateFormatterShortStyle")]
    Short = 1,

    #[doc(alias = "kCFDateFormatterMediumStyle")]
    Medium = 2,

    #[doc(alias = "kCFDateFormatterLongStyle")]
    Long = 3,

    #[doc(alias = "kCFDateFormatterFullStyle")]
    Full = 4,
}

define_opts!(
    #[doc(alias = "CFISO8601DateFormatOptions")]
    pub Iso8601DateFormatOpts(usize)
);

impl Iso8601DateFormatOpts {
    #[doc(alias = "kCFISO8601DateFormatWithYear")]
    pub const WITH_YEAR: Self = Self(1 << 0);

    #[doc(alias = "kCFISO8601DateFormatWithMonth")]
    pub const WITH_MONTH: Self = Self(1 << 1);

    #[doc(alias = "kCFISO8601DateFormatWithWeekOfYear")]
    pub const WITH_WEEK_OF_YEAR: Self = Self(1 << 2);

    #[doc(alias = "kCFISO8601DateFormatWithDay")]
    pub const WITH_DAY: Self = Self(1 << 4);

    #[doc(alias = "kCFISO8601DateFormatWithTime")]
    pub const WITH_TIME: Self = Self(1 << 5);

    #[doc(alias = "kCFISO8601DateFormatWithTimeZone")]
    pub const WITH_TIME_ZONE: Self = Self(1 << 6);

    #[doc(alias = "kCFISO8601DateFormatWithSpaceBetweenDateAndTime")]
    pub const WITH_SPACE_BETWEEN_DATE_AND_TIME: Self = Self(1 << 7);

    #[doc(alias = "kCFISO8601DateFormatWithDashSeparatorInDate")]
    pub const WITH_DASH_SEPARATOR_IN_DATE: Self = Self(1 << 8);

    #[doc(alias = "kCFISO8601DateFormatWithColonSeparatorInTime")]
    pub const WITH_COLON_SEPARATOR_IN_TIME: Self = Self(1 << 9);

    #[doc(alias = "kCFISO8601DateFormatWithColonSeparatorInTimeZone")]
    pub const WITH_COLON_SEPARATOR_IN_TIME_ZONE: Self = Self(1 << 10);

    #[doc(alias = "kCFISO8601DateFormatWithFractionalSeconds")]
    pub const WITH_FRACTIONAL_SECONDS: Self = Self(1 << 11);

    #[doc(alias = "kCFISO8601DateFormatWithFullDate")]
    pub const WITH_FULL_DATE: Self = Self(
        Self::WITH_YEAR.0
            | Self::WITH_MONTH.0
            | Self::WITH_DAY.0
            | Self::WITH_DASH_SEPARATOR_IN_DATE.0,
    );

    #[doc(alias = "kCFISO8601DateFormatWithFullTime")]
    pub const WITH_FULL_TIME: Self = Self(
        Self::WITH_TIME.0
            | Self::WITH_COLON_SEPARATOR_IN_TIME.0
            | Self::WITH_TIME_ZONE.0
            | Self::WITH_COLON_SEPARATOR_IN_TIME_ZONE.0,
    );

    #[doc(alias = "kCFISO8601DateFormatWithInternetDateTime")]
    pub const WITH_INTERNET_DATE_TIME: Self = Self(Self::WITH_FULL_DATE.0 | Self::WITH_FULL_TIME.0);
}

define_cf_type!(
    #[doc(alias = "CFDateFormatter")]
    DateFormatter(cf::Type)
);

impl DateFormatter {
    #[inline]
    pub fn type_id() -> cf::TypeId {
        unsafe { CFDateFormatterGetTypeID() }
    }

    #[inline]
    pub fn new_iso_8601_with_opts(format_options: Iso8601DateFormatOpts) -> arc::R<Self> {
        unsafe { std::mem::transmute(CFDateFormatterCreateISO8601Formatter(None, format_options)) }
    }

    #[inline]
    pub fn new_iso_8601() -> arc::R<Self> {
        unsafe {
            std::mem::transmute(CFDateFormatterCreateISO8601Formatter(
                None,
                Iso8601DateFormatOpts::WITH_INTERNET_DATE_TIME,
            ))
        }
    }

    #[inline]
    pub fn create_iso_8601_in(
        format_options: Iso8601DateFormatOpts,
        allocator: Option<&cf::Allocator>,
    ) -> Option<arc::R<Self>> {
        unsafe { CFDateFormatterCreateISO8601Formatter(allocator, format_options) }
    }

    #[inline]
    pub fn locale(&self) -> &cf::Locale {
        unsafe { CFDateFormatterGetLocale(self) }
    }

    #[inline]
    pub fn date_style(&self) -> cf::DateFormatterStyle {
        unsafe { CFDateFormatterGetDateStyle(self) }
    }

    #[inline]
    pub fn time_style(&self) -> cf::DateFormatterStyle {
        unsafe { CFDateFormatterGetTimeStyle(self) }
    }

    #[inline]
    pub fn format(&self) -> &cf::String {
        unsafe { CFDateFormatterGetFormat(self) }
    }

    #[inline]
    pub fn set_format(&mut self, val: &cf::String) {
        unsafe { CFDateFormatterSetFormat(self, val) }
    }

    #[inline]
    pub fn string_from_date_in(
        &self,
        date: &cf::Date,
        allocator: Option<&cf::Allocator>,
    ) -> Option<arc::R<cf::String>> {
        unsafe { CFDateFormatterCreateStringWithDate(allocator, self, date) }
    }

    #[inline]
    pub fn string_from_date(&self, date: &cf::Date) -> arc::R<cf::String> {
        unsafe { std::mem::transmute(CFDateFormatterCreateStringWithDate(None, self, date)) }
    }

    #[inline]
    pub fn string_from_abs_time_in(
        &self,
        at: cf::AbsTime,
        allocator: Option<&cf::Allocator>,
    ) -> Option<arc::R<cf::String>> {
        unsafe { CFDateFormatterCreateStringWithAbsoluteTime(allocator, self, at) }
    }

    #[inline]
    pub fn string_from_abs_time(&self, at: cf::AbsTime) -> arc::R<cf::String> {
        unsafe { std::mem::transmute(CFDateFormatterCreateStringWithAbsoluteTime(None, self, at)) }
    }
}

#[link(name = "CoreFoundation", kind = "framework")]
extern "C" {
    fn CFDateFormatterGetTypeID() -> cf::TypeId;
    fn CFDateFormatterCreateISO8601Formatter(
        allocator: Option<&cf::Allocator>,
        format_options: Iso8601DateFormatOpts,
    ) -> Option<arc::R<DateFormatter>>;

    fn CFDateFormatterGetLocale(formatter: &cf::DateFormatter) -> &cf::Locale;
    fn CFDateFormatterGetDateStyle(formatter: &cf::DateFormatter) -> cf::DateFormatterStyle;
    fn CFDateFormatterGetTimeStyle(formatter: &cf::DateFormatter) -> cf::DateFormatterStyle;
    fn CFDateFormatterGetFormat(formatter: &cf::DateFormatter) -> &cf::String;
    fn CFDateFormatterSetFormat(formatter: &mut cf::DateFormatter, format_string: &cf::String);
    fn CFDateFormatterCreateStringWithDate(
        allocator: Option<&cf::Allocator>,
        formatter: &cf::DateFormatter,
        date: &cf::Date,
    ) -> Option<arc::R<cf::String>>;
    fn CFDateFormatterCreateStringWithAbsoluteTime(
        allocator: Option<&cf::Allocator>,
        formatter: &cf::DateFormatter,
        at: cf::AbsTime,
    ) -> Option<arc::R<cf::String>>;
}

#[cfg(test)]
mod test {
    use crate::cf;

    #[test]
    fn basics() {
        let formatter = cf::DateFormatter::new_iso_8601_with_opts(Default::default());
        let _locale = formatter.locale();
        let date_style = formatter.date_style();
        assert_eq!(cf::DateFormatterStyle::No, date_style);
        let time_style = formatter.time_style();
        assert_eq!(cf::DateFormatterStyle::No, time_style);
        let format = formatter.format();
        assert!(format.is_empty());

        let date = cf::Date::current();
        let string = formatter.string_from_date(&date);
        assert!(string.is_empty());

        let formatter = cf::DateFormatter::new_iso_8601();

        let _locale = formatter.locale();
        let date_style = formatter.date_style();
        assert_eq!(cf::DateFormatterStyle::No, date_style);
        let time_style = formatter.time_style();
        assert_eq!(cf::DateFormatterStyle::No, time_style);
        let format = formatter.format();
        assert_eq!(format, "yyyy-MM-dd'T'HH:mm:ssXXXXX");

        let date = cf::Date::new_at(0.0);
        let string = formatter.string_from_date(&date);
        assert_eq!(string.as_ref(), "2001-01-01T03:00:00+03:00");

        let string = formatter.string_from_abs_time(0.0);
        assert_eq!(string.as_ref(), "2001-01-01T03:00:00+03:00");
    }
}
