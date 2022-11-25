use std::{rc::Rc, sync::Mutex};

use cidre::{dispatch, objc::blocks_runtime::BlockFn};
// use parking_lot::Mutex;

fn main() {
    let q = dispatch::Queue::new();
    let c = Rc::new(Mutex::new(0));

    let cc = c.clone();
    let mut block = BlockFn::<(), (), _>::with(move || {
        let mut lock = cc.lock().unwrap();
        *lock += 1;
    });

    for _ in 0..1_000_000_000 {
        q.sync_b(&mut block);
    }

    println!("{:?}", c)
}
