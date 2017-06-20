use std::fmt::Debug;
use std::time::Duration;

pub const TRAIN_ACCELERATION: u32 = 2;


pub fn to_string<T: Debug>(o: &T) -> String {
    return format!("Current ride {:?}", o);
}

pub trait ChangeVelocity {
    fn accelerate(self, t: Duration) -> ImmutableTrain;
    fn decelerate(self, t: Duration) -> ImmutableTrain;
}

#[derive(Debug)]
pub struct ImmutableTrain {
    v: i32,
}


impl ChangeVelocity for ImmutableTrain {
    fn accelerate(self, t: Duration) -> ImmutableTrain {
        let v = self.v + (TRAIN_ACCELERATION * t.as_secs() as u32) as i32;
        let tmp = ImmutableTrain { v: v };
        println!("Impure debug {}", to_string(&tmp));
        tmp
    }

    fn decelerate(self, t: Duration) -> ImmutableTrain {
        let v = self.v - (TRAIN_ACCELERATION * t.as_secs() as u32) as i32;
        let tmp = ImmutableTrain { v: v };
        println!("Impure debug {}", to_string(&tmp));
        tmp
    }
}


pub fn current_functional_use() {
    println!("-------------------- {} --------------------", file!());
    let train = ImmutableTrain { v: 12 };
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