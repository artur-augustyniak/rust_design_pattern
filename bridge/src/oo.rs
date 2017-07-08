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

impl Vehicle for Train {
    fn ride(&self) {
        println!("Ride {:?}", self);
    }

    fn stop(&self) {
        println!("Stop {:?}", self);
    }
}


pub fn oo_run() {
    println!("-------------------- {} --------------------", file!());
    let trains = vec![Train { engine: Engine::Diesel }, Train { engine: Engine::Electric }];
    for t in trains {
        t.ride();
        t.stop();
    }
}