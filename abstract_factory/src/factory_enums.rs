enum Car {
    Sedan,
    Coupe,
}


impl Car {
    fn ride(&self) {
        match *self {
            Car::Coupe => { println!("Coupe.ride()"); }
            Car::Sedan => { println!("Sedan.ride()"); }
        }
    }
}


fn generalized_run(c: Car) {
    c.ride();
}

pub fn run() {
    generalized_run(Car::Sedan);
    generalized_run(Car::Coupe);
    Car::Coupe.ride();
    Car::Sedan.ride();

    let cars = vec![Car::Coupe, Car::Sedan];
    for c in cars {
        generalized_run(c);
    }
}