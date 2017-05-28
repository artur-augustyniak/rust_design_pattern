trait Bag: std::fmt::Debug {
    fn add(&mut self, e: i32);
}

#[derive(Debug)]
struct BagContainer {
    content: Vec<i32>
}


#[derive(Debug)]
struct SumContainer {
    content: i32
}


impl Bag for BagContainer {
    fn add(&mut self, e: i32) {
        self.content.push(e);
    }
}


impl Bag for SumContainer {
    fn add(&mut self, e: i32) {
        self.content += e;
    }
}


fn handle_trait_obj(to: &mut Bag) {
    to.add(1);
    to.add(2);
    to.add(3);
    println!("&mut Bag {:?}", to);
}


fn main() {
    let mut bc = BagContainer { content: vec!() };
    handle_trait_obj(&mut bc);
    println!("BagContainer {:?}", bc);

    let mut sc = SumContainer { content: 0 };
    handle_trait_obj(&mut sc);
    println!("SumContainer {:?}", sc);
}