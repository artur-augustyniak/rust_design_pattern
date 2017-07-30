use rand::random;
use std::collections::HashMap;
use std::collections::hash_map::Entry::{Occupied, Vacant};

fn rand_string() -> String {
    (0..4).map(|_| (0x20u8 + (random::<f32>() * 96.0) as u8) as char).collect()
}


#[derive(Debug, )]
struct Footer {
    data: String
}

impl Footer {
    fn dump_content_address(&self) {
        println!("{:p}", self.data.as_ptr());
    }
}


#[derive(Eq, Hash, PartialEq)]
enum FootType {
    Official,
    Private,
}

#[derive(Debug)]
struct EmployeeData<'a> {
    first_name: String,
    last_name: String,
    mail_footer: &'a Footer
}


pub fn run() {
    println!("-------------------- {} --------------------", file!());
    let a = Footer { data: "Official".to_string() };
    let b = Footer { data: "Private".to_string() };

    let mut cache: HashMap<FootType, &Footer> = HashMap::new();
    let mut employee_vector: Vec<EmployeeData> = vec![];


    for i in 0..100 {
        let name = rand_string();
        let last_name = rand_string();
        let footer;
        if i % 2 == 0 {
            footer = match cache.entry(FootType::Official) {
                Vacant(entry) => entry.insert(&a),
                Occupied(entry) => entry.into_mut(),
            };
        } else {
            footer = match cache.entry(FootType::Private) {
                Vacant(entry) => entry.insert(&b),
                Occupied(entry) => entry.into_mut(),
            };
        };


        let e = EmployeeData { first_name: name, last_name: last_name, mail_footer: footer };
        employee_vector.push(e)
    }


    for e in employee_vector {
        println!("{:?}", e);
        println!("{:p}", e.first_name.as_ptr());
        e.mail_footer.dump_content_address();
    }
}
