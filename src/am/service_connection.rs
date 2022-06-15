use std::os::unix::prelude::RawFd;

pub use crate::am::device::base::ServiceConnection;

#[derive(Debug)]
pub struct InvalidSocketError;

impl ServiceConnection {
    pub fn socket(&self) -> Result<RawFd, InvalidSocketError> {
        unsafe {
            let res = AMDServiceConnectionGetSocket(self);
            if res == -1 {
                Err(InvalidSocketError)
            } else {
                Ok(res)
            }
        }
    }
}

extern "C" {
    fn AMDServiceConnectionGetSocket(service: &ServiceConnection) -> RawFd;
}
