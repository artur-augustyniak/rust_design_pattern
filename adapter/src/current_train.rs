use std::time::Duration;

pub const TRAIN_ACCELERATION: u32 = 2;

pub trait CurrentMovable {
    fn accelerate(&mut self, t: Duration) -> i32;
    fn decelerate(&mut self, t: Duration) -> i32;
    fn print(&self);
}

#[derive(Debug)]
pub struct CurrentTrain {
    current_v: i32,
}

impl CurrentTrain {
    pub fn new(v: i32) -> CurrentTrain {
        CurrentTrain { current_v: v }
    }
}


impl CurrentMovable for CurrentTrain {
    fn print(&self) {
        println!("Current ride {:?}", self);
    }
    fn accelerate(&mut self, t: Duration) -> i32 {
        self.current_v = self.current_v + (TRAIN_ACCELERATION * t.as_secs() as u32) as i32;
        self.current_v
    }

    fn decelerate(&mut self, t: Duration) -> i32 {
        self.current_v = self.current_v - (TRAIN_ACCELERATION * t.as_secs() as u32) as i32;
        self.current_v
    }
}


pub fn current_use() {
    println!("-------------------- {} --------------------", file!());
    let mut c = CurrentTrain::new(12);
    c.print();
    let d1 = Duration::new(5, 0);
    let d2 = Duration::new(12, 0);
    c.accelerate(d1);
    c.print();
    c.decelerate(d1);
    c.print();
    c.decelerate(d1);
    c.print();
    c.decelerate(d2);
    c.print();
    c.accelerate(d1);
    c.print();
}