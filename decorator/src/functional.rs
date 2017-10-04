use std::fmt::Debug;


#[derive(Debug)]
struct Vehicle {
    handbrake_enabled: bool
}

fn manifest_presence<T>(v: T) -> T
    where T: Debug
{
    println!("Me be: {:?}", v);
    v
}

type CPS = Box<Fn(Vehicle) -> Vehicle>;

fn vehicle_run(cps: CPS) -> CPS {
    Box::new(move |v: Vehicle| {
        let v: Vehicle = cps(v);
        println!("Running: ({:?})", &v);
        v
    }
    )
}

fn vehicle_lights_on(cps: CPS) -> CPS {
    Box::new(move |v: Vehicle| {
        let v: Vehicle = cps(v);
        println!("Lights on: ({:?})", &v);
        v
    }
    )
}


pub fn run() {
    println!("-------------------- {} --------------------", file!());

    let v = Vehicle { handbrake_enabled: true };
    println!("%## RAW ###");
    let v = manifest_presence(v);
    println!("%## DECORATED ###");
    let vrl: Box<Fn(Vehicle) -> Vehicle> = vehicle_lights_on(vehicle_run(Box::new(manifest_presence)));
    vrl(v);
}