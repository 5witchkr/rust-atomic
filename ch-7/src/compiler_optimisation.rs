use std::{hint::black_box, sync::atomic::AtomicU64, time::Instant};


static A: AtomicU64 = AtomicU64::new(0);
pub fn co_main() {
    let start = Instant::now();
    for _ in 0..1_000_000_000 {
        A.load(std::sync::atomic::Ordering::Relaxed);
    }
    panic!("FINSIH: {:?}", start.elapsed());
}

pub fn co_main_without_optimisation() {
    black_box(&A);
    let start = Instant::now();
    for _ in 0..1_000_000_000 {
        black_box(A.load(std::sync::atomic::Ordering::Relaxed));
    }
    panic!("FINSIH: {:?}", start.elapsed());
}