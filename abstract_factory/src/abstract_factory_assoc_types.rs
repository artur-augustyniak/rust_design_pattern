use::*;


trait CarFactory {
    type C: Car;
    fn make_car(&self) -> Self::C;
}


impl CarFactory for SedanFactory {
    type C = Sedan;
    fn make_car(&self) -> Sedan {
        Sedan
    }
}


impl CarFactory for CoupeFactory {
    type C = Coupe;
    fn make_car(&self) -> Coupe {
        Coupe
    }
}


fn client_mock<F: CarFactory>(f: &F) {
    let car = f.make_car();
    car.ride();
}

pub fn run() {
    let f = SedanFactory;
    client_mock(&f);
    let f = CoupeFactory;
    client_mock(&f);
}
