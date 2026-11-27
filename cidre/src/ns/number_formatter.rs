use crate::{arc, define_obj_type, ns, objc};

define_obj_type!(
    #[doc(alias = "NSNumberFormatter")]
    pub NumberFormatter(ns::Formatter),
    sym NSNumberFormatter
);

unsafe impl Send for NumberFormatter {}
unsafe impl Sync for NumberFormatter {}

#[doc(alias = "NSNumberFormatterStyle")]
#[derive(Debug, Eq, PartialEq, Copy, Clone)]
#[repr(usize)]
pub enum NumberFormatterStyle {
    #[doc(alias = "NSNumberFormatterNoStyle")]
    No = 0,
    #[doc(alias = "NSNumberFormatterDecimalStyle")]
    Decimal = 1,
    #[doc(alias = "NSNumberFormatterCurrencyStyle")]
    Currency = 2,
    #[doc(alias = "NSNumberFormatterPercentStyle")]
    Percent = 3,
    #[doc(alias = "NSNumberFormatterScientificStyle")]
    Scientific = 4,
    #[doc(alias = "NSNumberFormatterSpellOutStyle")]
    SpellOut = 5,
    #[doc(alias = "NSNumberFormatterOrdinalStyle")]
    Ordinal = 6,
    #[doc(alias = "NSNumberFormatterCurrencyISOCodeStyle")]
    CurrencyIsoCode = 8,
    #[doc(alias = "NSNumberFormatterCurrencyPluralStyle")]
    CurrencyPlural = 9,
    #[doc(alias = "NSNumberFormatterCurrencyAccountingStyle")]
    CurrencyAccounting = 10,
}

impl NumberFormatter {
    #[objc::msg_send(stringFromNumber:)]
    pub fn string_from_number(&self, number: &ns::Number) -> Option<arc::R<ns::String>>;

    #[objc::msg_send(numberStyle)]
    pub fn number_style(&self) -> ns::NumberFormatterStyle;

    #[objc::msg_send(setNumberStyle:)]
    pub fn set_number_style(&mut self, val: ns::NumberFormatterStyle);

    #[objc::msg_send(locale)]
    pub fn locale(&self) -> arc::R<ns::Locale>;

    #[objc::msg_send(setLocale:)]
    pub fn set_locale(&mut self, val: Option<&ns::Locale>);

    #[objc::msg_send(currencyCode)]
    pub fn currency_code(&self) -> arc::R<ns::String>;

    #[objc::msg_send(setCurrencyCode:)]
    pub fn set_currency_code(&mut self, val: Option<&ns::String>);

    #[objc::msg_send(minimumFractionDigits)]
    pub fn min_fraction_digits(&self) -> usize;

    #[objc::msg_send(setMinimumFractionDigits:)]
    pub fn set_min_fraction_digits(&mut self, val: usize);

    #[objc::msg_send(maximumFractionDigits)]
    pub fn max_fraction_digits(&self) -> usize;

    #[objc::msg_send(setMaximumFractionDigits:)]
    pub fn set_max_fraction_digits(&mut self, val: usize);
}

#[cfg(test)]
mod tests {
    use crate::ns;

    #[test]
    fn basics() {
        let mut formatter = ns::NumberFormatter::new();
        formatter.set_locale(Some(&ns::Locale::with_locale_id(ns::str!(c"en_US"))));
        formatter.set_number_style(ns::NumberFormatterStyle::Decimal);
        assert_eq!(formatter.number_style(), ns::NumberFormatterStyle::Decimal);
        let s = formatter.string_from_number(&ns::Number::with_u32(1204)).unwrap();
        assert_eq!(s.to_string(), "1,204");

        formatter.set_number_style(ns::NumberFormatterStyle::Currency);
        formatter.set_currency_code(Some(ns::str!(c"USD")));
        let s = formatter.string_from_number(&ns::Number::with_f64(25.)).unwrap();
        assert_eq!(s.to_string(), "$25.00");
    }
}
