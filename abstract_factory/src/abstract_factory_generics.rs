use::*;


trait CarFactory<C: Car> {
    fn make_car(&self) -> C;
}


impl CarFactory<Sedan> for SedanFactory {
    fn make_car(&self) -> Sedan {
        Sedan
    }
}


impl CarFactory<Coupe> for CoupeFactory {
    fn make_car(&self) -> Coupe {
        Coupe
    }
}


fn client<C: Car>(f: &CarFactory<C>) {
    let car = f.make_car();
    car.ride();
}


pub fn run() {
    let f = SedanFactory;
    client(&f);
    let f = CoupeFactory;
    client(&f);
}
