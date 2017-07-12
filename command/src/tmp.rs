#[derive(Debug)]
enum Command {
    Enable,
    Disable
}

#[derive(Debug)]
enum Receiver {
    ON,
    OFF
}

impl Receiver {
    fn exec(self, c: Command) -> Receiver {
        match c {
            Command::Enable => {
                println!("{:?}(Receiver({:?}))", &c, &self);
                Receiver::ON
            }
            Command::Disable => {
                println!("{:?}(Receiver({:?}))", &c, &self);
                Receiver::OFF
            }
        }
    }
}


pub fn run_func() {
    println!("-------------------- {} --------------------", "RUN FUNC");
    let initial_state = Receiver::ON;
    println!("Receiver - {:?}", &initial_state);
    let commands = vec![Command::Enable, Command::Disable, Command::Disable];
    let invoker = |r: Receiver, c: Command| r.exec(c);
    println!("-|| ACTION LOG ||-");
    let final_state = commands.into_iter().fold(initial_state, invoker);
    println!("-|| END ||-");
    println!("Receiver - {:?}", final_state);
}