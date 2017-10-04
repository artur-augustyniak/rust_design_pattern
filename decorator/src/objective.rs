use std::fmt::Debug;

trait Movable: Debug {
    fn ride(&self);
}

trait MovableDecorator: Movable {
    fn get_decorated(&self) -> &Movable;
}

#[derive(Debug)]
struct Vehicle {
    handbrake_enabled: bool
}

#[derive(Debug)]
struct PaintedVehicle<'a> {
    vehicle: &'a Movable
}

#[derive(Debug)]
struct IlluminatedVehicle<'a> {
    vehicle: &'a Movable
}


impl Movable for Vehicle {
    fn ride(&self) {
        println!("Roll on: {:?}", self);
    }
}

impl<'a> Movable for PaintedVehicle<'a> {
    fn ride(&self) {
        self.vehicle.ride();
        println!("Painted: {:?}", self);
    }
}

impl<'a> Movable for IlluminatedVehicle<'a> {
    fn ride(&self) {
        self.vehicle.ride();
        println!("Illuminated: {:?}", self);
    }
}

impl<'a> MovableDecorator for PaintedVehicle<'a> {
    fn get_decorated(&self) -> &Movable {
        self.vehicle
    }
}

impl<'a> MovableDecorator for IlluminatedVehicle<'a> {
    fn get_decorated(&self) -> &Movable {
        self.vehicle
    }
}


pub fn run() {
    println!("-------------------- {} --------------------", file!());
    let v = Vehicle { handbrake_enabled: true };
    let pv = PaintedVehicle { vehicle: &v };
    let iv = IlluminatedVehicle { vehicle: &v };
    let ipv = IlluminatedVehicle { vehicle: &pv };
    println!("####### V #########");
    v.ride();
    println!("####### PV #########");
    pv.ride();
    println!("####### IV #########");
    iv.ride();
    println!("####### IPV #########");
    ipv.ride();
    println!("####### IPV ONION1 #########");
    let ipv = ipv.get_decorated();
    pv.ride();
    println!("####### PV ONION2 #########");
    let v = pv.get_decorated();
    v.ride();
}