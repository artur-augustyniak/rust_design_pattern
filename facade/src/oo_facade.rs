pub ( in oo_facade) mod complex_parts {
    const TORQUE_MUL: u32 = 1000;

    pub use std::cell::Cell;

    #[derive(Debug, Clone, Copy)]
    pub enum Throttle {
        Fw,
        N
    }

    #[derive(Debug)]
    pub struct Engine {
        rpm: Cell<u32>
    }


    impl Engine {
        pub fn new() -> Engine {
            Engine { rpm: Cell::new(0) }
        }

        pub fn set_rpm(&self, rpm: u32) {
            self.rpm.replace(rpm);
        }

        pub fn get_torque(&self) -> u32 {
            let t = self.rpm.get() * TORQUE_MUL;
            println!("Engine torque is: {}", t);
            t
        }
    }

    #[derive(Debug)]
    pub struct DrivenAxles;


    impl DrivenAxles {
        pub fn set_rotations(&self, torque: u32) {
            let rpm: f64 = torque as f64 / 150.0;
            println!("Driven axles rpm: {}", rpm);
        }
    }
}

use self::complex_parts::*;

#[derive(Debug)]
struct VehicleFacade {
    throttler: Cell<Throttle>,
    engine: Engine,
    axles: DrivenAxles
}


impl VehicleFacade {
    fn ride(&self, power: u32) {
        self.throttler.replace(Throttle::Fw);
        self.engine.set_rpm(power);
        self.axles.set_rotations(self.engine.get_torque());
    }


    fn stop(&self) {
        self.throttler.replace(Throttle::N);
        self.engine.set_rpm(0);
        self.axles.set_rotations(self.engine.get_torque());
    }

    fn dump_state(&self) {
        println!("{:?}", self);
        println!("-------------------- dump_vehicle_state_end --------------------");
    }
}


pub fn run_oo() {
    println!("-------------------- {} --------------------", file!());

    let vf = VehicleFacade {
        throttler: Cell::new(Throttle::N),
        engine: Engine::new(),
        axles: DrivenAxles
    };

    vf.ride(234);
    vf.dump_state();
    vf.ride(235);
    vf.dump_state();
    vf.stop();
    vf.dump_state();
}