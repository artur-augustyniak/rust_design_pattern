extern crate rand;

use self::rand::Rng;


trait SpeedController {
    fn stop(&self);
}

struct EmergencyBrake;

impl SpeedController for EmergencyBrake {
    fn stop(&self) {
        println!("stop using EMERGENCY brake !!!");
    }
}


struct PanicBrake;

impl SpeedController for PanicBrake {
    fn stop(&self) {
        println!("stop using PANIC brake !!!");
    }
}


struct RegularBrake {
    emergency_brake: Box<SpeedController>
}

impl<'a> SpeedController for RegularBrake {
    fn stop(&self) {
        let mut rng = rand::thread_rng();
        if rng.gen() {
            println!("stop using regular brake...");
        } else {
            self.emergency_brake.stop();
        }
    }
}

struct BrakeSystem {
    current_brake: Box<SpeedController>
}


impl SpeedController for BrakeSystem {
    fn stop(&self) {
        self.current_brake.stop();
    }
}


struct BrakeSystemBuilder {
    emergency_subsystem: Box<SpeedController>,

}

impl BrakeSystemBuilder {
    fn new() -> BrakeSystemBuilder {
        BrakeSystemBuilder {
            emergency_subsystem: Box::new(EmergencyBrake)
        }
    }

    fn change_emergency_brake(mut self, emb: Box<SpeedController>) -> BrakeSystemBuilder {
        self.emergency_subsystem = emb;
        self
    }


    fn finalize(self) -> BrakeSystem {
        BrakeSystem {
            current_brake: Box::new(
                RegularBrake { emergency_brake: self.emergency_subsystem }
            )
        }
    }
}

fn main() {
    let bs: BrakeSystem = BrakeSystemBuilder::new()
        .change_emergency_brake(Box::new(PanicBrake))
        .finalize();

    for _ in 0..10 {
        bs.stop();
    }
}
