use std::cell::RefCell;

trait Operation {
    fn touch(&self) -> ();
}

#[derive(Debug)]
struct BigBlob {
    payload: String,
}

impl BigBlob {
    fn new() -> Self {
        println!("BigBlob created");
        BigBlob { payload: "Me be the BigBlob!".to_string() }
    }
}


impl Operation for BigBlob {
    fn touch(&self) -> () {
        println!("BigBlob {:?} touched", self);
    }
}


struct BigBlobProxy {
    target: RefCell<Option<Box<Operation>>>,
}

impl BigBlobProxy {
    fn new() -> Self {
        println!("BigBlobProxy created");
        BigBlobProxy { target: RefCell::new(None) }
    }
}


impl Operation for BigBlobProxy {
    fn touch(&self) -> () {
        {
            let base = self.target.borrow();
            if base.is_some() {
                println!("Performing proxied touch");
                base.as_ref().unwrap().touch();
                return;
            }
        }
        let bb = BigBlob::new();
        bb.touch();
        *self.target.borrow_mut() = Some(Box::new(bb));
    }
}


//Here's a step-by-step guide to getting from your Java example to some decent Clojure:
//
//recognize that your singleton is just global, mutable state.
//The singleton may work fine but it's not necessary.
//So we've moved from Java: singleton -> Java: global, mutable state
//
//refactor your Java example to use local mutable state instead of global mutable state,
//by passing the map to the methods that modify it.
//So let's move from Java: global, mutable state -> Java: local, mutable state
//
//*now, instead of destructively updating the map every time, find/write a Java library
//(such as the one the Clojure implementation uses) that does
//not mutate when adding/removing key/value pairs to/from maps.
//Remember to return values that have been "updated", otherwise,
//the changes won't be visible to other code.
//So we've just moved from Java: local, mutable state -> Java: local, immutable state
//
//at this point, you have an FP solution, but it's coded in Java.
//Your initial Clojure solution could end up as a nearly 1:1 translation,
//but as you learn more Clojure you'll figure out how to take advantage of its strengths and improve
//and shorten the code. Plus, you might learn some cool patterns for dealing with "mutable"
//state in a purely functional way. So it's up to you to make the leap from Java: local,
//immutable state -> Clojure: local, immutable state

//From point 3 above: one of the major points of FP is "same thing in, same thing out".
//Mutable state totally destroys this concept, and with it, the advantages of pure code.
//Clojure doesn't force you to be pure, but it certainly makes it easy to do so.
//So if you want to learn how to write good Clojure code, you'll have to learn how to avoid (most)
//mutable state at some point.

//here  interior mutability makes the day
pub fn run() {
    println!("-------------------- {} --------------------", file!());
    //virtual proxy
    let r = BigBlobProxy::new();
    r.touch();
    r.touch();
    r.touch();
}