#[derive(Debug)]
struct Counter {
    count: usize,
}

impl Counter {
    fn new() -> Counter {
        Counter { count: 0 }
    }
}

impl Iterator for Counter {
    // we will be counting with usize
    type Item = usize;

    // next() is the only required method
    fn next(&mut self) -> Option<usize> {
        self.count += 1;
        // check to see if we've finished counting or not.
        if self.count < 6 {
            Some(self.count)
        } else {
            None
        }
    }
}

//iter(), which iterates over &T.
//iter_mut(), which iterates over &mut T.
//Default from trait
//into_iter(), which iterates over T.

pub fn run() {
    println!("-------------------- {} --------------------", file!());
    let counter = Counter::new();
    println!("Size hint {:?}", counter.size_hint());

    println!("Iterator base struct: {:?}", &counter);

    //for x in counter.into_iter() {
    for x in counter {
        println!("Custom iterator returned: {}", x);
    }
    //counter consumed/moved
}