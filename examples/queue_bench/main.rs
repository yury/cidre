use std::{ffi::c_void, rc::Rc, sync::Mutex};

use cidre::{dispatch, objc::blocks_runtime};
// // use parking_lot::Mutex;

extern "C" fn block_fn(_ctx: *const c_void) {}

fn main() {
    let q = dispatch::Queue::new();
    let c = Rc::new(Mutex::new(0));

    let cc = c.clone();
    let b = Box::new(5);
    // let mut block = blocks_runtime::with_fn(block_fn);
    let mut block = blocks_runtime::new_mut(|| {
        // println!("nice");
    });

    // let esc = block.escape();
    for _ in 0..1_000_000_000 {
        // q.async_b(block.escape());
        q.sync_b(&mut block);
        // q.sync_b(&mut block);
    }

    // q.sync_b(&mut block);

    println!("{:?}", c)
}
