use crate::{simple_mutex_chan::MutexOneShotChannel, unsafe_atomic_chan::UnsafeAtomicOneshotChannel};

mod simple_mutex_chan;
mod unsafe_atomic_chan;

fn main() {
    println!("process start");

    #[derive(Debug)]
    struct Input(u8,u8);


    //simple mutex oneshot channel
    let input = Input(1,2);
    let chan = MutexOneShotChannel::new();
    chan.send(input);
    let output = chan.recv();
    println!("{:?}", output);

    //simple unsafe oneshot channel
    let input = Input(1,2);
    let chan = UnsafeAtomicOneshotChannel::new();
    unsafe { chan.send(input) };
    let output = unsafe { chan.recv() };
    println!("{:?}", output);
}
