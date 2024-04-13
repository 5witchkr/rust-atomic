use crate::{background_thread::{bg_main, bg_main_store}, compiler_optimisation::{co_main, co_main_without_optimisation}};

mod compiler_optimisation;
mod background_thread;

fn main() {
    println!("process start");

    //co_main();

    //co_main_without_optimisation();

    //bg_main(); //cache

    bg_main_store(); //store operation. not shared cache line
}
