use std::{
    rc::Rc,
    sync::Mutex,
};

use cidre::{dispatch, objc::native_block::DispatchBlock};
// use parking_lot::Mutex;

fn main() {
    let q = dispatch::Queue::new();
    // let c = Arc::new(Mutex::new(0));
    let c = Rc::new(Mutex::new(0));

    let cc = c.clone();
    let block = DispatchBlock::stack(move || {
        let mut lock = cc.lock().unwrap();
        *lock += 1;
    });

    for _ in 0..1_000_000_000 {
        q.sync_b(&block);
    }

    println!("{:?}", c)
}
