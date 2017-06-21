use std::time::Duration;
use std::cmp::Ordering;

use super::legacy_train::*;

use super::current_functional_train::*;

pub const ACCELERATION: u64 = 2;

pub trait ChangeVelocity {
    fn accelerate(self, t: Duration) -> ImmutableLegacyTrainAdaptor;
    fn decelerate(self, t: Duration) -> ImmutableLegacyTrainAdaptor;
    fn textural_dump(self) -> ImmutableLegacyTrainAdaptor;
}

#[derive(Debug)]
pub struct ImmutableLegacyTrainAdaptor {
    adaptee: LegacyTrain,
}


fn accelerate_with<F>(lt: LegacyTrain, t: Duration, op: F) -> ImmutableLegacyTrainAdaptor
    where F: FnOnce(i32, i32) -> i32 {
    let v_0 = match lt.d {
        Direction::Forward => lt.v as i32,
        Direction::Backward => -1 * (lt.v as i32)
    };
    let v_t = op(v_0, (ACCELERATION * t.as_secs()) as i32);
    let dir = match v_t.cmp(&0) {
        Ordering::Less => Direction::Backward,
        _ => Direction::Forward
    };
    ImmutableLegacyTrainAdaptor { adaptee: LegacyTrain::new(dir, v_t.abs() as u32) }
}


impl ChangeVelocity for ImmutableLegacyTrainAdaptor {
    fn accelerate(self, t: Duration) -> ImmutableLegacyTrainAdaptor {
        let fw_acceleration = |x, y| x + y;
        accelerate_with(self.adaptee, t, fw_acceleration).textural_dump()
    }

    fn decelerate(self, t: Duration) -> ImmutableLegacyTrainAdaptor {
        let rev_acceleration = |x, y| x - y;
        accelerate_with(self.adaptee, t, rev_acceleration).textural_dump()
    }
    fn textural_dump(self) -> ImmutableLegacyTrainAdaptor {
        println!("Impure debug {}", to_string(&self));
        self
    }
}


impl Clone for ImmutableLegacyTrainAdaptor {
    fn clone(&self) -> ImmutableLegacyTrainAdaptor {
        ImmutableLegacyTrainAdaptor {
            adaptee:
            LegacyTrain::new(self.adaptee.d, self.adaptee.v)
        }
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
    let new_algebraic_train = train
        .accelerate(d1)
        .decelerate(d1)
        .decelerate(d1)
        .decelerate(d2)
        .accelerate(d1);
    println!("{}", to_string(&new_algebraic_train));

    println!("-------------------- {} --------------------", "collect() to final pos");

    let legacy_train = LegacyTrain::new(Direction::Forward, 12);
    let mut train = ImmutableLegacyTrainAdaptor { adaptee: legacy_train };
    let accelerations: Vec<i32> = vec![5, -5, -5, -12, 5]; //Backward 1
    let final_pos: Vec<ImmutableLegacyTrainAdaptor> = accelerations.iter().map(
        |acc| {
            match acc.cmp(&0) {
                Ordering::Less =>
                    {
                        let v: i32 = *acc;
                        let v = v.abs();
                        train = train.clone().decelerate(Duration::new(v as u64, 0));
                        train.clone()
                    }
                _ => {
                    train = train.clone().accelerate(Duration::new(*acc as u64 as u64, 0));
                    train.clone()
                }
            }
        }).collect();

    println!("{:?}", &final_pos);

    for x in final_pos {
        x.textural_dump();
    }
}