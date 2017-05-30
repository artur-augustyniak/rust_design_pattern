pub mod abstract_factory_trait_objects;
pub mod abstract_factory_assoc_types;
pub mod abstract_factory_generics;
pub mod factory_enums;

use abstract_factory_trait_objects::run as t_obj_run;
use abstract_factory_assoc_types::run as assoc_t_run;
use abstract_factory_generics::run as generic_run;
use factory_enums::run as enums_run;

pub trait Car {
    fn ride(&self);
}

pub struct Sedan;

pub struct Coupe;

pub struct SedanFactory;

pub struct CoupeFactory;

pub struct ExternalParametrizedFactory;

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


fn main() {
    println!("~~~~ Trait object factory ~~~~");
    t_obj_run();
    println!("~~~~ Assoc types factory ~~~~");
    assoc_t_run();
    println!("~~~~ Generics factory ~~~~");
    generic_run();
    println!("~~~~ Enums factory ~~~~");
    enums_run();
}
