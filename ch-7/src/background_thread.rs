use std::{hint::black_box, sync::atomic::AtomicU64, thread, time::Instant};


static A: AtomicU64 = AtomicU64::new(0);

pub fn bg_main() {
    black_box(&A);
    thread::spawn(||{
        loop {
            black_box(A.load(std::sync::atomic::Ordering::Relaxed));
        }
    });
    let start = Instant::now();
    for _ in 0..1_000_000_000 {
        black_box(A.load(std::sync::atomic::Ordering::Relaxed));
    }
    panic!("FINSIH: {:?}", start.elapsed());
}

pub fn bg_main_store() {
    black_box(&A);
    thread::spawn(||{
        loop {
            black_box(A.store(0, std::sync::atomic::Ordering::Relaxed));
        }
    });
    let start = Instant::now();
    for _ in 0..1_000_000_000 {
        black_box(A.load(std::sync::atomic::Ordering::Relaxed));
    }
    panic!("FINSIH: {:?}", start.elapsed());
}