// we define (or just common?) all selectrs calls here
extern "C" {
    #[link_name = "objc_msgSend$count"]
    pub fn count();

    #[link_name = "objc_msgSend$length"]
    pub fn length();

    #[link_name = "objc_msgSend$bytes"]
    pub fn bytes();

    #[link_name = "objc_msgSend$lowercaseString"]
    pub fn lowercase_string();

    #[link_name = "objc_msgSend$intValue"]
    pub fn int_value();

    #[link_name = "objc_msgSend$unsignedIntValue"]
    pub fn unsingned_int_value();

    #[link_name = "objc_msgSend$longLongValue"]
    pub fn long_long_value();

    #[link_name = "objc_msgSend$unsignedLongLongValue"]
    pub fn unsigned_long_long_value();

    #[link_name = "objc_msgSend$charValue"]
    pub fn char_value();

    #[link_name = "objc_msgSend$unsignedCharValue"]
    pub fn unsigned_char_value();

    #[link_name = "objc_msgSend$shortValue"]
    pub fn short_value();

    #[link_name = "objc_msgSend$unsignedShortValue"]
    pub fn unsigned_short_value();

    #[link_name = "objc_msgSend$floatValue"]
    pub fn float_value();

    #[link_name = "objc_msgSend$doubleValue"]
    pub fn double_value();

    #[link_name = "objc_msgSend$boolValue"]
    pub fn bool_value();

    #[link_name = "objc_msgSend$integerValue"]
    pub fn integer_value();

    #[link_name = "objc_msgSend$unsignedIntegerValue"]
    pub fn unsigned_integer_value();

    #[link_name = "objc_msgSend$stringValue"]
    pub fn string_value();

    #[link_name = "objc_msgSend$description"]
    pub fn description();

    #[link_name = "objc_msgSend$debugDescription"]
    pub fn debug_description();

    #[link_name = "objc_msgSend$objectAtIndex:"]
    pub fn object_at_index();

    #[link_name = "objc_msgSend$thermalState"]
    pub fn thermal_state();

    #[link_name = "objc_msgSend$countByEnumeratingWithState:objects:count:"]
    pub fn count_by_enumerating_with_state_objects_count();

    #[link_name = "objc_msgSend$UTF8String"]
    pub fn utf8_string();

    #[link_name = "objc_msgSend$isEqualToString:"]
    pub fn is_equal_to_string();

    #[link_name = "objc_msgSend$mutableCopy"]
    pub fn mutable_copy();

}
