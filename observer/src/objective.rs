type Observer = Box<Fn(&Observable)>;

trait Observable {
    fn add_observer(&mut self, o: Observer) -> ();

    fn notify_observers(&self) -> ();

    fn get_state(&self) -> i32;
}


fn basic_observer(o: &Observable) {
    println!("Observable state change: {}", o.get_state());
}


struct Particle {
    pos: i32,
    observers: Vec<Observer>
}


impl Particle {
    fn new() -> Particle {
        Particle { pos: 0, observers: vec!() }
    }

    fn run(&mut self) -> () {
        for x in 0..10 {
            self.pos = x;
            self.notify_observers();
        }
    }
}

impl Observable for Particle {
    fn add_observer(&mut self, o: Observer) {
        self.observers.push(o);
    }

    fn notify_observers(&self) {
        for o in &self.observers {
            o(self);
        }
    }
    fn get_state(&self) -> i32 {
        self.pos
    }
}


pub fn run() {
    println!("-------------------- {} --------------------", file!());
    let mut p = Particle::new();

    p.add_observer(Box::new(basic_observer));

    p.run();
}