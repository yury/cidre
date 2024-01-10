use cidre::{mtl, objc::ar_pool};

fn main() {
    ar_pool(|| {
        let device = mtl::Device::sys_default().unwrap();

        let cmd_queue = device.new_cmd_queue().unwrap();
        let mut cmd_buf = cmd_queue.new_cmd_buf().unwrap();

        let cmd_queue = cmd_buf.cmd_queue();
        let dev = cmd_queue.device();

        assert!(dev.as_type_ref().equal(device.as_type_ref()));
        assert!(cmd_queue.as_type_ref().equal(cmd_queue.as_type_ref()));

        let fence = device.new_fence().unwrap();

        cmd_buf.blit(|enc| enc.update_fence(&fence));
        cmd_buf.compute(|enc| enc.wait_for_fence(&fence));

        cmd_buf.commit();
        cmd_buf.wait_until_completed();
    });
}
