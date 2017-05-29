extern crate rand;

use self::rand::Rng;
use cars::*;

pub struct SedanFactory;

pub struct CoupeFactory;

pub struct ExternalParametrizedFactory;

pub trait CarFactory {
    fn make_car(&self) -> Box<Car>;
}

impl CarFactory for SedanFactory {
    fn make_car(&self) -> Box<Car> {
        Box::new(Sedan)
    }
}

impl CarFactory for CoupeFactory {
    fn make_car(&self) -> Box<Car> {
        Box::new(Coupe)
    }
}


impl CarFactory for ExternalParametrizedFactory {
    fn make_car(&self) -> Box<Car> {
        let mut rng = rand::thread_rng();
        if rng.gen() {
            Box::new(Coupe)
        } else {
            Box::new(Sedan)
        }
    }
}

