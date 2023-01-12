extern "C" {
    #[link_name = "objc_msgSend$count"]
    pub fn count();

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

}
