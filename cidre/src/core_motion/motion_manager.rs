use crate::{arc, blocks, core_motion as cm, define_obj_type, ns, objc};

define_obj_type!(
    #[doc(alias = "CMMotionManager")]
    pub MotionManager(ns::Id),
    CM_MOTION_MANAGER
);

impl MotionManager {
    #[objc::msg_send(accelerometerUpdateInterval)]
    pub fn accelerometer_update_interval(&self) -> ns::TimeInterval;

    #[objc::msg_send(setAccelerometerUpdateInterval:)]
    pub fn set_accelerometer_update_interval(&mut self, val: ns::TimeInterval);

    #[objc::msg_send(isAccelerometerAvailable)]
    pub fn is_accelerometer_available(&self) -> bool;

    #[objc::msg_send(isAccelerometerActive)]
    pub fn is_accelerometer_active(&self) -> bool;

    #[objc::msg_send(accelerometerData)]
    pub fn accelerometer_data(&self) -> Option<&cm::AccelerometerData>;

    #[objc::msg_send(startAccelerometerUpdates)]
    pub fn start_accelerometer_updates(&mut self);

    #[objc::msg_send(startAccelerometerUpdatesToQueue:withHandler:)]
    pub fn start_accelerometer_updates_to_queue_handler(
        &mut self,
        queue: &ns::OpQueue,
        handler: &mut blocks::ResultCompletionHandler<cm::AccelerometerData>,
    );

    #[inline]
    pub fn start_accelerometer_updates_to_queue(
        &mut self,
        queue: &ns::OpQueue,
        handler: impl FnMut(Option<&cm::AccelerometerData>, Option<&ns::Error>)
            + 'static
            + std::marker::Sync,
    ) {
        let mut handler = blocks::ResultCompletionHandler::new2(handler);
        self.start_accelerometer_updates_to_queue_handler(queue, &mut handler)
    }

    #[objc::msg_send(stopAccelerometerUpdates)]
    pub fn stop_accelerometer_updates(&mut self);

    #[objc::msg_send(gyroUpdateInterval)]
    pub fn gyro_update_interval(&self) -> ns::TimeInterval;

    #[objc::msg_send(setGyroUpdateInterval:)]
    pub fn set_gyro_update_interval(&mut self, val: ns::TimeInterval);

    #[objc::msg_send(isGyroAvailable)]
    pub fn is_gyro_available(&self) -> bool;

    #[objc::msg_send(isGyroActive)]
    pub fn is_gyro_active(&self) -> bool;

    #[objc::msg_send(gyroData)]
    pub fn gyro_data(&self) -> Option<&cm::GyroData>;

    #[objc::msg_send(startGyroUpdates)]
    pub fn start_gyro_updates(&mut self);

    #[objc::msg_send(startGyroUpdatesToQueue:withHandler:)]
    pub fn start_gyro_updates_to_queue_handler(
        &mut self,
        queue: &ns::OpQueue,
        handler: &mut blocks::ResultCompletionHandler<cm::GyroData>,
    );

    #[inline]
    pub fn start_gyro_updates_to_queue(
        &mut self,
        queue: &ns::OpQueue,
        handler: impl FnMut(Option<&cm::GyroData>, Option<&ns::Error>) + 'static + std::marker::Sync,
    ) {
        let mut handler = blocks::ResultCompletionHandler::new2(handler);
        self.start_gyro_updates_to_queue_handler(queue, &mut handler)
    }

    #[objc::msg_send(stopGyroUpdates)]
    pub fn stop_gyro_updates(&mut self);

    #[objc::msg_send(magnetometerUpdateInterval)]
    pub fn magnetometer_update_interval(&self) -> ns::TimeInterval;

    #[objc::msg_send(setMagnetometerUpdateInterval:)]
    pub fn set_magnetometer_update_interval(&mut self, val: ns::TimeInterval);

    #[objc::msg_send(isMagnetometerAvailable)]
    pub fn is_magnetometer_available(&self) -> bool;

    #[objc::msg_send(isMagnetometerActive)]
    pub fn is_magnetometer_active(&self) -> bool;

    #[objc::msg_send(magnetometerData)]
    pub fn magnetometer_data(&self) -> Option<&cm::MagnetometerData>;

    #[objc::msg_send(startMagnetometerUpdates)]
    pub fn start_magnetometer_updates(&mut self);

    #[objc::msg_send(startMagnetometerUpdatesToQueue:withHandler:)]
    pub fn start_magnetometer_updates_to_queue_handler(
        &mut self,
        queue: &ns::OpQueue,
        handler: &mut blocks::ResultCompletionHandler<cm::MagnetometerData>,
    );

    #[inline]
    pub fn start_magnetometer_updates_to_queue(
        &mut self,
        queue: &ns::OpQueue,
        handler: impl FnMut(Option<&cm::MagnetometerData>, Option<&ns::Error>)
            + 'static
            + std::marker::Sync,
    ) {
        let mut handler = blocks::ResultCompletionHandler::new2(handler);
        self.start_magnetometer_updates_to_queue_handler(queue, &mut handler)
    }

    #[objc::msg_send(stopMagnetometerUpdates)]
    pub fn stop_magnetometer_updates(&mut self);

    #[objc::msg_send(deviceMotionUpdateInterval)]
    pub fn device_motion_update_interval(&self) -> ns::TimeInterval;

    #[objc::msg_send(setDeviceMotionUpdateInterval:)]
    pub fn set_device_motion_update_interval(&mut self, val: ns::TimeInterval);

    #[objc::cls_msg_send(availableAttitudeReferenceFrames)]
    pub fn available_attitude_ref_frames() -> cm::AttitudeRefFrame;

    #[objc::msg_send(attitudeReferenceFrame)]
    pub fn attitude_ref_frame(&self) -> &cm::AttitudeRefFrame;

    #[objc::msg_send(isDeviceMotionAvailable)]
    pub fn is_device_motion_available(&self) -> bool;

    #[objc::msg_send(isDeviceMotionActive)]
    pub fn is_device_motion_active(&self) -> bool;

    #[objc::msg_send(deviceMotion)]
    pub fn device_motion(&self) -> Option<&cm::DeviceMotion>;

    #[objc::msg_send(startDeviceMotionUpdates)]
    pub fn start_device_motion_updates(&mut self);

    #[objc::msg_send(startDeviceMotionUpdatesToQueue:withHandler:)]
    pub fn start_device_motion_updates_to_queue_handler(
        &mut self,
        queue: &ns::OpQueue,
        handler: &blocks::ResultCompletionHandler<cm::DeviceMotion>,
    );

    #[inline]
    pub fn start_device_motion_updates_to_queue(
        &mut self,
        queue: &ns::OpQueue,
        handler: impl FnMut(Option<&cm::DeviceMotion>, Option<&ns::Error>) + 'static,
    ) {
        let mut handler = blocks::ResultCompletionHandler::new2(handler);
        self.start_device_motion_updates_to_queue_handler(queue, &mut handler)
    }

    #[objc::msg_send(stopDeviceMotionUpdates)]
    pub fn stop_device_motion_updates(&mut self);

    #[objc::msg_send(startDeviceMotionUpdatesUsingReferenceFrame:)]
    pub fn start_device_motion_updates_using_ref_frame(&mut self, ref_frame: &cm::AttitudeRefFrame);

    #[objc::msg_send(startDeviceMotionUpdatesUsingReferenceFrame:toQueue:withHandler:)]
    pub fn start_device_motion_updates_using_ref_frame_to_queue_handler(
        &mut self,
        ref_frame: &cm::AttitudeRefFrame,
        queue: &ns::OpQueue,
        handler: &mut blocks::ResultCompletionHandler<cm::DeviceMotion>,
    );

    pub fn start_device_motion_updates_using_ref_frame_to_queue(
        &mut self,
        ref_frame: &cm::AttitudeRefFrame,
        queue: &ns::OpQueue,
        handler: impl FnMut(Option<&cm::DeviceMotion>, Option<&ns::Error>) + 'static + std::marker::Sync,
    ) {
        let mut handler = blocks::ResultCompletionHandler::new2(handler);
        self.start_device_motion_updates_using_ref_frame_to_queue_handler(
            ref_frame,
            queue,
            &mut handler,
        )
    }
}

#[link(name = "core_motion", kind = "static")]
extern "C" {
    static CM_MOTION_MANAGER: &'static objc::Class<MotionManager>;
}
