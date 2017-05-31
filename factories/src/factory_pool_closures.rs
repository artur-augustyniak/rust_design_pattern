extern crate rand;

use self::rand::Rng;
use::*;

impl Sedan {
    fn new() -> Sedan {
        Sedan
    }
}

impl Drop for Sedan {
    fn drop(&mut self) {
        println!("Dropping Sedan!");
    }
}


struct CarFactory<F, T: Car> where F: Fn() -> T {
    create_car: F,
    pool: Vec<T>
}


impl<F, T: Car> CarFactory<F, T> where F: Fn() -> T {
    fn new(create_car: F) -> Self {
        let pool = vec![];
        CarFactory { create_car: create_car, pool: pool }
    }

    fn borrow_car(&mut self) -> Option<T> {
        if self.pool.len() == 0 {
            println!("Creating new Car in pool");
            self.pool.push((self.create_car)()) //<-- call closure
        } else {
            println!("Get existing from pool");
        }
        self.pool.pop()
    }

    fn return_car(&mut self, conn: T) {
        self.pool.push(conn);
    }
}

fn client_mock<F, T: Car>(f: &mut CarFactory<F, T>) where F: Fn() -> T {
    let car = f.borrow_car().unwrap();
    car.ride();
    let mut rng = rand::thread_rng();
    if rng.gen() {
        f.return_car(car);
    }
}


pub fn run() {
    let mut sp = CarFactory::new(Sedan::new); //<--reference to creating function
    for _ in 0..10 {
        client_mock(&mut sp);
    }
}
