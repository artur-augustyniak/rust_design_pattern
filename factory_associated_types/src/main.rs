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

trait CarFactory {
    type C;
    fn make_car(&self) -> Self::C;
}

struct SedanFactory;

impl CarFactory for SedanFactory {
    type C = Sedan;
    fn make_car(&self) -> Sedan {
        Sedan
    }
}


struct CoupeFactory;

impl CarFactory for CoupeFactory {
    type C = Coupe;
    fn make_car(&self) -> Coupe {
        Coupe
    }
}


fn main() {
    let f = SedanFactory;
    let sedan = f.make_car();
    sedan.ride();


    let f = CoupeFactory;
    let coupe = f.make_car();
    coupe.ride();
}
