use std::{ffi::c_void, mem::MaybeUninit};

use crate::mach;

use super::{TaskBasicInfo, TaskVmInfo};

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

/// The task's memory (`TASK_VM_INFO`): its footprint, and the graphics share of it.
#[inline]
pub fn vm_info(target_task: mach::Port) -> Result<mach::TaskVmInfo, mach::KernReturn> {
    let mut info = TaskVmInfo::default();
    let mut cnt = TaskVmInfo::count();
    let res = unsafe {
        task_info(
            target_task,
            TaskVmInfo::flavor(),
            &mut info as *mut TaskVmInfo as _,
            &mut cnt as *mut mach::message::Number,
        )
    };
    if res.is_ok() { Ok(info) } else { Err(res) }
}

unsafe extern "C-unwind" {
    fn task_info(
        target_task: mach::Port,
        flavor: mach::TaskFlavor,
        task_info_out: *mut c_void,
        task_info_out_cnt: *mut mach::message::Number,
    ) -> mach::KernReturn;
}

#[cfg(test)]
mod tests {
    use crate::mach;

    #[test]
    fn basics() {
        let res =
            mach::task::basic_info(mach::Port::current_task()).expect("Failed to get task info");
        println!("task info {:?}", res);
        assert!(res.resident_size > 0);
        assert!(res.resident_size_max > 0);
    }

    #[test]
    fn vm_info() {
        // 372 bytes in the SDK's `task_vm_info_data_t` (`#pragma pack(4)`): 93 naturals.
        assert_eq!(mach::TaskVmInfo::count(), 93);
        let res = mach::task::vm_info(mach::Port::current_task()).expect("Failed to get vm info");
        let footprint = res.phys_footprint;
        assert!(footprint > 0);
        assert!(res.page_size > 0);
    }
}
