use std::{
    ffi::c_void,
    rc::Rc,
    sync::{Arc, Mutex},
};

use cidre::{blocks, dispatch};
// // use parking_lot::Mutex;

extern "C" fn block_fn(_ctx: *const c_void) {}

fn main() {
    let q = dispatch::Queue::global(0).unwrap();
    let c = Arc::new(Mutex::new(0));

    let cc = c.clone();

    let mut block = blocks::mut0(move || {
        let mut v = cc.lock().unwrap();
        *v += 1;
    });

    // let esc = block.escape();
    for _ in 0..1_000_000_000 {
        // q.async_b(block.escape());
        q.async_b(block.escape());
        // q.sync_b(&mut block);
    }

    println!("{:?}", c)
}
