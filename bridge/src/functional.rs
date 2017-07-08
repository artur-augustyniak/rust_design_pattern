fn ride(t: &Box<Fn() -> ()>) {
    print!("ride ");
    t();
}

fn stop(t: &Box<Fn() -> ()>) {
    print!("stop ");
    t();
}


pub fn f_run() {
    println!("-------------------- {} --------------------", file!());
    let electric: &str = "electric";
    let diesel: &str = "diesel";

    let train = |move_fn| -> Box<Fn() -> ()> {
        Box::new(move || println!("train {}", move_fn))
    };
    let trains = vec![train(electric), train(diesel)];
    for t in trains {
        ride(&t);
        stop(&t);
    }
}