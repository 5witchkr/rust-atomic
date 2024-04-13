use std::{cell::UnsafeCell, mem::MaybeUninit, sync::atomic::AtomicBool};



pub struct UnsafeAtomicOneshotChannel<T> {
    msg: UnsafeCell<MaybeUninit<T>>,
    ready: AtomicBool
}

unsafe impl<T> Sync for UnsafeAtomicOneshotChannel<T> where T: Send {}

impl<T> UnsafeAtomicOneshotChannel<T> {
    pub const fn new() -> Self {
        Self { msg: UnsafeCell::new(MaybeUninit::uninit()), ready: AtomicBool::new(false) }
    }

    pub unsafe fn send(&self, msg: T) {
        (*self.msg.get()).write(msg);
        self.ready.store(true, std::sync::atomic::Ordering::Release);
    }

    pub fn is_ready(&self) -> bool {
        self.ready.load(std::sync::atomic::Ordering::Acquire)
    }

    pub unsafe fn recv(&self) -> T {
        if !self.is_ready() { panic!("not ready") };
        (*self.msg.get()).assume_init_read()
    }
}