pub struct Sedan;

pub struct Coupe;

pub trait Car {
    fn ride(&self);
}

impl Car for Sedan {
    fn ride(&self) {
        println!("Sedan.ride()");
    }
}

impl Car for Coupe {
    fn ride(&self) {
        println!("Coupe.ride()");
    }
}
