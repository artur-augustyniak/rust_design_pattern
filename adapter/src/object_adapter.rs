use super::legacy_train::*;
use super::current_train::*;
use std::time::Duration;

#[derive(Debug)]
struct LegacyTrainAdaptor {
    adaptee: LegacyTrain
}

impl LegacyTrainAdaptor {
    pub fn new(lt: LegacyTrain) -> LegacyTrainAdaptor {
        LegacyTrainAdaptor { adaptee: lt }
    }
}


impl CurrentMovable for LegacyTrainAdaptor {
    fn print(&self) {
        println!("Current ride of Legacy train {:?}", self);
    }

    fn accelerate(&mut self, t: Duration) -> i32 {
        match self.adaptee.d {
            Direction::Forward => {
                let v = self.adaptee.v as i32 + (TRAIN_ACCELERATION * t.as_secs() as u32) as i32;
                self.adaptee.ride(Direction::Forward, v as u32);
                v
            }
            Direction::Backward => {
                let v = self.adaptee.v as i32 - (TRAIN_ACCELERATION * t.as_secs() as u32) as i32;
                self.adaptee.ride(Direction::Backward, v as u32);
                v
            }
        }
    }

    fn decelerate(&mut self, t: Duration) -> i32 {
        match self.adaptee.d {
            Direction::Forward => {
                let v = self.adaptee.v as i32 - (TRAIN_ACCELERATION * t.as_secs() as u32) as i32;
                if v < 0 {
                    self.adaptee.ride(Direction::Backward, v.abs() as u32);
                } else {
                    self.adaptee.ride(Direction::Forward, v as u32);
                }
                v
            }
            Direction::Backward => {
                let v = self.adaptee.v as i32 + (TRAIN_ACCELERATION * t.as_secs() as u32) as i32;
                self.adaptee.ride(Direction::Backward, v as u32);
                v
            }
        }
    }
}


pub fn object_adapter_use() {
    println!("-------------------- {} --------------------", file!());
    let l = LegacyTrain::new(Direction::Forward, 12);
    l.print();
    let mut lta = LegacyTrainAdaptor::new(l);
    lta.print();
    let d1 = Duration::new(5, 0);
    let d2 = Duration::new(12, 0);
    lta.accelerate(d1);
    lta.print();
    lta.decelerate(d1);
    lta.print();
    lta.decelerate(d1);
    lta.print();
    lta.decelerate(d2);
    lta.print();
    lta.accelerate(d1);
    lta.print();
}