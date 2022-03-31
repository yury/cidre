use cidre::{mtl, objc::autoreleasepool};

fn main() {
    autoreleasepool(|| {
        let device = mtl::Device::default().unwrap();

        let command_queue = device.command_queue().unwrap();
        let command_buffer = command_queue.command_buffer().unwrap();

        let cmd_queue = command_buffer.command_queue();
        let dev = cmd_queue.device();

        assert!(dev.as_type_ref().equal(device.as_type_ref()));
        assert!(cmd_queue.as_type_ref().equal(command_queue.as_type_ref()));

        let fence = device.fence().unwrap();

        let mut blit_encoder = command_buffer.blit_command_encoder().unwrap();

        blit_encoder.update_fence(&fence);
        blit_encoder.end_encoding();

        let mut compute_encoder = command_buffer.compute_command_encoder().unwrap();
        compute_encoder.wait_for_fence(&fence);
        compute_encoder.end_encoding();

        command_buffer.commit();
        command_buffer.wait_until_completed();
    });
}
