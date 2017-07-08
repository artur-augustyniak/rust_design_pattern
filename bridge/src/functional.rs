fn ride(t: &Box<Fn() -> ()>) {
    print!("ride ");
    t();
}

fn stop(t: &Box<Fn() -> ()>) {
    print!("stop ");
    t();
}


#[derive(Debug)]
enum Vehicle {
    Car,
    Train
}


pub fn f_run() {
    println!("-------------------- {} --------------------", file!());
    let electric: &str = "electric";
    let diesel: &str = "diesel";

    let vehicle = |move_fn, vehicle| -> Box<Fn() -> ()> {
        Box::new(move || println!("{:?} {}", move_fn, vehicle))
    };
    let trains = vec![
        vehicle(Vehicle::Train, electric),
        vehicle(Vehicle::Train, diesel),
        vehicle(Vehicle::Car, electric),
    ];
    for t in trains {
        ride(&t);
        stop(&t);
    }
}