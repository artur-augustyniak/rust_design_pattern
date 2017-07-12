pub mod oo {
    use std::cell::RefCell;
    use std::fmt::Debug;

    #[derive(Debug)]
    struct Receiver;

    impl Receiver {
        fn on(&self) {
            println!("{:?} - ON", self);
        }
        fn off(&self) {
            println!("{:?} - OFF", self);
        }
    }

    trait Action: Debug {
        fn execute(&self);
    }

    #[derive(Debug)]
    enum Command<'a> {
        On(&'a Receiver),
        Off(&'a Receiver)
    }

    impl<'a> Action for Command<'a> {
        fn execute(&self) {
            match *self {
                Command::On(r) => { r.on() }
                Command::Off(r) => { r.off() }
            }
        }
    }


    struct Invoker<'a> {
        command_history: RefCell<Vec<&'a Action>>
    }

    impl<'a> Invoker<'a> {
        fn memorize_and_execute(&self, c: &'a Action) -> &Self {
            let mut hist = self.command_history.borrow_mut();
            hist.push(c);
            c.execute();
            &self
        }

        fn print_hist(&self) {
            println!("-|| ACTION LOG ||-");
            for c in self.command_history.borrow().iter() {
                println!("{:?}", c);
            }
            println!("-|| END ||-");
        }
    }


    pub fn run_oo() {
        println!("-------------------- {} --------------------", "RUN OO");

        let receiver = Receiver;
        let enable_comm = Command::On(&receiver);
        let disable_comm = Command::Off(&receiver);

        let invoker = Invoker {
            command_history: RefCell::new(Vec::new())
        };


        invoker.memorize_and_execute(&enable_comm)
            .memorize_and_execute(&disable_comm)
            .memorize_and_execute(&enable_comm)
            .memorize_and_execute(&enable_comm);
        invoker.print_hist();
        invoker.memorize_and_execute(&enable_comm);
    }
}

pub mod func {
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
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {}
}
