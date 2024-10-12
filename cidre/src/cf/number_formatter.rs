use crate::{arc, cf, define_cf_type};

#[doc(alias = "CFNumberFormatterStyle")]
#[derive(Debug, Eq, PartialEq, Copy, Clone)]
#[repr(isize)]
pub enum FormatterStyle {
    #[doc(alias = "kCFNumberFormatterNoStyle")]
    No = 0,

    #[doc(alias = "kCFNumberFormatterDecimalStyle")]
    Decimal = 1,

    #[doc(alias = "kCFNumberFormatterCurrencyStyle")]
    Currency = 2,

    #[doc(alias = "kCFNumberFormatterPercentStyle")]
    Percent = 3,

    #[doc(alias = "kCFNumberFormatterScientificStyle")]
    Scientific = 4,

    #[doc(alias = "kCFNumberFormatterSpellOutStyle")]
    SpellOut = 5,

    #[doc(alias = "kCFNumberFormatterOrdinalStyle")]
    Ordinal = 6,

    #[doc(alias = "kCFNumberFormatterCurrencyISOCodeStyle")]
    CurrencyIsoCode = 8,

    #[doc(alias = "kCFNumberFormatterCurrencyPluralStyle")]
    CurrencyPlural = 9,

    #[doc(alias = "kCFNumberFormatterCurrencyAccountingStyle")]
    CurrencyAccounting = 10,
}

define_cf_type!(
    #[doc(alias = "CFNumberFormatter")]
    NumberFormatter(cf::Type)
);

impl NumberFormatter {
    #[inline]
    pub fn type_id() -> cf::TypeId {
        unsafe { CFNumberFormatterGetTypeID() }
    }

    #[inline]
    pub fn new_in(
        locale: Option<&cf::Locale>,
        style: cf::NumberFormatterStyle,
        allocator: Option<&cf::Allocator>,
    ) -> Option<arc::R<Self>> {
        unsafe { CFNumberFormatterCreate(allocator, locale, style) }
    }

    #[inline]
    pub fn new(locale: Option<&cf::Locale>, style: cf::NumberFormatterStyle) -> arc::R<Self> {
        unsafe { std::mem::transmute(CFNumberFormatterCreate(None, locale, style)) }
    }

    #[inline]
    pub fn locale(&self) -> &cf::Locale {
        unsafe { CFNumberFormatterGetLocale(self) }
    }

    #[inline]
    pub fn style(&self) -> cf::NumberFormatterStyle {
        unsafe { CFNumberFormatterGetStyle(self) }
    }

    #[inline]
    pub fn format(&self) -> &cf::String {
        unsafe { CFNumberFormatterGetFormat(self) }
    }

    #[inline]
    pub fn set_format(&mut self, val: &cf::String) {
        unsafe { CFNumberFormatterSetFormat(self, val) }
    }

    #[inline]
    pub fn string_from_number_in(
        &self,
        number: &cf::Number,
        allocator: Option<&cf::Allocator>,
    ) -> Option<arc::R<cf::String>> {
        unsafe { CFNumberFormatterCreateStringWithNumber(allocator, self, number) }
    }

    #[inline]
    pub fn string_from_number(&self, number: &cf::Number) -> arc::R<cf::String> {
        unsafe { std::mem::transmute(CFNumberFormatterCreateStringWithNumber(None, self, number)) }
    }
}

#[link(name = "CoreFoundation", kind = "framework")]
extern "C-unwind" {
    fn CFNumberFormatterGetTypeID() -> cf::TypeId;
    fn CFNumberFormatterCreate(
        allocator: Option<&cf::Allocator>,
        locale: Option<&cf::Locale>,
        style: cf::NumberFormatterStyle,
    ) -> Option<arc::R<NumberFormatter>>;
    fn CFNumberFormatterGetLocale(formatter: &cf::NumberFormatter) -> &cf::Locale;
    fn CFNumberFormatterGetStyle(formatter: &cf::NumberFormatter) -> cf::NumberFormatterStyle;
    fn CFNumberFormatterGetFormat(formatter: &cf::NumberFormatter) -> &cf::String;
    fn CFNumberFormatterSetFormat(formatter: &mut cf::NumberFormatter, val: &cf::String);
    fn CFNumberFormatterCreateStringWithNumber(
        allocator: Option<&cf::Allocator>,
        formatter: &cf::NumberFormatter,
        number: &cf::Number,
    ) -> Option<arc::R<cf::String>>;
}

#[cfg(test)]
mod tests {
    use crate::cf;

    #[test]
    fn basics() {
        let num = cf::Number::tagged_i8(100);
        let formatter = cf::NumberFormatter::new(None, cf::NumberFormatterStyle::No);
        let str = formatter.string_from_number(&num);
        assert_eq!("100".to_string(), str.to_string());
    }
}
