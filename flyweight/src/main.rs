extern crate rand;


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
}

#[derive(Debug)]
struct EmployeeData<'a> {
    first_name: String,
    last_name: String,
    mail_footer: &'a Footer
}


struct FooterProvider {
    cache: HashMap<FootType, Footer>
}

impl<'a> FooterProvider {
    fn new() -> FooterProvider {
        FooterProvider { cache: HashMap::new() }
    }

    fn make_footer(&mut self, t: FootType) -> &Footer {
        match t {
            FootType::Official => {
                self.cache.entry(t).or_insert(Footer { data: "officials".to_string() })
            }
            FootType::Private => {
                self.cache.entry(t).or_insert(Footer { data: "private".to_string() })
            }
        }
    }
}


fn main() {
    let mut ft_prv = FooterProvider::new();
    let  employee_vector: Vec<&Footer> = vec![];

    for i in 0..100 {
        let name = rand_string();
        let last_name = rand_string();
        let footer;

            if i % 2 == 0 {
                footer = ft_prv.make_footer(FootType::Private);
            } else {
                footer = ft_prv.make_footer(FootType::Official);
            }

        println!("{:?}", footer);
        footer.dump_content_address();

//        let ref e = EmployeeData { first_name: name, last_name: last_name, mail_footer: footer };
//        employee_vector.push(footer)
    }


    //    for e in employee_vector {
    //        println!("{:?}", e);
    ////        println!("{:p}", e.first_name.as_ptr());
    //        e.mail_footer.dump_content_address();
    //    }
}
