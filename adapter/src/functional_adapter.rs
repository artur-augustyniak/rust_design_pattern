use std::time::Duration;
use std::cmp::Ordering;

use super::legacy_train::*;

use super::current_functional_train::*;

pub trait ChangeVelocity {
    fn accelerate(self, t: Duration) -> ImmutableLegacyTrainAdaptor;
    fn decelerate(self, t: Duration) -> ImmutableLegacyTrainAdaptor;
}

#[derive(Debug)]
pub struct ImmutableLegacyTrainAdaptor {
    adaptee: LegacyTrain,
}


fn recreate_adaptee(v: i32) -> ImmutableLegacyTrainAdaptor {
    let threshold = 0;
    match v.cmp(&threshold) {
        Ordering::Less => {
            let tmp = ImmutableLegacyTrainAdaptor { adaptee: LegacyTrain::new(Direction::Backward, v.abs() as u32) };
            println!("Impure debug {}", to_string(&tmp));
            tmp
        }
        Ordering::Greater => {
            let tmp = ImmutableLegacyTrainAdaptor { adaptee: LegacyTrain::new(Direction::Forward, v.abs() as u32) };
            println!("Impure debug {}", to_string(&tmp));
            tmp
        }
        Ordering::Equal => {
            let tmp = ImmutableLegacyTrainAdaptor { adaptee: LegacyTrain::new(Direction::Forward, threshold as u32) };
            println!("Impure debug {}", to_string(&tmp));
            tmp
        }
    }
}


fn pluss_adaptee_recreation_strategy_by(d: &Direction, v: u32, t: Duration) -> ImmutableLegacyTrainAdaptor {
    match *d {
        Direction::Forward => {
            let current_v = v as i32;
            let v = current_v + (TRAIN_ACCELERATION * t.as_secs() as u32) as i32;
            recreate_adaptee(v)
        }
        Direction::Backward => {
            let current_v = -1 * (v as i32);
            let v = current_v + (TRAIN_ACCELERATION * t.as_secs() as u32) as i32;
            recreate_adaptee(v)
        }
    }
}


fn minuss_adaptee_recreation_strategy_by(d: &Direction, v: u32, t: Duration) -> ImmutableLegacyTrainAdaptor {
    match *d {
        Direction::Forward => {
            let current_v = v as i32;
            let v = current_v - (TRAIN_ACCELERATION * t.as_secs() as u32) as i32;
            recreate_adaptee(v)
        }
        Direction::Backward => {
            let current_v = -1 * (v as i32);
            let v = current_v - (TRAIN_ACCELERATION * t.as_secs() as u32) as i32;
            recreate_adaptee(v)
        }
    }
}


impl ChangeVelocity for ImmutableLegacyTrainAdaptor {
    fn accelerate(self, t: Duration) -> ImmutableLegacyTrainAdaptor {
        pluss_adaptee_recreation_strategy_by(self.adaptee.get_dir(), self.adaptee.get_v(), t)
    }

    fn decelerate(self, t: Duration) -> ImmutableLegacyTrainAdaptor {
        minuss_adaptee_recreation_strategy_by(self.adaptee.get_dir(), self.adaptee.get_v(), t)
    }
}


pub fn functional_adapter_use() {
    println!("-------------------- {} --------------------", file!());
    let legacy_train = LegacyTrain::new(Direction::Forward, 12);
    let train = ImmutableLegacyTrainAdaptor { adaptee: legacy_train };
    println!("{}", to_string(&train));
    let d1 = Duration::new(5, 0);
    let d2 = Duration::new(12, 0);
    //nowy bo to jest inny pociąg z dziedziny "pociągów z możliwymi prędkościami"
    // po n przekrztałeceniach
    let new_algebraic_train = train.accelerate(d1)
        .decelerate(d1)
        .decelerate(d1)
        .decelerate(d2)
        .accelerate(d1);
    println!("{}", to_string(&new_algebraic_train));
}