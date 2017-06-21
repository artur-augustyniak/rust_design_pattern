#[derive(Debug, Copy, Clone)]
pub enum Direction {
    Forward,
    Backward
}

pub trait LegacyMovable {
    fn ride(&mut self, d: Direction, velocity: u32);
    fn print(&self);
}

#[derive(Debug, Copy, Clone)]
pub struct LegacyTrain {
    pub d: Direction,
    pub  v: u32,
}

impl LegacyTrain {
    pub fn new(d: Direction, v: u32) -> LegacyTrain {
        LegacyTrain { d: d, v: v }
    }
}

impl LegacyMovable for LegacyTrain {
    fn ride(&mut self, d: Direction, velocity: u32) {
        self.d = d;
        self.v = velocity;
    }
    fn print(&self) {
        println!("Legacy ride {:?}", self);
    }
}

pub fn legacy_use() {
    println!("-------------------- {} --------------------", file!());
    let mut l = LegacyTrain::new(Direction::Forward, 12);
    l.print();
    l.ride(Direction::Backward, 5);
    l.print();
}


