extern crate rand;

use self::rand::{thread_rng, Rng};
use std::{thread, time};


pub fn run() {
    println!("-------------------- {} --------------------", file!());
    let sleep_time = time::Duration::from_millis(1);
    let mut rng = thread_rng();
    for _ in 0..1 {
        let x = rng.gen_iter::<u8>().take(10).map(|x| {
            println!("yeah stream observer in map : val {}", x);
            x
        }).collect::<Vec<u8>>();
        println!("{:?}", x);
        thread::sleep(sleep_time);
    }
}