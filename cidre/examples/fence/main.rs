use cidre::{mtl, objc::autoreleasepool};

fn main() {
    autoreleasepool(|| {
        let device = mtl::Device::default().unwrap();

        let cmd_queue = device.new_cmd_queue().unwrap();
        let cmd_buf = cmd_queue.new_cmd_buf().unwrap();

        let cmd_queue = cmd_buf.cmd_queue();
        let dev = cmd_queue.device();

        assert!(dev.as_type_ref().equal(device.as_type_ref()));
        assert!(cmd_queue.as_type_ref().equal(cmd_queue.as_type_ref()));

        let fence = device.new_fence().unwrap();

        let mut blit_enc = cmd_buf.new_blit_cmd_enc().unwrap();

        blit_enc.update_fence(&fence);
        blit_enc.end_encoding();

        let mut compute_enc = cmd_buf.new_compute_cmd_enc().unwrap();
        compute_enc.wait_for_fence(&fence);
        compute_enc.end_encoding();

        cmd_buf.commit();
        cmd_buf.wait_until_completed();
    });
}
