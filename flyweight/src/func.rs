use rand::random;
use std::collections::HashMap;


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
    AddedLater,
}

#[derive(Debug)]
struct EmployeeData<'a> {
    first_name: String,
    last_name: String,
    mail_footer: &'a Footer
}


pub fn run() {
    println!("-------------------- {} --------------------", file!());
    let a = FootType::Official;
    let b = FootType::Private;
    let ref c = FootType::AddedLater;
    let ref stub_footer = Footer { data: "Stub footer".to_string() };

    let mut footer_cache = HashMap::new();
    footer_cache.insert(FootType::Official, Footer { data: "Official".to_string() });
    footer_cache.insert(FootType::Private, Footer { data: "Private".to_string() });


    let employee_names: Vec<_> = (0..10).map(
        |id| EmployeeData {
            first_name: rand_string(),
            last_name: rand_string(),
            mail_footer: match footer_cache.get(
                if id % 2 == 0 { &a } else if id % 9 == 0 { c } else { &b }) {
                Some(cached_footer) => cached_footer,
                None => stub_footer
            }
        }).collect();


    for e in employee_names.iter() {
        println!("{:?}", e);
        println!("{:p}", e.first_name.as_ptr());
        e.mail_footer.dump_content_address();
    }


    println!("{:?}", employee_names);
}
