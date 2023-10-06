use std::{ffi::c_void, mem::MaybeUninit};

use crate::mach;

use super::TaskBasicInfo;

#[inline]
pub fn basic_info(target_task: mach::Port) -> Result<mach::TaskBasicInfo, mach::KernReturn> {
    unsafe {
        let mut info: MaybeUninit<TaskBasicInfo> = MaybeUninit::uninit();
        let mut cnt = TaskBasicInfo::count();
        let res = task_info(
            target_task,
            TaskBasicInfo::flavor(),
            info.as_mut_ptr() as _,
            &mut cnt as *mut mach::message::Number,
        );

        if res.is_ok() {
            Ok(info.assume_init())
        } else {
            Err(res)
        }
    }
}

extern "C" {
    fn task_info(
        target_task: mach::Port,
        flavor: mach::TaskFlavor,
        task_info_out: *mut c_void,
        task_info_out_cnt: *mut mach::message::Number,
    ) -> mach::KernReturn;
}

#[cfg(test)]
mod tests {
    use std::{thread::sleep, time::Duration};

    use crate::mach;
    #[test]
    fn basics() {
        let res =
            mach::task::basic_info(mach::Port::current_task()).expect("Failed to get task info");
        println!("task info {:?}", res);
        assert!(res.resident_size > 0);
        assert!(res.resident_size_max > 0);
        assert_eq!(res.user_time.microseconds, 0);
        assert_eq!(res.system_time.microseconds, 0);
    }
}
