use std::{collections::VecDeque, sync::{Condvar, Mutex}};



pub struct MutexOneShotChannel<T> {
    queue : Mutex<VecDeque<T>>,
    ready: Condvar
}

impl <T> MutexOneShotChannel<T> {
    pub fn new() -> Self {
        Self { queue: Mutex::new(VecDeque::new()), ready: Condvar::new() }
    }    

    pub fn send(&self , msg: T) {
        self.queue.lock().unwrap().push_back(msg);
        self.ready.notify_one();
    }

    pub fn recv(&self) -> T {
        let mut b = self.queue.lock().unwrap();
        loop {
            if let Some(msg) = b.pop_front() {
                return msg;
            }
            b = self.ready.wait(b).unwrap();
        }
    }

}