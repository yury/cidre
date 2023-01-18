use std::os::unix::prelude::RawFd;

use super::Error;

pub fn add_file_descriptor(fd: RawFd) -> Error {
    unsafe { AMDAddLogFileDescriptor(fd) }
}

pub fn remove_file_descriptor(fd: RawFd) -> Error {
    unsafe { AMDRemoveLogFileDescriptor(fd) }
}

pub fn set_level(level: i32) {
    unsafe { AMDSetLogLevel(level) }
}

extern "C" {
    fn AMDAddLogFileDescriptor(fd: RawFd) -> Error;
    fn AMDRemoveLogFileDescriptor(fd: RawFd) -> Error;
    fn AMDSetLogLevel(level: i32);
}
