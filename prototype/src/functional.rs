trait Train {
    fn get_name(&self) -> String;
    fn clone(&self) -> Box<Train>;
}

struct TrainProto {
    name: String,
}

struct OtherProto {
    name: String,
}

impl Train for TrainProto {
    fn get_name(&self) -> String {
        self.name.clone()
    }

    fn clone(&self) -> Box<Train> {
        Box::new(TrainProto { name: self.name.clone() })
    }
}


impl Train for OtherProto {
    fn get_name(&self) -> String {
        self.name.clone()
    }

    fn clone(&self) -> Box<Train> {
        Box::new(TrainProto { name: self.name.clone() })
    }
}


pub fn run() {
    println!("-------------------- {} --------------------", file!());
    let protos: Vec<Box<Train>> = vec![
        Box::new(TrainProto { name: "Military train".to_string() }),
        Box::new(TrainProto { name: "Civilian train".to_string() }),
        Box::new(OtherProto { name: "Other kind of train".to_string() })
    ];


    let clones: Vec<Box<Train>> = protos.into_iter()
        .map(|p| { p.clone() }).collect();


    for t in clones {
        println!("clone plyd: {}", t.get_name());
        println!("clone addr: {:p}", Box::into_raw(t) as *const TrainProto);
    }
}