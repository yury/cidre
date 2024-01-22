use crate::{arc, blocks, core_motion as cm, define_obj_type, ns, objc};

define_obj_type!(
    #[doc(alias = "CMPedometerData")]
    pub PedometerData(ns::Id)
);

impl PedometerData {
    #[objc::msg_send(startDate)]
    pub fn start_date(&self) -> &ns::Date;

    #[objc::msg_send(endDate)]
    pub fn end_date(&self) -> &ns::Date;

    #[objc::msg_send(numberOfSteps)]
    pub fn number_of_steps(&self) -> &ns::Number;

    #[objc::msg_send(distance)]
    pub fn distance(&self) -> Option<&ns::Number>;

    #[objc::msg_send(floorsAscended)]
    pub fn floors_ascended(&self) -> Option<&ns::Number>;

    /// Approximate number of floors descended by way of stairs. Value is nil
    /// on unsupported platforms.
    #[objc::msg_send(floorsDescended)]
    pub fn floors_descended(&self) -> Option<&ns::Number>;

    #[objc::msg_send(currentPace)]
    pub fn current_pace(&self) -> Option<&ns::Number>;

    #[objc::msg_send(currentCadence)]
    pub fn current_cadence(&self) -> Option<&ns::Number>;

    #[objc::msg_send(averageActivePace)]
    pub fn avg_active_pace(&self) -> Option<&ns::Number>;
}

/// Events describing the transitions of pedestrian activity.
#[doc(alias = "CMPedometerEventType")]
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
#[repr(isize)]
pub enum PedometerEventType {
    Pause,
    Resume,
}

define_obj_type!(
    #[doc(alias = "CMPedometerEvent")]
    pub PedometerEvent(ns::Id)
);

impl PedometerEvent {
    #[objc::msg_send(date)]
    pub fn date(&self) -> &ns::Date;

    #[objc::msg_send(type)]
    pub fn type_(&self) -> PedometerEventType;
}

define_obj_type!(
    #[doc(alias = "CMPedometer")]
    pub Pedometer(ns::Id),
    CM_PEDOMETER
);

impl Pedometer {
    /// Determines whether the device supports step counting functionality.
    #[objc::cls_msg_send(isStepCountingAvailable)]
    pub fn is_step_counting_available() -> bool;

    /// Determines whether the device supports distance estimation
    /// in addition to step counting.
    #[objc::cls_msg_send(isDistanceAvailable)]
    pub fn is_distance_available() -> bool;

    /// Determines whether the device supports counting flights of stairs
    /// in addition to step counting.
    #[objc::cls_msg_send(isFloorCountingAvailable)]
    pub fn is_floor_counting_available() -> bool;

    /// Determines whether the device supports pace estimation
    /// in addition to step counting.
    #[objc::cls_msg_send(isPaceAvailable)]
    pub fn is_pace_available() -> bool;

    #[cfg(not(target_os = "macos"))]
    #[objc::cls_msg_send(isCadenceAvailable)]
    pub fn is_cadence_available() -> bool;

    #[cfg(not(target_os = "macos"))]
    #[objc::cls_msg_send(isPedometerEventTrackingAvailable)]
    pub fn is_pedometer_event_tracking_available() -> bool;

    #[cfg(any(target_os = "ios", target_os = "watchos"))]
    #[objc::cls_msg_send(authorizationStatus)]
    pub fn authorization_status() -> cm::AuthorizationStatus;

    #[objc::msg_send(queryPedometerDataFromDate:toDate:withHandler:)]
    pub unsafe fn _query_pedometer_data_ch(
        &self,
        start: &ns::Date,
        end: &ns::Date,
        handler: *mut std::ffi::c_void,
    );

    #[inline]
    pub fn query_pedometer_data_ch_block<'a, F>(
        &self,
        start: &ns::Date,
        end: &ns::Date,
        handler: &'static mut blocks::Block<F>,
    ) where
        F: FnOnce(Option<&'a cm::PedometerData>, Option<&'a ns::Error>),
    {
        unsafe { self._query_pedometer_data_ch(start, end, handler.as_mut_ptr()) }
    }

    #[inline]
    pub fn query_pedometer_data_ch<F>(&self, start: &ns::Date, end: &ns::Date, handler: F)
    where
        F: FnOnce(Option<&cm::PedometerData>, Option<&ns::Error>) + 'static,
    {
        let handler = blocks::once2(handler);
        self.query_pedometer_data_ch_block(start, end, handler.escape())
    }

    #[cfg(feature = "async")]
    pub async fn query_pedometer_data(
        &self,
        start: &ns::Date,
        end: &ns::Date,
    ) -> Result<arc::R<cm::PedometerData>, arc::R<ns::Error>> {
        let (future, block) = blocks::result();
        self.query_pedometer_data_ch_block(start, end, block.escape());
        future.await
    }

    #[objc::msg_send(startPedometerUpdatesFromDate:withHandler:)]
    pub unsafe fn _start_pedometer_updates_from_date_handler(
        &mut self,
        start: &ns::Date,
        handler: *mut std::ffi::c_void,
    );

    #[inline]
    pub fn start_pedometer_updates_from_date_handler<'a, F>(
        &mut self,
        start: &ns::Date,
        handler: &'static mut blocks::Block<F>,
    ) where
        F: FnMut(Option<&'a cm::PedometerData>, Option<&'a ns::Error>),
    {
        unsafe { self._start_pedometer_updates_from_date_handler(start, handler.as_mut_ptr()) }
    }

    #[inline]
    pub fn start_pedometer_updates_from_date<F>(&mut self, start: &ns::Date, handler: F)
    where
        F: FnMut(Option<&cm::PedometerData>, Option<&ns::Error>) + 'static,
    {
        let mut handler = blocks::mut2(handler);
        self.start_pedometer_updates_from_date_handler(start, handler.escape())
    }

    #[objc::msg_send(stopPedometerUpdates)]
    pub fn stop_pedometer_updates(&mut self);
}

#[link(name = "core_motion", kind = "static")]
extern "C" {
    static CM_PEDOMETER: &'static objc::Class<Pedometer>;
}

#[cfg(test)]
mod tests {
    use crate::core_motion as cm;

    #[test]
    fn basics() {
        assert_eq!(cm::Pedometer::is_step_counting_available(), false);
        assert_eq!(cm::Pedometer::is_distance_available(), false);
        assert_eq!(cm::Pedometer::is_floor_counting_available(), false);
        assert_eq!(cm::Pedometer::is_pace_available(), false);
    }

    // #[tokio::test]
    // async fn asynchronious() {
    //     let pedomemer = cm::Pedometer::new();
    //     let start = ns::Date::new();
    //     let end = ns::Date::new();
    //     let err = pedomemer
    //         .query_pedometer_data(&start, &end)
    //         .await
    //         .expect_err("should be err");
    // }
}
