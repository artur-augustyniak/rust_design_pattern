use std::fmt::Display;

const TORQUE_START_THRESHOLD: u32 = 1000;

#[derive(Debug)]
pub enum Vehicle {
    Running,
    Stopped
}


fn move_vehicle<T, F>(rpm: T, torque_eq: F) -> Vehicle where F: Fn(T) -> u32, T: Display + Copy {
    let curr_torque = torque_eq(rpm);
    println!("Engine torque is: {}", rpm);
    let axles_rpm: f64 = curr_torque as f64 / 150.0;
    println!("Driven axles rpm: {}", axles_rpm);
    if curr_torque > TORQUE_START_THRESHOLD {
        Vehicle::Running
    } else {
        Vehicle::Stopped
    }
}


fn dump_state(v: &Vehicle) {
    println!("{:?}", v);
    println!("-------------------- dump_vehicle_state_end --------------------");
}

fn make_movement_energy_operator_cps(clbk: Box<Fn(&Vehicle) -> ()>) -> Box<Fn(u32)>
{
    Box::new(move |energy: u32| {
        let torque_eq = |rpm: u32| -> u32 { rpm * 1000 };
        let v = move_vehicle(energy, torque_eq);
        clbk(&v);
    })
}


fn move_over(energies: Vec<u32>, clbk: Box<Fn(&Vehicle) -> ()>) -> Box<Iterator<Item=Vehicle>>
{
    Box::new(
        energies.into_iter().map(move |energy: u32| {
            let torque_eq = |rpm: u32| -> u32 { rpm * 1000 };
            let v = move_vehicle(energy, torque_eq);
            clbk(&v);
            v
        })
    )
}


pub fn run_fn() {
    println!("-------------------- {} --------------------", file!());
    let m_op = make_movement_energy_operator_cps(Box::new(dump_state));
    m_op(12);
    m_op(12);
    m_op(0);

    let energies: Vec<u32> = vec![12, 12, 0];
    let energy_states: Vec<Vehicle> = move_over(energies, Box::new(dump_state)).collect();
    println!("{:?}", energy_states);

    let energies: Vec<u32> = vec![12, 12, 0];
    let energy_ops = energies.into_iter().map(move |energy: u32| {
        let torque_eq = |rpm: u32| -> u32 { rpm * 1000 };
        let v = move_vehicle(energy, torque_eq);
        dump_state(&v);
    });
    println!("{:?}", energy_ops);

    for x in energy_ops {
        println!("{:?}", x);
    }

    let boxed_fn = Box::new(|i| i * 2);
    for n in (0..10).map(*boxed_fn) {
        println!("{}", n);
    }
}