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
        println!("not so true sedan rid!");
    }
}

trait CarFactory<C: Car> {
    fn make_car(&self) -> C;
}

struct SedanFactory;

impl CarFactory<Sedan> for SedanFactory {
    fn make_car(&self) -> Sedan {
        Sedan
    }
}


struct CoupeFactory;

impl CarFactory<Coupe> for CoupeFactory {
    fn make_car(&self) -> Coupe {
        Coupe
    }
}


fn make_and_run<C: Car>(f: &CarFactory<C>) {
    let car = f.make_car();
    car.ride();
}


fn main() {
    let f = SedanFactory;
    make_and_run(&f);
    let f = CoupeFactory;
    make_and_run(&f);
}
