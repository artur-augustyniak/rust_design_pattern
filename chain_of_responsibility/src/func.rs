pub fn run() {
    println!("-------------------- {} --------------------", file!());
    let local_request = "dbus";
    let remote_request = "http";
    let malformed_request = "@@@";


    let undefined_processor = Box::new(|req: &str| {
        println!("undefined_processor::{}", req);
    });


    let remote_processor = |clbk: Box<Fn(&str) -> ()>| {
        Box::new(move |req: &str| {
            if "http" == req {
                println!("remote_processor::{}", req);
            } else {
                clbk(req);
            }
        })
    };


    let local_processor = |clbk: Box<Fn(&str) -> ()>| {
        Box::new(move |req: &str| {
            if "dbus" == req {
                println!("local_processor::{}", req);
            } else {
                clbk(req);
            }
        })
    };


    let chain_of_responsibility = remote_processor(
        local_processor(
            undefined_processor
        )
    );

    chain_of_responsibility(remote_request);
    chain_of_responsibility(local_request);
    chain_of_responsibility(malformed_request);
}
