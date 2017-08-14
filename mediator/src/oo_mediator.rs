use std::cell::Cell;

trait Mediator {
    fn request_close_barriers(&self);

    fn request_open_barriers(&self);
}

trait Participant {
    fn execute(&self);
}


struct CarParticipant<'a> {
    med: Cell<Option<&'a Mediator>>
}

impl<'a> CarParticipant<'a> {
    fn go(&self) {
        println!("Car arrives!");
    }
}


impl<'a> Participant for CarParticipant<'a> {
    fn execute(&self) {
        if let Some(mediator) = self.med.get() {
            mediator.request_open_barriers();
        }
    }
}


struct TrainParticipant<'a> {
    med: Cell<Option<&'a Mediator>>
}


impl<'a> TrainParticipant<'a> {
    fn go(&self) {
        println!("Train passing!");
    }
}


impl<'a> Participant for TrainParticipant<'a> {
    fn execute(&self) {
        if let Some(mediator) = self.med.get() {
            mediator.request_close_barriers();
        }
    }
}

struct TrafficMediator<'a> {
    cp: CarParticipant<'a>,
    tp: TrainParticipant<'a>
}


impl<'a> Mediator for TrafficMediator<'a> {
    fn request_close_barriers(&self) {
        println!("Mediator closing barrier and moving train!");
        self.tp.go();
    }

    fn request_open_barriers(&self) {
        println!("Mediator opening barrier and moving car!");
        self.cp.go();
    }
}


pub fn run() {
    println!("-------------------- {} --------------------", file!());
    let dispatcher = TrafficMediator {
        cp: CarParticipant { med: Cell::new(None) },
        tp: TrainParticipant { med: Cell::new(None) }
    };
    dispatcher.cp.med.set(Some(&dispatcher));
    dispatcher.tp.med.set(Some(&dispatcher));
    dispatcher.cp.execute();
    dispatcher.tp.execute();
}