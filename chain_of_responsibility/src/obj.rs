#[derive(Debug)]
enum Req {
    Local,
    Remote,
    Distorted,
    Unknown
}


trait Process {
    fn handle(&self, Req);
}


struct LocalHandler {
    next: Option<Box<Process>>,
}


impl Process for LocalHandler {
    fn handle(&self, r: Req) {
        if let Req::Local = r {
            println!("local processing :: {:?}", r);
        } else {
            if let Some(ref h) = self.next {
                h.handle(r);
            } else {
                println!("dropping processing :: {:?}", r);
            }
        }
    }
}


struct RemoteHandler {
    next: Option<Box<Process>>,
}


impl Process for RemoteHandler {
    fn handle(&self, r: Req) {
        if let Req::Remote = r {
            println!("remote processing :: {:?}", r);
        } else {
            if let Some(ref h) = self.next {
                h.handle(r);
            } else {
                println!("dropping processing :: {:?}", r);
            }
        }
    }
}

struct NullHandler;


impl Process for NullHandler {
    fn handle(&self, r: Req) {
        println!("distorted processing :: {:?} ", r);
    }
}


pub fn run() {
    println!("-------------------- {} --------------------", file!());
    let local_handler = LocalHandler { next: None };
    local_handler.handle(Req::Remote);
    println!("-------------------- {} --------------------", "CHAIN");
    let chain = RemoteHandler { next: Some(Box::new(LocalHandler { next: Some(Box::new(NullHandler)) })) };
    chain.handle(Req::Local);
    chain.handle(Req::Remote);
    chain.handle(Req::Distorted);
    chain.handle(Req::Unknown);

    println!("-------------------- {} --------------------", "NON TERM CHAIN");
    let non_term_chain = RemoteHandler { next: Some(Box::new(LocalHandler { next: None })) };
    non_term_chain.handle(Req::Local);
    non_term_chain.handle(Req::Remote);
    non_term_chain.handle(Req::Distorted);
    non_term_chain.handle(Req::Unknown);

    //RemoteHandler { next: NullHandler }
}