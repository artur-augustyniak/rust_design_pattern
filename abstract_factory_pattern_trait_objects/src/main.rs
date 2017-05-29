pub mod cars;
pub mod cars_factory;

use cars_factory::*;

fn make_and_run(f: &CarFactory) {
    let car = f.make_car();
    car.ride();
}

fn main() {
    let sf = SedanFactory;
    make_and_run(&sf);
    let cf = CoupeFactory;
    make_and_run(&cf);

    let factories: Vec<&CarFactory> = vec![&sf, &cf];
    for fact in factories {
        let c = fact.make_car();
        c.ride();
        make_and_run(fact)
    }

    for _ in 0..10 {
        let f = ExternalParametrizedFactory;
        make_and_run(&f);
    }
}
