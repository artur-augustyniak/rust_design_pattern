extern crate rand;

use self::rand::Rng;

trait SpeedController {
    fn stop(&self);
}

struct EmergencyBrake;

impl SpeedController for EmergencyBrake {
    fn stop(&self) {
        println!("stop using EMERGENCY brake...");
    }
}


struct RegularBrake<'a> {
    emergency_brake: &'a SpeedController
}

impl<'a> SpeedController for RegularBrake<'a> {
    fn stop(&self) {
        let mut rng = rand::thread_rng();
        if rng.gen() {
            println!("stop using regular brake...");
        } else {
            self.emergency_brake.stop();
        }
    }
}

struct BrakeSystem<'a> {
    current_brake: &'a SpeedController
}


impl<'a> SpeedController for BrakeSystem<'a> {
    fn stop(&self) {
        self.current_brake.stop();
    }
}


pub fn run() {
    let e = EmergencyBrake;
    let r = RegularBrake { emergency_brake: &e };
    let bs = BrakeSystem { current_brake: &r };
    for _ in 0..10 {
        bs.stop();
    }
}
