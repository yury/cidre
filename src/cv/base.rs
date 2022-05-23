
#[derive(Debug)]
#[repr(C)]
pub struct Time {
    pub time_value: i64,
    pub time_scale: i32,
    pub flags: i32
}


#[derive(Debug, Default)]
#[repr(C)]
pub struct SMPTETime {
    pub subframes: i16,
    pub subframes_divisor: i16,
    pub counter: u32,
    pub r#type: u32,
    pub flags: u32,
    pub hours: i16,
    pub minutes: i16,
    pub seconds: i16,
    pub frames: i16
}

#[derive(Debug, Default)]
#[repr(C)]
pub struct TimeStamp {
    pub version: u32,
    pub video_time_scale: i32,
    pub video_time: i64,
    pub host_time: u64,
    pub rate_scalar: f64,
    pub video_refresh_period: i64,
    pub smpte_time: SMPTETime,
    pub flags: u64,
    pub reserved: u64,
}