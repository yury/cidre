#![cfg(feature = "blocks")]

use cidre::blocks;
use std::sync::{
    Arc,
    atomic::{AtomicUsize, Ordering},
};

#[global_allocator]
static GLOBAL: mimalloc::MiMalloc = mimalloc::MiMalloc;

#[repr(align(256))]
struct Payload(Arc<AtomicUsize>);
impl Payload {
    fn value(&self) -> usize {
        assert_eq!((self as *const Self).addr() % 256, 0);
        17
    }
}
impl Drop for Payload {
    fn drop(&mut self) {
        self.0.fetch_add(1, Ordering::SeqCst);
    }
}

#[test]
fn native_block_owns_aligned_payload_with_custom_global_allocator() {
    let drops = Arc::new(AtomicUsize::new(0));
    let payload = Payload(drops.clone());
    let mut block: cidre::arc::R<blocks::EscBlock<fn() -> usize>> =
        blocks::EscBlock::new0(move || payload.value());
    assert_eq!(block.call(), 17);
    let retained = block.retained();
    drop(block);
    assert_eq!(drops.load(Ordering::SeqCst), 0);
    drop(retained);
    assert_eq!(drops.load(Ordering::SeqCst), 1);
}
