#[derive(Debug)]
enum Engine {
    Electric,
    Diesel
}


trait Vehicle {
    fn ride(&self);
    fn stop(&self);
}

#[derive(Debug)]
struct Train {
    engine: Engine
}


#[derive(Debug)]
struct Car {
    engine: Engine
}

impl Vehicle for Train {
    fn ride(&self) {
        println!("Train ride {:?}", self);
    }

    fn stop(&self) {
        println!("Train stop {:?}", self);
    }
}


impl Vehicle for Car {
    fn ride(&self) {
        println!("Car ride {:?}", self);
    }

    fn stop(&self) {
        println!("Car stop {:?}", self);
    }
}


pub fn oo_run() {
    println!("-------------------- {} --------------------", file!());

    let dt = Train { engine: Engine::Diesel };
    let et = Train { engine: Engine::Electric };
    let ec = Car { engine: Engine::Electric };

    let vehicles: Vec<&Vehicle> = vec![&dt, &et, &ec];
    for t in vehicles {
        t.ride();
        t.stop();
    }
}