trait Car {
    fn ride(&self);
}

struct Sedan;

impl Car for Sedan {
    fn ride(&self) {
        println!("true sedan ride!");
    }
}

struct Coupe;

impl Car for Coupe {
    fn ride(&self) {
        println!("not so true sedan ride!");
    }
}

trait CarFactory {
    fn make_car(&self) -> Box<Car>;
}

struct SedanFactory;

impl CarFactory for SedanFactory {
    fn make_car(&self) -> Box<Car> {
        Box::new(Sedan)
    }
}


struct CoupeFactory;

impl CarFactory for CoupeFactory {
    fn make_car(&self) -> Box<Car> {
        Box::new(Coupe)
    }
}


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
    }
}
