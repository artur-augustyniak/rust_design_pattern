#[derive(Debug)]
struct TrainOriginator {
    orig: String,
    dst: String,
}

#[derive(Debug)]
struct Memento {
    orig: String,
    dst: String,
}

impl TrainOriginator {
    fn new() -> TrainOriginator {
        TrainOriginator { orig: "".to_string(), dst: "".to_string() }
    }

    fn save_to_memento(&self) -> Memento {
        println!("TrainOriginator: Saving to Memento.");
        Memento { orig: self.orig.clone(), dst: self.dst.clone() }
    }


    fn restore_from_memento(&mut self, m: Memento) {
        println!("TrainOriginator: Restoring Memento.");
        self.orig = m.orig;
        self.dst = m.dst;
    }

    fn set_origin(&mut self, org: String) {
        self.orig = org;
    }

    fn set_destination(&mut self, dst: String) {
        self.dst = dst;
    }

    fn get_relation(&self) -> String {
        format!("TrainOriginator: relation is {} {}", self.orig, self.dst)
    }
}


pub fn run() {
    println!("-------------------- {} --------------------", file!());



    let mut train = TrainOriginator::new();

    train.set_origin("Warszawa".to_string());
    let m1 = train.save_to_memento();
    train.set_destination("Rzeszów".to_string());
    println!("{}", train.get_relation());

    let m2 = train.save_to_memento();

    train.set_origin("Gdańśk".to_string());
    train.set_destination("Radom".to_string());
    println!("{}", train.get_relation());

    train.restore_from_memento(m1);
    println!("{}", train.get_relation());

    train.restore_from_memento(m2);
    println!("{}", train.get_relation());
}