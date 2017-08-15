#[derive(Debug)]
enum Participant {
    CAR,
    TRAIN
}


#[derive(Debug)]
enum ParticipantCommand {
    GO,
    STOP
}

fn move_barriers_mediator(p: &Participant, c: ParticipantCommand) {
    println!("Mediation!");
    match *p {
        Participant::CAR => {
            match c {
                ParticipantCommand::GO => {
                    println!("Opening barrier {:?} {:?}", c, p);
                }
                ParticipantCommand::STOP => {
                    println!("Closing barrier {:?} {:?}", c, p);
                }
            }
        }
        Participant::TRAIN => {
            match c {
                ParticipantCommand::GO => {
                    println!("Closing barrier {:?} {:?}", c, p);
                }
                ParticipantCommand::STOP => {
                    println!("Opening barrier {:?} {:?}", c, p);
                }
            }
        }
    }
}


fn participant<F>(p: Participant, mediator: F) -> Box<Fn(ParticipantCommand)>
    where F: Fn(&Participant, ParticipantCommand) + 'static {
    Box::new(
        move |action: ParticipantCommand| {
            match action {
                ParticipantCommand::GO => {
                    mediator(&p, action);
                    println!("{:?} passing!", p);
                }
                ParticipantCommand::STOP => {
                    mediator(&p, action);
                    println!("{:?} waiting!", p);
                }
            }
        }
    )
}

pub fn run() {
    println!("-------------------- {} --------------------", file!());
    let car = participant(Participant::CAR, move_barriers_mediator);
    let train = participant(Participant::TRAIN, move_barriers_mediator);
    car(ParticipantCommand::GO);
    train(ParticipantCommand::GO);
    car(ParticipantCommand::STOP);
    train(ParticipantCommand::STOP);
}