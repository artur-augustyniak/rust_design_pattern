extern crate rand;

use self::rand::Rng;
use::*;


trait CarFactory {
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


fn client_mock(f: &CarFactory) {
    let car = f.make_car();
    car.ride();
}

pub fn run() {
    let sf = SedanFactory;
    client_mock(&sf);
    let cf = CoupeFactory;
    client_mock(&cf);

    let factories: Vec<&CarFactory> = vec![&sf, &cf];
    for fact in factories {
        let c = fact.make_car();
        c.ride();
        client_mock(fact)
    }

    for _ in 0..10 {
        let f = ExternalParametrizedFactory;
        client_mock(&f);
    }
}


