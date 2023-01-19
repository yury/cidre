// we define (or just common?) all selectrs calls here
extern "C" {
    #[link_name = "objc_msgSend$alloc"]
    pub fn alloc();

    #[link_name = "objc_msgSend$init"]
    pub fn init();

    #[link_name = "objc_msgSend$new"]
    pub fn new();

    #[link_name = "objc_msgSend$respondsToSelector:"]
    pub fn responds_to_selector();

    #[link_name = "objc_msgSend$code"]
    pub fn code();

    #[link_name = "objc_msgSend$domain"]
    pub fn domain();

    #[link_name = "objc_msgSend$count"]
    pub fn count();

    #[link_name = "objc_msgSend$length"]
    pub fn length();

    #[link_name = "objc_msgSend$size"]
    pub fn size();

    #[link_name = "objc_msgSend$setSize:"]
    pub fn set_size();

    #[link_name = "objc_msgSend$bytes"]
    pub fn bytes();

    #[link_name = "objc_msgSend$name"]
    pub fn name();

    #[link_name = "objc_msgSend$setName:"]
    pub fn set_name();

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

    #[link_name = "objc_msgSend$isLowPowerModeEnabled"]
    pub fn is_low_power_mode_enabled();

    #[link_name = "objc_msgSend$processorCount"]
    pub fn processor_count();

    #[link_name = "objc_msgSend$activeProcessorCount"]
    pub fn active_processor_count();

    #[link_name = "objc_msgSend$isMacCatalystApp"]
    pub fn is_mac_catalyst_app();

    #[link_name = "objc_msgSend$isiOSAppOnMac"]
    pub fn is_ios_app_on_mac();

    #[link_name = "objc_msgSend$isEqualToString:"]
    pub fn is_equal_to_string();

    #[link_name = "objc_msgSend$isEqual:"]
    pub fn is_equal();

    #[link_name = "objc_msgSend$range"]
    pub fn range();

    #[link_name = "objc_msgSend$resultType"]
    pub fn result_type();

    #[link_name = "objc_msgSend$type"]
    pub fn _type();

    #[link_name = "objc_msgSend$setType:"]
    pub fn set_type();

    #[link_name = "objc_msgSend$substringWithRange:"]
    pub fn substring_with_range();

    #[link_name = "objc_msgSend$cStringUsingEncoding:"]
    pub fn c_string_using_encoding();

    #[link_name = "objc_msgSend$mutableCopy"]
    pub fn mutable_copy();

    #[link_name = "objc_msgSend$memoryCapacity"]
    pub fn memory_capacity();

    #[link_name = "objc_msgSend$setMemoryCapacity:"]
    pub fn set_memory_capacity();

    #[link_name = "objc_msgSend$diskCapacity"]
    pub fn disk_capacity();

    #[link_name = "objc_msgSend$setDiskCapacity:"]
    pub fn set_disk_capacity();

    #[link_name = "objc_msgSend$currentMemoryUsage"]
    pub fn current_memory_usage();

    #[link_name = "objc_msgSend$currentDiskUsage"]
    pub fn current_disk_usage();

    #[link_name = "objc_msgSend$absoluteString"]
    pub fn absolute_string();

    #[link_name = "objc_msgSend$initWithObjects:count:"]
    pub fn init_with_objects_count();

    #[link_name = "objc_msgSend$initWithCapacity:"]
    pub fn init_with_capacity();

    #[link_name = "objc_msgSend$URL"]
    pub fn url();

    #[link_name = "objc_msgSend$setURL:"]
    pub fn set_url();

    #[link_name = "objc_msgSend$width"]
    pub fn width();

    #[link_name = "objc_msgSend$height"]
    pub fn height();

    #[link_name = "objc_msgSend$depth"]
    pub fn depth();

    #[link_name = "objc_msgSend$label"]
    pub fn label();

    #[link_name = "objc_msgSend$setWidth:"]
    pub fn set_width();

    #[link_name = "objc_msgSend$setHeight:"]
    pub fn set_height();

    #[link_name = "objc_msgSend$setDepth:"]
    pub fn set_depth();

    #[link_name = "objc_msgSend$setLabel:"]
    pub fn set_label();

    #[link_name = "objc_msgSend$reset"]
    pub fn reset();

    #[link_name = "objc_msgSend$device"]
    pub fn device();

    #[link_name = "objc_msgSend$frame"]
    pub fn frame();

    #[link_name = "objc_msgSend$bounds"]
    pub fn bounds();

    #[link_name = "objc_msgSend$setFrame:"]
    pub fn set_frame();

    #[link_name = "objc_msgSend$setBounds:"]
    pub fn set_bounds();

    #[link_name = "objc_msgSend$setEnabled:"]
    pub fn set_enabled();

    #[link_name = "objc_msgSend$isEnabled"]
    pub fn is_enabled();

    #[link_name = "objc_msgSend$invalidate"]
    pub fn invalidate();

    #[link_name = "objc_msgSend$isValid"]
    pub fn is_valid();

    #[link_name = "objc_msgSend$delegate"]
    pub fn delegate();

    #[link_name = "objc_msgSend$setDelegate:"]
    pub fn set_delegate();

    #[link_name = "objc_msgSend$processInfo"]
    pub fn process_info();

    #[link_name = "objc_msgSend$sharedSession"]
    pub fn shared_session();

    #[link_name = "objc_msgSend$objectAtIndexedSubscript:"]
    pub fn object_at_indexed_subscript();

    #[link_name = "objc_msgSend$setObject:atIndexedSubscript::"]
    pub fn set_object_at_indexed_subscript();
}